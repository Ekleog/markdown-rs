//! Label end is a construct that occurs in the [text][] content type.
//!
//! It forms with the following BNF:
//!
//! ```bnf
//! label_end ::= ']' [ resource | reference_full | reference_collapsed ]
//!
//! resource ::= '(' [ whitespace ] destination [ whitespace title ] [ whitespace ] ')'
//! reference_full ::= '[' label ']'
//! reference_collapsed ::= '[' ']'
//!
//! ; See the `destination`, `title`, and `label` constructs for the BNF of
//! ; those parts.
//! ```
//!
//! See [`destination`][destination], [`label`][label], and [`title`][title]
//! for grammar, notes, and recommendations.
//!
//! Label end does not, on its own, relate to anything in HTML.
//! When matched with a [label start (link)][label_start_link], they together
//! relate to the `<a>` element in HTML.
//! See [*§ 4.5.1 The `a` element*][html-a] in the HTML spec for more info.
//! It can also match with [label start (image)][label_start_image], in which
//! case they form an `<img>` element.
//! See [*§ 4.8.3 The `img` element*][html-img] in the HTML spec for more info.
//!
//! In the case of a resource, the destination and title are given directly
//! with the label end.
//! In the case of a reference, this information is provided by a matched
//! [definition][].
//! Full references (`[x][y]`) match to definitions through their explicit,
//! second, label (`y`).
//! Collapsed references (`[x][]`) and shortcut references (`[x]`) match by
//! interpreting the text provided between the first, implicit, label (`x`).
//! To match, the effective label of the reference must be equal to the label
//! of the definition after normalizing with
//! [`normalize_identifier`][normalize_identifier].
//!
//! Importantly, while the label of a full reference *can* include [string][]
//! content, and in case of collapsed and shortcut references even [text][]
//! content, that content is not considered when matching.
//! To illustrate, neither label matches the definition:
//!
//! ```markdown
//! [a&b]: https://example.com
//!
//! [x][a&amp;b], [a\&b][]
//! ```
//!
//! When the resource or reference matches, the destination forms the `href`
//! attribute in case of a [label start (link)][label_start_link], and an
//! `src` attribute otherwise.
//! The title is formed, optionally, on either `<a>` or `<img>`.
//!
//! For info on how to encode characters in URLs, see
//! [`destination`][destination].
//! For info on how characters are encoded as `href` on `<a>` or `src` on
//! `<img>` when compiling, see
//! [`sanitize_uri`][sanitize_uri].
//!
//! In case of a matched [label start (link)][label_start_link], the interpreted
//! content between it and the label end, is placed between the opening and
//! closing tags.
//! Otherwise, the text is also interpreted, but used *without* the resulting
//! tags:
//!
//! ```markdown
//! [a *b* c](#)
//!
//! ![a *b* c](#)
//! ```
//!
//! Yields:
//!
//! ```html
//! <p><a href="#">a <em>b</em> c</a></p>
//! <p><img src="#" alt="a b c" /></p>
//! ```
//!
//! It is possible to use images in links.
//! It’s somewhat possible to have links in images (the text will be used, not
//! the HTML, see above).
//! But it’s not possible to use links in links.
//! The “deepest” link wins.
//! To illustrate:
//!
//! ```markdown
//! a [b [c](#) d](#) e
//! ```
//!
//! Yields:
//!
//! ```html
//! <p>a [b <a href="#">c</a> d](#) e</p>
//! ```
//!
//! This limiation is imposed because links in links is invalid according to
//! HTML.
//! Technically though, in markdown it is still possible to construct them by
//! using an [autolink][] in a link.
//! You definitely should not do that.
//!
//! ## Tokens
//!
//! *   [`Data`][Token::Data]
//! *   [`Image`][Token::Image]
//! *   [`Label`][Token::Label]
//! *   [`LabelEnd`][Token::LabelEnd]
//! *   [`LabelMarker`][Token::LabelMarker]
//! *   [`LabelText`][Token::LabelText]
//! *   [`LineEnding`][Token::LineEnding]
//! *   [`Link`][Token::Link]
//! *   [`Reference`][Token::Reference]
//! *   [`ReferenceMarker`][Token::ReferenceMarker]
//! *   [`ReferenceString`][Token::ReferenceString]
//! *   [`Resource`][Token::Resource]
//! *   [`ResourceDestination`][Token::ResourceDestination]
//! *   [`ResourceDestinationLiteral`][Token::ResourceDestinationLiteral]
//! *   [`ResourceDestinationLiteralMarker`][Token::ResourceDestinationLiteralMarker]
//! *   [`ResourceDestinationRaw`][Token::ResourceDestinationRaw]
//! *   [`ResourceDestinationString`][Token::ResourceDestinationString]
//! *   [`ResourceMarker`][Token::ResourceMarker]
//! *   [`ResourceTitle`][Token::ResourceTitle]
//! *   [`ResourceTitleMarker`][Token::ResourceTitleMarker]
//! *   [`ResourceTitleString`][Token::ResourceTitleString]
//! *   [`SpaceOrTab`][Token::SpaceOrTab]
//!
//! ## References
//!
//! *   [`label-end.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-core-commonmark/dev/lib/label-end.js)
//! *   [*§ 4.7 Link reference definitions* in `CommonMark`](https://spec.commonmark.org/0.30/#link-reference-definitions)
//! *   [*§ 6.3 Links* in `CommonMark`](https://spec.commonmark.org/0.30/#links)
//! *   [*§ 6.4 Images* in `CommonMark`](https://spec.commonmark.org/0.30/#images)
//!
//! [string]: crate::content::string
//! [text]: crate::content::text
//! [destination]: crate::construct::partial_destination
//! [title]: crate::construct::partial_title
//! [label]: crate::construct::partial_label
//! [label_start_image]: crate::construct::label_start_image
//! [label_start_link]: crate::construct::label_start_link
//! [definition]: crate::construct::definition
//! [autolink]: crate::construct::autolink
//! [sanitize_uri]: crate::util::sanitize_uri::sanitize_uri
//! [normalize_identifier]: crate::util::normalize_identifier::normalize_identifier
//! [html-a]: https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element
//! [html-img]: https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element

