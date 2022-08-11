//! Attention is a construct that occurs in the [text][] content type.
//!
//! How attention parses is too complex to explain in BNF.
//! Essentially, one or more of `*` or `_` form attention sequences.
//! Depending on the code before and after a sequence, it can open or close
//! attention.
//! When everything is parsed, we find each sequence that can close, and a
//! corresponding sequence that can open which uses the same marker.
//! If both sequences have two or more markers, strong is formed.
//! Otherwise emphasis is formed.
//!
//! Attention sequences do not, on their own, relate to anything in HTML.
//! When matched with another sequence, and two markers can be “taken” from
//! them, they together relate to the `<strong>` element in HTML.
//! When one marker can be taken, they relate to the `<em>` element.
//! See [*§ 4.5.2 The `em` element*][html-em] and
//! [*§ 4.5.3 The `strong` element*][html-strong] in the HTML spec for more
//! info.
//!
//! It is recommended to use asterisks for attention when writing markdown.
//!
//! There are some small differences in whether sequences can open and/or close
//! based on whether they are formed with asterisks or underscores.
//! Because underscores also frequently occur in natural language inside words,
//! while asterisks typically never do, `CommonMark` prohobits underscore
//! sequences from opening or closing when *inside* a word.
//!
//! Because asterisks can be used to form the most markdown constructs, using
//! them has the added benefit of making it easier to gloss over markdown: you
//! can look for asterisks to find syntax while not worrying about other
//! characters.
//!
//! ## Tokens
//!
//! *   [`Emphasis`][Token::Emphasis]
//! *   [`EmphasisSequence`][Token::EmphasisSequence]
//! *   [`EmphasisText`][Token::EmphasisText]
//! *   [`Strong`][Token::Strong]
//! *   [`StrongSequence`][Token::StrongSequence]
//! *   [`StrongText`][Token::StrongText]
//!
//! > 👉 **Note**: while parsing, [`AttentionSequence`][Token::AttentionSequence]
//! > is used, which is later compiled away.
//!
//! ## References
//!
//! *   [`attention.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-core-commonmark/dev/lib/attention.js)
//! *   [*§ 6.2 Emphasis and strong emphasis* in `CommonMark`](https://spec.commonmark.org/0.30/#emphasis-and-strong-emphasis)
//!
//! [text]: crate::content::text
//! [html-em]: https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-em-element
//! [html-strong]: https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-strong-element

use crate::event::{Event, Kind, Name, Point};
use crate::resolve::Name as ResolveName;
use crate::state::{Name as StateName, State};
use crate::tokenizer::Tokenizer;
use crate::unicode::PUNCTUATION;
use crate::util::slice::Slice;

/// Character code kinds.
#[derive(Debug, PartialEq)]
enum GroupKind {
    /// Whitespace.
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | **a_b_ c**.
    ///    ^      ^    ^
    /// ```
    Whitespace,
    /// Punctuation.
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | **a_b_ c**.
    ///     ^^ ^ ^    ^
    /// ```
    Punctuation,
    /// Everything else.
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | **a_b_ c**.
    ///       ^ ^  ^
    /// ```
    Other,
}

/// Attentention sequence that we can take markers from.
#[derive(Debug)]
struct Sequence {
    /// Marker as a byte (`u8`) used in this sequence.
    marker: u8,
    /// The depth in events where this sequence resides.
    balance: usize,
    /// The index into events where this sequence’s `Enter` currently resides.
    event_index: usize,
    /// The (shifted) point where this sequence starts.
    start_point: Point,
    /// The (shifted) point where this sequence end.
    end_point: Point,
    /// The number of markers we can still use.
    size: usize,
    /// Whether this sequence can open attention.
    open: bool,
    /// Whether this sequence can close attention.
    close: bool,
}

/// Before a sequence.
///
/// ```markdown
/// > | **
///     ^
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'*' | b'_') if tokenizer.parse_state.constructs.attention => {
            tokenizer.tokenize_state.marker = tokenizer.current.unwrap();
            tokenizer.enter(Name::AttentionSequence);
            State::Retry(StateName::AttentionInside)
        }
        _ => State::Nok,
    }
}

/// In a sequence.
///
/// ```markdown
/// > | **
///     ^^
/// ```
pub fn inside(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'*' | b'_') if tokenizer.current.unwrap() == tokenizer.tokenize_state.marker => {
            tokenizer.consume();
            State::Next(StateName::AttentionInside)
        }
        _ => {
            tokenizer.exit(Name::AttentionSequence);
            tokenizer.register_resolver(ResolveName::Attention);
            tokenizer.tokenize_state.marker = b'\0';
            State::Ok
        }
    }
}

