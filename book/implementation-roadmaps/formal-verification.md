# Formal Verification

**Roadmap identifier:** `formal-verification`

## Goal

Prove the authority-validation logic actually executed by the engine, and make
that proof repeatable locally and on GitHub. The engine is the first and current
subject: its own proof is exercised here before any other project relies on
proof support. Start with
[`records-valid-authority`](../design/rust-engine.md#records-valid-authority), grounded
in [traceable properties](../authority/requirements.md#requirement-traceable-properties).
Rust Engine's [authority proof](../design/rust-engine.md#authority-proof) defines the
claim, production boundary, and trusted base, and [Verification](../design/verification.md)
the general method and evidence. Make Verus optional: the engine
and every project that depends on it build without Verus or `vstd`, the engine's
proof builds behind an optional Cargo feature, and the verified functions remain
the ones ordinary builds execute.

The roadmap uses the engine as the first experiment in associating anchor
criteria with formal specifications, production functions, and proof evidence.
The [Verification Evidence roadmap](verification-evidence.md) owns reusable
source associations, consumer tool execution, and criterion reports. Engine
proofs supply evidence for this Book; they never establish a consumer's criteria.

The [proof-requirement convention](../design/anchor-definitions.md#does-an-anchor-require-formal-verification)
determines which criteria require proof. This roadmap's `formal-authority` criterion
now states its claim, domain, boundary, assumptions, and exclusions explicitly;
`production-integration` retains tests and review for those excluded boundaries.
The declaration is not evidence that either criterion is complete.

This roadmap owns the anchor's `formal-authority` and `production-integration`
criteria; [Record Rendering milestone 3](record-rendering.md#record-rendering-3-authority-coverage)
owns its three behavioral test criteria.

The sequence is **local production-code proof, GitHub proof execution, then merge
protection**. Merge protection is completed together with
[Mutation Testing milestone 2](mutation-testing.md#mutation-testing-2). Until it is
enforced, review existing CI results before merging; unfinished enforcement does
not waive tests or permit unexplained mutation findings.

Milestone 4 makes Verus optional and starts with a bounded feasibility
experiment. It does not depend on the consumer runner. Any project that adds the
engine as a Cargo dependency depends on it, whether or not that project uses
Verus, and so does [consumer acceptance](verification-evidence.md#verification-evidence-5). Existing proof evidence remains tied to its recorded revision; packaging
changes require new proof and integration evidence.

## Milestone 1. Prove production authority validation locally {#formal-verification-1}

**Outcome:** Authority validation has a reproducible local proof over the executable functions used by the record API, with tested integration and explicit limits.

**Status:** Complete — local and CI proof, integration tests, and specification assessment recorded

**Identifier:** `formal-verification-1`

**Design basis:** [Authority proof](../design/rust-engine.md#authority-proof), [authority rules](../design/authority-definitions.md#validating-authority-relationships), and [records-valid-authority](../design/rust-engine.md#records-valid-authority).

**Dependencies:** The implemented record reader and renderer, existing independent tests, and reviewed mutation findings. Merge protection is deferred. Review the specification against the anchor before proving the extracted implementation.

**Scope:** One local proof of the authority validator used by `read_records`, covering every anchor authority and decision requirement. Parser correctness, diagnostic presentation proofs, GitHub proof jobs, merge protection, and generalized evidence tooling are outside this task.

**Tasks**

- [x] Check feasibility: pin compatible Verus, its Rust toolchain, and vstd; verify and compile a small representative validator slice using the required types and collection operations. Preserve Rust 1.85 and stable builds. Review any necessary package or minimum-version change before the full refactor.
- [x] Define the formal acceptance predicate from the authority rules and review it against the design's input matrix. Include invalid relationships in the domain; prove both acceptance and rejection. State assumptions and associate the specification with the anchor and criterion IDs.
- [x] Extract lookup and authority checks into `src/records/authority.rs` and develop the proof alongside the executable functions. Include required-authority checks currently in `schema.rs`. Keep function contracts beside the implementation; place reusable lemmas in supporting modules only when useful.
- [x] Connect `inventory.rs` to those same functions. Test faithful candidate conversion and failure propagation through the library and actual mdBook adapter, preserving identities, ambiguity, invalid candidates, status, references, and located diagnostics.
- [x] Verify the complete selected proof scope. Introduce one deliberate rule violation in a disposable copy and confirm a proof obligation fails; restore the code and verify again. Run focused integration and mutation checks, explicitly checking whether cargo-mutants reaches the annotated executable bodies.
- [x] Document the pinned local command and the proved claim, assumptions, tested boundaries, revision, and result. Reconcile the design and milestone evidence without claiming the entire engine or anchor is proved.
- [x] Assess the formal specification against the explicit proof scope (a specification assessment) and map production-integration tests and review to every excluded boundary. Record any remaining coverage gaps before accepting the milestone.

### Completion criteria

#### Authority proof {#formal-verification-1-authority-proof}

The production validator verifies acceptance if and only if the defined authority rules hold over the complete typed input. Preconditions do not assume valid authority relationships. The run identifies the verified functions; a deliberate rule violation fails a proof obligation, and the restored code passes.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority` — Full. The claim is limited to the specified authority logic and recorded assumptions.
- **Evidence:** Passed: local proof, the deliberate-defect failure, the current main proof run, and the specification assessment; see milestone evidence.

#### Production path {#formal-verification-1-production-path}

Review identifies the same verified functions on the normal read_records call path. Independent fixtures exercise conversion, empty and invalid authority, all resolution/status cases, and rejection through the real adapter. State which boundaries are tested rather than proved.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `production-integration` — Full. Behavioral test coverage also contributes to `required-authority`, `resolved-authority`, and `compatible-status`; reconcile with Record Rendering milestone 3.
- **Evidence:** Passed locally; see milestone evidence.

#### Compatible tooling {#formal-verification-1-tooling}

Pinned tools reproduce verification without rendering the Book. Ordinary builds, tests, formatting, and Clippy pass on supported toolchains. Record whether mutation testing reaches the annotated executable bodies and any gaps; a skipped or empty mutation scope is not coverage.

- **Anchor coverage:** None — developer tooling and compatibility; proof success alone does not complete the mutation roadmap.
- **Evidence:** Passed locally; see milestone evidence.

### Evidence {#formal-verification-1-evidence}

**Latest local result:** On 2026-11-10, Verus `0.2026.10.18.7d82994` and Z3 4.16.0, built from source at their pinned tags on Linux, verified the revised specification on branch `book-upkeep`: 15 verified, zero errors, full crate, all seven functions, accepted by `.github/verus-report.jq`, in about 30 seconds. A deliberate defect that reports a reference failure without its reference index failed a loop invariant; the restored source verified. Ordinary tests, formatting, and Clippy pass on Rust 1.88.0 and stable 1.99.1.

**First local result:** Working tree based on `45125f6`, tested 2026-10-21. Verus `0.2026.10.18.7d82994`, verifier Rust 1.99.1, and bundled Z3 4.16.0 report 15 verification units passed, zero errors. All seven executable authority functions appear in the report. The native command and exactly pinned library are documented in CONTRIBUTING; no verification wrapper script is required.

**Scope and assumptions:** The production `validate_authorities` function satisfies its acceptance biconditional over complete typed candidates. Project assumptions and trusted function bodies are prohibited by `--no-cheating`; standard-library specifications, verifier/solver, and compiler remain trusted. Parsing, conversion, diagnostics, and backlinks are tested boundaries. Review the specification's interpretation of the anchor before accepting the milestone.

**Failure demonstration:** Reversing the accepted-target status condition in a disposable source copy caused a postcondition failure: 14 verification units passed, one failed. The original source was unchanged and its proof passed again. Independent tests cover all authority outcomes and an uncited decision failing through the real adapter. Tests and Clippy pass on Rust 1.85 and local stable 1.90.

**Mutation assessment:** cargo-mutants 27.1.0 discovers no mutations inside `verus!`, so automatic mutation coverage of the validator is unavailable. The focused inventory integration run initially found four diagnostic assertion gaps; after strengthening message and related-location assertions, all 24 selected mutations were attempted: 22 caught, two unviable, no survivors or timeouts. Command: `target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/records/inventory.rs --output target/verus-mutations-fixed`. This is scoped evidence, not a new whole-engine campaign. Raw reports remain ignored under target. Milestone 2 records subsequent remote proof results.

**Specification assessment:** Accepted. A first assessment of `src/records/authority.rs` at `cea6ba3`, verified on main by the [Formal Verification](https://github.com/anchor-book/anchor-book/actions/runs/42741006859) workflow, accepted its claim and found that `inventory.rs` relied on violation properties the contract did not state. The contract now states them in `violation_is_located`, an added `ensures` on `validate_authorities`, and `validate_reference`'s guarantee that it never reports missing authority. Because that changed the specification, a fresh agent that wrote none of it assessed the revised specification on 2026-11-10 from the code, the criterion, and the design, against `formal-authority`'s claim, domain, boundary, assumptions, and exclusions, and accepted it. There is no assessment-record file yet, because [assessment records](verification-evidence.md#verification-evidence-6) are not implemented.

- **Claim.** `validate_authorities` ensures `violations.len() == 0` exactly when `records_satisfy_authority_rules` holds over the whole collection, so acceptance and rejection are both proved. The rules state each part of the claim: anchors need a reference; references within a record are distinct by identity, which also rejects one identity cited as two kinds; a reference resolves to exactly one candidate, counting undecodable candidates with a recovered identity; the target is decoded, of the declared kind, and accepted when the source is accepted. Every decoded record is checked, including decisions no anchor cites.
- **Domain.** `validate_authorities`, `has_required_authority`, `lookup`, `has_unknown_identity`, `matches_identity`, and `matches_target_kind` have no preconditions. `validate_reference` requires only valid record and reference indices, which its verified caller establishes; none assumes a valid relationship. Undecodable source records carry no reference obligations, matching the claim's "successfully decoded record"; their parse errors are kept by `read_records`.
- **Boundary.** The seven executable functions above are the ones `read_records` runs: `inventory.rs` passes every parser candidate, and `schema.rs` calls `has_required_authority` while decoding. There is no proof-only copy.
- **Assumptions.** The file contains no `assume`, `admit`, or trusted body, and `--no-cheating` is enforced. Trusted: the verifier, Z3, the compiler, and `vstd` contracts for string equality through the character-sequence view and for slice and vector indexing.
- **Scope notes.** Which source kinds may cite which target kinds (decisions cite only requirements; requirements cite nothing) is enforced by decoding, not by the specification; decoding is excluded and tested. A decoded target with invalid relationships of its own is still a suitable target, which does not weaken the result because that target is checked as a source.
- **Equivalent survivors.** None judged; sensitivity changes are planned in [Verification Evidence milestone 7](verification-evidence.md#verification-evidence-7).

**Exclusion coverage:** Each boundary that `formal-authority` excludes is covered by `production-integration` tests:

| Excluded boundary | Covering tests |
| --- | --- |
| Parsing | `tests/records.rs`: noncanonical fences, schema and field errors, Markdown containers and code boundaries, truncated and Unicode input |
| Candidate conversion | `tests/authority_criteria.rs`: every anchor status, target status, and target kind, and anchors citing one requirement or one decision; `tests/records.rs`: invalid candidates keep their identity and unknown identities block resolution; unit tests in `authority.rs` |
| Global identity checks | `tests/records.rs` `duplicates_have_no_winner_and_bad_authorities_do_not_look_missing`, including duplicates across kinds |
| Diagnostic classification and locations | `tests/authority_criteria.rs` reference-text locations for each failure; `tests/records.rs` codes, field locations, Unicode columns, and related locations |
| Inventory construction | `tests/records.rs` ranges, backlinks, and chapter-order independence; every failing case returns no inventory |
| Adapter failure propagation | `tests/rendering.rs` invalid inputs, `uncited_decision_authority_failure_reaches_the_real_adapter`, and `anchor_authority_failures_reach_the_real_adapter` |

**Integration guarantee:** `inventory.rs` indexes the violating record, expects it to be decoded, treats `MissingAuthority` as having no reference index, and indexes its references. The contract now proves these facts about the converted candidates. That they carry over to `inventory.rs`'s own records depends on the conversion keeping order, decoded state, and reference counts; that holds by construction and is exercised only through correctly located diagnostics, with no direct conversion test.

### Completion record

Complete on 2026-11-10. The production validator's proof establishes the `formal-authority` claim within its stated scope, an independent specification assessment accepts the revised specification, and each excluded boundary has covering tests. Conversion fidelity remains tested rather than proved.

## Milestone 2. GitHub verifies the production authority logic {#formal-verification-2}

**Outcome:** Changes receive repeatable remote verification with revision-specific proof and integration evidence, and the required check protects implementation merges.

**Status:** In progress — proof and integration pass on GitHub; failure demonstration and merge protection pending

**Identifier:** `formal-verification-2`

**Design basis:** [Authority proof](../design/rust-engine.md#authority-proof), [authority rules](../design/authority-definitions.md#validating-authority-relationships), and [records-valid-authority](../design/rust-engine.md#records-valid-authority).

**Dependencies:** Formal Verification milestone 1 and the existing PR workflows. Establish passing and failing proof runs first, then complete merge protection together with Mutation Testing milestone 2. Remote execution follows an authorized push.

**Tasks**

- [x] Add a separate GitHub verification job using the pinned tools and full selected proof scope, alongside ordinary engine tests. Verify actual function coverage and retain public-safe results on failure.
- [ ] Exercise a deliberate verification failure and correction through a PR; confirm that failed, skipped, or incomplete verification cannot count as success. The passing run is recorded below.
- [x] Record proof and production integration results for the same revision, including assumptions, tool versions, commands, function coverage, and evidence links.
- [ ] Add the observed verification check to the intended merge rules and demonstrate that failure blocks merging; preserve existing required tests.
- [ ] Demonstrate the agent correction loop without weakening the formal property, then update contributor instructions and roadmap evidence.

### Completion criteria

#### Remote proof {#formal-verification-2-remote-proof}

Actual GitHub results show the intended production functions verified for the recorded revision and a deliberate defect rejected. Retained results identify the specification, inputs, assumptions, toolchain, and scope.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority` — Partial: remote reproduction of milestone 1 proof evidence.
- **Evidence:** Passing PR and main-branch runs are recorded below: 15 verified units, zero errors, all seven required functions present. The deliberate-failure GitHub run remains pending; the local negative experiment does not complete this criterion.

#### Same revision {#formal-verification-2-same-revision}

Record passing integration tests and proof results for the same code revision; review evidence applicability when design, code, conversion, or dependencies change.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `production-integration` — Partial: remote integration evidence and revision alignment.
- **Evidence:** Passed: the retained artifact from [main verification](https://github.com/anchor-book/anchor-book/actions/runs/39737287057) identifies `03fb98136809d30caba3b9c4a04e7617035a294a`, with successful proof and ordinary engine/integration-test commands for that checkout. Scope and integration-coverage review remain in milestone 1.

#### Merge and correction loop {#formal-verification-2-merge-loop}

A real PR demonstrates a failed required verification check preventing merge, followed by an authorized correction and passing checks for the updated revision. Record the applied rule and any bypass policy.

- **Anchor coverage:** None — repository enforcement and agent workflow; a workflow definition is not enforcement.
- **Evidence:** Pending.

### Evidence {#formal-verification-2-evidence}

**Local checks:** Working tree based on `95fa70b`, checked 2026-10-21. actionlint 1.7.12 accepts the workflow. The official Linux archive checksum and tool paths are confirmed. A fresh macOS verification using the same command options passes 15 verification units with zero errors; the report check confirms all seven required executable functions.

**Report rejection:** Local experiments reject empty or malformed output, duplicate reports, a different verifier version, zero verified units, verification errors, partial-crate verification, and missing, failed, or non-executable required function results. Fresh dependency success summaries are handled without accepting arbitrary non-JSON output.

**Remote evidence:** [PR #2](https://github.com/anchor-book/anchor-book/pull/2) merged as `03fb981`. The [PR proof run](https://github.com/anchor-book/anchor-book/actions/runs/39735753233) passed for head `687a4bf`, testing merge revision `4e0d6c1`; [main verification](https://github.com/anchor-book/anchor-book/actions/runs/39737287057) passed for `03fb981`. Both report 15 verified units, zero errors, full-crate verification, and successful results for all seven required executable functions. The retained `authority-proof-results` artifacts identify revisions, commands, exit codes, tools, scope, and assumptions; ordinary engine/integration tests passed in the same jobs. Linux runs use the pinned Verus release, Rust 1.99.1 for verification and 1.85.0 for ordinary tests.

**Limits:** The deliberate-failure/correction GitHub run and merge enforcement remain pending. Passing execution does not complete milestone 1 scope and integration-coverage review.

### Completion record

Passing remote proof and integration are recorded; failure/correction demonstration, evidence review, and merge protection remain open.

## Milestone 3. Criteria declare their required verification methods {#formal-verification-3}

**Outcome:** Engineers and agents can read explicit proof, test, and review obligations on each criterion; the engine validates and renders those selections without claiming results.

**Status:** Complete — local schema implementation and verification

**Identifier:** `formal-verification-3`

**Design basis:** [Criterion methods](../design/anchor-definitions.md#verification-methods), [record presentation](../design/record-rendering.md#reader-presentation), and agent verification guidance (`anchorbook contract`, “Develop verification with the code”).

**Dependencies:** The existing record parser and renderer. This scoped schema change is assigned before the proof milestones; it does not assign the Verus refactor or settle its prerequisites.

**Tasks**

- [x] Define required criterion methods and clarify authoring and code/proof association guidance.
- [x] Add typed methods, strict parsing, and required-evidence labels; migrate Book records, examples, and fixtures.
- [x] Verify positive and negative cases, rendering, current-Book acceptance, toolchain compatibility, and mutation sensitivity; record results and limits.

### Completion criteria

#### Strict schema {#formal-verification-3-strict-schema}

The record API accepts every supported method and combinations in authored order; it rejects missing, empty, repeated, unknown, or incorrectly typed methods on any criterion with located diagnostics.

- **Anchor coverage:** None — criterion schema preparation; no proof or test result is inferred from a method.
- **Evidence:** Passed locally; see the milestone evidence below for checks and limits.

#### Presentation {#formal-verification-3-presentation}

The actual adapter displays every criterion’s methods as required verification, preserves design acceptance status, and rejects invalid methods without success JSON. The current Book and print view preserve the authored criteria and links.

- **Anchor coverage:** [render-records-in-context](../design/record-rendering.md#render-records-in-context) · `record-presentation` — Partial: the new required-method labels.
- **Evidence:** Passed locally; see the milestone evidence below for checks and limits.

#### Migration and guidance {#formal-verification-3-migration}

All Book definitions, literal examples, and fixtures use the same required field. The contract distinguishes obligations from results and guides production-code proof associations. Tests, supported-toolchain checks, rendered acceptance, and scoped mutation findings are recorded.

- **Anchor coverage:** None — schema migration and contributor guidance.
- **Evidence:** Passed locally; see the milestone evidence below for checks and limits.

### Evidence {#formal-verification-3-evidence}

*Schema preparation evidence*

**Revision and integration:** Uncommitted working tree on main based on `5c86d1a`, verified 2026-10-21. Schema version 1 is an unpublished draft revised together with its Book records and fixtures. The tested changes were subsequently committed as `7ab9616`.

**Strict schema:** Independent record tests accept each supported method and combined methods in authored order. They reject missing, empty, duplicate, unknown, incorrectly typed, aliased, and duplicate-key forms on the first and a later criterion, with located invalid-definition diagnostics. No proof artifacts are required to parse a declared obligation.

**Presentation and migration:** Actual-process adapter tests verify per-criterion labels, unchanged design status, and no success JSON for invalid methods. Current-Book acceptance checks the authority anchor's exact selections. The Book renders 25 complete records and its local links pass; browser review confirms readable required-method labels. Literal examples remain code. The working contract and verification design describe production-code proofs and reviewable assumptions without selecting a tool in the anchor.

**Commands and tools:** Ordinary tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. The suite contains 14 record tests, 7 rendering/process tests, 2 CLI tests, and 1 documentation test. Formatting, separate current-Book acceptance, `scripts/check-engine-independence.sh`, mdBook 0.5.2 build, and the rendered acceptance script with Python 3.12 pass. The engine remains usable with an absent or invalid Book.

**Mutation scope:** cargo-mutants 27.1.0 tested the changed method mapping, schema record validation, and record rendering functions with `target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/records/schema.rs --file src/records/model.rs --file src/rendering/html.rs --re 'VerificationMethod::as_str|::record ->|in Context.*::record|in record$' --output target/mutation-verification-methods`. The initial 15-mutation run had 13 caught, 1 missed, 1 unviable, and no timeouts. It reproduced the existing heading-label survivor: tests did not distinguish an anchor's Authority heading from a decision's Requirements heading. Added assertions scoped to each record. The repeat run passed in 45 seconds: 14 caught, 1 unviable, zero missed or timed out. The unviable mutation attempts to construct a Record with Default, which that type does not implement; it is not counted as detection.

**Supporting records and limits:** Tests and fixture changes accompany this implementation; raw mutation output remains ignored. This is a scoped campaign, not a rerun of the full engine baseline. Other findings remain in the Mutation Testing roadmap. No Verus proof, automatic evidence ingestion, remote result for this revision, or merge-readiness claim is established.

### Completion record

Complete locally on 2026-10-21. Required-method declarations are parsed, rendered, and exercised with independent and current-Book inputs. The schema change is committed as `7ab9616`; proof implementation and subsequent remote evidence are tracked in milestones 1 and 2.

## Milestone 4. Verus is an optional engine feature {#formal-verification-4}

**Outcome:** Ordinary builds of the engine, and of every project that depends on it, need no Verus toolchain or `vstd`; the engine's proof builds only with an optional Cargo feature, over the same maintained source that ordinary builds execute.

**Status:** Complete

**Identifier:** `formal-verification-4`

**Design basis:** [Proof packaging for consumers](../design/verified-code-packaging.md#proof-packaging-for-consumers), [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies), and [records-valid-authority](../design/rust-engine.md#records-valid-authority).

**Dependencies:** The existing production authority proof. Begin with a small source-arrangement experiment and review its feasibility before assigning the full packaging change. Registry publication, contract delivery, and the consumer runner are not prerequisites.

**Tasks**

- [x] Test one maintained source arrangement on a representative validator slice. A feature alone does not compile: code inside `verus!` needs Verus's macro crates even with the feature off. Candidates are (a) an optional `verus` Cargo feature with the validator written as ordinary Rust, its contracts attached through Verus's attribute syntax (`#[verus_verify]`, `#[verus_spec]`, present in the pinned release) under `cfg_attr`, and specification functions and proofs compiled only with the feature; and (b) a development proof workspace with reproducible erasure. For the chosen one, establish supported forms, tool availability, drift detection, and runtime correspondence.
- [x] Remove engine verification libraries from the default dependency graph; with the feature off, `cargo tree` shows no `vstd` or other Verus crate. Keep the authority implementation on the production path; relocating a still-required proof crate or maintaining a second validator does not establish isolation.
- [x] Preserve `fixtures/consumers/verus-resolution` as the consumer-resolution regression fixture: its exact `vstd` pin fails against the current engine and must resolve and build after isolation. Repeat with the packaged candidate and an ordinary consumer without Verus installed.
- [x] Verify the source corresponding to that package, run ordinary tests on the minimum and stable toolchains, and reject a deliberate authority defect before restoring a passing proof. Reassess conversion, callers, diagnostics, and any new generation boundary.
- [x] Record package contents and dependency review, source correspondence, tools, commands, tested revisions, proof and integration results, and remaining limits. Reconcile contributor commands and the proof workflow with the chosen arrangement.

### Completion criteria

#### Runtime correspondence {#formal-verification-4-runtime-correspondence}

Review identifies one maintained authority implementation and a reproducible relationship between verified and packaged functions. Independent tests and drift checks reject divergence; any erasure or conversion assumptions are explicit. A separately maintained proof-only validator cannot complete this criterion.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority`, `production-integration` — Partial: correspondence across the packaging boundary; specification acceptance remains in milestone 1.
- **Evidence:** Passed. There is one implementation: the seven functions in `src/records/authority.rs` are ordinary Rust, and the `verus` feature adds only their `verus_spec` attributes, the specification-function block, one `proof!` block, and the `vstd` import. No executable statement is gated, so verified and compiled bodies are the same source with no generation or erasure step. The packaged `.crate`'s `authority.rs` is byte-identical to the branch's. Drift is prevented by construction and by the rule in [Shipping Verified Code](../design/verified-code-packaging.md#proof-packaging-for-consumers), which review enforces; no automated check for gated executable code exists yet.

#### Consumer resolution {#formal-verification-4-consumer-resolution}

The regression consumer keeps its exact `vstd = "=0.0.0-2026-10-11-0230"` pin: resolution fails against the baseline engine and resolution and build pass against the isolated packaged candidate. Independent consumers build without Verus installed; the engine's ordinary dependency tree contains no vstd or Verus dependency.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `independent-resolution` — Partial: engine proof dependency isolation. Other exact pins, supported toolchains, and renderer compatibility retain their separate integration and release scope.
- **Baseline evidence:** On 2026-11-08, Cargo 1.99.1 (`5f94df478`) reproduced the conflict against engine code at `fab05d6`, on the design branch for PR #19. The standalone consumer in `fixtures/consumers/verus-resolution` depends on the engine and exactly pins `vstd = "=0.0.0-2026-10-11-0230"`. From the repository root, `cargo metadata --offline --format-version 1 --manifest-path fixtures/consumers/verus-resolution/Cargo.toml` exits 101: the engine requires `=0.0.0-2026-10-18-0147`, and Cargo reports ``failed to select a version for `vstd` which could resolve this conflict``. Compatible-version unification cannot satisfy both exact prerelease requirements; no `links` conflict is involved.
- **Isolation evidence:** Passed on 2026-11-10 with Cargo 1.99.1. With the consumer's pin unchanged, `cargo metadata` resolves its own `vstd 0.0.0-2026-10-11-0230` and `cargo build --locked` succeeds, both against this branch and against the extracted packaged `.crate` with only the `anchor-book` path changed in a disposable copy; the fixture's lock file needed no change. Against the pre-change engine the same command still fails with ``failed to select a version for `vstd` ``. An ordinary consumer with no `vstd` of its own builds against the packaged engine with no Verus on `PATH`, and its lock file contains no `vstd`. The engine's default dependency tree contains no `vstd` or Verus crate.

#### Preserved proof {#formal-verification-4-preserved-proof}

A native proof and ordinary integration tests pass for the same packaged candidate and recorded source revision. A deliberate authority defect fails verification and its correction passes. Review reassesses the claim, assumptions, production callers, and excluded boundaries after the packaging change; old proof reports do not establish this result.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority`, `production-integration` — Partial: revalidation of proof and integration after packaging changes, under the anchor's declared proof scope.
- **Evidence:** Passed. `cargo verus verify --fwd-verus-args-to roots --locked --features verus --lib -- --no-cheating` reports 15 verified, 0 errors, full crate, and `.github/verus-report.jq` accepts the report with all seven functions, both on the branch and in the extracted packaged source. Making the accepted-status check always pass fails a postcondition (14 verified, 1 error); the restored source verifies. The packaged source's own tests pass, and ordinary tests, formatting, and Clippy pass on Rust 1.88.0 and stable 1.99.1. Review of the claim, assumptions, callers, and exclusions: the contracts, invariants, and executable bodies are unchanged in content from milestone 1's assessed specification; only their syntax moved from `verus!` to attributes. Verus 0.2026.10.18.7d82994 and Z3 4.16.0 were built from source on Linux; the GitHub proof job uses the release archive.

#### Dependency review {#formal-verification-4-dependency-review}

A recorded review of the candidate's package contents, dependency set, and features confirms proof isolation and explains remaining exact requirements. The record identifies the package revision and explicitly limits its conclusion to verification dependencies.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `dependency-review` — Partial: engine verification dependencies; general release dependency review remains separate.
- **Evidence:** Reviewed for the branch's packaged candidate (`cargo package --locked`, 106 files). Default normal dependencies are `mdbook-core`, `mdbook-summary`, `pulldown-cmark`, `serde`, `serde_json`, and `toml_edit`; no Verus crate. The `verus` feature adds `vstd` and its `verus_*` crates. Remaining exact requirements are `mdbook-summary` and `mdbook-core` at `=0.5.4`, because that parser version defines the supported navigation input; they belong to release review, not verification isolation. The package also contains repository files such as workflows and tests, also a release-readiness question.

### Evidence {#formal-verification-4-evidence}

**Arrangement chosen:** candidate (a), Verus's attribute syntax under an optional `verus` feature. A feasibility slice (two helpers, `lookup` with its loop invariant, and the specification functions they use) verified with the feature and built without it, and a deliberate defect failed; the full validator then followed. Research into the Verus guide for the pinned release supports the choice: the guide recommends `#[verus_spec]` for executable functions and `verus!` for specification functions when production code must build without Verus, with dependencies controlled by feature flags. The gate is the Cargo feature rather than Verus's internal `verus_keep_ghost` cfg. Candidate (b), generated runtime source, was not needed.

**Mutation coverage:** the validator's bodies are now ordinary Rust, so cargo-mutants 27.1.0 reaches them: 50 mutations, against none inside `verus!`. Five loop-counter mutations (`+= 1` to `*= 1`) hang the loops; each fails verification with "decreases not satisfied", so they are excluded in `.cargo/mutants.toml` with that reason. Of the remaining 45, tests catch 42 and 3 are unviable; none survive or time out.

**Changes:** `src/records/authority.rs`, `src/lib.rs` (enables `proc_macro_hygiene` only with the feature, for loop attributes), `Cargo.toml`, `.cargo/mutants.toml`, the proof workflow's `--features verus`, CONTRIBUTING, and the Rust Engine, Running Verification, and Shipping Verified Code chapters.

**Limits:** The attribute syntax is a contributed Verus extension that changes between releases; reverify on every upgrade. `cargo build --features verus` with an ordinary toolchain fails, because the loop attributes need the feature only Verus's compiler allows; the feature is for verification only. No automated check yet enforces that the feature gates no executable code.

### Completion record

Complete on 2026-11-10. Ordinary builds of the engine and of dependents need no Verus crate, the regression consumer resolves and builds with its own `vstd`, and the proof and tests pass for the same packaged implementation. GitHub results for the change are recorded with its pull request.

## Later work

Reusable proof execution and normalized results belong to
[Verification Evidence milestone 2](verification-evidence.md#verification-evidence-2),
proof guards and the assessment record for `formal-authority` to
[milestone 6](verification-evidence.md#verification-evidence-6),
independent consumers to [milestone 5](verification-evidence.md#verification-evidence-5),
and retained CI reports to [milestone 3](verification-evidence.md#verification-evidence-3).
The engine's proof evidence remains manually recorded until that runner exists.
Anchor progress views and support for other verifiers retain their later scope.
Verification runs remain separate from structural checking and mdBook
preprocessing; they neither prove the whole engine nor change an anchor's
acceptance status automatically.
