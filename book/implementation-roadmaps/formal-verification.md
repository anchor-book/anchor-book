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

**Status:** In progress — local proof and integration verified; specification assessment pending

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
- [ ] Assess the formal specification against the explicit proof scope (a specification assessment) and map production-integration tests and review to every excluded boundary. Record any remaining coverage gaps before accepting the milestone.

### Completion criteria

#### Authority proof {#formal-verification-1-authority-proof}

The production validator verifies acceptance if and only if the defined authority rules hold over the complete typed input. Preconditions do not assume valid authority relationships. The run identifies the verified functions; a deliberate rule violation fails a proof obligation, and the restored code passes.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority` — Full. The claim is limited to the specified authority logic and recorded assumptions.
- **Evidence:** Passed locally; see milestone evidence. The specification assessment is pending.

#### Production path {#formal-verification-1-production-path}

Review identifies the same verified functions on the normal read_records call path. Independent fixtures exercise conversion, empty and invalid authority, all resolution/status cases, and rejection through the real adapter. State which boundaries are tested rather than proved.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `production-integration` — Full. Behavioral test coverage also contributes to `required-authority`, `resolved-authority`, and `compatible-status`; reconcile with Record Rendering milestone 3.
- **Evidence:** Passed locally; see milestone evidence.

#### Compatible tooling {#formal-verification-1-tooling}

Pinned tools reproduce verification without rendering the Book. Ordinary builds, tests, formatting, and Clippy pass on supported toolchains. Record whether mutation testing reaches the annotated executable bodies and any gaps; a skipped or empty mutation scope is not coverage.

- **Anchor coverage:** None — developer tooling and compatibility; proof success alone does not complete the mutation roadmap.
- **Evidence:** Passed locally; see milestone evidence.

### Evidence {#formal-verification-1-evidence}

**Local result:** Working tree based on `45125f6`, tested 2026-10-21. Verus `0.2026.10.18.7d82994`, verifier Rust 1.99.1, and bundled Z3 4.16.0 report 15 verification units passed, zero errors. All seven executable authority functions appear in the report. The native command and exactly pinned library are documented in CONTRIBUTING; no verification wrapper script is required.

**Scope and assumptions:** The production `validate_authorities` function satisfies its acceptance biconditional over complete typed candidates. Project assumptions and trusted function bodies are prohibited by `--no-cheating`; standard-library specifications, verifier/solver, and compiler remain trusted. Parsing, conversion, diagnostics, and backlinks are tested boundaries. Review the specification's interpretation of the anchor before accepting the milestone.

**Failure demonstration:** Reversing the accepted-target status condition in a disposable source copy caused a postcondition failure: 14 verification units passed, one failed. The original source was unchanged and its proof passed again. Independent tests cover all authority outcomes and an uncited decision failing through the real adapter. Tests and Clippy pass on Rust 1.85 and local stable 1.90.

**Mutation assessment:** cargo-mutants 27.1.0 discovers no mutations inside `verus!`, so automatic mutation coverage of the validator is unavailable. The focused inventory integration run initially found four diagnostic assertion gaps; after strengthening message and related-location assertions, all 24 selected mutations were attempted: 22 caught, two unviable, no survivors or timeouts. Command: `target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/records/inventory.rs --output target/verus-mutations-fixed`. This is scoped evidence, not a new whole-engine campaign. Raw reports remain ignored under target. Milestone 2 records subsequent remote proof results.

### Completion record

Pending the specification assessment and the mapping of production-integration tests and review to the excluded boundaries.

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

**Design basis:** [Criterion methods](../design/anchor-definitions.md#verification-methods), [record presentation](../design/record-rendering.md#reader-presentation), and [agent verification guidance](../ai-working-contract.md#develop-verification-with-the-code).

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

**Status:** Proposed

**Identifier:** `formal-verification-4`

**Design basis:** [Proof packaging for consumers](../design/verified-code-packaging.md#proof-packaging-for-consumers), [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies), and [records-valid-authority](../design/rust-engine.md#records-valid-authority).

**Dependencies:** The existing production authority proof. Begin with a small source-arrangement experiment and review its feasibility before assigning the full packaging change. Registry publication, contract delivery, and the consumer runner are not prerequisites.

**Tasks**

- [ ] Test one maintained source arrangement on a representative validator slice. A feature alone does not compile: code inside `verus!` needs Verus's macro crates even with the feature off. Candidates are (a) an optional `verus` Cargo feature with the validator written as ordinary Rust, its contracts attached through Verus's attribute syntax (`#[verus_verify]`, `#[verus_spec]`, present in the pinned release) under `cfg_attr`, and specification functions and proofs compiled only with the feature; and (b) a development proof workspace with reproducible erasure. For the chosen one, establish supported forms, tool availability, drift detection, and runtime correspondence.
- [ ] Remove engine verification libraries from the default dependency graph; with the feature off, `cargo tree` shows no `vstd` or other Verus crate. Keep the authority implementation on the production path; relocating a still-required proof crate or maintaining a second validator does not establish isolation.
- [ ] Preserve `fixtures/consumers/verus-resolution` as the consumer-resolution regression fixture: its exact `vstd` pin fails against the current engine and must resolve and build after isolation. Repeat with the packaged candidate and an ordinary consumer without Verus installed.
- [ ] Verify the source corresponding to that package, run ordinary tests on the minimum and stable toolchains, and reject a deliberate authority defect before restoring a passing proof. Reassess conversion, callers, diagnostics, and any new generation boundary.
- [ ] Record package contents and dependency review, source correspondence, tools, commands, tested revisions, proof and integration results, and remaining limits. Reconcile contributor commands and the proof workflow with the chosen arrangement.

