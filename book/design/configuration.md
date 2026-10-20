# Configuration

**Status: draft for review.** The example is a proposed schema; no configuration
reader or `anchorbook` command exists yet.

`anchorbook.toml` maps a project into the Book model. Directory names are project
choices. Role names describe responsibilities, independently of page titles.

## Initial schema

The proposed configuration for this repository is:

```toml
schema-version = 1

[book]
source = "book"
output = "target/anchorbook"

[book.sections]
design = "design/index.md"
authority = "authority/index.md"
implementation-roadmaps = "implementation-roadmaps/index.md"
```

All shown fields are required in the first schema. Reject unknown fields and
unsupported schema versions so a typo cannot silently change behavior.
Do not add this configuration to the repository until the reader exists.

The three section roles are the initial required set. Each maps to a distinct
chapter at the top level of `SUMMARY.md`. The display title can differ.
Later schemas can add more detailed roles and policies; the initial mapping
does not establish semantic validity of requirements, decisions, or anchors.

## Paths

Resolve source and output relative to the directory containing
`anchorbook.toml`. Resolve section paths relative to the source directory.
The CLI resolves its explicit configuration argument relative to the caller's
working directory once; no other paths depend on that directory.

For the first schema, require nonempty relative paths within the configuration
directory; section paths must stay within the source directory. Reject absolute
paths and escapes, including escapes through symlinks. Supporting Books spread
across other roots needs an explicit later design.

Source and output must be separate: neither may equal or contain the other.
Compare resolved locations, including existing symlink ancestors. An output
directory may not exist yet; resolve the existing portion and the remaining
components without creating it. If resolution cannot establish a safe relation,
return an error rather than assume separation. A missing source is a structural
finding when its intended location can be resolved.

This check describes the observed filesystem. A future writing operation must
revalidate paths and ownership immediately before applying changes.

## Relationship to mdBook

For the first increment, `book.toml` sits beside `anchorbook.toml`.
The engine configuration expresses the intended source and output; mdBook's
configuration must resolve to the same locations.

| Anchor Book setting | mdBook setting |
| --- | --- |
| `book.source` | `book.src` |
| `book.output` | `build.build-dir` |

Interpret mdBook settings, including defaults, according to the declared
supported mdBook version. The initial check compares the file-based settings;
it does not claim to reproduce arbitrary environment or command-line overrides.
The later build adapter must make the effective settings it uses explicit.

`check` reports configuration drift without changing either file. Later,
materialization can reconcile the selected mdBook settings while preserving
unrelated configuration. The renderer continues to consume ordinary
`book.toml`; it need not understand Anchor Book's model.

Separately located mdBook configuration, optional profiles, extensions, and
cross-Book dependencies remain future schema work. They do not make the
initial required roles optional.
