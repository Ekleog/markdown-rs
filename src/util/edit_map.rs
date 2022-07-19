//! Helpers to deal with several changes in events, batching them together.
//!
//! Preferably, changes should be kept to a minumum.
//! Sometimes, it’s needed to change the list of events, because parsing can be
//! messy, and it helps to expose a cleaner interface of events to the compiler
//! and other users.
//! It can also help to merge many adjacent similar events.
//! And, in other cases, it’s needed to parse subcontent: pass some events
//! through another tokenizer and inject the result.

use crate::tokenizer::Event;

/// Shift `previous` and `next` links according to `jumps`.
///
/// This fixes links in case there are events removed or added between them.
fn shift_links(events: &mut [Event], jumps: &[(usize, isize)]) {
    let map = |before| {
        let mut jump_index = 0;
        let mut jump = 0;

        while jump_index < jumps.len() {
            if jumps[jump_index].0 > before {
                break;
            }

            jump = jumps[jump_index].1;
            jump_index += 1;
        }

        #[allow(clippy::pedantic)]
        let next_i = (before as isize) + jump;
        assert!(next_i >= 0, "cannot shift before `0`");
        #[allow(clippy::pedantic)]
        let next = next_i as usize;
        next
    };

    let mut index = 0;

    while index < events.len() {
        let event = &mut events[index];
        event.previous = event.previous.map(map);
        event.next = event.next.map(map);
        index += 1;
    }
}

/// Make it easy to insert and remove things while being performant and keeping
/// links in check.
#[derive(Debug)]
pub struct EditMap {
    /// Whether this map was consumed already.
    consumed: bool,
    /// Record of changes.
    map: Vec<(usize, usize, Vec<Event>)>,
}

impl EditMap {
    /// Create a new edit map.
    pub fn new() -> EditMap {
        EditMap {
            consumed: false,
            map: vec![],
        }
    }
    /// Create an edit: a remove and/or add at a certain place.
    pub fn add(&mut self, index: usize, remove: usize, add: Vec<Event>) {
        add_impl(self, index, remove, add, false);
    }
    /// Create an edit: but insert `add` before existing additions.
    pub fn add_before(&mut self, index: usize, remove: usize, add: Vec<Event>) {
        add_impl(self, index, remove, add, true);
    }
    /// Done, change the events.
    pub fn consume(&mut self, events: &mut [Event]) -> Vec<Event> {
        self.map
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut next_events: Vec<Event> = vec![];
        let mut start = 0;

        assert!(!self.consumed, "cannot consume after consuming");
        self.consumed = true;

        let mut jumps: Vec<(usize, isize)> = vec![];
        let mut index = 0;
        let mut shift = 0;
        while index < self.map.len() {
            let (at, remove, add) = &self.map[index];

            #[allow(clippy::pedantic)]
            let next = shift + (add.len() as isize) - (*remove as isize);
            shift = next;
            jumps.push((*at, shift));
            index += 1;
        }

        let mut index = 0;

        while index < self.map.len() {
            let at = self.map[index].0;
            let remove = self.map[index].1;
            let mut add = self.map[index].2.drain(..).collect::<Vec<_>>();

            if start < at {
                let append = &mut events[start..at].to_vec();
                shift_links(append, &jumps);
                next_events.append(append);
            }

            if !add.is_empty() {
                let append = &mut add;
                let mut index = 0;

                while index < append.len() {
                    let event = &mut append[index];
                    assert!(event.previous.is_none(), "to do?");
                    assert!(event.next.is_none(), "to do?");
                    index += 1;
                }

                next_events.append(append);
            }

            start = at + remove;
            index += 1;
        }

        if start < events.len() {
            let append = &mut events[start..].to_vec();
            shift_links(append, &jumps);
            next_events.append(append);
        }

        next_events
    }
}

/// Create an edit.
fn add_impl(edit_map: &mut EditMap, at: usize, remove: usize, mut add: Vec<Event>, before: bool) {
    assert!(!edit_map.consumed, "cannot add after consuming");
    let mut index = 0;

    while index < edit_map.map.len() {
        if edit_map.map[index].0 == at {
            edit_map.map[index].1 += remove;

            // To do: these might have to be split into several chunks instead
            // of one, if links in `curr_add` are supported.
            if before {
                add.append(&mut edit_map.map[index].2);
                edit_map.map[index].2 = add;
            } else {
                edit_map.map[index].2.append(&mut add);
            }

            return;
        }

        index += 1;
    }

    edit_map.map.push((at, remove, add));
}