use crate::constant::RESOURCE_DESTINATION_BALANCE_MAX;
use crate::construct::{
    partial_destination::{start as destination, Options as DestinationOptions},
    partial_label::{start as label, Options as LabelOptions},
    partial_space_or_tab::space_or_tab_eol,
    partial_title::{start as title, Options as TitleOptions},
};
use crate::token::Token;
use crate::tokenizer::{Code, Event, EventType, Media, State, Tokenizer};
use crate::util::{
    normalize_identifier::normalize_identifier,
    span::{serialize, Span},
};

/// State needed to parse label end.
#[derive(Debug)]
struct Info {
    /// Index into `label_start_stack` of the corresponding opening.
    label_start_index: usize,
    /// The proposed `Media` that this seems to represent.
    media: Media,
}

/// Start of label end.
///
/// ```markdown
/// > | [a](b) c
///       ^
/// > | [a][b] c
///       ^
/// > | [a][] b
///       ^
/// > | [a] b
/// ```
pub fn start(tokenizer: &mut Tokenizer, code: Code) -> State {
    if Code::Char(']') == code && tokenizer.parse_state.constructs.label_end {
        let mut label_start_index = None;
        let mut index = tokenizer.label_start_stack.len();

        while index > 0 {
            index -= 1;

            if !tokenizer.label_start_stack[index].balanced {
                label_start_index = Some(index);
                break;
            }
        }

        // If there is an okay opening:
        if let Some(label_start_index) = label_start_index {
            let label_start = tokenizer
                .label_start_stack
                .get_mut(label_start_index)
                .unwrap();

            // Mark as balanced if the info is inactive.
            if label_start.inactive {
                return nok(tokenizer, code, label_start_index);
            }

            let label_end_start = tokenizer.events.len();
            let info = Info {
                label_start_index,
                media: Media {
                    start: label_start.start,
                    end: (label_end_start, label_end_start + 3),
                    id: normalize_identifier(&serialize(
                        &tokenizer.parse_state.codes,
                        &Span {
                            start_index: tokenizer.events[label_start.start.1].point.index,
                            end_index: tokenizer.events[label_end_start - 1].point.index,
                        },
                        false,
                    )),
                },
            };

            tokenizer.enter(Token::LabelEnd);
            tokenizer.enter(Token::LabelMarker);
            tokenizer.consume(code);
            tokenizer.exit(Token::LabelMarker);
            tokenizer.exit(Token::LabelEnd);

            return State::Fn(Box::new(move |t, c| after(t, c, info)));
        }
    }

    State::Nok
}