/// Resolve attention sequences.
#[allow(clippy::too_many_lines)]
pub fn resolve(tokenizer: &mut Tokenizer) {
    let mut start = 0;
    let mut balance = 0;
    let mut sequences = vec![];

    // Find sequences of sequences and information about them.
    while start < tokenizer.events.len() {
        let enter = &tokenizer.events[start];

        if enter.kind == Kind::Enter {
            balance += 1;

            if enter.name == Name::AttentionSequence {
                let end = start + 1;
                let exit = &tokenizer.events[end];

                let before_end = enter.point.index;
                let before_start = if before_end < 4 { 0 } else { before_end - 4 };
                let string_before =
                    String::from_utf8_lossy(&tokenizer.parse_state.bytes[before_start..before_end]);
                let char_before = string_before.chars().last();

                let after_start = exit.point.index;
                let after_end = if after_start + 4 > tokenizer.parse_state.bytes.len() {
                    tokenizer.parse_state.bytes.len()
                } else {
                    after_start + 4
                };
                let string_after =
                    String::from_utf8_lossy(&tokenizer.parse_state.bytes[after_start..after_end]);
                let char_after = string_after.chars().next();

                let marker = Slice::from_point(tokenizer.parse_state.bytes, &enter.point)
                    .head()
                    .unwrap();
                let before = classify_character(char_before);
                let after = classify_character(char_after);
                let open = after == GroupKind::Other
                    || (after == GroupKind::Punctuation && before != GroupKind::Other);
                // To do: GFM strikethrough?
                // || attentionMarkers.includes(code)
                let close = before == GroupKind::Other
                    || (before == GroupKind::Punctuation && after != GroupKind::Other);
                // To do: GFM strikethrough?
                // || attentionMarkers.includes(previous)

                sequences.push(Sequence {
                    event_index: start,
                    balance,
                    start_point: enter.point.clone(),
                    end_point: exit.point.clone(),
                    size: exit.point.index - enter.point.index,
                    open: if marker == b'*' {
                        open
                    } else {
                        open && (before != GroupKind::Other || !close)
                    },
                    close: if marker == b'*' {
                        close
                    } else {
                        close && (after != GroupKind::Other || !open)
                    },
                    marker,
                });
            }
        } else {
            balance -= 1;
        }

        start += 1;
    }

    // Walk through sequences and match them.
    let mut close = 0;

    while close < sequences.len() {
        let sequence_close = &sequences[close];
        let mut next_index = close + 1;

        // Find a sequence that can close.
        if sequence_close.close {
            let mut open = close;

            // Now walk back to find an opener.
            while open > 0 {
                open -= 1;

                let sequence_open = &sequences[open];

                // We found a sequence that can open the closer we found.
                if sequence_open.open
                    && sequence_close.marker == sequence_open.marker
                    && sequence_close.balance == sequence_open.balance
                {
                    // If the opening can close or the closing can open,
                    // and the close size *is not* a multiple of three,
                    // but the sum of the opening and closing size *is*
                    // multiple of three, then **don’t** match.
                    if (sequence_open.close || sequence_close.open)
                        && sequence_close.size % 3 != 0
                        && (sequence_open.size + sequence_close.size) % 3 == 0
                    {
                        continue;
                    }

                    // We’ve found a match!

                    // Number of markers to use from the sequence.
                    let take = if sequence_open.size > 1 && sequence_close.size > 1 {
                        2
                    } else {
                        1
                    };

                    // We’re *on* a closing sequence, with a matching opening
                    // sequence.
                    // Now we make sure that we can’t have misnested attention:
                    //
                    // ```html
                    // <em>a <strong>b</em> c</strong>
                    // ```
                    //
                    // Do that by marking everything between it as no longer
                    // possible to open anything.
                    // Theoretically we could mark non-closing as well, but we
                    // don’t look for closers backwards.
                    let mut between = open + 1;

                    while between < close {
                        sequences[between].open = false;
                        between += 1;
                    }

                    let sequence_close = &mut sequences[close];
                    let close_event_index = sequence_close.event_index;
                    let seq_close_enter = sequence_close.start_point.clone();
                    // No need to worry about `VS`, because sequences are only actual characters.
                    sequence_close.size -= take;
                    sequence_close.start_point.column += take;
                    sequence_close.start_point.index += take;
                    let seq_close_exit = sequence_close.start_point.clone();

                    // Stay on this closing sequence for the next iteration: it
                    // might close more things.
                    next_index -= 1;

                    // Remove closing sequence if fully used.
                    if sequence_close.size == 0 {
                        sequences.remove(close);
                        tokenizer.map.add(close_event_index, 2, vec![]);
                    } else {
                        // Shift remaining closing sequence forward.
                        // Do it here because a sequence can open and close different
                        // other sequences, and the remainder can be on any side or
                        // somewhere in the middle.
                        let mut enter = &mut tokenizer.events[close_event_index];
                        enter.point = seq_close_exit.clone();
                    }

                    let sequence_open = &mut sequences[open];
                    let open_event_index = sequence_open.event_index;
                    let seq_open_exit = sequence_open.end_point.clone();
                    // No need to worry about `VS`, because sequences are only actual characters.
                    sequence_open.size -= take;
                    sequence_open.end_point.column -= take;
                    sequence_open.end_point.index -= take;
                    let seq_open_enter = sequence_open.end_point.clone();

                    // Remove opening sequence if fully used.
                    if sequence_open.size == 0 {
                        sequences.remove(open);
                        tokenizer.map.add(open_event_index, 2, vec![]);
                        next_index -= 1;
                    } else {
                        // Shift remaining opening sequence backwards.
                        // See note above for why that happens here.
                        let mut exit = &mut tokenizer.events[open_event_index + 1];
                        exit.point = seq_open_enter.clone();
                    }

                    // Opening.
                    tokenizer.map.add_before(
                        // Add after the current sequence (it might remain).
                        open_event_index + 2,
                        0,
                        vec![
                            Event {
                                kind: Kind::Enter,
                                name: if take == 1 {
                                    Name::Emphasis
                                } else {
                                    Name::Strong
                                },
                                point: seq_open_enter.clone(),
                                link: None,
                            },
                            Event {
                                kind: Kind::Enter,
                                name: if take == 1 {
                                    Name::EmphasisSequence
                                } else {
                                    Name::StrongSequence
                                },
                                point: seq_open_enter.clone(),
                                link: None,
                            },
                            Event {
                                kind: Kind::Exit,
                                name: if take == 1 {
                                    Name::EmphasisSequence
                                } else {
                                    Name::StrongSequence
                                },
                                point: seq_open_exit.clone(),
                                link: None,
                            },
                            Event {
                                kind: Kind::Enter,
                                name: if take == 1 {
                                    Name::EmphasisText
                                } else {
                                    Name::StrongText
                                },
                                point: seq_open_exit.clone(),
                                link: None,
                            },
                        ],
                    );
                    // Closing.
                    tokenizer.map.add(
                        close_event_index,
                        0,
                        vec![
                            Event {
                                kind: Kind::Exit,
                                name: if take == 1 {
                                    Name::EmphasisText
                                } else {
                                    Name::StrongText
                                },
                                point: seq_close_enter.clone(),
                                link: None,
                            },
                            Event {
                                kind: Kind::Enter,
                                name: if take == 1 {
                                    Name::EmphasisSequence
                                } else {
                                    Name::StrongSequence
                                },
                                point: seq_close_enter.clone(),
                                link: None,
                            },
                            Event {
                                kind: Kind::Exit,
                                name: if take == 1 {
                                    Name::EmphasisSequence
                                } else {
                                    Name::StrongSequence
                                },
                                point: seq_close_exit.clone(),
                                link: None,
                            },
                            Event {
                                kind: Kind::Exit,
                                name: if take == 1 {
                                    Name::Emphasis
                                } else {
                                    Name::Strong
                                },
                                point: seq_close_exit.clone(),
                                link: None,
                            },
                        ],
                    );

                    break;
                }
            }
        }

        close = next_index;
    }

    // Mark remaining sequences as data.
    let mut index = 0;
    while index < sequences.len() {
        let sequence = &sequences[index];
        tokenizer.events[sequence.event_index].name = Name::Data;
        tokenizer.events[sequence.event_index + 1].name = Name::Data;
        index += 1;
    }

    tokenizer.map.consume(&mut tokenizer.events);
}

/// Classify whether a character code represents whitespace, punctuation, or
/// something else.
///
/// Used for attention (emphasis, strong), whose sequences can open or close
/// based on the class of surrounding characters.
///
/// > 👉 **Note** that eof (`None`) is seen as whitespace.
///
/// ## References
///
/// *   [`micromark-util-classify-character` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-util-classify-character/dev/index.js)
fn classify_character(char: Option<char>) -> GroupKind {
    match char {
        // EOF.
        None => GroupKind::Whitespace,
        // Unicode whitespace.
        Some(char) if char.is_whitespace() => GroupKind::Whitespace,
        // Unicode punctuation.
        Some(char) if PUNCTUATION.contains(&char) => GroupKind::Punctuation,
        // Everything else.
        Some(_) => GroupKind::Other,
    }
}
