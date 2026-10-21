# Configuration

**Status: draft for review.** The example is a proposed schema; no configuration
reader or `anchorbook check` command exists yet. The separate `anchorbook md`
rendering command is implemented.

`anchorbook.toml` maps a project into the Book model. Directory names are project
choices. Role names describe responsibilities, independently of page titles.

## Anchors

```anchorbook-anchor
schema-version = 1
id = "check-book-selection"
title = "The explicit configuration selects the Book"
status = "draft"
property = '''
For the same explicit configuration file and filesystem state, checking selects
the same Book and resolves the same source, output, and section roles regardless
of the caller's working directory. Directory names and display titles do not
determine section roles.
'''
authorities = [
  { kind = "requirement", id = "requirement-configured-structure" },
  { kind = "decision", id = "decision-explicit-configuration" },
]

[[criteria]]
id = "invocation-directory"
statement = "Invoke from different working directories with arguments naming the same configuration file; compare resolved paths, roles, and findings."

[[criteria]]
id = "alternative-layout"
statement = "Check equivalent fixtures under book, docs, and md with different display titles; role interpretation and conformance are equivalent."
```

```anchorbook-anchor
schema-version = 1
id = "check-configuration-validity"
title = "Only valid configuration reaches structural evaluation"
status = "draft"
property = '''
Structural evaluation uses validated configuration with the required schema
fields and distinct section mappings. Paths remain within their declared roots,
and source and output neither coincide nor contain each other, including through
symlink aliases. Invalid or unresolvable configuration cannot produce a pass.
'''
authorities = [
  { kind = "requirement", id = "requirement-configured-structure" },
  { kind = "decision", id = "decision-explicit-configuration" },
]

[[criteria]]
id = "schema-rejection"
statement = "Fixtures with missing or unknown fields, invalid types, unsupported versions, invalid project phases, and duplicate section mappings produce specific configuration errors."

[[criteria]]
id = "path-boundaries"
statement = "Reject empty or absolute paths, escapes, overlaps, and symlink aliases that violate containment or separation; include both directions of source/output containment."

[[criteria]]
id = "missing-paths"
statement = "Accept a resolvable separate output that does not exist without creating it; classify a resolvable missing source as a structural violation and unresolved path relationships as errors."
```

## Initial schema

The proposed configuration for this repository is:

```toml
schema-version = 1

[project]
phase = "prototyping"

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

## Project phase

`[project].phase` selects the working expectations described in the
[AI Working Contract](../ai-working-contract.md#work-in-the-projects-phase).
It belongs in `anchorbook.toml`, alongside the project's Book configuration;
mdBook's `book.toml` only configures rendering.

The field is required and accepts exactly one of these strings:

| Value | Meaning |
| --- | --- |
| `"prototyping"` | Explore design and implementation through assigned experiments; draft governing records are allowed. |
| `"building"` | Implement assigned roadmap work against accepted governing design and authority. |
| `"complete"` | Declare the defined goal system complete for a recorded scope and revision, supported by completion and verification evidence. |

Reject missing, unknown, or non-string values; do not silently choose a phase.
Until the reader exists and this configuration is added,
[Working on Anchor Book](../working-on-anchor-book.md#current-phase) records this
repository's phase. The contract defines phase behavior without declaring a
project's current value.

The initial configuration reader validates and exposes the phase as data. All
three phases use the same initial structural rules: prototyping does not disable
layout checks, and a structural pass in Complete does not establish implementation
or verification. Automated enforcement of design readiness and completion is
later work. The phase value expresses the chosen workflow; it is not evidence.

The engineer directs transitions, with their scope and basis recorded in the
roadmap. Later development can begin a new iteration while preserving the prior
completed baseline. A scoped experiment alongside accepted work is described in
its roadmap and assignment; no per-capability override syntax is introduced here.

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
