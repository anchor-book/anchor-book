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
verification = ["test"]
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
| `criteria` | Nonempty sequence of `[[criteria]]` tables, each with exactly `id`, `statement`, and `verification` |

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
the anchor identifier and criterion identifier together.

### Verification methods

Every criterion requires a nonempty `verification` array of distinct strings.
The only values are `"proof"`, `"test"`, and `"review"`, with exactly that spelling.
Every listed method is required; the list is not a choice between alternatives.
Order has no semantic significance and is preserved for presentation. Missing,
empty, repeated, unknown, or incorrectly typed methods are invalid definitions.

| Value | Required evidence |
| --- | --- |
| `"proof"` | Formal verification of the relevant production code against an explicit specification expressing the criterion, with assumptions and coverage stated. |
| `"test"` | Executed tests whose assertions derive from the criterion, with inputs, results, and coverage recorded. |
| `"review"` | A recorded assessment against the criterion, identifying what was examined, the conclusion, and limits. |

Different criteria within an anchor may require different methods. Listing proof
does not make an anchor accepted or verified. Methods declare obligations; parsing
and rendering them do not run tools or establish evidence. Concrete tool choices
and commands belong in the verification design and roadmap, not in this field.

For example, one criterion can require a proof of the validation algorithm while
another requires tests and review of its integration. If a method covers only a
portion, identify that portion and the remaining obligations explicitly; partial
evidence does not satisfy the whole criterion.

### Does an anchor require formal verification?

Determine this from the criterion declarations, for the current anchor revision:

| Declaration | Meaning |
| --- | --- |
| At least one criterion includes `"proof"` | Formal verification is required for every criterion that lists it, within that criterion's stated scope. |
| Every criterion has a valid method list and none includes `"proof"` | Formal verification is not required; the listed tests and reviews remain required. |
| A method list is missing or invalid | The definition is invalid. Do not interpret missing information as “proof not required.” |

Do not add a separate anchor-level proof flag or infer proof requirements from
the anchor's title, acceptance status, or the presence of proof code. The method
lists are the single source of truth. Requiring proof for one criterion does not
mean the entire anchor is formally proved, nor does it describe a completed run.

Choose methods during design review. Consider the consequences of a violation,
the combinations or states that examples would need to cover, whether the claim
can be stated precisely over actual production code, and proof maintenance cost.
Record the rationale in the verification design. Tests and proof can complement
each other; extending proof across every boundary is not an automatic obligation.

### Required scope of a proof criterion

For each criterion listing `"proof"`, its `statement` must use labeled paragraphs
to make the following explicit:

| Label | Required content |
| --- | --- |
| **Claim** | The property the proof must establish, including acceptance and rejection when both are required. |
| **Domain** | The inputs, states, conditions, and bounds covered. Include relevant invalid cases; do not assume the property being proved. |
| **Boundary** | The production behavior covered, such as the validation algorithm or the path from typed input to the public API result. |
| **Assumptions** | Caller obligations and trusted components or contracts. Preconditions must not assume the claim itself. |
| **Exclusions** | Behavior outside the proof, with the criterion IDs that cover it through tests, review, or another proof. An uncovered obligation remains an explicit gap. |

Use these concrete boundaries rather than a generic proof “level.” Keep formal
syntax, tool versions, commands, and function/file associations in the implementing
codebase and verification design; the anchor states the required assurance.
Exclusions must not silently remove part of the anchor's property.

The [authority anchor](rust-engine.md#engine-valid-authority) demonstrates this:
`formal-authority` requires a proof of the authority algorithm, while
`production-integration` requires tests and review of the surrounding boundaries.

These are authoring and review requirements within the existing `statement`
string, not new TOML fields. The current engine validates method declarations and
nonempty statements; it does not assess whether proof scope is adequately stated
or whether evidence satisfies it.

### Displaying verification requirements

The renderer displays **Required verification** beside each criterion. It must
not turn these selections into success badges or inferred results.

### Writing properties for verification

State the behavior, relevant conditions, and required outcome precisely in
`property`; define terms in the surrounding design. Criteria state what evidence
must establish. Keep implementation steps, tool commands, and proof mechanics in
their owning design and roadmap. The working contract defines the agent's
[verification responsibilities](../ai-working-contract.md#develop-verification-with-the-code).

Formal specifications, production code, and supporting proofs live together in
the implementing codebase. Associate each specification with the anchor and
criterion IDs it addresses. A source-association syntax is still experimental;
do not invent new anchor fields for file paths, proof text, results, or timestamps.
Assumptions that narrow the intended property require design review; assumptions
about unverified implementation boundaries must remain visible in evidence.

### Value validation and schema evolution

Whitespace-only strings are empty. Do not coerce strings to numbers, infer
missing values from surrounding prose, or accept misspelled status values.
Unknown schema versions are errors. Changes to these rules require an explicit
schema revision rather than parser leniency. During bootstrap, an unpublished
draft can be revised in place with its records; this does not require a new
version number for every design iteration. The [authority schemas](authority-definitions.md#evolving-the-authority-model)
remain provisional and will evolve through use.

This unpublished schema-1 revision adds required criterion methods. Existing
records must supply them explicitly; there is no default, legacy fallback, or
inference from the statement. The Book, parser, renderer, and fixtures migrate
together. This is not a published-schema compatibility promise.

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