/// After `]`.
///
/// ```markdown
/// > | [a](b) c
///       ^
/// > | [a][b] c
///       ^
/// > | [a][] b
///       ^
/// > | [a] b
///       ^
/// ```
fn after(tokenizer: &mut Tokenizer, code: Code, info: Info) -> State {
    let defined = tokenizer.parse_state.definitions.contains(&info.media.id);

    match code {
        // Resource (`[asd](fgh)`)?
        Code::Char('(') => tokenizer.attempt(resource, move |is_ok| {
            Box::new(move |t, c| {
                // Also fine if `defined`, as then it’s a valid shortcut.
                if is_ok || defined {
                    ok(t, c, info)
                } else {
                    nok(t, c, info.label_start_index)
                }
            })
        })(tokenizer, code),
        // Full (`[asd][fgh]`) or collapsed (`[asd][]`) reference?
        Code::Char('[') => tokenizer.attempt(full_reference, move |is_ok| {
            Box::new(move |t, c| {
                if is_ok {
                    ok(t, c, info)
                } else if defined {
                    reference_not_full(t, c, info)
                } else {
                    nok(t, c, info.label_start_index)
                }
            })
        })(tokenizer, code),
        // Shortcut reference: `[asd]`?
        _ => {
            if defined {
                ok(tokenizer, code, info)
            } else {
                nok(tokenizer, code, info.label_start_index)
            }
        }
    }
}

/// After `]`, at `[`, but not at a full reference.
///
/// > 👉 **Note**: we only get here if the label is defined.
///
/// ```markdown
/// > | [a][] b
///        ^
/// > | [a] b
///        ^
/// ```
fn reference_not_full(tokenizer: &mut Tokenizer, code: Code, info: Info) -> State {
    tokenizer.attempt(collapsed_reference, move |is_ok| {
        Box::new(move |t, c| {
            if is_ok {
                ok(t, c, info)
            } else {
                nok(t, c, info.label_start_index)
            }
        })
    })(tokenizer, code)
}

/// Done, we found something.
///
/// ```markdown
/// > | [a](b) c
///           ^
/// > | [a][b] c
///           ^
/// > | [a][] b
///          ^
/// > | [a] b
///        ^
/// ```
fn ok(tokenizer: &mut Tokenizer, code: Code, mut info: Info) -> State {
    // Remove this one and everything after it.
    let mut left = tokenizer
        .label_start_stack
        .split_off(info.label_start_index);
    // Remove this one from `left`, as we’ll move it to `media_list`.
    left.remove(0);
    tokenizer.label_start_list_loose.append(&mut left);

    let is_link = tokenizer.events[info.media.start.0].token_type == Token::LabelLink;

    if is_link {
        let mut index = 0;
        while index < tokenizer.label_start_stack.len() {
            let label_start = &mut tokenizer.label_start_stack[index];
            if tokenizer.events[label_start.start.0].token_type == Token::LabelLink {
                label_start.inactive = true;
            }
            index += 1;
        }
    }

    info.media.end.1 = tokenizer.events.len() - 1;
    tokenizer.media_list.push(info.media);
    tokenizer.register_resolver_before("media".to_string(), Box::new(resolve_media));
    State::Ok(if matches!(code, Code::None) { 0 } else { 1 })
}

/// Done, it’s nothing.
///
/// There was an okay opening, but we didn’t match anything.
///
/// ```markdown
/// > | [a](b c
///        ^
/// > | [a][b c
///        ^
/// > | [a] b
///        ^
/// ```
fn nok(tokenizer: &mut Tokenizer, _code: Code, label_start_index: usize) -> State {
    let label_start = tokenizer
        .label_start_stack
        .get_mut(label_start_index)
        .unwrap();
    label_start.balanced = true;
    State::Nok
}

/// Before a resource, at `(`.
///
/// ```markdown
/// > | [a](b) c
///        ^
/// ```
fn resource(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char('(') => {
            tokenizer.enter(Token::Resource);
            tokenizer.enter(Token::ResourceMarker);
            tokenizer.consume(code);
            tokenizer.exit(Token::ResourceMarker);
            State::Fn(Box::new(resource_start))
        }
        _ => unreachable!("expected `(`"),
    }
}

/// At the start of a resource, after `(`, before a destination.
///
/// ```markdown
/// > | [a](b) c
///         ^
/// ```
fn resource_start(tokenizer: &mut Tokenizer, code: Code) -> State {
    tokenizer.attempt_opt(space_or_tab_eol(), resource_open)(tokenizer, code)
}

