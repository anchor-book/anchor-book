# Anchor Definitions

**Status: draft schema for review; parsing and rendering implemented.**

An anchor defines a property of the goal system, its authority, and verification criteria.
Surrounding prose explains it but is not part of the definition. The
[record library and renderer](record-rendering.md) implement this format separately
from layout/navigation checking.

## One canonical format

An anchor is a top-level fenced Markdown code block. Its opening line is exactly
three backticks followed by `anchorbook-anchor`; its closing line is exactly
three backticks. Its body is TOML matching schema version 1 below.

There are no inline or heading-based definitions, alternate formats, inferred
fields, or aliases. Enclose literal examples in a different outer code block:

````text
```anchorbook-anchor
schema-version = 1
id = "check-observation-only"
title = "Checking preserves project files"
status = "draft"
property = '''
A check leaves project content and directory entries unchanged,
whether it passes, fails, or cannot complete.
'''
authorities = [
  { kind = "requirement", id = "requirement-observation-only" },
  { kind = "decision", id = "decision-separate-check-and-repair" },
]

[[criteria]]
id = "file-preservation"
statement = "Compare project file contents and directory entries before and after passing, failing, and incomplete checks; they are unchanged."
```
````

### Required fields

Every field below is required. Unknown fields and duplicate fields are errors.
There are no defaults. TOML whitespace and comments do not change meaning;
they do not provide alternate field names, record shapes, or definitions.

| Field | Type and constraint |
| --- | --- |
| `schema-version` | Integer, exactly `1` |
| `id` | Nonempty identifier, unique across this Book's anchor and authority records |
| `title` | Nonempty string, for display |
| `status` | Exactly `"draft"` or `"accepted"`; this is design acceptance only |
| `property` | Nonempty string containing the complete property, including any qualifications |
| `authorities` | Nonempty array of inline tables, each with exactly `kind` and `id` |
| `criteria` | Nonempty sequence of `[[criteria]]` tables, each with exactly `id` and `statement` |

Identifiers consist of lowercase ASCII letters and digits separated by single
hyphens, start with a letter, and have no trailing hyphen. Anchor identifiers
are explicit and do not change when a chapter moves. Titles can change without
changing identity. Do not reuse an identifier for a different property.

An authority's `kind` is exactly `"requirement"` or `"decision"`. Its `id` must
resolve to a record of that kind in the same Book. Duplicate authority references
are errors. At least one authority is required; both kinds are allowed but not
required. Cross-Book references are outside this schema.

Each criterion has an identifier unique within its anchor and a nonempty
statement of what verification must establish. Evidence can later reference
the anchor identifier and criterion identifier together. A criterion is not a
test result and does not require a particular test framework or proof tool.

Whitespace-only strings are empty. Do not coerce strings to numbers, infer
missing values from surrounding prose, or accept misspelled status values.
Unknown schema versions are errors. Changes to these rules require an explicit
schema revision rather than parser leniency. During bootstrap, an unpublished
draft can be revised in place with its records; this does not require a new
version number for every design iteration. The [authority schemas](authority-definitions.md#evolving-the-authority-model)
remain provisional and will evolve through use.

## Parsing and validation

The shared engine library parses Markdown to locate definition blocks, retaining
source offsets. It then parses their TOML into strict Rust data types and
validates the model. It does not extract definitions with regular expressions.

Reject an unclosed definition fence, a noncanonical definition fence, a nested
definition, malformed TOML, invalid fields, duplicate identity, or unresolved
authority with a diagnostic at the source location. A literal example inside
another code block is not a definition. Ordinary code blocks are not anchors.
Never silently discard an invalid recognized definition.

The canonical language label is reserved. Once this schema is adopted, legacy
`{anchor}` definitions are rejected as unsupported authoring syntax rather than
supported as a second format. They may appear as literal examples.

Resolve references after collecting the Book inventory, so chapter order does
not affect validity. An accepted anchor must cite accepted authority; draft
anchors may cite draft authority while the design is being reviewed. Reference
validity does not establish that an authority logically justifies the property;
that relationship still needs review.

[Typed authority definitions](authority-definitions.md) supply reference identity;
headings and HTML targets do not. Accepted decisions must also cite only accepted
requirements. The shared index checks all these relationships.

The library owns these rules. The mdBook preprocessor consumes the same
validated model to render anchors and links. Rendering must not define a second
interpretation of the source.

## Design, implementation, and evidence

`status = "accepted"` means the property has been accepted into the goal design.
It does not mean implemented, tested, or proven. Keep implementation associations,
executed results, and roadmap completion outside the authored definition.
Reports can derive progress from those records as the engine gains that support.

The initial checker anchors remain drafts with planned verification criteria.
The renderer derives navigation targets from record identities.
