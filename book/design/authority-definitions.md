# Authority Definitions

**Status: draft schemas for review; parsing, reference validation, and rendering
implemented.** Requirements and decisions are typed records. Their identities,
kinds, and acceptance states support authority-link validation independently of
layout/navigation checks.

## Evolving the authority model

The authority model remains provisional. Its fields, which fields are required,
record kinds, acceptance states, and relationships will be refined through use.
Maintain one defined current schema while experimenting. When it changes, update
the affected records, examples, and validation expectations together. Preserve
record identities when their meaning is unchanged; reconcile references when it
changes. Propose schema changes rather than adding ad hoc fields.

`schema-version = 1` is an unpublished draft that can evolve in place with this
Book. Before publication, define compatibility and migration policy for consuming
Books; subsequent changes follow that policy. Accepting a record does not freeze
its schema. An authority schema change need not change the anchor schema unless
it affects how anchors identify or interpret authority references.

## One format for each record kind

Use a top-level fenced Markdown code block with an opening line of exactly three
backticks followed by `anchorbook-requirement` or `anchorbook-decision`, and a
closing line of exactly three backticks. The label determines the record kind;
there is no second `kind` field in the body. The body is TOML.

These labels and `anchorbook-anchor` are the only definition labels, with one
schema per kind. Headings, links, prose, aliases, and alternate formats do not
define records. Examples inside outer code blocks are excluded from the inventory.

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

Anchors own their authority references; decisions own their requirement references.
The engine derives backlinks from that index without reverse-reference fields on
targets. The preprocessor renders the validated model; rendered links do not
supply identity to validation.

Valid references establish structural consistency and declared acceptance status.
They do not prove that an authority justifies a property, that a human actually
approved a status change, or that the system implements it. Those remain review
and evidence responsibilities.
