# Authority Definitions

**Status: draft for review.** Requirements and decisions are typed records written
in the [record syntax](anchor-definitions.md#record-syntax). Their identities,
kinds, and acceptance states support authority-link validation independently of
layout/navigation checks. The engine currently reads the earlier TOML form; see
[Current implementation](anchor-definitions.md#current-implementation).

## Evolving the authority model

The authority model remains provisional. Its fields, which fields are required,
record kinds, acceptance states, and relationships will be refined through use.
Maintain one defined current syntax while experimenting. When it changes, update
the affected records, examples, and validation expectations together. Preserve
record identities when their meaning is unchanged; reconcile references when it
changes. Propose syntax changes rather than adding ad hoc fields.

The syntax is an unpublished draft that can evolve in place with this Book.
Before publication, define compatibility and migration policy for consuming
Books; subsequent changes follow that policy. Accepting a record does not freeze
its syntax. An authority change need not change anchors unless it affects how
anchors identify or interpret authority references.

## One form for each record kind

A requirement or decision is one blockquote whose heading names its kind,
`Requirement` or `Decision`; there is no separate kind field. The
[heading, identifier, status, paragraph, reference, and line-accounting rules](anchor-definitions.md#record-syntax)
apply to all three kinds. Headings, links, prose, aliases, and alternate forms do
not define records. Examples inside code blocks are excluded from the inventory.

Every field listed for a kind is required, in the listed order, with no defaults.
Unknown, repeated, or misordered labels and malformed headings are errors. Do not
infer data from surrounding prose.

Identity does not depend on a record's file, heading position, or title. Moving a
record must preserve its identifier; changing a title does not require updating
authority references. Do not reuse an identifier for a different obligation or choice.
Requirement and decision identifiers follow the [identifier rules](anchor-definitions.md#identifiers):
`requirement-<outcome>` and `decision-<choice>`, named for their meaning and
never numbered.

## Requirement fields

| Field | Constraint |
| --- | --- |
| `Statement:` | Nonempty; the required outcome or constraint |
| `Source:` | Nonempty; where the requirement comes from |

```markdown
> **Requirement `requirement-observation-only`** (draft): **Preserve the project during checking**
> Statement: Checking must leave project content and directory entries unchanged.
> Source: The product separates inspection from operations that create or manage Book content.
```

`Source` records provenance for review. It is not an authority reference or proof
of approval. Reference validation cannot establish whether this explanation is
accurate or whether the requirement is appropriate.

## Decision fields

| Field | Constraint |
| --- | --- |
| `Choice:` | Nonempty; the selected approach |
| `Requirements:` | Distinct requirement references separated by `;`, each resolving to a requirement in this Book, or exactly `none` |
| `Rationale:` | Nonempty; why the choice was made |
| `Alternatives:` | Followed by one or more list items, each a nonempty alternative and its tradeoffs |
| `Reconsider when:` | Nonempty; the conditions for revisiting the choice |

```markdown
> **Decision `decision-separate-check-and-repair`** (draft): **Keep checking separate from repair**
> Choice: The checker observes and reports; repairs are separate actions.
> Requirements: requirement `requirement-observation-only`
> Rationale: A caller can inspect defects without changing the inputs being diagnosed.
> Alternatives:
> - Automatic repair inside check mixes diagnosis with changes.
> Reconsider when: Managed writes have an agreed ownership and conflict model; add them through a separate operation.
```

`Requirements: none` explicitly declares that the decision is not derived from a
recorded requirement; its rationale must explain its independent basis. A
decision can itself provide authority for an anchor. The syntax does not invent a
requirement merely to fill a reference.

## Validating authority relationships

Collect records from the Book's chapter inventory into one typed index, then
resolve references. Use the reader shared with anchor definitions and retain
source locations for diagnostics. This is independent of chapter order, rendered
HTML, and manually maintained navigation links.

The defined relationships are:

| Source | Reference field | Required target |
| --- | --- | --- |
| Anchor | `Authority:` reference with kind `requirement` | Requirement with the given identifier |
| Anchor | `Authority:` reference with kind `decision` | Decision with the given identifier |
| Decision | `Requirements:` reference | Requirement with the given identifier |

Requirements have no outgoing authority-reference field. Anchors still need at
least one direct authority reference. A decision's referenced requirements are
additional traceable relationships, not a reason to infer or rewrite the anchor's
authored references.

Validate the whole inventory, including unreferenced definitions:

- Every record must conform to its kind's fields. Invalid definitions cannot be
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
model evolves, updating the syntax and records together rather than allowing
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