### Completion criteria

#### Runtime correspondence {#formal-verification-4-runtime-correspondence}

Review identifies one maintained authority implementation and a reproducible relationship between verified and packaged functions. Independent tests and drift checks reject divergence; any erasure or conversion assumptions are explicit. A separately maintained proof-only validator cannot complete this criterion.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority`, `production-integration` — Partial: correspondence across the packaging boundary; specification acceptance remains in milestone 1.
- **Evidence:** Pending.

#### Consumer resolution {#formal-verification-4-consumer-resolution}

The regression consumer keeps its exact `vstd = "=0.0.0-2026-10-11-0230"` pin: resolution fails against the baseline engine and resolution and build pass against the isolated packaged candidate. Independent consumers build without Verus installed; the engine's ordinary dependency tree contains no vstd or Verus dependency.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `independent-resolution` — Partial: engine proof dependency isolation. Other exact pins, supported toolchains, and renderer compatibility retain their separate integration and release scope.
- **Baseline evidence:** On 2026-11-08, Cargo 1.99.1 (`5f94df478`) reproduced the conflict against engine code at `fab05d6`, on the design branch for PR #19. The standalone consumer in `fixtures/consumers/verus-resolution` depends on the engine and exactly pins `vstd = "=0.0.0-2026-10-11-0230"`. From the repository root, `cargo metadata --offline --format-version 1 --manifest-path fixtures/consumers/verus-resolution/Cargo.toml` exits 101: the engine requires `=0.0.0-2026-10-18-0147`, and Cargo reports ``failed to select a version for `vstd` which could resolve this conflict``. Compatible-version unification cannot satisfy both exact prerelease requirements; no `links` conflict is involved.
- **Limits and remaining evidence:** This measures dependency resolution against a checkout using cached registry metadata. It does not establish a build, native proof, or packaged-engine compatibility. Post-isolation resolution and build with the unchanged consumer pin remain Pending, as does the ordinary consumer without Verus.

#### Preserved proof {#formal-verification-4-preserved-proof}

A native proof and ordinary integration tests pass for the same packaged candidate and recorded source revision. A deliberate authority defect fails verification and its correction passes. Review reassesses the claim, assumptions, production callers, and excluded boundaries after the packaging change; old proof reports do not establish this result.

- **Anchor coverage:** [records-valid-authority](../design/rust-engine.md#records-valid-authority) · `formal-authority`, `production-integration` — Partial: revalidation of proof and integration after packaging changes, under the anchor's declared proof scope.
- **Evidence:** Pending.

#### Dependency review {#formal-verification-4-dependency-review}

A recorded review of the candidate's package contents, dependency set, and features confirms proof isolation and explains remaining exact requirements. The record identifies the package revision and explicitly limits its conclusion to verification dependencies.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `dependency-review` — Partial: engine verification dependencies; general release dependency review remains separate.
- **Evidence:** Pending.

### Completion record

Pending. The source arrangement and native compatibility have not been established; source review or synthetic report checks are not packaging or proof acceptance evidence.

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
