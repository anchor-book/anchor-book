# Authority Definitions

**Status: draft schema for review.** Requirements and design decisions are typed
records, just as anchors are. The authority pages use these definitions now;
engine parsing, reference validation, and rendering are not implemented yet.
They belong to the later record-validation increment, not the first structural
check of layout and navigation.

An authority reference identifies a record, not a heading or an arbitrary link.
The engine can therefore establish whether the record exists, has the expected
kind, and has an acceptance status compatible with the referencing record.

## Evolving the authority model

The authority schemas are provisional. We will refine them through use before
deciding which parts to stabilize. The current fields, requiredness, authority
kinds, acceptance states, and relationships are starting choices, not a final
model of every authority a Book may need.

Keep one clear definition of the current schema while iterating. When a field or
relationship needs to change, update its schema, affected records, examples, and
validation expectations together. Preserve record identities when their meaning
is unchanged, and reconcile references when their meaning changes. Agents should
propose these changes during design work rather than introduce ad hoc fields.

For now, `schema-version = 1` identifies an unpublished draft. We can revise that
draft in place and update this Book's records together; each design iteration
does not need a new version number or a second supported format. Before publishing
a supported schema, define its compatibility and migration policy. Later changes
must follow that policy so consuming Books can be updated deliberately.

Accepting a particular requirement or decision does not freeze its storage
schema. An authority schema change also need not change the anchor schema unless
it affects how anchors identify or interpret their authority references.

## One format for each record kind

Use a top-level fenced Markdown code block with an opening line of exactly three
backticks followed by `anchorbook-requirement` or `anchorbook-decision`, and a
closing line of exactly three backticks. The label determines the record kind;
there is no second `kind` field in the body. The body is TOML.

These labels, together with `anchorbook-anchor`, are the only definition labels.
Each kind has exactly one schema. Headings, HTML link targets, and formatted prose
do not define records. There are no aliases, shorthand forms, or alternate
serialization formats. Literal examples belong inside an outer code block, as
shown below; they do not enter the record inventory.

The [anchor format's rules](anchor-definitions.md#required-fields) for identifiers,
schema version, nonempty strings, and acceptance status apply to all three kinds.
Every field listed for a kind is required, with no defaults. Unknown or duplicate
fields, wrong types, unsupported versions, and malformed boundaries are errors.
Do not infer data from surrounding prose or coerce values to another type.

### Common fields

| Field | Type and constraint |
| --- | --- |
| `schema-version` | Integer, exactly `1` |
| `id` | Identifier unique across all anchor, requirement, and decision records in this Book |
| `title` | Nonempty display string |
| `status` | Exactly `"draft"` or `"accepted"`; records design acceptance, not implementation or verification |

Identity does not depend on a record's file, heading, or title. Moving a record
must preserve its identifier; changing a title does not require updating authority
references. Do not reuse an identifier for a different obligation or choice.

## Requirement schema

A requirement adds exactly these fields to the common fields:

| Field | Type and constraint |
| --- | --- |
| `statement` | Nonempty string stating the required outcome or constraint |
| `source` | Nonempty string explaining where the requirement comes from |

````text
```anchorbook-requirement
schema-version = 1
id = "requirement-observation-only"
title = "Preserve the project during checking"
status = "draft"
statement = "Checking must leave project content and directory entries unchanged."
source = "The product separates inspection from operations that create or manage Book content."
```
````

`source` records provenance for review. It is not an authority reference or proof
of approval. Reference validation cannot establish whether this explanation is
accurate or whether the requirement is appropriate.

## Design decision schema

A decision adds exactly these fields to the common fields:

| Field | Type and constraint |
| --- | --- |
| `choice` | Nonempty string stating the selected approach |
| `requirements` | Array of distinct requirement identifiers; every entry must resolve to a requirement in this Book |
| `rationale` | Nonempty string explaining the choice |
| `alternatives` | Nonempty array of nonempty strings describing alternatives and relevant tradeoffs |
| `reconsider-when` | Nonempty string stating the conditions for revisiting the choice |

````text
```anchorbook-decision
schema-version = 1
id = "decision-separate-check-and-repair"
title = "Keep checking separate from repair"
status = "draft"
choice = "The checker observes and reports; repairs are separate actions."
requirements = ["requirement-observation-only"]
rationale = "A caller can inspect defects without changing the inputs being diagnosed."
alternatives = ["Automatic repair inside check mixes diagnosis with changes."]
reconsider-when = "Managed writes have an agreed ownership and conflict model; add them through a separate operation."
```
````

The `requirements` field is always present. An empty array explicitly declares
that the decision is not derived from a recorded requirement; its rationale must
explain its independent basis. A decision can itself provide authority for an
anchor. The schema does not invent a requirement merely to fill a reference.
All five initial checker decisions do reference requirements.

## Validating authority relationships

Collect definition blocks from the Book's chapter inventory into one typed index,
then resolve references. Use the Markdown and TOML parsers shared with anchor
definitions and retain source locations for diagnostics. This is independent of
chapter order, rendered HTML, and manually maintained navigation links.

The defined relationships are:

| Source | Reference field | Required target |
| --- | --- | --- |
| Anchor | `authorities` entry with `kind = "requirement"` | Requirement with the given identifier |
| Anchor | `authorities` entry with `kind = "decision"` | Decision with the given identifier |
| Decision | Entry in `requirements` | Requirement with the given identifier |

Requirements have no outgoing authority-reference field in this schema. Anchors
still need at least one direct authority reference. A decision's referenced
requirements are additional traceable relationships, not a reason to infer or
rewrite the anchor's authored references.

Validate the whole inventory, including unreferenced definitions:

- Every record must conform to its kind's schema. Invalid definitions cannot be
  skipped simply because no anchor refers to them yet.
- Identifiers must be unique across kinds. A duplicate is an error, even if the
  duplicate records have the same contents; there is no first or last winner.
- Each reference must resolve to an existing record of the required kind.
  A matching heading or HTML target cannot satisfy it.
- Repeated references within a field are errors.
- Every authority cited by an accepted anchor must be accepted. Every requirement
  cited by an accepted decision must be accepted. Draft records may reference
  either draft or accepted authority. Validation checks all these edges, including
  the requirements behind an accepted decision.

If a target definition is malformed, report that definition error and identify
the dependent reference as unresolved due to invalid authority. Do not treat it
as a valid target or fabricate a separate missing-record cause. Report missing,
wrong-kind, duplicate, and incompatible-status references at their source, with
the target location when available. Any invalid record or relationship prevents
a passing record-validation result.

Removing a referenced record therefore breaks validation. Renaming an identifier
requires updating its references. Cross-Book references, retirement, and
supersession are outside the current draft. We can design them as the authority
model evolves, updating the schema and records together rather than allowing
unrecognized fields to acquire meaning implicitly.

## Authored relationships and derived views

The referencing record owns the relationship: an anchor lists its authorities,
and a decision lists its requirements. Do not author a second reverse-reference
field on the targets. The engine can derive which anchors rely on a requirement
or decision from that same index.

For now, the authority pages have ordinary links below their blocks to aid human
review. Those links are not another source of relationships or valid definitions.
A future preprocessor will generate readable records and backlinks from the
validated model. Temporary HTML link targets do not supply identity to validation.

Valid references establish structural consistency and declared acceptance status.
They do not prove that an authority justifies a property, that a human actually
approved a status change, or that the system implements it. Those remain review
and evidence responsibilities.
