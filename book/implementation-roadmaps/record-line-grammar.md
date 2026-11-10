# Record Line Grammar

**Roadmap identifier:** `record-line-grammar`

## Goal

The engine and this Book use the goal [record syntax](../design/anchor-definitions.md#record-syntax),
a strict line grammar, as their only record format, with validation at least as
strict as today. Measure the syntax with agents first, so the design is confirmed
or revised on evidence before the migration.

## Gap

The engine and all of this Book's records use the earlier
[TOML form](../design/anchor-definitions.md#current-implementation). Searches
against this Book's sources show what that costs agents:

| Question | Result with TOML records |
| --- | --- |
| What cites `requirement-observation-only`? | 6 lines in three shapes: the definition, an anchor citation, a decision citation, and three lines from literal schema examples that look the same without reading the surrounding block |
| Which records are drafts? | 29 lines of `status = "draft"`, none stating the record's kind or identity |
| Which criteria require proof? | `verification = ["proof"]`, without its criterion or anchor identifier |

Proof scope is a writing convention inside a statement string that the engine
cannot check. The renderer needs no change beyond the inline-text
[open question](../design/anchor-definitions.md#open-questions): it replaces each
record's source range with HTML from the typed inventory and never reads TOML.

## Timing

Tabled on 2026-11-10: decide before
[Symposium Integration milestone 2](symposium-integration.md#symposium-integration-2),
which writes most of Symposium's records, so they are written once in the chosen
syntax. A mechanical conversion of this Book's 54 records then measured the line
grammar at 13% fewer bytes than TOML (about 2% of the Book); its larger benefit is
one-line identity and one spelling for every reference, which milestone 1 measures.

## Milestone 1. Measure how agents search each record syntax {#record-line-grammar-1}

**Outcome:** Recorded measurements show how many searches, lines read, and wrong matches agents need to answer fixed record questions in the goal line grammar, the current TOML form, and TOML with kind, identity, and status on the fence line, so the goal syntax is confirmed or revised on evidence.

**Status:** Proposed

**Dependencies:** None.

**Design basis:** [Record syntax](../design/anchor-definitions.md#record-syntax), [Authority Definitions](../design/authority-definitions.md), and the [gap](#gap)

**Identifier:** `record-line-grammar-1`

**Tasks**

- [ ] Produce three disposable copies of this Book's records and literal examples: current TOML, the goal line grammar, and TOML with kind, identity, and status on the fence line. Convert mechanically and check each copy preserves every identity, field, reference, and method.
- [ ] Fix the task set: find a record's definition; list every reference to a requirement; list draft anchors; list criteria requiring proof with their anchor; change one criterion's methods; add one authority reference.
- [ ] For the four search tasks, record the minimal searches for each variant, the lines matched, wrong matches, and further lines read to answer.
- [ ] Run the full task set in fresh Claude Code and Codex sessions against each variant, without engine tools. Record tool versions, tool calls, lines read, correctness, and edits that would fail validation.
- [ ] Record results and limits, answer or carry forward the design's open questions, and revise the record syntax in the design if the results warrant it.

### Completion criteria

#### Faithful variants {#record-line-grammar-1-faithful-variants}

Each variant represents the same records: a recorded comparison shows identical identities, kinds, statuses, fields, references, criteria, and methods across all three.

- **Anchor coverage:** None — experiment preparation
- **Evidence:** Pending.

#### Search measurements {#record-line-grammar-1-search-measurements}

For every search task and variant, the recorded searches, matched lines, wrong matches, and further reads are reproducible from committed scripts or listed commands.

- **Anchor coverage:** None — design experiment; informs [records-searchable-lines](../design/anchor-definitions.md#records-searchable-lines)
- **Evidence:** Pending.

#### Agent trials {#record-line-grammar-1-agent-trials}

Fresh-session results for each agent and variant record tool calls, lines read, answer correctness, and edit validity, with versions and any human assistance. Inconclusive or failed sessions remain in the record.

- **Anchor coverage:** None — design experiment
- **Evidence:** Pending.

#### Assessment {#record-line-grammar-1-assessment}

A written assessment compares the variants on searches, reads, wrong matches, and edit validity, states its limits, and recommends confirming or revising the goal syntax. Any revision is made in the design chapters, not here.

- **Anchor coverage:** None — decision input
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 2. The engine and this Book use the chosen syntax {#record-line-grammar-2}

**Outcome:** The record reader, renderer, fixtures, and this Book use the goal record syntax as the only record format, with validation at least as strict as today.

**Status:** Proposed

**Dependencies:** Record Line Grammar — Milestone 1, with any resulting design revision agreed.

**Design basis:** [Anchor Definitions](../design/anchor-definitions.md), [Authority Definitions](../design/authority-definitions.md), [Record Rendering](../design/record-rendering.md), and [Rust Engine](../design/rust-engine.md)

**Identifier:** `record-line-grammar-2`

**Tasks**

- [ ] Replace the TOML record reader with the line scanner, keeping record types and diagnostic codes; decide and implement inline-field presentation.
- [ ] Enforce the [identifier forms](../design/anchor-definitions.md#identifiers), the kind codes each reference field accepts, and the criterion references named in proof-scope [exclusions](../design/anchor-definitions.md#required-scope-of-a-proof-criterion). Rename every identifier to its kind-coded form in the same change, updating references, link fragments, evidence links, code comments, and workflow metadata that name them.
- [ ] Migrate this Book's records, literal examples, and test fixtures in the same change, and reject TOML record blocks as unsupported syntax.
- [ ] Repeat the authority proof's production-integration tests and review for the new conversion, and run mutation testing on the reader.
- [ ] Define completion criteria against [records-searchable-lines](../design/anchor-definitions.md#records-searchable-lines), [records-every-line](../design/anchor-definitions.md#records-every-line), [records-named-identifiers](../design/anchor-definitions.md#records-named-identifiers), and the existing record anchors before the milestone is assigned.

**Completion record:** Pending.
