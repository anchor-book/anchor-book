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



## Sequence and scope

Start with the shared foundation in milestone 4. Milestones 1 and 2 then provide
test and proof runners independently, both piloted on this Book; milestone 3 adds
CI and versioned retained reports for this Book. Milestone 5, proofs in other
projects, is deferred until this Book's runners and guards are complete and
[Verus is optional](formal-verification.md#formal-verification-4). Milestone 6 adds the proof guards, assessment
records, and the agent brief once both runners exist. Milestone 7 is a prototyping
experiment that tries the proof guards by hand on the engine's authority proof,
needs none of the tooling, and informs milestones 4 and 6. Existing milestone and
criterion IDs stay stable. Milestone 7 ran first, as agreed. Each
remaining milestone stays Proposed until its scope is agreed and assigned.

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

**Status:** Complete

**Dependencies:** [Milestone 4](#verification-evidence-4), complete; cargo-nextest, agreed.

**Design basis:** [Running Verification](../design/verification-evidence.md), [Configuration](../design/configuration.md), [Diagnostics](../design/check-diagnostics.md), and [Rust Engine](../design/rust-engine.md)

**Identifier:** `verification-evidence-1`

**Tasks**

- [x] Use milestone 4's configuration, link inventory, states, and report API; implement nextest selection, an exact filter with ignored tests included, JUnit interpretation, and test-name mapping.
- [x] Connect the runner to `anchorbook verify --config`, preserving the shared scope and exit statuses. Collect code and lockfile content identities and compare input snapshots before and after execution.
- [x] Exercise changed-input and stale-report cases with the actual runner; neither can produce satisfied evidence.
- [x] Pin nextest and document installing it in CONTRIBUTING; ordinary `cargo test` and CI test jobs are unchanged.
- [x] Wrap `scripts/check-engine-independence.sh` in an ignored integration test so `A-check-independent-build#broken-book` can be linked.
- [x] Link the test criteria of the `verify-*` anchors, and the tests behind every test criterion this Book [claims](../design/verification-evidence.md#claims-of-completion) through Complete milestones (29 claims when the gate was designed, most of them checker and record criteria), so a whole-Book run passes its test claims; record a build-loop run. Claimed proof and review methods stay unchecked until milestones 2 and 6. Link only a test that exercises its criterion. Where a claimed criterion has none, add the test, or change the claiming milestone's coverage to Partial with its scope, and record the case; never link a loosely related test to clear a claim.
- [x] Decide how ignored acceptance tests, such as `current_book` and `rendered_book`, which needs a built Book, count when linked: run them with their prerequisites, or report them as no-evidence with the reason. Decided: an ignored test that reads only current sources can be linked; one that reads earlier build output, such as `rendered_book`, cannot, as [Test runner](../design/verification-evidence.md#test-runner) states.

### Completion criteria

#### Test states {#verification-evidence-1-test-states}

Independent fixtures produce every state for the test method; criteria requiring several methods report partial or not evaluated correctly; scoped runs name what they exclude.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method`, `scope-and-limits` — Partial: test method and its execution scope; proof scope is in milestone 2.
- **Evidence:** Passed. Fixture tests produce every test-method state from supplied results and through the recording runner: satisfied, failed, skipped and absent as no-evidence, inconclusive for a failed build, a missing report, or changed inputs, and not evaluated for a missing nextest or one not listed in `runners`; a test-and-review criterion is partial; scoped runs give the excluded methods' reasons.

#### Progress report {#verification-evidence-1-progress-report}

For a fixture Book with satisfied, partial, failing, and unlinked anchors, the library and actual CLI report every anchor by chapter with states, links, and summary counts and exit 0; invalid inputs make the run incomplete; an unbuildable or missing runner affects only its method.

- **Anchor coverage:** [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: native test progress; shared reporting is in milestone 4, proof results in milestone 2, and retained CI reports in milestone 3.
- **Evidence:** Passed. The CLI reports every anchor by chapter with states, linked functions, the nextest version, and summary counts; exits 0 with every checked claim holding, 1 with a failing claim, and 2 for invalid inputs; and an unbuildable or missing runner affects only the test method (`tests/verification.rs`).

#### Links {#verification-evidence-1-links}

Fixtures with dangling, mismatched, misplaced, and duplicate links and unlinked criteria produce located findings; corrected fixtures resolve.

- **Anchor coverage:** [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: test links
- **Evidence:** Passed in [milestone 4](#verification-evidence-4-function-links), with the scanner correction recorded there; every link in this Book's sources resolves, with no link findings.

#### Selected runs {#verification-evidence-1-selected-runs}

Recording runners and native nextest runs select exactly the linked tests in scope and identify code and lockfile contents. Before-and-after input snapshots detect changes and make affected evidence inconclusive; earlier reports never count. Linked tests not executed, skipped, or missing have no evidence; the runner preserves Book and source files.

- **Anchor coverage:** [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: native test selection, current inputs, and preservation; proof execution is in milestone 2.
- **Evidence:** Passed. The recording runner shows one `cargo nextest run` per package with the exact filter, ignored tests included, and the tool configuration under the output directory, and that only that directory changes. Native runs on this repository select exactly the linked tests; with a passing report from an earlier run planted in the store and a test file that fails to build, the method is inconclusive (exit 101, no report), and a source edited during a whole-Book run makes the evidence inconclusive and the claims fail. Reports include the code identity over sources, manifests, and lockfiles; a lockfile change at the root of a member package changes it.

#### Build loop {#verification-evidence-1-build-loop}

In this repository, `anchorbook verify --anchor` for a linked anchor reports its criteria after a passing change, a deliberately failing change, and the restored code, and a whole-Book run reports the remaining unlinked criteria and exits 0 with every test claim satisfied; record the commands and output.

- **Anchor coverage:** None — acceptance against this Book
- **Evidence:** Passed on 2026-11-10 with cargo-nextest 0.9.151. `anchorbook verify --config anchorbook.toml --anchor project-agent-bootstrap --method test`: `contract-retrieval` satisfied, exit 0; with `contract::text` deliberately returning other text, failed, naming `contract_output_is_the_packaged_text_for_every_project` and the claiming `project-integration-1-contract`, exit 1; restored, satisfied, exit 0. The whole-Book run takes about 8 seconds and exits 0: 28 claims, 23 satisfied and 5 unchecked (review and proof methods); 65 criteria, 34 satisfied, 2 partial, 25 unlinked, 4 not evaluated. The ignored `native_nextest_follows_a_change_through_the_build_loop` repeats the loop on a fixture package.

### Evidence {#verification-evidence-1-evidence}

Branch `verification`, 2026-11-10, after PR #34. Formatting, Clippy, and the test suite pass on Rust 1.88.0 and stable; the structural check, `mdbook build`, and current-Book and rendered-Book acceptance pass. Linking: every test criterion that a Complete milestone claims has linked tests that exercise it, plus the `verify-*` anchors' criteria. One claim had no such test: Record Rendering milestone 2's `relationship-navigation`, whose site-prefix and print-view checks read previously built output, so its coverage is now Partial with that scope. `A-check-independent-build#broken-book` is linked to a new ignored test that runs `scripts/check-engine-independence.sh`, and `roadmap-validation` to a new current-Book acceptance test of this Book's roadmaps. Mutation testing on the runner: the first run had 9 survivors, 6 in JUnit index arithmetic, rewritten so none remains, and 3 test gaps, now covered; the repeat run tested 67 mutations: 41 caught, 26 unviable, none missed. Limits: one nextest version; builds use the project's ordinary target directory; reports are read, not retained, until milestone 3.

### Completion record

Complete on 2026-11-10; merged in PR #35 as `c07dbc5`. `verify` runs linked tests with cargo-nextest and reports their current results, and this Book's whole-Book run passes its test claims; proof and review methods stay unchecked until milestones 2 and 6.

## Milestone 2. Verify runs and reports proofs {#verification-evidence-2}

**Outcome:** `verify` runs the configured project's Verus over complete linked targets and returns criterion evidence from a supported, mode-aware report adapter. The engine's `formal-authority` criterion is the first pilot.

**Status:** Proposed

**Dependencies:** [Milestone 4](#verification-evidence-4), the existing engine proof, and a scoped native compatibility experiment that settles configuration fields and supported function/target forms before the complete runner is assigned. Nextest completion is not required.

**Design basis:** [Verus runner](../design/verification-evidence.md#verus-runner), [authority proof](../design/rust-engine.md#authority-proof), and [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence).

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

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: proof method; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run` — Partial: proof runner; [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: proof links
- **Evidence:** Pending.

#### Report integrity {#verification-evidence-2-report-integrity}

Real and independent fixture reports establish mode-aware, unique function matching and full-target success. Missing, duplicate, skipped, wrong-mode, partial, malformed, zero-coverage, or unsupported-version evidence never satisfies a method. Failures outside a criterion's links make its proof evidence inconclusive and retain the failing obligations and locations.

- **Anchor coverage:** [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: proof modes and result identities; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: proof report interpretation.
- **Evidence:** Pending.

#### Current runs {#verification-evidence-2-current-runs}

Recording runners and native runs establish exactly the required targets, actual features and tools, fresh reports, and code and lockfile content identities including dirty sources. Before-and-after input snapshots detect changes and make affected evidence inconclusive. Earlier reports and successful commands without new verification output never count as passing evidence; the runner preserves Book and source files.

- **Anchor coverage:** [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: native proof execution, completing the test-runner scope in milestone 1; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: proof target scope.
- **Evidence:** Pending.

#### Trust policy {#verification-evidence-2-trust-policy}

For the supported verifier, deliberate assumptions and trusted project bodies fail the configured policy. Reports retain trusted dependencies and excluded boundaries; zero errors or a successful linked function cannot override partial-target verification or an unsupported report version.

- **Anchor coverage:** [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence) · `trusted-assumptions` — Partial: the first supported adapter and engine pilot; independent consumers and their tool selections are in milestone 5.
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
- [ ] Run the tests once in CI: let `verify` read a nextest report supplied by the project's own CI test run, after checking that the report's recorded inputs match the current checkout, instead of running the linked tests a second time. A report whose inputs differ is no evidence. Local runs keep running the linked tests themselves, as [`D-verify-runs-linked-checks`](../authority/decisions.md#D-verify-runs-linked-checks) allows for runs `verify` does not invoke.
- [ ] Demonstrate an authorized failure/correction cycle and record actual revision-specific workflow results, retention, and runtime.

### Completion criteria

#### Retained report {#verification-evidence-3-retained-report}

A versioned artifact preserves the typed report's scope and states, input identities, tools, and limits, with private paths and raw source/proof payloads omitted. Invalid or stale artifacts never become evidence for another run.

- **Anchor coverage:** [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: retained CI presentation; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: artifact scope and limits.
- **Evidence:** Pending.

#### CI results {#verification-evidence-3-ci-results}

Actual runs identify the tested checkout and demonstrate passing, failing-evidence, and incomplete outcomes, followed by correction at a new revision. Unsatisfied evidence remains progress; merge enforcement and rendered progress are separately agreed work.

- **Anchor coverage:** None — CI execution, artifact retention, and correction workflow.
- **Evidence:** Pending; local results alone cannot complete this criterion.

### Completion record

Pending.

## Milestone 4. Both runners share criterion evidence and report behavior {#verification-evidence-4}

**Outcome:** Configuration, function associations, target planning, report identity fields, evidence states, and library/CLI reports are usable before either native runner.

**Status:** Complete

**Dependencies:** The record library, configuration loader, and navigation reader. Existing record identifiers remain usable.

**Design basis:** [Running Verification](../design/verification-evidence.md), including [links](../design/verification-evidence.md#evidence-links), [run identity](../design/verification-evidence.md#run-identity), states, and command behavior.

**Identifier:** `verification-evidence-4`

**Tasks**

- [x] Add the common `[verification]` fields to the loader, ignored by `check`; validate sources and separate output without invoking tools.
- [x] Select and record the Rust source parser and supported forms under the [Rust Engine dependency conventions](../design/rust-engine.md#rust-development-conventions). Evaluate ordinary Rust parsing (for example `syn`) and the additional parsing needed inside `verus!`; reject parser dependencies that defeat proof isolation.
- [x] Implement supported source associations and function identities, distinguishing tests, Verus executable/proof functions, and specification context. Report misplaced, duplicate, dangling, mismatched, ambiguous, and unsupported links.
- [x] Define target planning, report identity fields, normalized evidence, and per-method/criterion/anchor aggregation behind `verification`. Collect criteria/configuration content identities and Git revision/dirty state when available; code/lockfile collection and execution snapshots belong to milestones 1 and 2.
- [x] Expose typed reports and `anchorbook verify --config` scope/report behavior. Use independent fixture results and unavailable-runner cases; no native runner or speculative plugin API is required for this milestone.
- [x] Read coverage levels in the roadmap reader, rejecting a qualified Full entry with a located diagnostic; read claims from Complete milestones' Full coverage; fail the run for unsatisfied claims in scope and list unchecked ones.

### Completion criteria

#### Shared states {#verification-evidence-4-shared-states}

Independent evidence fixtures produce every method state and correct multi-method, criterion, and anchor aggregation, including linked proof failure versus target failures elsewhere. The review method remains not evaluated until milestone 6, and only all-required-method success satisfies a criterion.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: normalized state model; native test/proof results are in milestones 1 and 2.
- **Evidence:** Passed. `tests/verification.rs` `every_method_state_comes_from_links_and_this_runs_results` and `the_worst_linked_test_decides_and_reasons_name_the_function` evaluate a fixture plan against supplied results: passed, failed, skipped, and absent tests; unusable and unavailable runners; a linked proof failure outranking failures elsewhere and incompleteness, which are inconclusive; and a verified function without its specification assessment as no-evidence. A criterion with test satisfied and review not evaluated is partial; an unlinked criterion stays unlinked when no runner ran. `states_combine_as_the_design_orders_them` checks aggregation order.

#### Function links {#verification-evidence-4-function-links}

A recorded parser/dependency review identifies supported Rust and Verus forms. Independent source fixtures resolve identities and methods against real inventory IDs, reject invalid links, and distinguish same-named functions by module/package/target. A specification declaration alone supplies no successful proof evidence; unsupported forms have located findings.

- **Anchor coverage:** [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: source discovery and inventory resolution; result matching is in each runner milestone.
- **Evidence:** Passed. Parser review: `syn` 2.0.119 with only `full` and `parsing`, and `proc-macro2` 1.0.107 with only `span-locations`; both were already in `Cargo.lock`, add no proc-macro, printing, or Verus dependency, and leave the default tree free of `vstd`. A line scanner finds every `anchorbook:` comment line and `syn` attaches doc-comment links to their functions, so ordinary comments, links on structs, methods, or non-test functions, and links inside macros are located findings. Supported forms are listed in [Evidence links](../design/verification-evidence.md#evidence-links). Fixture tests resolve links against the inventory and report misplaced, malformed, duplicate, dangling, method-mismatched, ambiguous, and unsupported links at their lines; distinguish same-named functions by module, crate target, and package; identify library, binary, binary-module, and integration-test-module targets, block doc comments, direct and `cfg_attr` Verus attributes, and witnesses; and show a corrected link removing its finding. This Book's nine existing links, written in the earlier `//` form, were reported as misplaced and converted.

#### Plans and reports {#verification-evidence-4-plans-and-reports}

Fixture plans identify requested criteria, selected tests/proof targets, excluded scope, criteria/configuration content identities, and Git revision/dirty state, remaining usable without Git. Library and actual CLI reports agree for valid, invalid, and unavailable-runner inputs, preserve sources, and use the designed progress/incomplete exit behavior. The report model includes execution identity fields; native collection and snapshots remain in milestones 1 and 2.

- **Anchor coverage:** [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output`, `incomplete-runs`; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits`; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: planning and reporting before native execution; milestones 1 and 2 supply native results.
- **Evidence:** Passed. Fixture plans list the tests and proof targets selected for the scope; a run scoped by anchor and method reports excluded methods as not evaluated with the reason. Input identities change with a criterion's statement and not otherwise, and Git revision and dirty state are reported in a repository and absent outside one. The actual CLI prints every anchor with the library's state, exact summary counts, link findings, and claims; exits 2 for unknown or non-anchor scope, a repeated `--config`, an unknown method, a failing structural check, and invalid `[verification]`, records, or roadmaps; and changes no file or directory. An invalid `[verification]` section leaves `check` passing.

#### Claims checked {#verification-evidence-4-claims}

Fixture Books with Complete and other milestones, Full and Partial coverage, and satisfied, unlinked, and failing criteria produce the designed claim failures, exit statuses, and unchecked lists through the library and actual CLI.

- **Anchor coverage:** [A-verify-claims-evidenced](../design/verification-evidence.md#A-verify-claims-evidenced) · `claim-failures`, `unchecked-claims` — Partial: fixture results; native runs are in milestones 1 and 2.
- **Evidence:** Passed. A Complete milestone's Full coverage is claimed, including a status with an explanation; In progress milestones and Partial coverage claim nothing, and roadmaps are read from the role chapter and its nested subtree only. A claimed criterion that is unlinked or failing exits 1 naming the claiming criterion and its line; passing linked tests satisfy it; a claim outside the requested anchors, or on review, is unchecked and exits 0. The roadmap reader rejects a qualified Full and a missing level (`tests/roadmaps.rs`). On this Book, `anchorbook verify --config anchorbook.toml` reports 29 anchors, 65 criteria, and 29 claims: 22 unsatisfied, because they are unlinked, and 7 unchecked; it exits 1 until [milestone 1](#verification-evidence-1) links them.

### Evidence {#verification-evidence-4-evidence}

Branch `verification`, 2026-11-10. Formatting, Clippy, and 131 tests pass on Rust 1.88.0 and stable; the structural check, `mdbook build`, current-Book and rendered-Book acceptance, and `scripts/check-engine-independence.sh` pass. Mutation testing (cargo-mutants 27.1.0, `--in-diff` against `main`): the first run had 44 survivors in untested layouts, nested roadmaps, flags, summary counts, dirty state, and display text, and one equivalent comparison in test-state aggregation, which was rewritten so none remains; the repeat run tested 243 mutations: 203 caught, 40 unviable, none missed or timed out. **Correction after merge:** an assessment of `main` at `efefa3c` found 16 false `verify.link-misplaced` findings: the scanner read fixture source inside string constants in `tests/verification.rs` as comments. Fixing it exposed two more gaps, a link in a comment after code on its line, which was ignored rather than reported, and a quoted `///` inside a comment's text, which was read as a link. A line's comment now starts at its first marker outside a string literal, using the parser's string spans; `comment_like_text_inside_string_literals_is_not_a_link` and `a_comment_starts_at_its_first_marker_outside_strings` cover all three, and this Book's run reports no link findings. Mutation testing on the changed lines: 36 tested, 30 caught, 6 unviable, none missed.

Limits: no runner, so native states come in milestones 1 and 2; content identities are 64-bit FNV-1a; `#[path]` modules and a non-default library path are not followed.

### Completion record

Complete on 2026-11-10; merged in PR #33 as `efefa3c`, and the scanner correction above in PR #34 as
`9d5d620`. The shared machinery, link reading, states, claims, and reports work against fixture results and this Book; native test and proof evidence remain in milestones 1 and 2.

## Milestone 5. Consumers verify their own production code {#verification-evidence-5}

**Outcome:** Independent Rust consumers use a packaged engine and their own Verus installations to obtain criterion evidence for their production code, with tested scope and dependency boundaries.

**Status:** Proposed — deferred until this Book's runners and guards are complete

**Dependencies:** Deferred until milestones 2, 3, and 6 are complete on this Book, and [Formal Verification milestone 4](formal-verification.md#formal-verification-4). Use a locally packaged release candidate; registry publication and contract delivery are not prerequisites for the fixtures.

**Design basis:** [Project verification](../design/project-integration.md#formal-verification-in-a-project), [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence), and [verification responsibilities](../design/verification.md#responsibilities).

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

- **Anchor coverage:** [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence) · `separate-toolchain` — Full; [A-project-no-imposed-dependencies](../design/project-integration.md#A-project-no-imposed-dependencies) · `independent-resolution` — Partial: proof dependencies and consumer `vstd` choice. Other exact pins and general release compatibility remain separate work.
- **Evidence:** Pending.

#### Consumer proof {#verification-evidence-5-consumer-proof}

The consumer's own linked functions produce satisfied proof evidence, a deliberate defect fails, and correction passes at new input identities. Assumed/trusted project bodies, partial verification, unsupported versions, and engine proof results cannot satisfy a consumer criterion.

- **Anchor coverage:** [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence) · `trusted-assumptions` — Full; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: independent consumer proof states.
- **Evidence:** Pending.

#### Scope review {#verification-evidence-5-scope-review}

Record the consumer criterion-to-contract assessment, production call path, satisfied preconditions, and test/review coverage of exclusions for the tested revisions. Reports disclose executed targets and any unsatisfied review method; proof success never implies all-method or whole-system verification.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: consumer scope presentation. Fixture criterion fidelity and excluded boundaries are reviewed against the consumer's own Book.
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

**Dependencies:** Milestones 1 and 2, and the reviewed results of [milestone 7](#verification-evidence-7), which settle the change operators, the witness form, the fingerprint inputs, and the runtime per change. Exclusion coverage in the brief uses the proof-scope items the record reader parses since [Record Line Grammar milestone 2](record-line-grammar.md#record-line-grammar-2).

**Design basis:** [Guards](../design/verification.md#guards), [assessment records](../design/verification.md#assessment-records), [proof guards](../design/verification-evidence.md#proof-guards), and the [brief](../design/verification-evidence.md#using-verify).

**Identifier:** `verification-evidence-6`

**Tasks**

- [ ] **Sensitivity:** Generate changes inside `verus!` executable bodies with the milestone 4 parser, never in specifications or proofs. Verify each in a fresh copy; classify survivors through the covering criteria's linked tests and the review's equivalent listing; report unviable, safety-only, and timed-out changes separately.
- [ ] **Witnesses:** Implement the `witness` link form and require a verified witness for each linked function with preconditions.
- [ ] **Fingerprints and assessments:** Compute token-based specification fingerprints over the criterion, linked contracts, and reachable specification functions and types. Add the `assessments` configuration field and the assessment record schema; read records, report malformed and stale ones, and evaluate the review method.
- [ ] **Brief:** Add `--brief`, running no tools, with each criterion's scope, links, fingerprint, assessment status, and exclusions with their covering criteria and tests.
- [ ] **Pilot:** Record a specification assessment of `formal-authority` and run every guard on it through the real CLI; record survivors and their classification.

### Completion criteria

#### Sensitivity {#verification-evidence-6-sensitivity}

Native fixtures show an omitted rule as an unclassified survivor that fails the proof method, the strengthened specification rejecting the same change, survivors classified only by failing covering tests or a current equivalent listing, and unviable, safety-only, and timed-out changes in their designed outcomes. Specifications and proofs are never changed.

- **Anchor coverage:** [A-verify-proof-sensitive](../design/verification.md#A-verify-proof-sensitive) · `surviving-changes`, `classified-survivors`, `uninformative-changes` — Full
- **Evidence:** Pending.

#### Witnesses {#verification-evidence-6-witnesses}

Fixtures with unsatisfiable, unwitnessed, witnessed, and failing-witness preconditions produce the designed proof states.

- **Anchor coverage:** [A-verify-proof-nonvacuous](../design/verification.md#A-verify-proof-nonvacuous) · `witness-required` — Full
- **Evidence:** Pending.

#### Assessments {#verification-evidence-6-assessments}

Fixtures show which changes alter the fingerprint and which do not; missing, stale, rejecting, and current assessment records produce the designed proof and review method states; malformed and duplicate records are located findings; `verify` never writes a record.

- **Anchor coverage:** [A-verify-proof-assessment-current](../design/verification.md#A-verify-proof-assessment-current) · `fingerprint-coverage`, `assessment-states` — Full; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: the review method; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `source-preservation` — Partial: assessment records are read only.
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

**Status:** Complete

**Dependencies:** The existing authority proof and the pinned Verus release documented in CONTRIBUTING. No engine tooling is required. This is a prototyping experiment: it changes no engine code or Book records, and its results do not verify any anchor.

**Design basis:** [Guards](../design/verification.md#guards), [proof guards](../design/verification-evidence.md#proof-guards), and the [authority proof](../design/rust-engine.md#authority-proof).

**Identifier:** `verification-evidence-7`

**Tasks**

- [x] In a disposable copy, apply candidate changes to the executable statements of the seven authority functions, one at a time, never to specifications or proofs. Include the accepted-status change, which must fail, and a failure-classification change, which the specification deliberately leaves to tests.
- [x] Verify the complete crate for each change with the pinned command. Record which obligation failed for each rejected change, and the time per run.
- [x] Compare with specification mutation: mutate the contracts and specification functions of `formal-authority`, keep the code and proof, and record which mutations still verify. Compare what each approach finds and its false positives, and recommend which the sensitivity guard should use.
- [x] For each survivor, run the `production-integration` tests against the same change and classify it as excluded and detected, equivalent with a reason, or a gap in the specification.
- [x] Write a witness for the preconditions of `validate_reference`, and record the Verus form that works.
- [x] List what a fingerprint of the `formal-authority` specification would cover, and check that a change to a reachable specification function, such as `target_is_suitable_authority`, falls inside it.
- [x] Record the results and limits, recommend changes to the guard design, and fix any specification gap found, through the Formal Verification roadmap.

### Completion criteria

#### Change results {#verification-evidence-7-change-results}

Every change tried is recorded with its location, its outcome, the obligation that failed or its classification as a survivor, and its runtime; the accepted-status change fails verification. The commands reproduce the results.

- **Anchor coverage:** None — design experiment; informs [A-verify-proof-sensitive](../design/verification.md#A-verify-proof-sensitive)
- **Evidence:** Passed. `scripts/proof-guards/code_changes.py` lists 29 changes to the executable statements of the seven functions, each with its exact original and replacement text; `scripts/proof-guards/run.py code` applies them one at a time to a disposable copy, verifies the complete crate, and runs the record-API tests against survivors. The accepted-status change (`R7`) fails a postcondition. 23 changes are rejected: 14 by postconditions, 7 by loop invariants, `U1` by both, and `A5` (skipping records) by an invariant and an overflow check. Six survive full verification (15 verified, 0 errors); see the milestone evidence. With Verus 0.2026.10.18.7d82994 built from source and six workers, each change took 1–2 seconds after a first build of about 45 seconds per worker.

#### Witness and fingerprint {#verification-evidence-7-witness-and-fingerprint}

A verified witness for `validate_reference` and the list of fingerprint inputs for `formal-authority` are recorded, with any limits found.

- **Anchor coverage:** None — design experiment; informs [A-verify-proof-nonvacuous](../design/verification.md#A-verify-proof-nonvacuous) and [A-verify-proof-assessment-current](../design/verification.md#A-verify-proof-assessment-current)
- **Evidence:** Passed in a disposable copy. **Witness:** an executable function that builds one decoded anchor candidate with one reference and calls `validate_reference(candidates.as_slice(), 0, 0)` verifies; changing the call to reference index 1 fails with "precondition not satisfied", so Verus checks the witness rather than trusting its link. In an ordinary build the function is dead code and trips Clippy's `vec_init_then_push`; gating it with `#[cfg(verus_keep_ghost)]` requires declaring that cfg for `unexpected_cfgs`. A proof-mode witness would need to construct a `Vec` in ghost code and was not pursued. **Fingerprint inputs:** the criterion's statement and proof scope; the signatures and contracts of `has_required_authority`, `matches_identity`, `matches_target_kind`, `lookup`, `has_unknown_identity`, `validate_reference`, and `validate_authorities`; the specification functions their contracts reach: `records_satisfy_authority_rules`, `record_satisfies_authority_rules`, `reference_satisfies_authority_rules`, `reference_is_not_repeated`, `resolves_uniquely_to`, `identified_by`, `target_is_suitable_authority`, `kind_matches`, and `violation_is_located`; the types `Candidate`, `Reference`, `Kind`, `TargetKind`, `Failure`, `Violation`, and `Lookup`; and the `vstd` version, whose string, sequence, and vector specifications the contracts use. `target_is_suitable_authority` is reached through `validate_reference` and through `reference_satisfies_authority_rules`, so a change to it falls inside; changing it (`SR1`, `SR3`, `SR10`) also changed the verification outcome. Loop invariants and proof blocks are excluded, as the design specifies.

#### Assessment {#verification-evidence-7-assessment}

A written assessment states what the guards found, what they cost, and their limits, and recommends confirming or revising the guard design. Revisions are made in the design chapters; specification gaps are recorded against the authority proof.

- **Anchor coverage:** None — decision input
- **Evidence:** Recorded in the milestone evidence below, with recommendations for the guard design.

### Evidence {#verification-evidence-7-evidence}

**Revision:** Branch `verification`, based on `60bb90d`, 2026-11-10; changes applied only in disposable copies. Verus 0.2026.10.18.7d82994 and Z3 4.16.0 built from source on Linux; Rust 1.99.1 for verification.

**Survivors and classification:** each was run against the `production-integration` tests in `tests/records.rs`, `tests/authority_criteria.rs`, and `tests/rendering.rs`.

| Change | What it does | Classification |
| --- | --- | --- |
| `H3` | `reference_count >= 1` for `> 0` | Equivalent: the same for `usize` |
| `R2` | Missing target with unknown identities reported as missing rather than invalid | Excluded and detected: failure classification; record-API tests fail |
| `R4` | Ambiguous target reported as missing | Excluded and detected |
| `R9` | Wrong kind reported as invalid target | Excluded and detected |
| `R10` | Incompatible status reported as missing target | Excluded and detected, including through the real adapter |
| `A4` | Every reference failure attributed to the record's first reference | Excluded diagnostic location. At first only an internal unit test caught it, a gap in the record-API tests; `each_failing_reference_is_reported_at_its_own_location` now detects it |

No survivor shows a gap in the specification. `A3`, which reports missing authority under another failure kind, is rejected only because the contract now includes `violation_is_located`; before that change it would have survived and been caught only by tests.

**Specification changes:** `scripts/proof-guards/run.py spec` applied 12 changes to specification functions and the contract, keeping code and proofs. All nine viable rule changes were rejected, `SR4` was unviable (its quantifier lost its trigger), and the two pure weakenings (`SW1`, a one-way `ensures`; `SW2`, dropping the decoded clause from `violation_is_located`) verified, as any weakening must.

**Assessment and recommendations:**

1. **Keep code changes for the sensitivity guard.** They found a real test gap (`A4`) and cross-checked four exclusions; specification changes found nothing, because they can only alter what the specification states, never expose behavior it leaves out, and every weakening survives. Specification changes could still check that each stated clause is needed, if weakenings are filtered out. Applied in the [sensitivity design](../design/verification-evidence.md#proof-guards), which closed this open question.
2. **Add replaced-argument and enum-variant operators.** The designed operators cover conditions, comparisons, returned values, and removed calls; the location survivor `A4` came from replacing a constructor argument, and the classification survivors from replacing enum variants in returned values. Both forms should be generated. Applied in the sensitivity design.
3. **Classify only with the covering criterion's tests.** Unit tests inside the proof module caught `A4` while the record-API tests did not. Keep the design's rule that survivors are classified only by the linked tests of criteria named in the exclusions, since it exposed this gap.
4. **Witness form.** An executable function in the proof module that constructs an input and calls the linked function works, and Verus checks its call site. The witness link must name the called function, and the runner should confirm the witness calls it. Ordinary builds need the function gated out. Applied in the [witness design](../design/verification-evidence.md#evidence-links), which closed this open question.
5. **Cost is low for this proof.** At 1–2 seconds per change, the full set runs in under a minute, so sampling ([open question 5](../design/verification-evidence.md#open-questions)) is not needed at this size.

**Limits:** One proof, 29 hand-written changes rather than generated ones, and a source-built verifier rather than the release binary. Survivor tests were the three record-API test files; `production-integration` has no evidence links yet, so these stand in for its linked tests.

### Completion record

Complete on 2026-11-10. Every change is recorded and reproducible, the accepted-status change fails, the witness and fingerprint inputs are recorded, and the assessment recommends design changes. No specification gap was found; the record-API test gap it exposed is fixed.

A rendered progress view remains an
[open question](../design/verification-evidence.md#open-questions) in the design;
it needs design before it gets a milestone.
