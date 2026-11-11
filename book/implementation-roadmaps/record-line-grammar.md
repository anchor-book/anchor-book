# Record Line Grammar

**Roadmap identifier:** `record-line-grammar`

## Goal

The engine and this Book use the goal [record syntax](../design/anchor-definitions.md#record-syntax),
a strict line grammar, as their only record format, with validation at least as
strict as today. Measure the syntax with agents first, so the design is confirmed
or revised on evidence before the migration.

## Gap

The engine and all of this Book's records use the earlier TOML form:

- **Records.** A top-level fenced block labeled `anchorbook-anchor`,
  `anchorbook-requirement`, or `anchorbook-decision`, whose TOML body carries
  `schema-version = 1`, `id`, `title`, `status`, and the kind's fields
  (`property`, inline-table `authorities`, and `[[criteria]]` with `id`,
  `statement`, and `verification` for anchors). Proof scope is written as
  labeled paragraphs inside the statement and is not checked.
- **Identifiers.** Lowercase, because the reader accepts no kind code:
  requirements and decisions carry `requirement-` and `decision-` prefixes and
  anchors have none. Name new records by the
  [identifier rules](../design/anchor-definitions.md#identifiers) in that form,
  so the migration only replaces the prefix with the kind code, such as
  `decision-separate-check-and-repair` becoming `D-separate-check-and-repair`,
  and adds `A-` to each anchor.
- **Reader.** `src/records/markdown.rs` and `src/records/schema.rs`. Milestone 2
  replaces them with the line reader, keeping `toml_edit` for configuration, and
  migrates the reader, fixtures, records, and identifiers together; TOML record
  blocks are then rejected as unsupported syntax.

Searches against this Book's sources show what that form costs agents:

| Question | Result with TOML records |
| --- | --- |
| What cites `requirement-observation-only`? | 6 lines in three shapes: the definition, an anchor citation, a decision citation, and three lines from literal schema examples that look the same without reading the surrounding block |
| Which records are drafts? | 29 lines of `status = "draft"`, none stating the record's kind or identity |
| Which criteria require proof? | `verification = ["proof"]`, without its criterion or anchor identifier |

Proof scope is a writing convention inside a statement string that the engine
cannot check. The renderer needs no change beyond rendering
[inline text](../design/record-rendering.md#reader-presentation): it replaces each
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

**Status:** Complete

**Dependencies:** None.

**Design basis:** [Record syntax](../design/anchor-definitions.md#record-syntax), [Authority Definitions](../design/authority-definitions.md), and the [gap](#gap)

**Identifier:** `record-line-grammar-1`

**Tasks**

- [x] Produce three disposable copies of this Book's records and literal examples: current TOML, the goal line grammar, and TOML with kind, identity, and status on the fence line. Convert mechanically and check each copy preserves every identity, field, reference, and method.
- [x] Fix the task set: find a record's definition; list every reference to a requirement; list draft anchors; list criteria requiring proof with their anchor; change one criterion's methods; add one authority reference.
- [x] For the four search tasks, record the minimal searches for each variant, the lines matched, wrong matches, and further lines read to answer.
- [x] Run the full task set in fresh Claude Code and Codex sessions against each variant, without engine tools. Record tool versions, tool calls, lines read, correctness, and edits that would fail validation.
- [x] Record results and limits, answer or carry forward the design's open questions, and revise the record syntax in the design if the results warrant it.

### Completion criteria

#### Faithful variants {#record-line-grammar-1-faithful-variants}

Each variant represents the same records: a recorded comparison shows identical identities, kinds, statuses, fields, references, criteria, and methods across all three.

- **Anchor coverage:** None — experiment preparation
- **Evidence:** Passed on branch `record-line-grammar`, based on `06725a0`. `scripts/record-syntax/variants.py` writes a copy of every chapter in each variant under `target/record-syntax/`, covering 56 records (29 anchors, 14 requirements, 13 decisions, 65 criteria) and the 4 literal examples, then reads each copy back: identities, kinds, statuses, titles, fields, references, criteria, and methods are identical to this Book's records in all three. The `toml` copy keeps every record byte for byte, converts only the examples, and builds through the engine's preprocessor; the `fence` copy changes only the fence, `id`, and `status` lines. Book source is 477,097 bytes as `toml`, 476,035 as `fence`, and 468,201 as `lines`. Limits: the comparison normalizes whitespace within paragraphs, since the line grammar puts each paragraph on one line; proof scope comes from the statement's labeled paragraphs; the `lines` copy renames identifiers in code spans and link fragments throughout, except in the sentence that states the rename; every copy's design chapters still describe the line grammar.

#### Search measurements {#record-line-grammar-1-search-measurements}

For every search task and variant, the recorded searches, matched lines, wrong matches, and further reads are reproducible from committed scripts or listed commands.

- **Anchor coverage:** None — design experiment; informs [records-searchable-lines](../design/anchor-definitions.md#records-searchable-lines)
- **Evidence:** Passed. `scripts/record-syntax/searches.py` runs one search per task and variant over the copies, for the tasks in `scripts/record-syntax/tasks.md`, classifies each matched line from the copy's record index, and counts the further lines above a match an agent must read to name the record or criterion. Every search returns its complete answer; the table lists matched lines, then how many are the answer, other record lines, literal examples, and prose, and the further lines read.

  | Task | `toml` | `fence` | `lines` |
  | --- | --- | --- | --- |
  | Definition of a requirement | 2: 1 answer, 1 example; 0 read | 2: 1, 1; 0 | 2: 1, 1; 0 |
  | Records citing it | 6: 2 answers, 1 definition, 3 examples; 15 read | 4: 2, 2 examples; 15 | 10: 2, 1 definition, 3 examples, 4 prose; 4 |
  | Draft anchors | 41: 20 answers, 17 other kinds' drafts, 3 examples, 1 prose; 148 read | 21: 20, 1 example; 0 | 21: 20, 1 example; 0 |
  | Proof criteria with their anchor | 2: 1 answer, 1 example; 31 read | 2: 1, 1; 31 | 2: 1, 1; 6 |

  Limits: one Book and one target record per task; one minimal search per variant, written by the agent that wrote the scripts; "further lines read" counts lines back to the needed fact, not what an agent actually opens, which the agent trials measure.

#### Agent trials {#record-line-grammar-1-agent-trials}

Fresh-session results for each agent and variant record tool calls, lines read, answer correctness, and edit validity, with versions and any human assistance. Inconclusive or failed sessions remain in the record.

- **Anchor coverage:** None — design experiment
- **Evidence:** Passed on 2026-11-11. `scripts/record-syntax/trials.py run` started six fresh non-interactive sessions, Claude Code 2.1.319 (model Opus 5.5) and Codex 0.167 each against each variant, every one in its own copy of the variant's `book/` outside any repository, so none could see the record index or the scripts. Each received only the prompt in `trials.py`: the six tasks in `tasks.md`, with identifiers in the variant's form, no syntax explanation, and no engine tools; Claude Code was limited to its read, search, and edit tools, and Codex ran in its workspace sandbox. `trials.py score` scores the answers against this Book's records and reads each edited copy back: an edit is valid only when every record matches this Book's with exactly the two requested changes. A diff of each copy shows no other change.

  | Session | Seconds | Tool calls | Lines returned by tools | Answers correct | Edits valid |
  | --- | --- | --- | --- | --- | --- |
  | Claude Code, `toml` | 36 | 13 | 1,052 | 4 of 4 | yes |
  | Claude Code, `fence` | 42 | 14 | 451 | 4 of 4 | yes |
  | Claude Code, `lines` | 45 | 13 | 388 | 4 of 4 | yes |
  | Codex, `toml` | 80 | 8 | 2,125 | 4 of 4 | yes |
  | Codex, `fence` | 58 | 7 | 1,672 | 4 of 4 | yes |
  | Codex, `lines` | 57 | 8 | 1,879 | 4 of 4 | yes |

  With TOML, Claude Code read both authority chapters whole to connect references to records; with the other variants its searches answered most tasks from matched lines. Codex read the record-syntax chapters in full before every task set, which dominates its counts; with TOML it wrote its own Python TOML parser to list records. Every session distinguished the literal examples from records. No human assistance. Limits: one session per agent and variant, so differences of tens of percent are within the variation of repeated runs; one Book whose design chapters explain the line grammar to every variant; tasks small enough that every session was fully correct, so correctness does not separate the variants.

#### Assessment {#record-line-grammar-1-assessment}

A written assessment compares the variants on searches, reads, wrong matches, and edit validity, states its limits, and recommends confirming or revising the goal syntax. Any revision is made in the design chapters, not here.

- **Anchor coverage:** None — decision input
- **Evidence:** Recorded in the milestone evidence below. The engineer confirmed the line grammar on 2026-11-11.

### Evidence {#record-line-grammar-1-evidence}

**Assessment.** All three syntaxes let agents answer and edit correctly at this Book's size; they differ in what an agent must read.

1. **TOML costs the most.** Its searches return lines that do not name their record: 41 matches and 148 lines read back for draft anchors, against 21 and none for the others. Claude Code read 2.7 times as many lines as with the line grammar, and Codex wrote a parser to list records, which [`requirement-searchable-records`](../authority/requirements.md#requirement-searchable-records) says agents must not need.
2. **The fence line gives most of the gain.** Putting kind, identity, and status on the fence line matches the line grammar for finding definitions and listing records by status, and Claude Code read 451 lines against 388.
3. **The line grammar gains the rest.** Tracing a match back to its record or criterion takes 4 lines instead of 15 for references and 6 instead of 31 for proof criteria, because each criterion is one line. It matches more prose (4 mentions for one requirement), since identifiers in prose are spelled as in records; those lines are references the design means a search to find.
4. **Unmeasured differences.** Only the line grammar gives proof scope a checked form, identifiers a kind code, and references one spelling. The trials do not measure these, and its migration is larger: a new reader and every identifier renamed, against a change to fence recognition alone.

**Recommendation.** Confirm the line grammar as the goal syntax. The evidence supports it over TOML clearly and over fence-line TOML modestly; the deciding reasons are the checked proof scope and one spelling per reference, which fence-line TOML cannot give without becoming a new syntax itself. Fence-line TOML is the fallback if the migration's cost outweighs those.

**Open questions.** (1) Inline Markdown in fields and (2) the `Property:` label were not exercised; the design now settles both for milestone 2: field text renders a defined inline subset, and `Property:` is required. (3) The one proof criterion converted into a statement and the four scope items without loss, supporting the shape. (4) Literal examples matched every search in every variant, 1 to 3 lines each, and no session mistook one for a record; no distinct example form is needed for now.

**Limits.** One Book of 56 records, one target record per search, one session per agent and variant, and scripts and searches written by one agent.

### Completion record

Complete on 2026-11-11 on branch `record-line-grammar`, not yet integrated. The variants, measurements, agent trials, and assessment are recorded, and the engineer confirmed the line grammar as the goal syntax; no syntax revision was needed.

## Milestone 2. The engine and this Book use the line grammar {#record-line-grammar-2}

**Outcome:** The engine's Rust record reader reads the line grammar as the only record format, with validation at least as strict as today; this Book, its fixtures, and its evidence links use kind-coded identifiers; and the preprocessor renders records with formatted inline text and identifiers linked to their records.

**Status:** Proposed

**Identifier:** `record-line-grammar-2`

**Dependencies:** [Milestone 1](#record-line-grammar-1), complete. The reader, records, identifiers, and evidence links change in one change, since none of them works with the others half migrated.

**Design basis:** [Record syntax](../design/anchor-definitions.md#record-syntax), [Authority Definitions](../design/authority-definitions.md), [reader presentation](../design/record-rendering.md#reader-presentation), [internal boundaries](../design/rust-engine.md#internal-boundaries), and the [authority proof](../design/rust-engine.md#authority-proof).

**Tasks**

- [ ] **Reader:** Replace `src/records/markdown.rs` and `schema.rs` with a Rust line scanner in `src/records/`, keeping the record types, inventory, and diagnostic codes, and using Markdown parsing only to find code regions.
- [ ] **Validation:** Enforce line accounting, the identifier forms, the kind codes each reference field accepts, proof-scope items, and the criteria exclusions name. A TOML record block is an invalid definition at its fence that names the line grammar.
- [ ] **Rendering:** Render the inline-text subset and identifier links in the preprocessor; record and milestone cards are otherwise unchanged.
- [ ] **Migration:** Convert this Book's records and literal examples and the test fixtures, and rename every identifier to its kind-coded form in Book prose, link fragments, roadmap coverage, evidence links, code comments, and workflow metadata. The conversion is a one-time script outside the engine; the engine's reader validates its result.
- [ ] **Proof:** Re-verify the authority proof and update the `production-integration` tests and review for the new conversion into candidates.
- [ ] **Mutation testing:** Run it on the reader and rendering changes, and resolve every survivor.

### Completion criteria

#### Searchable records {#record-line-grammar-2-searchable}

For an independent fixture Book, heading searches return exactly the authored records with their kind, identity, status, and title, and a search for each identifier returns its definition and exactly its authored references, each matched line classified from its own text.

- **Anchor coverage:** [records-searchable-lines](../design/anchor-definitions.md#records-searchable-lines) · `heading-search`, `reference-search` — Full
- **Evidence:** Pending.

#### Every line read {#record-line-grammar-2-every-line}

Fixtures with stray, misordered, repeated, unknown, and missing lines, proof-scope faults, exclusions naming missing criteria, and TOML record blocks each produce a located invalid-definition diagnostic and no inventory; the corrected fixtures produce the expected records.

- **Anchor coverage:** [records-every-line](../design/anchor-definitions.md#records-every-line) · `line-accounting`, `proof-scope` — Full
- **Evidence:** Pending.

#### Named identifiers {#record-line-grammar-2-identifiers}

Fixtures with missing, unknown, lowercase, and mismatched kind codes, names starting with a digit, and reference fields citing the wrong kind produce located diagnostics; names with digits inside a word are accepted.

- **Anchor coverage:** [records-named-identifiers](../design/anchor-definitions.md#records-named-identifiers) · `identifier-forms`, `reference-codes` — Full
- **Evidence:** Pending.

#### Validation kept {#record-line-grammar-2-validation-kept}

The invalid-inventory and authority tests pass against line-grammar fixtures, with every existing diagnostic code still produced for its case.

- **Anchor coverage:** [render-validated-records](../design/record-rendering.md#render-validated-records) · `invalid-inventory` — Full; [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `required-authority`, `resolved-authority`, `compatible-status` — Full
- **Evidence:** Pending.

#### Proof and conversion {#record-line-grammar-2-proof}

The authority proof verifies the full crate with all seven functions and zero errors; the `production-integration` tests pass over the new conversion, a recorded review confirms the conversion keeps identities, kinds, statuses, references, and invalid candidates, and mutation testing of the reader leaves no unexplained survivor.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority` — Partial: re-verification after the conversion change; `production-integration` — Partial: tests and review of the new conversion.
- **Evidence:** Pending.

#### Rendered records {#record-line-grammar-2-rendered}

Through the actual adapter, line-grammar records render as the current cards, field text renders the inline subset with identifiers linked to their records and raw HTML inert, and this Book renders and passes rendered-Book acceptance.

- **Anchor coverage:** [render-records-in-context](../design/record-rendering.md#render-records-in-context) · `inline-text` — Full; `record-presentation` — Partial: line-grammar records keep their presentation.
- **Evidence:** Pending.

#### Book migrated {#record-line-grammar-2-migrated}

This Book and the fixtures contain only line-grammar records and kind-coded identifiers; the structural check, `mdbook build`, and rendered-Book acceptance pass; and a whole-Book `verify` run reports no link findings and every claim it checks satisfied.

- **Anchor coverage:** None — migration acceptance
- **Evidence:** Pending.

### Completion record

Pending.
