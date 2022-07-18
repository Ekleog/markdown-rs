# micromark-rs

Crate docs are currently at
[`wooorm.com/micromark-rs/micromark/`](https://wooorm.com/micromark-rs/micromark/).

There’s still a lot to do, but, already: **100%** CommonMark 🥳

## Some useful scripts for now

Run examples:

```sh
RUST_BACKTRACE=1 RUST_LOG=debug cargo run --example lib
```

Format:

```sh
cargo fmt --all
```

Lint:

```sh
cargo fmt --all -- --check && cargo clippy -- -W clippy::pedantic
```

Tests:

```sh
RUST_BACKTRACE=1 cargo test
```

Docs:

```sh
cargo doc --document-private-items
```

(add `--open` to open them in a browser)

## To do

### Some major obstacles

- [ ] (5) There’s a lot of rust-related choosing whether to pass (mutable)
      references or whatever around that should be refactored
- [ ] (5) Figure out extensions

### All the things

#### Docs

- [ ] (1) Go through all bnf
- [ ] (1) Go through all docs
- [ ] (1) Add overview docs on how everything works

#### Refactor

- [ ] (1) Use `edit_map` in `subtokenize` (needs to support links in edits)
- [ ] (1) Improve `interrupt`, `concrete`, `lazy` fields somehow?

#### Parse

- [ ] (3) Make tokens extendable for extensions?

#### Test

- [ ] (1) Make sure positional info is perfect
- [ ] (3) Share a bunch of tests with `micromark-js`

#### Misc

- [ ] (3) `no_std`: remove all `HashSet`s/`HashMap` to use vecs, vecs w/ tuples?
- [ ] (3) Remove splicing and cloning in subtokenizer
- [ ] (3) Pass more references around
- [ ] (1) Get markers from constructs (`string`, `text`)
- [ ] (3) Read through rust docs to figure out what useful functions there are,
      and fix stuff I’m doing manually now
- [ ] (5) Do some research on rust best practices for APIs, e.g., what to accept,
      how to integrate with streams or so?
- [ ] (1) Go through clippy rules, and such, to add strict code styles
- [ ] (1) Any special handling of surrogates?
- [ ] (1) Make sure debugging, assertions are useful for other folks
- [ ] (3) Add some benchmarks (against comrak, pulldown-cmark, kramdown?), do some perf testing
- [ ] (3) Write comparison to other parsers
- [ ] (3) Add node/etc bindings?
- [ ] (3) Bunch of docs
- [ ] (5) Site

### Extensions

The main thing here is is to figure out if folks could extend from the outside
with their own code, or if we need to maintain it all here.
Regardless, it is essential for the launch of `micromark-rs` that extensions
are theoretically or practically possible.
The extensions below are listed from top to bottom from more important to less
important.

- [ ] (1) frontmatter (yaml, toml) (flow)
      — [`micromark-extension-frontmatter`](https://github.com/micromark/micromark-extension-frontmatter)
- [ ] (3) autolink literal (GFM) (text)
      — [`micromark-extension-gfm-autolink-literal`](https://github.com/micromark/micromark-extension-gfm-autolink-literal)
- [ ] (3) footnote (GFM) (flow, text)
      — [`micromark-extension-gfm-footnote`](https://github.com/micromark/micromark-extension-gfm-footnote)
- [ ] (3) strikethrough (GFM) (text)
      — [`micromark-extension-gfm-strikethrough`](https://github.com/micromark/micromark-extension-gfm-strikethrough)
- [ ] (5) table (GFM) (flow)
      — [`micromark-extension-gfm-table`](https://github.com/micromark/micromark-extension-gfm-table)
- [ ] (1) task list item (GFM) (text)
      — [`micromark-extension-gfm-task-list-item`](https://github.com/micromark/micromark-extension-gfm-task-list-item)
- [ ] (3) math (flow, text)
      — [`micromark-extension-math`](https://github.com/micromark/micromark-extension-math)
- [ ] (8) directive (flow, text)
      — [`micromark-extension-directive`](https://github.com/micromark/micromark-extension-directive)
- [ ] (8) expression (MDX) (flow, text)
      — [`micromark-extension-mdx-expression`](https://github.com/micromark/micromark-extension-mdx-expression)
- [ ] (5) JSX (MDX) (flow, text)
      — [`micromark-extension-mdx-jsx`](https://github.com/micromark/micromark-extension-mdx-jsx)
- [ ] (3) ESM (MDX) (flow)
      — [`micromark-extension-mdxjs-esm`](https://github.com/micromark/micromark-extension-mdxjs-esm)
- [ ] (1) tagfilter (GFM) (n/a, renderer)
      — [`micromark-extension-gfm-tagfilter`](https://github.com/micromark/micromark-extension-gfm-tagfilter)

#### After

- [ ] (8) After all extensions, including MDX, are done, see if we can integrate
      this with SWC to compile MDX

### Done

- [x] (8) Subtokenization: figure out a good, fast way to deal with constructs in
      one content type that also are another content type
- [x] (3) Encode urls
- [x] (1) Optionally remove dangerous protocols when compiling
- [x] (1) Add docs to html (text)
- [x] (1) Add docs on bnf
- [x] (1) Reorganize to split util
- [x] (1) Add examples to `Options` docs
- [x] (3) Fix deep subtokenization
- [x] (1) text in heading
- [x] (1) Setext headings, solved in flow
- [x] (1) Add docs to partials
- [x] (1) Remove all `pub fn`s from constructs, except for start
- [x] (1) Remove `content` content type, as it is no longer needed
- [x] (1) Paragraph
- [x] (1) Parse whitespace in each flow construct
- [x] (1) Connect `ChunkString` in label, destination, title
- [x] (1) Add support for line endings in `string`
- [x] (1) Handle BOM at start
- [x] (1) Make sure tabs are handled properly
- [x] (1) Add tests for `default-line-ending`, `line-ending`
- [x] (1) Use preferred line ending style in markdown
- [x] (1) Make sure crlf/cr/lf are working perfectly
- [x] (1) Figure out lifetimes of things (see `life time` in source)
- [x] (1) Use traits for a bunch of enums, e.g., markers
- [x] (1) Move safe protocols to constants
- [x] (1) Make text data, string data constructs (document in
      `construct/mod.rs`)
- [x] (1) Configurable tokens (destination, label, title)
- [x] (1) Configurable limit (destination)
- [x] (1) Add docs for `default_line_ending`
- [x] (1) Add docs for virtual spaces
- [x] (1) Add docs to `subtokenize.rs`
- [x] (1) Add docs for `link.rs`
- [x] (1) Add docs for token types
- [x] (1) Do not capture in `tokenizer.go`
- [x] (1) Clean attempts
- [x] (1) Add docs for tokenizer
- [x] (1) Add docs for sanitation
- [x] (1) Get definition identifiers (definition)
- [x] (1) Add docs to `normalize_identifier`
- [x] (1) Add docs for how references and definitions match
- [x] (1) Add module docs to parser
- [x] (1) Add improved docs in compiler
- [x] (1) Add docs for `RESOURCE_DESTINATION_BALANCE_MAX`
- [x] (1) Add docs for `label_start_image`, `label_start_link`
- [x] (1) Add docs for `label_end`
- [x] (1) Move map handling from `resolve_media`
- [x] (5) Add support for sharing identifiers, references before definitions
- [x] (2) Refactor to externalize handlers of compiler
- [x] (1) Add support for compiling shared references and definitions
- [x] (1) Add docs to Image, Link, and other media tokens
- [x] (1) Add docs on resolver, clean feed
- [x] (3) Clean compiler
- [x] (1) Parse initial and final space_or_tab of paragraphs (in string, text)
- [x] (1) Refactor to clean and document `space_or_tab`
- [x] (1) Refactor to clean and document `edit_map`
- [x] (8) Make paragraphs fast by merging them at the end, not checking whether
      things interrupt them each line
- [x] (3) Add support for interrupting (or not)
- [x] (5) attention
- [x] (3) Unicode punctuation
- [x] (1) Use rust to crawl unicode
- [x] (1) Document attention
- [x] (1) Remove todos in `span.rs` if not needed
- [x] (2) Fix resizing attention bug
- [x] (2) Fix interleaving of attention/label
- [x] (8) Add basic support for block quotes
- [x] (1) Use `char::REPLACEMENT_CHARACTER`?
- [x] (3) Add support for concrete constructs
      (html (flow) or code (fenced) cannot be “pierced” into by containers)
- [x] (1) Make sure that rust character groups match CM character groups
- [x] (3) Fix block quote bug
- [x] (3) Add support for lazy lines
- [x] (5) Containers!
- [x] (3) Check subtokenizer unraveling is ok
- [x] (1) Add list of void tokens, check that they’re void
- [x] (3) Use `commonmark` tests
- [x] (3) Add support for turning off constructs
