# Record Line Grammar

**Roadmap identifier:** `record-line-grammar`

## Goal

The engine and this Book use the goal [record syntax](../design/anchor-definitions.md#record-syntax),
a strict line grammar, as their only record format, with validation at least as
strict as today. Measure the syntax with agents first, so the design is confirmed
or revised on evidence before the migration.

## Gap

Before [milestone 2](#record-line-grammar-2), the engine and this Book's records
used an earlier TOML form: each record a fenced block labeled `anchorbook-anchor`,
`anchorbook-requirement`, or `anchorbook-decision`, with lowercase identifiers
that carried no kind code and proof scope written as unchecked paragraphs inside
the statement. Searches against this Book's sources showed what that form cost
agents:

| Question | Result with TOML records |
| --- | --- |
| What cites `R-observation-only`? | 6 lines in three shapes: the definition, an anchor citation, a decision citation, and three lines from literal schema examples that look the same without reading the surrounding block |
| Which records are drafts? | 29 lines of `status = "draft"`, none stating the record's kind or identity |
| Which criteria require proof? | `verification = ["proof"]`, without its criterion or anchor identifier |

The renderer needed no change beyond rendering
[inline text](../design/record-rendering.md#reader-presentation): it replaces each
record's source range with HTML from the typed inventory.

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

- **Anchor coverage:** None — design experiment; informs [A-records-searchable-lines](../design/anchor-definitions.md#A-records-searchable-lines)
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

1. **TOML costs the most.** Its searches return lines that do not name their record: 41 matches and 148 lines read back for draft anchors, against 21 and none for the others. Claude Code read 2.7 times as many lines as with the line grammar, and Codex wrote a parser to list records, which [`R-searchable-records`](../authority/requirements.md#R-searchable-records) says agents must not need.
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

**Status:** Complete

**Identifier:** `record-line-grammar-2`

**Dependencies:** [Milestone 1](#record-line-grammar-1), complete. The reader, records, identifiers, and evidence links change in one change, since none of them works with the others half migrated.

**Design basis:** [Record syntax](../design/anchor-definitions.md#record-syntax), [Authority Definitions](../design/authority-definitions.md), [reader presentation](../design/record-rendering.md#reader-presentation), [internal boundaries](../design/rust-engine.md#internal-boundaries), and the [authority proof](../design/rust-engine.md#authority-proof).

**Tasks**

- [x] **Reader:** Replace `src/records/markdown.rs` and `schema.rs` with a Rust line scanner in `src/records/`, keeping the record types, inventory, and diagnostic codes, and using Markdown parsing only to find code regions.
- [x] **Validation:** Enforce line accounting, the identifier forms, the kind codes each reference field accepts, proof-scope items, and the criteria exclusions name. A TOML record block is an invalid definition at its fence that names the line grammar.
- [x] **Rendering:** Render the inline-text subset and identifier links in the preprocessor; record and milestone cards are otherwise unchanged.
- [x] **Migration:** Convert this Book's records and literal examples and the test fixtures, and rename every identifier to its kind-coded form in Book prose, link fragments, roadmap coverage, evidence links, code comments, and workflow metadata. The conversion is a one-time script outside the engine; the engine's reader validates its result.
- [x] **Proof:** Re-verify the authority proof and update the `production-integration` tests and review for the new conversion into candidates.
- [x] **Mutation testing:** Run it on the reader and rendering changes, and resolve every survivor.

### Completion criteria

#### Searchable records {#record-line-grammar-2-searchable}

For an independent fixture Book, heading searches return exactly the authored records with their kind, identity, status, and title, and a search for each identifier returns its definition and exactly its authored references, each matched line classified from its own text.

- **Anchor coverage:** [A-records-searchable-lines](../design/anchor-definitions.md#A-records-searchable-lines) · `heading-search`, `reference-search` — Full
- **Evidence:** Passed. `tests/record_syntax.rs` `a_heading_search_returns_every_record_with_its_kind_identity_status_and_title` applies the heading search line by line to an independent fixture Book of two requirements, two decisions, and two anchors, and reads each record's kind, identity, status, and title from the matched line alone; they equal the inventory's, and a search for draft decisions returns exactly one. `an_identifier_search_returns_its_definition_and_exactly_its_references` classifies every line matching each identifier from its own text: one definition, one reference line per citing record, and the one prose mention the fixture contains.

#### Every line read {#record-line-grammar-2-every-line}

Fixtures with stray, misordered, repeated, unknown, and missing lines, proof-scope faults, exclusions naming missing criteria, and TOML record blocks each produce a located invalid-definition diagnostic and no inventory; the corrected fixtures produce the expected records.

- **Anchor coverage:** [A-records-every-line](../design/anchor-definitions.md#A-records-every-line) · `line-accounting`, `proof-scope` — Full
- **Evidence:** Passed. `stray_misordered_repeated_unknown_and_missing_lines_are_located` and `proof_criteria_carry_their_scope_and_exclusions_name_real_criteria` in `tests/record_syntax.rs` each produce one invalid-definition diagnostic at the expected line for a stray line, a misordered, repeated, foreign, or missing field, an unexpected indented line, a missing, empty, repeated, or misordered scope item, exclusions naming a missing criterion of the same anchor, of another anchor, or of a missing anchor, a non-criterion code span in exclusions, and scope items on a criterion without proof; the unchanged fixture reads as the expected records. `tests/records.rs` adds TOML record blocks, nested and indented records, a lazy continuation line, two records in one quote, and malformed quote prefixes.

#### Named identifiers {#record-line-grammar-2-identifiers}

Fixtures with missing, unknown, lowercase, and mismatched kind codes, names starting with a digit, and reference fields citing the wrong kind produce located diagnostics; names with digits inside a word are accepted.

- **Anchor coverage:** [A-records-named-identifiers](../design/anchor-definitions.md#A-records-named-identifiers) · `identifier-forms`, `reference-codes` — Full
- **Evidence:** Passed. `identifiers_state_their_kind_and_never_a_number` rejects a missing, unknown, and lowercase kind code, an uppercase name, `D-12`, `R-2026-01`, an underscore, and trailing or doubled hyphens at the heading line, rejects a decision code on a requirement's fields at the first foreign field, and accepts `R-utf8-reads`. `reference_fields_accept_only_their_kinds_codes_even_when_the_record_exists` rejects an `Authority:` citing an existing anchor and a `Requirements:` citing an existing decision, each at the reference.

#### Validation kept {#record-line-grammar-2-validation-kept}

The invalid-inventory and authority tests pass against line-grammar fixtures, with every existing diagnostic code still produced for its case.

- **Anchor coverage:** [A-render-validated-records](../design/record-rendering.md#A-render-validated-records) · `invalid-inventory` — Full; [A-records-valid-authority](../design/rust-engine.md#A-records-valid-authority) · `required-authority`, `resolved-authority`, `compatible-status` — Full
- **Evidence:** Passed. `tests/records.rs` and `tests/authority_criteria.rs` were rewritten for the line grammar and keep every case: malformed records, missing, repeated, misordered, empty, and foreign fields, duplicate identities, and dangling, invalid, ambiguous, repeated, and status-incompatible references, each with its existing diagnostic code. A reference can no longer name a record of the wrong kind, since the identifier states the kind: a reference with the wrong code reports a missing target, and a reference to a kind the field does not accept is an invalid definition; the validator's own wrong-kind check is unchanged and still proved.

#### Proof and conversion {#record-line-grammar-2-proof}

The authority proof verifies the full crate with all seven functions and zero errors; the `production-integration` tests pass over the new conversion, a recorded review confirms the conversion keeps identities, kinds, statuses, references, and invalid candidates, and mutation testing of the reader leaves no unexplained survivor.

- **Anchor coverage:** [A-records-valid-authority](../design/rust-engine.md#A-records-valid-authority) · `formal-authority` — Partial: re-verification after the conversion change; `production-integration` — Partial: tests and review of the new conversion.
- **Evidence:** Passed. The authority proof, `cargo verus verify --fwd-verus-args-to roots --locked --features verus --lib -- --no-cheating`, reports 15 verified and 0 errors for the full crate, and `.github/verus-report.jq` accepts it with all seven functions; `src/records/authority.rs` is unchanged apart from its documentation. The `production-integration` tests in `tests/records.rs`, `tests/authority_criteria.rs`, and `tests/rendering.rs` pass over the new reader. **Review of the conversion** (by the agent that wrote it): the line reader returns one candidate per record and per rejected block; a candidate keeps the identity its heading states even when a later line is invalid, and has none when its heading or identifier is malformed or it is a TOML, nested, or legacy block, so references to it report invalid authority rather than a missing record. A decoded record's kind comes from its identifier's code, its status from its heading, and its references in authored order with their kinds from their codes; `inventory.rs`'s conversion into validator candidates is unchanged except that its kind match now names every kind, so the indices the validator reports still select the right record and reference. The reader calls the verified `has_required_authority` for anchors. **Mutation testing** (cargo-mutants 27.1.0) of `src/records/lines.rs`, `src/records/inventory.rs`, and `src/rendering/html.rs` first found 31 survivors and 5 timeouts: redundant conditions and unreachable match arms, which were removed; loop counters that could only time out, removed by scanning slices; an option mask built with `|`, now built with `insert`; diagnostic messages, CRLF ranges, a record directly after a fence, a proof criterion with no scope items, and exclusion forms that tests did not distinguish, now tested; and one defect, an exclusion naming a requirement's identifier with a criterion counted as found, now fixed. One equivalent change remains, excluded with its reason in `.cargo/mutants.toml`. A changed-lines campaign over the whole change then tested 222 mutations: 191 caught, 31 unviable, none missed or timed out. Limits: the review is by the implementing agent; an assessment record awaits [Verification Evidence milestone 6](verification-evidence.md#verification-evidence-6).

#### Rendered records {#record-line-grammar-2-rendered}

Through the actual adapter, line-grammar records render as the current cards, field text renders the inline subset with identifiers linked to their records and raw HTML inert, and this Book renders and passes rendered-Book acceptance.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `inline-text` — Full; `record-presentation` — Partial: line-grammar records keep their presentation.
- **Evidence:** Passed. Through the actual adapter, `field_text_renders_inline_markdown_and_links_record_identifiers` in `tests/rendering.rs` shows code spans, emphasis, strong emphasis, relative chapter links (rewritten to `.html`), external links, and autolinks formatted; code-spanned identifiers of records, including `<anchor>#<criterion>`, linked to their records with the title as a hint; unknown identifiers left as code; and raw HTML, other Markdown, images, `javascript:` and root-relative links left as literal text. `renders_required_methods_per_criterion_without_claiming_results` checks a proof criterion's scope renders as a labeled list. This Book's 56 records render, and rendered-Book acceptance passes, comparing every record field and proof scope with the inventory and resolving every local link.

#### Book migrated {#record-line-grammar-2-migrated}

This Book and the fixtures contain only line-grammar records and kind-coded identifiers; the structural check, `mdbook build`, and rendered-Book acceptance pass; and a whole-Book `verify` run reports no link findings and every claim it checks satisfied.

- **Anchor coverage:** None — migration acceptance
- **Evidence:** Passed. This Book's 56 records were converted by a one-time script, using the converter milestone 1 validated, and identifiers were renamed in Book prose, link fragments, roadmap coverage, the evidence links in 17 Rust source and test files, the authority module's documentation, and the proof workflow's evidence metadata; one record's link fragment and one exclusion's code span were fixed by hand. The structural check, `mdbook build`, current-Book and rendered-Book acceptance, and `scripts/check-engine-independence.sh` pass. `anchorbook verify --config anchorbook.toml` exits 0 with no link findings: with this milestone Complete, 39 claims, 34 satisfied and 5 unchecked; 67 criteria, 41 satisfied.

### Completion record

Complete on 2026-11-11 on branch `record-line-grammar`, not yet integrated. The engine reads the line grammar as its only record format, this Book and its fixtures use it with kind-coded identifiers, and records render with formatted inline text and linked identifiers. The conversion review is the implementing agent's.