/// At the start of a resource, after optional whitespace.
///
/// ```markdown
/// > | [a](b) c
///         ^
/// ```
fn resource_open(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char(')') => resource_end(tokenizer, code),
        _ => tokenizer.go(
            |t, c| {
                destination(
                    t,
                    c,
                    DestinationOptions {
                        limit: RESOURCE_DESTINATION_BALANCE_MAX,
                        destination: Token::ResourceDestination,
                        literal: Token::ResourceDestinationLiteral,
                        marker: Token::ResourceDestinationLiteralMarker,
                        raw: Token::ResourceDestinationRaw,
                        string: Token::ResourceDestinationString,
                    },
                )
            },
            destination_after,
        )(tokenizer, code),
    }
}

/// In a resource, after a destination, before optional whitespace.
///
/// ```markdown
/// > | [a](b) c
///          ^
/// ```
fn destination_after(tokenizer: &mut Tokenizer, code: Code) -> State {
    tokenizer.attempt(space_or_tab_eol(), |ok| {
        Box::new(if ok { resource_between } else { resource_end })
    })(tokenizer, code)
}

/// In a resource, after a destination, after whitespace.
///
/// ```markdown
/// > | [a](b ) c
///           ^
/// ```
fn resource_between(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char('"' | '\'' | '(') => tokenizer.go(
            |t, c| {
                title(
                    t,
                    c,
                    TitleOptions {
                        title: Token::ResourceTitle,
                        marker: Token::ResourceTitleMarker,
                        string: Token::ResourceTitleString,
                    },
                )
            },
            title_after,
        )(tokenizer, code),
        _ => resource_end(tokenizer, code),
    }
}

/// In a resource, after a title.
///
/// ```markdown
/// > | [a](b "c") d
///              ^
/// ```
fn title_after(tokenizer: &mut Tokenizer, code: Code) -> State {
    tokenizer.attempt_opt(space_or_tab_eol(), resource_end)(tokenizer, code)
}

/// In a resource, at the `)`.
///
/// ```markdown
/// > | [a](b) d
///          ^
/// ```
fn resource_end(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char(')') => {
            tokenizer.enter(Token::ResourceMarker);
            tokenizer.consume(code);
            tokenizer.exit(Token::ResourceMarker);
            tokenizer.exit(Token::Resource);
            State::Ok(0)
        }
        _ => State::Nok,
    }
}

/// In a reference (full), at the `[`.
///
/// ```markdown
/// > | [a][b] d
///        ^
/// ```
fn full_reference(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char('[') => tokenizer.go(
            |t, c| {
                label(
                    t,
                    c,
                    LabelOptions {
                        label: Token::Reference,
                        marker: Token::ReferenceMarker,
                        string: Token::ReferenceString,
                    },
                )
            },
            full_reference_after,
        )(tokenizer, code),
        _ => unreachable!("expected `[`"),
    }
}

/// In a reference (full), after `]`.
///
/// ```markdown
/// > | [a][b] d
///          ^
/// ```
fn full_reference_after(tokenizer: &mut Tokenizer, code: Code) -> State {
    let events = &tokenizer.events;
    let mut index = events.len() - 1;
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    while index > 0 {
        index -= 1;
        let event = &events[index];
        if event.token_type == Token::ReferenceString {
            if event.event_type == EventType::Exit {
                end = Some(event.point.index);
            } else {
                start = Some(event.point.index);
                break;
            }
        }
    }

    if tokenizer
        .parse_state
        .definitions
        .contains(&normalize_identifier(&serialize(
            &tokenizer.parse_state.codes,
            &Span {
                // Always found, otherwise we don’t get here.
                start_index: start.unwrap(),
                end_index: end.unwrap(),
            },
            false,
        )))
    {
        State::Ok(if matches!(code, Code::None) { 0 } else { 1 })
    } else {
        State::Nok
    }
}

/// In a reference (collapsed), at the `[`.
///
/// > 👉 **Note**: we only get here if the label is defined.
///
/// ```markdown
/// > | [a][] d
///        ^
/// ```
fn collapsed_reference(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char('[') => {
            tokenizer.enter(Token::Reference);
            tokenizer.enter(Token::ReferenceMarker);
            tokenizer.consume(code);
            tokenizer.exit(Token::ReferenceMarker);
            State::Fn(Box::new(collapsed_reference_open))
        }
        _ => State::Nok,
    }
}

