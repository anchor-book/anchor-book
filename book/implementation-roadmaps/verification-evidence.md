# Verification Evidence

**Roadmap identifier:** `verification-evidence`

## Goal

`anchorbook verify` reports progress toward the goal design for each anchor and
criterion, running the linked tests and proofs and checking that proofs establish
their criteria, as defined in [Verification](../design/verification.md) and
[Running Verification](../design/verification-evidence.md). It serves agents in
the build loop and CI with the same command. The Book build exposes the current
results to agents and in the rendered Book through one shared model. This Book comes first: each runner
and guard works on the engine's own tests and its authority proof before another
project relies on it. Verus is optional, so a project can use `verify` for its
tests alone. Proofs in other projects are later work, exercised with fixture
projects in milestone 5.



## Sequence and scope

The [first-major-milestone sequence](index.md#first-major-milestone-symposium-adopts-anchor-book)
puts milestone 2 next: encapsulate Verus and move the engine's Authority proof CI
onto the shared runner before expanding the Symposium trial. The shared model,
test runner, and manual guard experiment are recorded in milestones 4, 1, and 7.

Milestone 3 adds retained CI reports, with a test-only path that needs no Verus.
Milestone 8 provides criterion review records independently of proof guards, so
the Symposium trial can assess test sufficiency and make changed reviews stale.
Milestone 9 brings structural findings, current evidence, and completion claims
into the agent-facing build report and rendered Book, using milestone 3's artifact
and milestone 8's reviews. It runs here before Symposium's real agent assignments
and is required for upstream adoption.
Milestone 6 later adds specification fingerprints, proof guards, and the proof
brief, reusing those records. Milestone 5, proofs in other projects, stays deferred
until this Book's runners and guards are complete and
[Verus is optional](formal-verification.md#formal-verification-4). Broader proof
guards and consumer proofs do not hold up Symposium's test-only adoption.

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
- [x] Link the test criteria of the `verify-*` anchors, and the tests behind every test criterion this Book [claims](../design/verification-evidence.md#claims-of-completion) through Complete milestones (29 claims when the gate was designed, most of them checker and record criteria), so a whole-Book run passes its test claims; record a build-loop run. Proof execution is in milestone 2, complete proof guards in milestone 6, and review records in milestone 8. Link only a test that exercises its criterion. Where a claimed criterion has none, add the test, or change the claiming milestone's coverage to Partial with its scope, and record the case; never link a loosely related test to clear a claim.
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

Complete on 2026-11-10; merged in PR #35 as `c07dbc5`. `verify` runs linked tests with cargo-nextest and reports their current results, and this Book's whole-Book run passes its test claims. Remaining proof execution and guards are in milestones 2 and 6; criterion review records are in milestone 8.

## Milestone 2. The engine encapsulates Verus and its proof CI {#verification-evidence-2}

**Outcome:** The engine owns configured Verus execution and report interpretation through one shared runner, and its Authority proof CI uses that path with a strict proof-target gate. The existing authority proof is the pilot; criterion reports preserve outstanding assessment and guard obligations.

**Status:** In progress — GitHub proof failure/correction demonstrated; mutation corrections awaiting CI

**Dependencies:** [Milestone 4](#verification-evidence-4) and the existing engine proof, both complete. The native compatibility experiment settled the supported configuration and CI interface in the design. This milestone precedes the next Symposium development trial.

**Design basis:** [Verus runner](../design/verification-evidence.md#verus-runner), [authority proof](../design/rust-engine.md#authority-proof), and [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence).

**Identifier:** `verification-evidence-2`

**Tasks**

- [x] **Setup experiment:** Use the existing authority proof, pinned Verus release, and separate verification toolchain. Settle installation/version/toolchain and manifest/package/target/feature fields, source-link forms, and the engine-owned CI entry point. Capture executable and proof-function reports, measure runtime, and reconcile the design before implementing the full runner. Keep the scope to this engine pilot and independent fixtures.
- [x] **Adapter:** Normalize one supported Verus report format into the shared model. Match identities and modes uniquely using resolved links, replacing the engine's hard-coded function inventory.
- [x] **Execution:** Run each required target once with locked dependencies, the trust policy, and fresh target/report directories. Collect code and lockfile content identities, before-and-after input snapshots, actual tools, invocation, and scope.
- [x] **Pilot:** Link the authority validator to `formal-authority` and exercise the real CLI, retaining its specification assessment and excluded integration obligations.
- [x] **Failure cases:** Exercise missing, duplicate, skipped, and wrong-mode functions; unsupported versions; empty, malformed, or partial reports; and zero coverage.
- [x] **Failure cases:** In a native fixture, distinguish a linked function's failure from a passing linked caller whose unlinked supporting lemma fails. Report failed and inconclusive respectively, list failing obligations, then restore full-target success.
- [x] **Failure cases:** Exercise cached runs with no new report, changed code or lockfile, absent tools, and build failures through the actual runner.
- [x] **Trust policy:** Demonstrate native rejection of deliberate assumptions and trusted project bodies for the supported version.
- [x] **CI encapsulation:** Switch the existing Authority proof job from its direct verifier command and `.github/verus-report.jq` interpretation to the engine's shared runner and proof-target gate. Tool installation remains pinned and explicit; report parsing and expected function selection live in the engine. Retain the existing proof boundary, required check, public-safe artifacts, and independent engine tests.
- [x] **Strict gate:** Require a nonempty scope and a current complete result for every required proof target, with the expected verifier and linked functions. Failed, absent, stale, unsupported, skipped, or incomplete results fail the CI gate. Do not use the progress command's exit 0 as evidence, since it permits unchecked claims. A target passing its contracts does not satisfy a criterion whose assessment or guards are outstanding.
- [x] **CI demonstration:** Exercise a deliberate proof failure and correction through the shared path on GitHub; test unavailable tools and unusable reports through the actual gate, and record the revision-specific results.
- [x] **Acceptance:** Record native failure/correction results, report fixtures, commands, runtime, scope, assumptions, and limits.

### Completion criteria

#### Proof states {#verification-evidence-2-proof-states}

Fixtures cover every proof-method state, supplying guard results only in independent fixtures that exercise aggregation. Native runs demonstrate a linked failure as failed, a passing linked caller with a failing unlinked lemma as inconclusive, and correction as a complete target result. Build failures and unavailable tools retain their designed states. The engine's proof criterion remains unsatisfied while required assessments or guards are missing or not evaluated; milestone 6 establishes the complete proof method. Test and review obligations retain their own states.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: proof method; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run` — Partial: proof runner; [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: proof links
- **Evidence:** Passed locally. The recording-runner fixtures and `native_proof_failure_support_failure_trust_and_correction` exercise the actual CLI: linked failure is failed, a passing caller with a failing supporting lemma is inconclusive, and restored target verification passes. The native test also rejects assumptions and trusted bodies. Successful target results remain no-evidence at the criterion level because specification assessments and guards are outstanding; review remains unevaluated.

#### Report integrity {#verification-evidence-2-report-integrity}

Real and independent fixture reports establish mode-aware, unique function matching and full-target success. Missing, duplicate, skipped, wrong-mode, partial, malformed, zero-coverage, or unsupported-version evidence never satisfies a method. Failures outside a criterion's links make its proof evidence inconclusive and retain the failing obligations and locations.

- **Anchor coverage:** [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: proof modes and result identities; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: proof report interpretation.
- **Evidence:** Passed locally. Native report projections in `tests/fixtures/verus/` cover executable and proof modes, success, linked and supporting failures, and trust violations. Independent variants reject unsupported versions/toolchains, duplicate or absent functions, wrong modes, missing fields, malformed or multiple reports, incomplete target results, zero coverage, and nonzero command exits. Source attributes determine expected modes; explicit library names determine result identities.

#### Current runs {#verification-evidence-2-current-runs}

Recording runners and native runs establish exactly the required targets, actual features and tools, fresh reports, and code and lockfile content identities including dirty sources. Before-and-after input snapshots detect changes and make affected evidence inconclusive. Earlier reports and successful commands without new verification output never count as passing evidence; the runner preserves Book and source files.

- **Anchor coverage:** [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run`, `source-preservation` — Partial: native proof execution, completing the test-runner scope in milestone 1; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: proof target scope.
- **Evidence:** Passed locally. Actual-process recording runners check locked manifests, package/features, complete library scope, fresh output directories, source preservation, successful commands with no new report, build failures, and changed Book, configuration, source, manifest, or lockfile inputs. Native Cargo runs use a fresh target for each invocation. The strict Authority gate passed with seven uniquely matched linked functions and no link findings.

#### Trust policy {#verification-evidence-2-trust-policy}

For the supported verifier, deliberate assumptions and trusted project bodies fail the configured policy. Reports retain trusted dependencies and excluded boundaries; zero errors or a successful linked function cannot override partial-target verification or an unsupported report version.

- **Anchor coverage:** [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence) · `trusted-assumptions` — Partial: the first supported adapter and engine pilot; independent consumers and their tool selections are in milestone 5.
- **Evidence:** Passed locally. With the pinned source-built tools, native `assume_(false)` and `#[verifier::external_body]` fixtures fail `--no-cheating`; the ordinary code and restored proof still run through the same adapter. Target reports retain trusted-tool and dependency boundaries without asserting criterion sufficiency.

#### Shared CI path {#verification-evidence-2-ci-path}

The Authority proof job invokes the engine-owned gate backed by the same runner
as `verify`; the workflow has no separate Verus result interpretation or function
inventory. A valid run preserves the current proof scope and retained evidence.
Native gate cases reject an empty scope, a failed proof, an unavailable verifier, stale or absent
output, malformed or unsupported reports, missing or wrong-mode functions, and
partial or zero-coverage results. Actual GitHub runs demonstrate deliberate
proof failure and correction for recorded revisions, with the required check
still enforced. Passing the target gate never labels outstanding criterion guards
or assessments satisfied.

- **Anchor coverage:** [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run` — Partial: shared native proof execution in CI; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: target-gate success distinguished from criterion satisfaction.
- **Evidence:** Passed locally and on GitHub. `.github/workflows/proof.yml` calls `anchorbook proof` and native fixture acceptance; expected functions come from source links. The deliberate-defect run rejects `has_required_authority`, and the restored revision passes with all seven linked functions. The job name, pinned installation, ordinary engine tests, retained public artifact, and required-check rule remain. Revision-specific runs are recorded below; mutation validation is pending.

### Local validation

**Tested checkout:** The local implementation was developed from `1b8274ec2460b63419e9b310606ef67de04efbbe` and committed as `263531b`; the review branch is `codex/verus-runner-ci`. Follow-up assertions cover report paths, ordering, configuration boundaries, selection across multiple packages, and stale target results. GitHub results identify their tested revisions below; the branch is not integrated into `main`.

**Native experiment:** Verus 0.2026.10.18.7d82994 and Z3 4.16.0, built from their pinned source tags on Linux, with Rust 1.99.1. The initial fresh Authority run took about 34 seconds and verified the existing production contract. Ordinary Rust contract attributes and `#[verifier::proof]` supply executable and proof identities without parsing `verus!` macros. Cargo's Verus options must precede forwarded Cargo options such as `--lib`; the native runner follows that order.

**Checks:** The independent suite passes on Rust 1.88.0 and stable (165 tests including doctests after the additional review assertions, with eight optional acceptance tests ignored); Clippy and formatting pass. The native failure/correction acceptance passes in about 327 seconds for ten fresh target runs. The strict Authority gate reports a complete target with all seven linked functions. Structural checking, five current/rendered Book acceptance tests, and the absent/invalid/incomplete Book independence experiments pass. Existing proof-loop mutation exclusions were relocated for the added doc comments and checked against the mutation list; no local mutation campaign was run.

**Limits and remaining work:** The local verifier is built from source; CI uses the pinned release archive. Whole-library targets and ordinary Rust attribute links are supported; macro-body links, additional target forms, proof guards, and assessment records retain their planned scope. Target success does not satisfy the complete proof criterion. The revision-specific GitHub proof failure/correction demonstration is complete. Mutation validation and required checks for the final revision remain pending; these results do not establish merge readiness.

**Whole-Book progress:** The local run evaluates 70 criteria across 30 anchors:
41 satisfied, two partial, one no-evidence, 23 unlinked, and three not evaluated.
Of 39 completion claims, 34 are satisfied, the Authority proof criterion is
no-evidence pending its assessment and guards, and four review claims are
unchecked. The target itself is complete. `verify` therefore exits 1 while the
strict proof-target gate passes; neither result replaces the other.

### GitHub proof validation

[PR #38](https://github.com/anchor-book/anchor-book/pull/38) contains the implementation on `codex/verus-runner-ci`; it is not integrated into `main`.

| Revision and run | Result |
| --- | --- |
| Deliberate defect `429649027d78a8397d81cd87c9a5d80c06bd57e3`, [run 43135338173](https://github.com/anchor-book/anchor-book/actions/runs/43135338173) | Returning `true` from `has_required_authority` fails ordinary tests and the strict proof gate. The retained report identifies that executable function as failed and `complete: false`; independent native fixture acceptance still passes. |
| Restored head `5536505b8f6d1ba64b86b0464879ebe73731626a`, tested merge `8071e9e1d2e4190783b6b3d8882a6fa35e7f576a`, [run 43135464616](https://github.com/anchor-book/anchor-book/actions/runs/43135464616) | The entire Authority proof job passes using the pinned release archive. The artifact records a clean checkout, complete target evidence, and all seven expected functions verified exactly once in executable mode. Native fixture acceptance passes in about 546 seconds. |
| Restored head, [Book run 43135464714](https://github.com/anchor-book/anchor-book/actions/runs/43135464714) | Rust 1.88.0 and stable engine checks, Book build, and installed-engine acceptance pass. |

Both proof runs retain `authority-proof-results` with revision, tool selection, ordinary-test exit status, and normalized proof evidence. Raw source and proof-note payloads are excluded. The active **Protect main** ruleset still requires both engine checks, Build mdBook, Installed engine, Authority proof, and Mutation tests. The [full mutation campaign](https://github.com/anchor-book/anchor-book/actions/runs/43135464769) on that restored head tested 1,235 mutations: 1,006 caught, 22 missed, 207 unviable, and no timeouts. It failed, so it does not establish merge readiness. The survivors exposed missing assertions for duplicate CLI options, planning-time input changes, configuration/tool boundaries, qualified attributes, selection across packages and exact manifest/target matching, incomplete or contradictory proof evidence, diagnostic paths, failure display, and criterion-link identity. Follow-up tests cover those cases; the planning-time input check is isolated for deterministic filesystem tests. No new exclusions were added. Corrected mutation and required-check results remain pending.

### Completion record

In progress. Native execution, report interpretation, source links, and the strict gate are implemented and exercised locally and on GitHub. Finish mutation validation and inspect all required checks before recording this milestone Complete.

## Milestone 3. CI reports progress for every revision {#verification-evidence-3}

**Outcome:** Each pull request and main revision retains a versioned whole-Book evidence report, and a project using only tests obtains the same reporting without installing Verus or waiting for proof guards.

**Status:** Proposed

**Dependencies:** Milestones 1 and 4 for the test-only path. Include the proof runner's results when milestone 2 is available; proof guards and consumer proof support are not prerequisites. This Book exercises report retention before Symposium's CI adopts it.

**Design basis:** [Running Verification](../design/verification-evidence.md)

**Identifier:** `verification-evidence-3`

**Tasks**

- [ ] Define a versioned artifact schema from the typed report, with current inputs, executed scope, tools, states, assumptions, and limits; review retained fields against public-content guidance.
- [ ] Run the whole-Book command in CI with supported tools and retain reports for passing, unsatisfied, and incomplete runs. Preserve the progress command's exit policy.
- [ ] Exercise a test-only fixture with no Verus installation and only nextest configured. Start this Book's retained CI progress report with `--method test`, explicitly listing unchecked proof and review claims, while the separate Authority proof gate from milestone 2 remains required. A full-method report must preserve unsatisfied claims until their assessments and guards exist; a retained progress report never replaces the strict gate.
- [ ] Include configured review results when milestone 8 is available. Keep excluded methods and their claims explicit in the artifact consumed by milestone 9; a successful scoped run never becomes a whole-Book verification badge.
- [ ] Run the tests once in CI: let `verify` read a nextest report supplied by the project's own CI test run, after checking that the report's recorded inputs match the current checkout, instead of running the linked tests a second time. A report whose inputs differ is no evidence. Local runs keep running the linked tests themselves, as [`D-verify-runs-linked-checks`](../authority/decisions.md#D-verify-runs-linked-checks) allows for runs `verify` does not invoke.
- [ ] Demonstrate an authorized failure/correction cycle and record actual revision-specific workflow results, retention, and runtime.

### Completion criteria

#### Retained report {#verification-evidence-3-retained-report}

A versioned artifact preserves the typed report's scope and states, input identities, tools, and limits, with private paths and raw source/proof payloads omitted. Invalid or stale artifacts never become evidence for another run.

- **Anchor coverage:** [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: retained CI presentation; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: artifact scope and limits.
- **Evidence:** Pending.

#### CI results {#verification-evidence-3-ci-results}

Actual runs identify the tested checkout and demonstrate passing, failing-evidence, and incomplete outcomes, followed by correction at a new revision. A test-only fixture needs no Verus or proof guards. Unclaimed unsatisfied evidence remains progress; a checked unsatisfied completion claim retains exit 1. The engine's strict proof gate remains separate. Milestone 9 consumes these reports for the Book build and agent workflow.

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

Independent evidence fixtures produce every method state and correct multi-method, criterion, and anchor aggregation, including linked proof failure versus target failures elsewhere. The review method remains not evaluated until milestone 8, and only all-required-method success satisfies a criterion.

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

**Outcome:** A proof method is satisfied only when its specification constrains the linked code, its preconditions can be met, and a current specification assessment accepts it; agents get a brief of each criterion before writing its proof. Criterion review records are provided independently by milestone 8.

**Status:** Proposed

**Dependencies:** Milestones 1, 2, and 8, and the reviewed results of [milestone 7](#verification-evidence-7), which settle the change operators, the witness form, the fingerprint inputs, and the runtime per change. Exclusion coverage in the brief uses the proof-scope items the record reader parses since [Record Line Grammar milestone 2](record-line-grammar.md#record-line-grammar-2). This broader proof work follows the first major milestone unless an adoption finding requires a bounded part sooner.

**Design basis:** [Guards](../design/verification.md#guards), [assessment records](../design/verification.md#assessment-records), [proof guards](../design/verification-evidence.md#proof-guards), and the [brief](../design/verification-evidence.md#using-verify).

**Identifier:** `verification-evidence-6`

**Tasks**

- [ ] **Sensitivity:** Generate changes in linked executable bodies, including ordinary Rust carrying Verus attributes and supported `verus!` bodies, never in specifications or proofs. Verify each in a fresh copy; classify survivors through the covering criteria's linked tests and the assessment's equivalent listing; report unviable, safety-only, and timed-out changes separately.
- [ ] **Witnesses:** Implement the `witness` link form and require a verified witness for each linked function with preconditions.
- [ ] **Fingerprints and assessments:** Extend milestone 8's record reader with the specification subject, token-based fingerprints over the criterion, linked contracts, and reachable specification functions and types, and equivalent-survivor listings. Keep criterion reviews usable without Verus.
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

Fixtures show which changes alter the specification fingerprint and which do not; missing, stale, rejecting, and current specification assessments produce the designed proof-method states; malformed and duplicate records are located findings; `verify` never writes a record. Criterion review regression cases from milestone 8 still pass without Verus.

- **Anchor coverage:** [A-verify-proof-assessment-current](../design/verification.md#A-verify-proof-assessment-current) · `fingerprint-coverage`, `assessment-states` — Full; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `source-preservation` — Partial: specification assessments are read only.
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

## Milestone 8. Review criteria use current assessment records {#verification-evidence-8}

**Outcome:** `verify` evaluates required criterion reviews from recorded judgments under the project's reviewer policy and makes changed or missing assessments visible, so the engine's Book and Symposium can assess test sufficiency without Verus or proof guards.

**Status:** Proposed

**Identifier:** `verification-evidence-8`

**Dependencies:** The shared evidence model in milestone 4 and the test runner in milestone 1, both complete. Settle the bounded record and fingerprint design using this Book's review criteria and the partial-coverage findings in Symposium milestone 4. The proof runner and milestone 6 are not prerequisites.

**Design basis:** [Assessment records](../design/verification.md#assessment-records), [evidence states](../design/verification-evidence.md#evidence-states), and [D-assessment-records-as-files](../authority/decisions.md#D-assessment-records-as-files).

**Tasks**

- [ ] Define the criterion-subject record schema, configured assessment directory, reviewer identity, conclusion and limits, and fingerprints for the criterion and examined sources. Settle what changes invalidate a judgment, including changed evidence links, before implementation; reconcile the design and fixtures.
- [ ] Implement the common record reader and review-method states. Missing or stale records supply no evidence, rejection fails the method, and malformed or duplicate records produce located findings. `verify` reads records and never authors or accepts them.
- [ ] Pilot the review method on this Book, including tests-and-review criteria, before Symposium relies on it. Keep each method independent: an accepting review cannot replace missing or failing test evidence.
- [ ] Assess a criterion linked to a test that covers only part of it, record the uncovered obligations, and demonstrate a corrected test set and a current assessment. Required judgments come from the project reviewer; a passing test report cannot author an accepting review.
- [ ] Repeat the partial-coverage case in Symposium. Document reviewer responsibilities, current-input checks, and remaining limits; preserve the production-code specification work for milestone 6.

### Completion criteria

#### Review states {#verification-evidence-8-review-states}

Independent fixtures and the actual CLI demonstrate current accepting, rejecting,
missing, stale, malformed, and duplicate records, and the designed aggregation
with test results. Changes to the inputs the agreed design includes stale the
record. Sources and assessments remain unchanged, and no Verus tool is invoked.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: review-method states and test/review aggregation; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `source-preservation` — Partial: criterion assessments are read only.
- **Evidence:** Pending.

#### Sufficiency assessed {#verification-evidence-8-sufficiency-assessed}

Recorded cases in this Book first, then Symposium, distinguish passing linked tests
from a current judgment that they address the whole criterion. A partial test set
retains its missing coverage, a changed subject makes its assessment stale, and a
corrected test set is assessed anew. Reports do not present a review as proof of
the property or as a substitute for another required method.

- **Anchor coverage:** None — acceptance of the review workflow in the two test subjects.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 9. The Book build returns current evidence to agents and readers {#verification-evidence-9}

**Outcome:** Agents developing from this Book receive current structural findings, criterion evidence, and completion-claim results through a shared report, and the rendered Book presents the same evaluated state. This works here before Symposium's real agent assignments and upstream adoption.

**Status:** Proposed

**Identifier:** `verification-evidence-9`

**Dependencies:** Milestone 3's versioned report and retained native CI evidence, milestone 8's review records, and the existing structural checker and record renderer. Milestone 2 supplies the engine's strict proof gate; broader proof guards and consumer Verus support are not prerequisites. Outstanding methods and claims remain explicit.

**Design basis:** [Agent workflow](../design/system-overview.md#working-through-agents), [Book build evidence](../design/verification-evidence.md#book-build-evidence), and [Record Rendering](../design/record-rendering.md).

**Tasks**

- [ ] Settle the native build configuration, report handoff, artifact compatibility, input identities, and failure diagnostics before implementation. Reconcile the design and fixtures; keep the engine usable with a broken Book.
- [ ] Compose structural findings, evidence states, and completion-claim results through shared library models. Expose stable record and criterion identities, source locations, scope, and limits through the CLI and versioned artifact so agents can act without HTML or handwritten result labels.
- [ ] Connect native project verification to rendering without duplicate test execution or recursive builds. Validate inputs before and after evaluation and rendering, including dirty working trees and assessment changes; reject missing, invalid, unsupported, and mismatched handoffs.
- [ ] Render the shared states beside criteria and link their evidence. Keep design acceptance, required methods, roadmap completion, and evaluated results distinct. Record-only rendering explicitly shows that verification was not evaluated.
- [ ] Exercise independent fixtures through the real CLI and mdBook adapter, then pilot the complete workflow in this Book. Demonstrate a failure, agent correction from the report, and current results in both agent and rendered views.
- [ ] Document the working commands and how agents use current reports alongside Markdown sources. Supply the tested integration for Symposium milestones 7 and 5; publication and completed milestones never imply that unevaluated methods passed.

### Completion criteria

#### Shared current state {#verification-evidence-9-shared-state}

Independent fixtures and the actual adapters establish the designed equivalence
between the library, CLI, versioned artifact, and rendered evidence, including
unsatisfied criteria, unchecked claims, and the distinction from authored status.
Changing inputs or supplying absent, malformed, unsupported, or mismatched
reports cannot produce a successful current-evidence build.

- **Anchor coverage:** [A-build-current-evidence](../design/system-overview.md#A-build-current-evidence) · `consistent-views`, `current-inputs` — Full.
- **Evidence:** Pending.

#### Independent build loop {#verification-evidence-9-build-loop}

The native fixture pipeline and this Book's acceptance run demonstrate verification
before rendering, no recursive or duplicate tests, unchanged sources, and engine
usability with a broken Book. A recorded agent session corrects a known failure
using source locations and evidence in the report; the rebuilt agent report and
rendered Book agree. Record-only rendering makes its unevaluated scope explicit.

- **Anchor coverage:** [A-build-current-evidence](../design/system-overview.md#A-build-current-evidence) · `independent-pipeline` — Full.
- **Evidence:** Pending.

### Completion record

Pending.
