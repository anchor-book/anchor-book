# Verification Evidence

**Roadmap identifier:** `verification-evidence`

## Goal

`anchorbook verify` reports progress toward the goal design for each anchor and
criterion, running the linked tests and proofs and checking that proofs establish
their criteria, as defined in [Verification](../design/verification.md) and
[Running Verification](../design/verification-evidence.md). It serves agents in
the build loop and CI with the same command. This Book comes first: each runner
and guard works on the engine's own tests and its authority proof before another
project relies on it. Verus is optional, so a project can use `verify` for its
tests alone. Proofs in other projects are later work, exercised with fixture
projects in milestone 5.

None of this Book's criteria are linked to tests or proofs yet, so `verify` will
initially report them as unlinked. Linking criteria is part of the work.

## Sequence and scope

Start with the shared foundation in milestone 4. Milestones 1 and 2 then provide
test and proof runners independently, both piloted on this Book; milestone 3 adds
CI and versioned retained reports for this Book. Milestone 5, proofs in other
projects, is deferred until this Book's runners and guards are complete and
[Verus is optional](formal-verification.md#formal-verification-4). Milestone 6 adds the proof guards, assessment
records, and the agent brief once both runners exist. Milestone 7 is a prototyping
experiment that tries the proof guards by hand on the engine's authority proof,
needs none of the tooling, and informs milestones 4 and 6. Existing milestone and
criterion IDs stay stable. Milestone 7 is agreed and Ready; the other milestones
remain Proposed until their scope is agreed and assigned.

Milestone 4 defines the report's identity fields and collects criteria and
configuration content identities plus Git revision and dirty state when available.
Milestones 1 and 2 collect code and lockfile content identities and compare inputs
before and after their native runs.

The Formal Verification roadmap owns engine proof correctness, its existing
evidence, and packaging isolation. This roadmap owns tool execution, report
interpretation, and criterion states. General CLI version alignment, contract
delivery, and registry publication retain their separate
[integration and release scope](index.md#later-work-retained-from-the-design).
Record syntax migration is not a prerequisite: links resolve the identifiers in
the current inventory and migrate with them when that work is assigned.

## Milestone 1. An agent sees test progress for an anchor in the build loop {#verification-evidence-1}

**Outcome:** An agent or engineer runs `anchorbook verify --anchor <id> --method test` after a change and sees that anchor's test criteria for the current files, with links resolved and drift reported; a whole-Book run reports progress by chapter.

**Status:** Proposed

**Dependencies:** [Milestone 4](#verification-evidence-4) and agreement on nextest support.

**Design basis:** [Running Verification](../design/verification-evidence.md), [Configuration](../design/configuration.md), [Diagnostics](../design/check-diagnostics.md), and [Rust Engine](../design/rust-engine.md)

**Identifier:** `verification-evidence-1`

**Tasks**

- [ ] Use milestone 4's configuration, link inventory, states, and report API; implement nextest selection, an exact filter with ignored tests included, JUnit interpretation, and test-name mapping.
- [ ] Connect the runner to `anchorbook verify --config`, preserving the shared scope and exit statuses. Collect code and lockfile content identities and compare input snapshots before and after execution.
- [ ] Exercise changed-input and stale-report cases with the actual runner; neither can produce satisfied evidence.
- [ ] Pin nextest and document installing it in CONTRIBUTING; ordinary `cargo test` and CI test jobs are unchanged.
- [ ] Link the test criteria of the `verify-*` anchors and at least one existing checker anchor, and record a build-loop run.

### Completion criteria

#### Test states {#verification-evidence-1-test-states}

Independent fixtures produce every state for the test method; criteria requiring several methods report partial or not evaluated correctly; scoped runs name what they exclude.

- **Anchor coverage:** [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method`, `scope-and-limits` — Partial: test method and its execution scope; proof scope is in milestone 2.
- **Evidence:** Pending.

#### Progress report {#verification-evidence-1-progress-report}

For a fixture Book with satisfied, partial, failing, and unlinked anchors, the library and actual CLI report every anchor by chapter with states, links, and summary counts and exit 0; invalid inputs make the run incomplete; an unbuildable or missing runner affects only its method.

- **Anchor coverage:** [verify-progress-report](../design/verification-evidence.md#verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: native test progress; shared reporting is in milestone 4, proof results in milestone 2, and retained CI reports in milestone 3.
- **Evidence:** Pending.

#### Links {#verification-evidence-1-links}

Fixtures with dangling, mismatched, misplaced, and duplicate links and unlinked criteria produce located findings; corrected fixtures resolve.

- **Anchor coverage:** [verify-links-resolve](../design/verification-evidence.md#verify-links-resolve) · `link-findings` — Partial: test links
- **Evidence:** Pending.

#### Selected runs {#verification-evidence-1-selected-runs}

Recording runners and native nextest runs select exactly the linked tests in scope and identify code and lockfile contents. Before-and-after input snapshots detect changes and make affected evidence inconclusive; earlier reports never count. Linked tests not executed, skipped, or missing have no evidence; the runner preserves Book and source files.

- **Anchor coverage:** [verify-runs-linked-checks](../design/verification-evidence.md#verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: native test selection, current inputs, and preservation; proof execution is in milestone 2.
- **Evidence:** Pending.

#### Build loop {#verification-evidence-1-build-loop}

In this repository, `anchorbook verify --anchor` for a linked anchor reports its criteria after a passing change, a deliberately failing change, and the restored code, and a whole-Book run reports the remaining unlinked criteria; record the commands and output.

- **Anchor coverage:** None — acceptance against this Book
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 2. Verify runs and reports proofs {#verification-evidence-2}

**Outcome:** `verify` runs the configured project's Verus over complete linked targets and returns criterion evidence from a supported, mode-aware report adapter. The engine's `formal-authority` criterion is the first pilot.

**Status:** Proposed

**Dependencies:** [Milestone 4](#verification-evidence-4), the existing engine proof, and a scoped native compatibility experiment that settles configuration fields and supported function/target forms before the complete runner is assigned. Nextest completion is not required.

**Design basis:** [Verus runner](../design/verification-evidence.md#verus-runner), [authority proof](../design/rust-engine.md#authority-proof), and [project-verus-evidence](../design/project-integration.md#project-verus-evidence).

**Identifier:** `verification-evidence-2`

**Tasks**

- [ ] **Setup:** Settle installation/version/toolchain and manifest/package/target/feature fields with the pinned native verifier. Capture executable and proof-function reports; declare supported forms.
- [ ] **Adapter:** Normalize one supported Verus report format into the shared model. Match identities and modes uniquely using resolved links, replacing the engine's hard-coded function inventory.
- [ ] **Execution:** Run each required target once with locked dependencies, the trust policy, and fresh target/report directories. Collect code and lockfile content identities, before-and-after input snapshots, actual tools, invocation, and scope.
- [ ] **Pilot:** Link the authority validator to `formal-authority` and exercise the real CLI, retaining its specification assessment and excluded integration obligations.
- [ ] **Failure cases:** Exercise missing, duplicate, skipped, and wrong-mode functions; unsupported versions; empty, malformed, or partial reports; and zero coverage.
- [ ] **Failure cases:** In a native fixture, distinguish a linked function's failure from a passing linked caller whose unlinked supporting lemma fails. Report failed and inconclusive respectively, list failing obligations, then restore full-target success.
- [ ] **Failure cases:** Exercise cached runs with no new report, changed code or lockfile, absent tools, and build failures through the actual runner.
- [ ] **Trust policy:** Demonstrate native rejection of deliberate assumptions and trusted project bodies for the supported version.
- [ ] **Acceptance:** Record native failure/correction results, report fixtures, commands, runtime, scope, assumptions, and limits.

### Completion criteria

#### Proof states {#verification-evidence-2-proof-states}

Fixtures cover every proof-method state. Native runs demonstrate a linked failure as failed, a passing linked caller with a failing unlinked lemma as inconclusive, correction as satisfied, and build failure and unavailable tools in their designed states. The engine's `formal-authority` proof method is satisfied for the tested inputs; test and review obligations retain their own states.

- **Anchor coverage:** [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method` — Partial: proof method; [verify-runs-linked-checks](../design/verification-evidence.md#verify-runs-linked-checks) · `selected-run` — Partial: proof runner; [verify-links-resolve](../design/verification-evidence.md#verify-links-resolve) · `link-findings` — Partial: proof links
- **Evidence:** Pending.

#### Report integrity {#verification-evidence-2-report-integrity}

Real and independent fixture reports establish mode-aware, unique function matching and full-target success. Missing, duplicate, skipped, wrong-mode, partial, malformed, zero-coverage, or unsupported-version evidence never satisfies a method. Failures outside a criterion's links make its proof evidence inconclusive and retain the failing obligations and locations.

- **Anchor coverage:** [verify-links-resolve](../design/verification-evidence.md#verify-links-resolve) · `link-findings` — Partial: proof modes and result identities; [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method` — Partial: proof report interpretation.
- **Evidence:** Pending.

#### Current runs {#verification-evidence-2-current-runs}

Recording runners and native runs establish exactly the required targets, actual features and tools, fresh reports, and code and lockfile content identities including dirty sources. Before-and-after input snapshots detect changes and make affected evidence inconclusive. Earlier reports and successful commands without new verification output never count as passing evidence; the runner preserves Book and source files.

- **Anchor coverage:** [verify-runs-linked-checks](../design/verification-evidence.md#verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: native proof execution, completing the test-runner scope in milestone 1; [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `scope-and-limits` — Partial: proof target scope.
- **Evidence:** Pending.

#### Trust policy {#verification-evidence-2-trust-policy}

For the supported verifier, deliberate assumptions and trusted project bodies fail the configured policy. Reports retain trusted dependencies and excluded boundaries; zero errors or a successful linked function cannot override partial-target verification or an unsupported report version.

- **Anchor coverage:** [project-verus-evidence](../design/project-integration.md#project-verus-evidence) · `trusted-assumptions` — Partial: the first supported adapter and engine pilot; independent consumers and their tool selections are in milestone 5.
- **Evidence:** Pending. Existing engine proof runs do not establish consumer-runner behavior.

### Completion record

Pending.

## Milestone 3. CI reports progress for every revision {#verification-evidence-3}

**Outcome:** Each pull request and main revision runs `anchorbook verify` for the whole Book and retains its report.

**Status:** Proposed

**Dependencies:** Verification Evidence — Milestones 1 and 2.

**Design basis:** [Running Verification](../design/verification-evidence.md)

**Identifier:** `verification-evidence-3`

**Tasks**

- [ ] Define a versioned artifact schema from the typed report, with current inputs, executed scope, tools, states, assumptions, and limits; review retained fields against public-content guidance.
- [ ] Run the whole-Book command in CI with supported tools and retain reports for passing, unsatisfied, and incomplete runs. Preserve the progress command's exit policy.
- [ ] Demonstrate an authorized failure/correction cycle and record actual revision-specific workflow results, retention, and runtime.

### Completion criteria

#### Retained report {#verification-evidence-3-retained-report}

A versioned artifact preserves the typed report's scope and states, input identities, tools, and limits, with private paths and raw source/proof payloads omitted. Invalid or stale artifacts never become evidence for another run.

- **Anchor coverage:** [verify-progress-report](../design/verification-evidence.md#verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: retained CI presentation; [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `scope-and-limits` — Partial: artifact scope and limits.
- **Evidence:** Pending.

#### CI results {#verification-evidence-3-ci-results}

Actual runs identify the tested checkout and demonstrate passing, failing-evidence, and incomplete outcomes, followed by correction at a new revision. Unsatisfied evidence remains progress; merge enforcement and rendered progress are separately agreed work.

- **Anchor coverage:** None — CI execution, artifact retention, and correction workflow.
- **Evidence:** Pending; local results alone cannot complete this criterion.

### Completion record

Pending.

## Milestone 4. Both runners share criterion evidence and report behavior {#verification-evidence-4}

**Outcome:** Configuration, function associations, target planning, report identity fields, evidence states, and library/CLI reports are usable before either native runner.

**Status:** Proposed

**Dependencies:** The record library, configuration loader, navigation reader, and engineer agreement on this bounded foundation. Existing record identifiers remain usable.

**Design basis:** [Running Verification](../design/verification-evidence.md), including [links](../design/verification-evidence.md#evidence-links), [run identity](../design/verification-evidence.md#run-identity), states, and command behavior.

**Identifier:** `verification-evidence-4`

**Tasks**

- [ ] Add the common `[verification]` fields to the loader, ignored by `check`; validate sources and separate output without invoking tools.
- [ ] Select and record the Rust source parser and supported forms under the [Rust Engine dependency conventions](../design/rust-engine.md#rust-development-conventions). Evaluate ordinary Rust parsing (for example `syn`) and the additional parsing needed inside `verus!`; reject parser dependencies that defeat proof isolation.
- [ ] Implement supported source associations and function identities, distinguishing tests, Verus executable/proof functions, and specification context. Report misplaced, duplicate, dangling, mismatched, ambiguous, and unsupported links.
- [ ] Define target planning, report identity fields, normalized evidence, and per-method/criterion/anchor aggregation behind `verification`. Collect criteria/configuration content identities and Git revision/dirty state when available; code/lockfile collection and execution snapshots belong to milestones 1 and 2.
- [ ] Expose typed reports and `anchorbook verify --config` scope/report behavior. Use independent fixture results and unavailable-runner cases; no native runner or speculative plugin API is required for this milestone.

### Completion criteria

#### Shared states {#verification-evidence-4-shared-states}

Independent evidence fixtures produce every method state and correct multi-method, criterion, and anchor aggregation, including linked proof failure versus target failures elsewhere. The review method remains not evaluated until milestone 6, and only all-required-method success satisfies a criterion.

- **Anchor coverage:** [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method` — Partial: normalized state model; native test/proof results are in milestones 1 and 2.
- **Evidence:** Pending.

#### Function links {#verification-evidence-4-function-links}

A recorded parser/dependency review identifies supported Rust and Verus forms. Independent source fixtures resolve identities and methods against real inventory IDs, reject invalid links, and distinguish same-named functions by module/package/target. A specification declaration alone supplies no successful proof evidence; unsupported forms have located findings.

- **Anchor coverage:** [verify-links-resolve](../design/verification-evidence.md#verify-links-resolve) · `link-findings` — Partial: source discovery and inventory resolution; result matching is in each runner milestone.
- **Evidence:** Pending.

#### Plans and reports {#verification-evidence-4-plans-and-reports}

Fixture plans identify requested criteria, selected tests/proof targets, excluded scope, criteria/configuration content identities, and Git revision/dirty state, remaining usable without Git. Library and actual CLI reports agree for valid, invalid, and unavailable-runner inputs, preserve sources, and use the designed progress/incomplete exit behavior. The report model includes execution identity fields; native collection and snapshots remain in milestones 1 and 2.

- **Anchor coverage:** [verify-progress-report](../design/verification-evidence.md#verify-progress-report) · `progress-output`, `incomplete-runs`; [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `scope-and-limits`; [verify-runs-linked-checks](../design/verification-evidence.md#verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: planning and reporting before native execution; milestones 1 and 2 supply native results.
- **Evidence:** Pending.

### Completion record

Pending. This milestone provides common machinery, not passing native proof or test evidence.

## Milestone 5. Consumers verify their own production code {#verification-evidence-5}

**Outcome:** Independent Rust consumers use a packaged engine and their own Verus installations to obtain criterion evidence for their production code, with tested scope and dependency boundaries.

**Status:** Proposed — deferred until this Book's runners and guards are complete

**Dependencies:** Deferred until milestones 2, 3, and 6 are complete on this Book, and [Formal Verification milestone 4](formal-verification.md#formal-verification-4). Use a locally packaged release candidate; registry publication and contract delivery are not prerequisites for the fixtures.

**Design basis:** [Project verification](../design/project-integration.md#formal-verification-in-a-project), [project-verus-evidence](../design/project-integration.md#project-verus-evidence), and [verification responsibilities](../design/verification.md#responsibilities).

**Identifier:** `verification-evidence-5`

**Tasks**

- [ ] Prepare two independent packaged consumer fixtures with different Book/module layouts, a build toolchain separate from verification, and a consumer-selected `vstd` version different from the engine's proof workspace.
- [ ] Link a production executable contract and a supporting proof function to the consumer's criteria. Establish the runtime call path, preconditions, and excluded conversion/caller boundaries through tests and review.
- [ ] Exercise passing, deliberate proof failure and correction, incomplete/unsupported reports, unavailable tools, and trust-policy rejection through the real CLI. Add supported-version adapters only with native report fixtures.
- [ ] Run the proof guards on the consumer's criteria: sensitivity with classified survivors, witnesses for any preconditions, and a recorded specification assessment, including a stale review after a specification change.
- [ ] Document consumer setup, supported versions/forms, exact invocation, scope, assumptions, limits, and results for the tested package; retain partial states where review evidence is unsupported.

### Completion criteria

#### Consumer tools {#verification-evidence-5-consumer-tools}

Both consumers build the packaged engine without its verifier dependencies and run their configured Verus installations using verification Rust separate from build Rust. A different consumer `vstd` resolves and builds; reports identify the actual tools and supported adapter used.

- **Anchor coverage:** [project-verus-evidence](../design/project-integration.md#project-verus-evidence) · `separate-toolchain` — Full; [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `independent-resolution` — Partial: proof dependencies and consumer `vstd` choice. Other exact pins and general release compatibility remain separate work.
- **Evidence:** Pending.

#### Consumer proof {#verification-evidence-5-consumer-proof}

The consumer's own linked functions produce satisfied proof evidence, a deliberate defect fails, and correction passes at new input identities. Assumed/trusted project bodies, partial verification, unsupported versions, and engine proof results cannot satisfy a consumer criterion.

- **Anchor coverage:** [project-verus-evidence](../design/project-integration.md#project-verus-evidence) · `trusted-assumptions` — Full; [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method` — Partial: independent consumer proof states.
- **Evidence:** Pending.

#### Scope review {#verification-evidence-5-scope-review}

Record the consumer criterion-to-contract assessment, production call path, satisfied preconditions, and test/review coverage of exclusions for the tested revisions. Reports disclose executed targets and any unsatisfied review method; proof success never implies all-method or whole-system verification.

- **Anchor coverage:** [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `scope-and-limits` — Partial: consumer scope presentation. Fixture criterion fidelity and excluded boundaries are reviewed against the consumer's own Book.
- **Evidence:** Pending.

#### Consumer guidance {#verification-evidence-5-consumer-guidance}

A fresh fixture setup reproduces packaged adoption, tool selection, supported report/function forms, deliberate failure and recovery using the documented commands, without depending on this development checkout. Unsupported workspace forms, review ingestion, and remaining release work are explicit.

- **Anchor coverage:** None — reproducible consumer setup and contributor guidance.
- **Evidence:** Pending.

### Completion record

Pending. Packaged fixture acceptance does not authorize publication or complete other project-integration capabilities.

## Milestone 6. Verify checks that proofs establish their criteria {#verification-evidence-6}

**Outcome:** A proof method is satisfied only when its specification constrains the linked code, its preconditions can be met, and a current specification assessment accepts it; review criteria are evaluated from assessment records; and agents get a brief of each criterion before writing its proof.

**Status:** Proposed

**Dependencies:** Milestones 1 and 2, and the reviewed results of [milestone 7](#verification-evidence-7), which settle the change operators, the witness form, the fingerprint inputs, and the runtime per change. Exclusion coverage in the brief needs the proof-scope items of [Record Line Grammar milestone 2](record-line-grammar.md#record-line-grammar-2); before then the brief shows the statement text.

**Design basis:** [Guards](../design/verification.md#guards), [assessment records](../design/verification.md#assessment-records), [proof guards](../design/verification-evidence.md#proof-guards), and the [brief](../design/verification-evidence.md#using-verify).

**Identifier:** `verification-evidence-6`

**Tasks**

- [ ] **Sensitivity:** Generate changes inside `verus!` executable bodies with the milestone 4 parser, never in specifications or proofs. Verify each in a fresh copy; classify survivors through the covering criteria's linked tests and the review's equivalent listing; report unviable, safety-only, and timed-out changes separately.
- [ ] **Witnesses:** Implement the `witness` link form and require a verified witness for each linked function with preconditions.
- [ ] **Fingerprints and assessments:** Compute token-based specification fingerprints over the criterion, linked contracts, and reachable specification functions and types. Add the `reviews` configuration field and the assessment record schema; read records, report malformed and stale ones, and evaluate the review method.
- [ ] **Brief:** Add `--brief`, running no tools, with each criterion's scope, links, fingerprint, assessment status, and exclusions with their covering criteria and tests.
- [ ] **Pilot:** Record a specification assessment of `formal-authority` and run every guard on it through the real CLI; record survivors and their classification.

### Completion criteria

#### Sensitivity {#verification-evidence-6-sensitivity}

Native fixtures show an omitted rule as an unclassified survivor that fails the proof method, the strengthened specification rejecting the same change, survivors classified only by failing covering tests or a current equivalent listing, and unviable, safety-only, and timed-out changes in their designed outcomes. Specifications and proofs are never changed.

- **Anchor coverage:** [verify-proof-sensitive](../design/verification.md#verify-proof-sensitive) · `surviving-changes`, `classified-survivors`, `uninformative-changes` — Full
- **Evidence:** Pending.

#### Witnesses {#verification-evidence-6-witnesses}

Fixtures with unsatisfiable, unwitnessed, witnessed, and failing-witness preconditions produce the designed proof states.

- **Anchor coverage:** [verify-proof-nonvacuous](../design/verification.md#verify-proof-nonvacuous) · `witness-required` — Full
- **Evidence:** Pending.

#### Assessments {#verification-evidence-6-assessments}

Fixtures show which changes alter the fingerprint and which do not; missing, stale, rejecting, and current assessment records produce the designed proof and review method states; malformed and duplicate records are located findings; `verify` never writes a record.

- **Anchor coverage:** [verify-proof-assessment-current](../design/verification.md#verify-proof-assessment-current) · `fingerprint-coverage`, `assessment-states` — Full; [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method` — Partial: the review method; [verify-runs-linked-checks](../design/verification-evidence.md#verify-runs-linked-checks) · `source-preservation` — Partial: assessment records are read only.
- **Evidence:** Pending.

#### Brief {#verification-evidence-6-brief}

For fixture criteria with and without proof scope, the brief lists scope, links, fingerprint, assessment status, and exclusions with their covering criteria and tests, reports uncovered exclusions as gaps, and runs no tools.

- **Anchor coverage:** None — agent guidance output; whether the brief needs its own anchor is open with the engineer.
- **Evidence:** Pending.

#### Pilot {#verification-evidence-6-pilot}

The engine's `formal-authority` proof method is satisfied with every guard and a recorded specification assessment for the tested revision, with its survivors and their classification recorded; a deliberate specification change makes the assessment stale.

- **Anchor coverage:** None — acceptance against this Book; the authority proof's own evidence stays in the Formal Verification roadmap.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 7. Try the proof guards by hand on the authority proof {#verification-evidence-7}

**Outcome:** Recorded results show whether the proof guards find real weaknesses in an existing proof and what they cost, so the guard design is confirmed or revised before any tooling is built.

**Status:** Ready — scope and criteria agreed with the engineer on 2026-11-09; not yet started

**Dependencies:** The existing authority proof and the pinned Verus release documented in CONTRIBUTING. No engine tooling is required. This is a prototyping experiment: it changes no engine code or Book records, and its results do not verify any anchor.

**Design basis:** [Guards](../design/verification.md#guards), [proof guards](../design/verification-evidence.md#proof-guards), and the [authority proof](../design/rust-engine.md#authority-proof).

**Identifier:** `verification-evidence-7`

**Tasks**

- [ ] In a disposable copy, apply candidate changes to the executable statements of the seven authority functions, one at a time, never to specifications or proofs. Include the accepted-status change, which must fail, and a failure-classification change, which the specification deliberately leaves to tests.
- [ ] Verify the complete crate for each change with the pinned command. Record which obligation failed for each rejected change, and the time per run.
- [ ] Compare with specification mutation: mutate the contracts and specification functions of `formal-authority`, keep the code and proof, and record which mutations still verify. Compare what each approach finds and its false positives, and recommend which the sensitivity guard should use.
- [ ] For each survivor, run the `production-integration` tests against the same change and classify it as excluded and detected, equivalent with a reason, or a gap in the specification.
- [ ] Write a witness for the preconditions of `validate_reference`, and record the Verus form that works.
- [ ] List what a fingerprint of the `formal-authority` specification would cover, and check that a change to a reachable specification function, such as `target_is_suitable_authority`, falls inside it.
- [ ] Record the results and limits, recommend changes to the guard design, and fix any specification gap found, through the Formal Verification roadmap.

### Completion criteria

#### Change results {#verification-evidence-7-change-results}

Every change tried is recorded with its location, its outcome, the obligation that failed or its classification as a survivor, and its runtime; the accepted-status change fails verification. The commands reproduce the results.

- **Anchor coverage:** None — design experiment; informs [verify-proof-sensitive](../design/verification.md#verify-proof-sensitive)
- **Evidence:** Pending.

#### Witness and fingerprint {#verification-evidence-7-witness-and-fingerprint}

A verified witness for `validate_reference` and the list of fingerprint inputs for `formal-authority` are recorded, with any limits found.

- **Anchor coverage:** None — design experiment; informs [verify-proof-nonvacuous](../design/verification.md#verify-proof-nonvacuous) and [verify-proof-assessment-current](../design/verification.md#verify-proof-assessment-current)
- **Evidence:** Pending.

#### Assessment {#verification-evidence-7-assessment}

A written assessment states what the guards found, what they cost, and their limits, and recommends confirming or revising the guard design. Revisions are made in the design chapters; specification gaps are recorded against the authority proof.

- **Anchor coverage:** None — decision input
- **Evidence:** Pending.

### Completion record

Pending.

A rendered progress view remains an
[open question](../design/verification-evidence.md#open-questions) in the design;
it needs design before it gets a milestone.