/// In a reference (collapsed), at the `]`.
///
/// > 👉 **Note**: we only get here if the label is defined.
///
/// ```markdown
/// > | [a][] d
///         ^
/// ```
fn collapsed_reference_open(tokenizer: &mut Tokenizer, code: Code) -> State {
    match code {
        Code::Char(']') => {
            tokenizer.enter(Token::ReferenceMarker);
            tokenizer.consume(code);
            tokenizer.exit(Token::ReferenceMarker);
            tokenizer.exit(Token::Reference);
            State::Ok(0)
        }
        _ => State::Nok,
    }
}

/// Resolve media.
///
/// This turns correct label start (image, link) and label end into links and
/// images, or turns them back into data.
#[allow(clippy::too_many_lines)]
pub fn resolve_media(tokenizer: &mut Tokenizer) {
    let mut left = tokenizer.label_start_list_loose.split_off(0);
    let mut left_2 = tokenizer.label_start_stack.split_off(0);
    let media = tokenizer.media_list.split_off(0);
    left.append(&mut left_2);

    let events = &tokenizer.events;

    // Remove loose label starts.
    let mut index = 0;
    while index < left.len() {
        let label_start = &left[index];
        let data_enter_index = label_start.start.0;
        let data_exit_index = label_start.start.1;

        tokenizer.map.add(
            data_enter_index,
            data_exit_index - data_enter_index + 1,
            vec![
                Event {
                    event_type: EventType::Enter,
                    token_type: Token::Data,
                    point: events[data_enter_index].point.clone(),
                    link: None,
                },
                Event {
                    event_type: EventType::Exit,
                    token_type: Token::Data,
                    point: events[data_exit_index].point.clone(),
                    link: None,
                },
            ],
        );

        index += 1;
    }

    // Add grouping events.
    let mut index = 0;
    while index < media.len() {
        let media = &media[index];
        // LabelLink:Enter or LabelImage:Enter.
        let group_enter_index = media.start.0;
        let group_enter_event = &events[group_enter_index];
        // LabelLink:Exit or LabelImage:Exit.
        let text_enter_index = media.start.0
            + (if group_enter_event.token_type == Token::LabelLink {
                4
            } else {
                6
            });
        // LabelEnd:Enter.
        let text_exit_index = media.end.0;
        // LabelEnd:Exit.
        let label_exit_index = media.end.0 + 3;
        // Resource:Exit, etc.
        let group_end_index = media.end.1;

        // Insert a group enter and label enter.
        tokenizer.map.add(
            group_enter_index,
            0,
            vec![
                Event {
                    event_type: EventType::Enter,
                    token_type: if group_enter_event.token_type == Token::LabelLink {
                        Token::Link
                    } else {
                        Token::Image
                    },
                    point: group_enter_event.point.clone(),
                    link: None,
                },
                Event {
                    event_type: EventType::Enter,
                    token_type: Token::Label,
                    point: group_enter_event.point.clone(),
                    link: None,
                },
            ],
        );

        // Empty events not allowed.
        if text_enter_index != text_exit_index {
            // Insert a text enter.
            tokenizer.map.add(
                text_enter_index,
                0,
                vec![Event {
                    event_type: EventType::Enter,
                    token_type: Token::LabelText,
                    point: events[text_enter_index].point.clone(),
                    link: None,
                }],
            );

            // Insert a text exit.
            tokenizer.map.add(
                text_exit_index,
                0,
                vec![Event {
                    event_type: EventType::Exit,
                    token_type: Token::LabelText,
                    point: events[text_exit_index].point.clone(),
                    link: None,
                }],
            );
        }

        // Insert a label exit.
        tokenizer.map.add(
            label_exit_index + 1,
            0,
            vec![Event {
                event_type: EventType::Exit,
                token_type: Token::Label,
                point: events[label_exit_index].point.clone(),
                link: None,
            }],
        );

        // Insert a group exit.
        tokenizer.map.add(
            group_end_index + 1,
            0,
            vec![Event {
                event_type: EventType::Exit,
                token_type: Token::Link,
                point: events[group_end_index].point.clone(),
                link: None,
            }],
        );

        index += 1;
    }

    tokenizer.map.consume(&mut tokenizer.events);
}
