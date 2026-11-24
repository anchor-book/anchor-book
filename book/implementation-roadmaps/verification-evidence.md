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



Historical completion evidence below describes its recorded revision. Review
results and old review-method states are not current verification evidence;
[milestone 11](#verification-evidence-11) records the policy migration.

## Sequence and scope

The [development sequence](index.md#first-major-milestone-symposium-adopts-anchor-book)
starts with engine reporting, using the existing model, test runner, and CLI from
milestones 4 and 1. Independent fixtures and this Book supply the first acceptance
cases; new Symposium tests or coding trials are not prerequisites. Milestone 2's
native proof runner and engine CI remain in place for their recorded scope;
Symposium's work uses tests and requires no Verus.

Milestone 3 exposes current results directly to agents through the library and
CLI. Milestone 9 then exercises correction from those results and connects
structural findings, criterion results, and completion claims to rendering.
Retention is separate: current-result transport needs no archive, and optional CI
artifacts need an explicit purpose and expiration policy. Durable completion
summaries remain in the Book.

Exercise each engine capability in this Book before Symposium relies on it.
Symposium trials later assess adoption and return findings to the engine; new
project tests and coding trials do not gate this reporting increment.

[Milestone 11](#verification-evidence-11) removes judgments from verification;
[milestone 12](#verification-evidence-12) owns executable checks for migrated
review criteria. The proposed assessment reader in milestone 8 is retired.
Broader programmatic proof guards and the brief remain in milestone 6; consumer
proofs in milestone 5 also need [optional Verus packaging](formal-verification.md#formal-verification-4).
They are separate from Symposium's test workflow.

Milestone 4 records criteria/configuration identities and Git context; milestones
1 and 2 add code/lockfile identities and current-run checks. Milestones 3 and 9
present those results, trusting project test outcomes. The retired
[execution-report proposal](#verification-evidence-10) is not required.
General command/version alignment and contributor setup remain in
[Project Integration](project-integration.md); registry release is separate.

## Milestone 1. An agent sees test progress for an anchor in the build loop {#verification-evidence-1}

**Outcome:** An agent or engineer runs `anchorbook verify --anchor <id> --method test` after a change and sees that anchor's test criteria for the current files, with links resolved and drift reported; a whole-Book run reports progress by chapter.

**Status:** Complete

**Identifier:** `verification-evidence-1`

**Dependencies:** [Milestone 4](#verification-evidence-4), complete; cargo-nextest, agreed.

**Design basis:** [Running Verification](../design/verification-evidence.md), [Configuration](../design/configuration.md), [Diagnostics](../design/check-diagnostics.md), and [Rust Engine](../design/rust-engine.md)

**Tasks**

- [x] Use milestone 4's configuration, link inventory, states, and report API; implement nextest selection, an exact filter with ignored tests included, JUnit interpretation, and test-name mapping.
- [x] Connect the runner to `anchorbook verify --config`, preserving the shared scope and exit statuses. Collect code and lockfile content identities and compare input snapshots before and after execution.
- [x] Exercise changed-input and stale-report cases with the actual runner; neither can produce satisfied evidence.
- [x] Pin nextest and document installing it in CONTRIBUTING; ordinary `cargo test` and CI test jobs are unchanged.
- [x] Wrap `scripts/check-engine-independence.sh` in an ignored integration test so `A-check-independent-build#broken-book` can be linked.
- [x] Link the test criteria of the `verify-*` anchors, and the tests behind every test criterion this Book [claims](../design/verification-evidence.md#claims-of-completion) through Complete milestones (29 claims when the gate was designed, most of them checker and record criteria), so a whole-Book run passes its test claims; record a build-loop run. Proof execution is in milestone 2, complete proof guards in milestone 6, with judgment-based verification retired in milestone 11. Link only a test that exercises its criterion. Where a claimed criterion has none, add the test, or change the claiming milestone's coverage to Partial with its scope, and record the case; never link a loosely related test to clear a claim.
- [x] Decide how ignored acceptance tests, such as `current_book` and `rendered_book`, which needs a built Book, count when linked: run them with their prerequisites, or report them as no-evidence with the reason. Decided: an ignored test that reads only current sources can be linked; one that reads earlier build output, such as `rendered_book`, cannot, as [Test runner](../design/verification-evidence.md#test-runner) states.

### Completion criteria

#### Test states {#verification-evidence-1-test-states}

Independent fixtures produce every state for the test method; criteria requiring several methods report partial or not evaluated correctly; scoped runs name what they exclude.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method`, `scope-and-limits` — Partial: test method and its execution scope; proof scope is in milestone 2.
- **Evidence:** Passed. Fixture tests produce every test-method state from supplied results and through the recording runner: satisfied, failed, skipped and absent as no-evidence, inconclusive for a failed build, a missing report, or changed inputs, and not evaluated for a missing nextest or one not listed in `runners`; a test-and-review criterion is partial; scoped runs give the excluded methods' reasons.

#### Progress report {#verification-evidence-1-progress-report}

For a fixture Book with satisfied, partial, failing, and unlinked anchors, the library and actual CLI report every anchor by chapter with states, links, and summary counts and exit 0; invalid inputs make the run incomplete; an unbuildable or missing runner affects only its method.

- **Anchor coverage:** [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: native test progress; shared reporting is in milestone 4, proof results in milestone 2, and shared structured results in milestone 3.
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

Branch `verification`, 2026-11-10, after PR #34. Formatting, Clippy, and the test suite pass on Rust 1.88.0 and stable; the structural check, `mdbook build`, and current-Book and rendered-Book acceptance pass. Linking: every test criterion that a Complete milestone claims has linked tests that exercise it, plus the `verify-*` anchors' criteria. One claim had no such test: Record Rendering milestone 2's `relationship-navigation`, whose site-prefix and print-view checks read previously built output, so its coverage is now Partial with that scope. `A-check-independent-build#broken-book` is linked to a new ignored test that runs `scripts/check-engine-independence.sh`, and `roadmap-validation` to a new current-Book acceptance test of this Book's roadmaps. Mutation testing on the runner: the first run had 9 survivors, 6 in JUnit index arithmetic, rewritten so none remains, and 3 test gaps, now covered; the repeat run tested 67 mutations: 41 caught, 26 unviable, none missed. Limits: one nextest version; builds use the project's ordinary target directory; runner results are read for the current run; shared structured output belongs to milestone 3.

### Completion record

Complete on 2026-11-10; merged in PR #35 as `c07dbc5`. `verify` runs linked tests with cargo-nextest and reports their current results, and this Book's whole-Book run at that revision passed its test claims. Remaining proof execution and guards are in milestones 2 and 6; judgment-based verification is retired in milestone 11.

## Milestone 2. The engine encapsulates Verus and its proof CI {#verification-evidence-2}

**Outcome:** The engine owns configured Verus execution and report interpretation through one shared runner, and its Authority proof CI uses that path with a strict proof-target gate. The existing authority proof is the pilot; criterion reports preserve outstanding programmatic guard obligations.

**Status:** Complete

**Identifier:** `verification-evidence-2`

**Dependencies:** [Milestone 4](#verification-evidence-4) and the existing engine proof, both complete. The native compatibility experiment settled the supported configuration and CI interface in the design. This milestone precedes the next Symposium development trial.

**Design basis:** [Verus runner](../design/verification-evidence.md#verus-runner), [authority proof](../design/rust-engine.md#authority-proof), and [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence).

**Tasks**

- [x] **Setup experiment:** Use the existing authority proof, pinned Verus release, and separate verification toolchain. Settle installation/version/toolchain and manifest/package/target/feature fields, source-link forms, and the engine-owned CI entry point. Capture executable and proof-function reports, measure runtime, and reconcile the design before implementing the full runner. Keep the scope to this engine pilot and independent fixtures.
- [x] **Adapter:** Normalize one supported Verus report format into the shared model. Match identities and modes uniquely using resolved links, replacing the engine's hard-coded function inventory.
- [x] **Execution:** Run each required target once with locked dependencies, the trust policy, and fresh target/report directories. Collect code and lockfile content identities, before-and-after input snapshots, actual tools, invocation, and scope.
- [x] **Pilot:** Link the authority validator to `formal-authority` and exercise the real CLI, retaining its programmatic guards and excluded integration obligations.
- [x] **Failure cases:** Exercise missing, duplicate, skipped, and wrong-mode functions; unsupported versions; empty, malformed, or partial reports; and zero coverage.
- [x] **Failure cases:** In a native fixture, distinguish a linked function's failure from a passing linked caller whose unlinked supporting lemma fails. Report failed and inconclusive respectively, list failing obligations, then restore full-target success.
- [x] **Failure cases:** Exercise cached runs with no new report, changed code or lockfile, absent tools, and build failures through the actual runner.
- [x] **Trust policy:** Demonstrate native rejection of deliberate assumptions and trusted project bodies for the supported version.
- [x] **CI encapsulation:** Switch the existing Authority proof job from its direct verifier command and `.github/verus-report.jq` interpretation to the engine's shared runner and proof-target gate. Tool installation remains pinned and explicit; report parsing and expected function selection live in the engine. Retain the existing proof boundary, required check, public-safe artifacts, and independent engine tests.
- [x] **Strict gate:** Require a nonempty scope and a current complete result for every required proof target, with the expected verifier and linked functions. Failed, absent, stale, unsupported, skipped, or incomplete results fail the CI gate. Do not use the progress command's exit 0 as evidence, since it permits unchecked claims. A target passing its contracts does not satisfy a criterion whose assessment or guards are outstanding.
- [x] **CI demonstration:** Exercise a deliberate proof failure and correction through the shared path on GitHub; test unavailable tools and unusable reports through the actual gate, and record the revision-specific results.
- [x] **Acceptance:** Record native failure/correction results, report fixtures, commands, runtime, scope, assumptions, and limits.
- [x] **State aggregation:** Supply simulated programmatic guard outcomes in independent fixtures and check every proof-method state through the evidence model. This tests state handling before milestone 6 implements the actual checks; simulated outcomes never count as evidence for a project proof.

### Completion criteria

#### Proof states {#verification-evidence-2-proof-states}

Fixtures cover every proof-method state, supplying guard results only in independent fixtures that exercise aggregation. Native runs demonstrate a linked failure as failed, a passing linked caller with a failing unlinked lemma as inconclusive, and correction as a complete target result. Build failures and unavailable tools retain their designed states. The engine's proof criterion remains unsatisfied while required programmatic guards are missing or not evaluated; milestone 6 establishes the complete proof method. Test obligations retain their own states; the review method is retired.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: proof method; [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run` — Partial: proof runner; [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: proof links
- **Evidence:** Runner states passed locally and in native CI. The recording-runner fixtures and `native_proof_failure_support_failure_trust_and_correction` exercise the actual CLI: linked failure is failed, a passing caller with a failing supporting lemma is inconclusive, and restored target verification passes. The native test also rejects assumptions and trusted bodies. Successful target results remain no-evidence at the criterion level because programmatic guards are outstanding. The guard-dependent fixtures in `tests/proof_states.rs` now establish all six proof-method states, precedence among guard failures and gaps, per-criterion isolation, test/proof aggregation, and completion-claim outcomes. They supply simulated outcomes to the evidence model; actual guard execution remains in milestone 6. The completion evidence below records the local results.

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
satisfied.

- **Anchor coverage:** [A-verify-runs-linked-checks](../design/verification-evidence.md#A-verify-runs-linked-checks) · `selected-run` — Partial: shared native proof execution in CI; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: target-gate success distinguished from criterion satisfaction.
- **Evidence:** Passed locally and on GitHub. `.github/workflows/proof.yml` calls `anchorbook proof` and native fixture acceptance; expected functions come from source links. The deliberate-defect run rejects `has_required_authority`, and the restored revision passes with all seven linked functions. The job name, pinned installation, ordinary engine tests, retained public artifact, and required-check rule remain. Revision-specific runs are recorded below; all required checks pass on the tested implementation revision.

### Local validation

**Tested checkout:** The local implementation was developed from `1b8274ec2460b63419e9b310606ef67de04efbbe` and committed as `263531b`; the review branch is `codex/verus-runner-ci`. Follow-up assertions cover report paths, ordering, configuration boundaries, selection across multiple packages, and stale target results. GitHub results identify their tested revisions below. PR #38 merged into `main` on 2026-11-12 as `15177478934198254815784fb077e169c3ad5508`.

**Native experiment:** Verus 0.2026.10.18.7d82994 and Z3 4.16.0, built from their pinned source tags on Linux, with Rust 1.99.1. The initial fresh Authority run took about 34 seconds and verified the existing production contract. Ordinary Rust contract attributes and `#[verifier::proof]` supply executable and proof identities without parsing `verus!` macros. Cargo's Verus options must precede forwarded Cargo options such as `--lib`; the native runner follows that order.

**Checks:** The independent suite passes on Rust 1.88.0 and stable (165 tests including doctests after the additional review assertions, with eight optional acceptance tests ignored); Clippy and formatting pass. The native failure/correction acceptance passes in about 327 seconds for ten fresh target runs. The strict Authority gate reports a complete target with all seven linked functions. Structural checking, five current/rendered Book acceptance tests, and the absent/invalid/incomplete Book independence experiments pass. Existing proof-loop mutation exclusions were relocated for the added doc comments and checked against the mutation list; no local mutation campaign was run.

**Limits and remaining work:** The local verifier is built from source; CI uses the pinned release archive. Whole-library targets and ordinary Rust attribute links are supported; macro-body links, additional target forms, and programmatic proof guards retain their planned scope. Assessment records are retired. Target success does not satisfy the complete proof criterion. The revision-specific GitHub proof failure/correction demonstration and corrected mutation validation are complete. All required checks pass on the tested runner implementation revision; the runner and CI are integrated into main through PR #38. The later guard-state aggregation change has separate local evidence below.

**Historical whole-Book progress:** Before review-method retirement, the final aggregation-checkout run evaluated 70 criteria
across 30 anchors with no link findings: 41 satisfied, three partial, one
no-evidence, 22 unlinked, and three not evaluated. Of 39 completion claims, 34 are
satisfied, the Authority proof criterion is no-evidence pending its assessment
and guards, and four review claims are unchecked. The target itself is complete.
`verify` therefore exits 1 while the strict proof-target gate passes; neither
result replaces the other. The new fixtures supply evidence for the engine's
state-reporting tests, never guard evidence for the Authority proof.

### GitHub proof validation

[PR #38](https://github.com/anchor-book/anchor-book/pull/38) merged the implementation from `codex/verus-runner-ci` into `main` on 2026-11-12 as `15177478934198254815784fb077e169c3ad5508`. The runs below identify their tested pre-merge revisions.

| Revision and run | Result |
| --- | --- |
| Deliberate defect `429649027d78a8397d81cd87c9a5d80c06bd57e3`, [run 43135338173](https://github.com/anchor-book/anchor-book/actions/runs/43135338173) | Returning `true` from `has_required_authority` fails ordinary tests and the strict proof gate. The retained report identifies that executable function as failed and `complete: false`; independent native fixture acceptance still passes. |
| Restored head `5536505b8f6d1ba64b86b0464879ebe73731626a`, tested merge `8071e9e1d2e4190783b6b3d8882a6fa35e7f576a`, [run 43135464616](https://github.com/anchor-book/anchor-book/actions/runs/43135464616) | The entire Authority proof job passes using the pinned release archive. The artifact records a clean checkout, complete target evidence, and all seven expected functions verified exactly once in executable mode. Native fixture acceptance passes in about 546 seconds. |
| Restored head, [Book run 43135464714](https://github.com/anchor-book/anchor-book/actions/runs/43135464714) | Rust 1.88.0 and stable engine checks, Book build, and installed-engine acceptance pass. |
| Mutation corrections through `f5399100f202ad742d6e9e70703cbd26685cdc19`, tested merge `10030a365df688a32a66cf682c8f130238c2aaba`, [proof run 43139766512](https://github.com/anchor-book/anchor-book/actions/runs/43139766512) and [Book run 43139766397](https://github.com/anchor-book/anchor-book/actions/runs/43139766397) | All seven expected functions verify in a clean checkout; native fixture acceptance passes in about 622 seconds. Both Rust engine checks, Book rendering, and installed-engine acceptance pass. The corrected mutation campaign also passes, as recorded below. |

The proof runs retain `authority-proof-results` with revision, tool selection, ordinary-test exit status, and normalized proof evidence. Raw source and proof-note payloads are excluded. The active **Protect main** ruleset still requires both engine checks, Build mdBook, Installed engine, Authority proof, and Mutation tests. The [full mutation campaign](https://github.com/anchor-book/anchor-book/actions/runs/43135464769) on that restored head tested 1,235 mutations: 1,006 caught, 22 missed, 207 unviable, and no timeouts. It failed, so it does not establish merge readiness. The survivors exposed missing assertions for duplicate CLI options, planning-time input changes, configuration/tool boundaries, qualified attributes, selection across packages and exact manifest/target matching, incomplete or contradictory proof evidence, diagnostic paths, failure display, and criterion-link identity. Follow-up tests cover those cases; the planning-time input check is isolated for deterministic filesystem tests. No new exclusions were added. The [corrected campaign](https://github.com/anchor-book/anchor-book/actions/runs/43139766456) at `f5399100f202ad742d6e9e70703cbd26685cdc19` tests 1,236 mutations: 1,029 caught, zero missed, 207 unviable, and zero timeouts. Every original survivor is now caught, including the planning guard moved into `check_planning_inputs`. All six required checks pass for this revision against main `1b8274ec2460b63419e9b310606ef67de04efbbe`; PR #38 subsequently merged as `15177478934198254815784fb077e169c3ad5508`; the results here retain their tested revision identities. Unviable mutations do not establish fault detection.

### Completion record

Complete locally on 2026-11-12. Native execution and the strict gate are integrated through PR #38, with the revision-specific CI evidence above. The remaining proof-state aggregation is implemented and verified in `256302c`, based on `e5edca6`, on `codex/proof-state-aggregation`. Integration and revision-specific required checks are tracked in [PR #40](https://github.com/anchor-book/anchor-book/pull/40). Passing target contracts still does not satisfy a criterion's outstanding assessments or guards.

**Aggregation evidence:** `ProofRun` carries normalized witness and sensitivity results keyed by anchor and criterion. `evaluate` consults them only after complete target evidence and every linked function passing. Missing entries remain no-evidence; the native runner supplies none. Six independent tests in `tests/proof_states.rs` cover satisfied, failed, inconclusive, no-evidence, unlinked, and not-evaluated proof states; witness failures; sensitivity timeouts and skips; failure precedence and retained reasons; isolation between criteria sharing a function; excluded scope; test/proof aggregation; and satisfied, unsatisfied, and unchecked claims. Passing one guard never makes the proof method Partial. These fixtures test engine reporting and are not evidence for a project's proof.

**Checks:** Formatting, all 173 independent tests (including doctests), and Clippy pass on stable and Rust 1.88.0. Structural checking, the mdBook 0.5.4 build, and all five current/rendered Book acceptance tests pass. The actual-process runner fixtures continue to report no-evidence after target success. A native run with the pinned source-built Verus passes `anchorbook proof --anchor A-records-valid-authority` with complete target evidence; `anchorbook verify --anchor A-records-valid-authority --method proof` reports no-evidence and exits 1 for its unsatisfied claim. These runs use `--config anchorbook.toml` through the checkout's binary.

**Mutation evidence:** cargo-mutants 27.1.0, one worker, a 120-second build timeout, and a ten-minute campaign budget, limited to `ProofGuards::state` and `method_state`: 16 mutations in 29 seconds, 13 caught, three unviable, none missed or timed out. The three unviable replacements require a nonexistent Default implementation and establish no fault detection. The tool generates no useful mutations inside the new guard match arms. Six deliberate changes in a disposable source copy therefore additionally omit each of the three obligations, reverse failure precedence, reuse another criterion's guards, or treat absent guards as satisfied. Every change fails `cargo test --locked --test proof_states`; unmodified and restored baselines pass. No exclusions were added. Full required GitHub results are tracked in [PR #40](https://github.com/anchor-book/anchor-book/pull/40); the local results above do not substitute for them.

## Milestone 3. Agents consume current verification results without report storage {#verification-evidence-3}

**Outcome:** The library and `verify --format json` expose the same current verification results as readable output, with criterion context and locations agents can act on. Results can be consumed directly; report files, history, and CI retention are optional.

**Status:** Complete

**Identifier:** `verification-evidence-3`

**Dependencies:** Milestones 1 and 4, complete. Use independent engine fixtures and this Book. New Symposium tests, project coding trials, proof guards, and report retention do not block the work.

**Design basis:** [Structured output and retention](../design/verification-evidence.md#structured-output-and-retention), [evidence states](../design/verification-evidence.md#evidence-states), and [claims of completion](../design/verification-evidence.md#claims-of-completion).

**Tasks**

- [x] Define JSON output on standard output, its version, fields, incomplete-result behavior, and existing exit policy. Separate current result transport from retention.
- [x] Expose the current typed evaluation through a reusable library projection and `verify --format json`, preserving default readable output, criterion states, and completion claims. Include criterion statements, source locations, scope, input identities, and evidence limits.
- [x] Exercise independent fixtures through the actual CLI: passing and failing tests, missing evidence, excluded methods, unchecked claims, incomplete input, changed inputs, and non-Git/dirty workspaces. Demonstrate correction and confirm that JSON output creates no report history or source edits.
- [x] Exercise this Book with the implemented command and record the result and limits. Update contribution guidance with a directly usable coding command.

### Completion criteria

#### Shared current result {#verification-evidence-3-retained-report}

The library and actual CLI expose a versioned current result with scope, states,
criterion statements, linked functions and locations, inputs, claims, counts,
and limits. Independent cases establish passing, failing, missing, excluded, and
incomplete behavior with unchanged exit meaning; changes cannot reuse earlier
passing runner evidence. Output can be consumed directly and creates no report
history. This Book exercises the interface. Native proof results remain separate
from outstanding criterion obligations. The fragment retains its original identity;
the outcome requires no artifact retention or report importer.

- **Anchor coverage:** [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output`, `incomplete-runs` — Partial: structured current-result presentation; [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: structured scope and limits.
- **Evidence:** Passed locally on engine revision `108d87b8acfd59e43e760e4a613bc9b7f722e2ea`, 2026-11-12. Five independent `structured::` cases in `tests/verification.rs` exercise the actual CLI and library projection: context and locations, default text compatibility, all relevant evidence states, unchecked and unsatisfied claims, partial methods and missing proof guards, incomplete diagnostics, malformed invocation, correction, changed inputs, and unchanged source/file inventories. The full independent suite and Clippy pass on stable and Rust 1.88.0, with the final structured cases and projection rechecked on both. Engine-independence experiments, the structural check, mdBook build, and all six Book acceptance tests pass.

  Native cargo-nextest 0.9.151 runs of this Book with `verify --config anchorbook.toml --anchor A-verify-progress-report --method test --format json` report both criteria satisfied (four and five linked tests respectively), exit 0, and 39 claims explicitly unchecked outside scope. The committed run identifies revision `108d87b8acfd59e43e760e4a613bc9b7f722e2ea` with `dirty: false`; an earlier working-tree run reports `dirty: true`. Results were consumed in memory. An independent non-Git Rust fixture additionally moves from a reported failure to satisfaction after correcting the implementation, then supports its recorded completion claim; the bounded coding probe below records the limits.

### Completion record

Complete locally on branch `codex/shared-verification-results` for the structured
current-result interface. The code revision and acceptance above establish that
scope. The implementation merged in [PR #44](https://github.com/anchor-book/anchor-book/pull/44)
as `76f415e4934734f29776dda15df8e5d060c8fbe7`; the local results keep their tested
revision, and GitHub checks are recorded on that PR. Renderer consumption and saved-input validation remain in
milestone 9. Retention is optional and needs a concrete purpose and expiration policy.

## Milestone 4. Both runners share criterion evidence and report behavior {#verification-evidence-4}

**Outcome:** Configuration, function associations, target planning, report identity fields, evidence states, and library/CLI reports are usable before either native runner.

**Status:** Complete

**Identifier:** `verification-evidence-4`

**Dependencies:** The record library, configuration loader, and navigation reader. Existing record identifiers remain usable.

**Design basis:** [Running Verification](../design/verification-evidence.md), including [links](../design/verification-evidence.md#evidence-links), [run identity](../design/verification-evidence.md#run-identity), states, and command behavior.

**Tasks**

- [x] Add the common `[verification]` fields to the loader, ignored by `check`; validate sources and separate output without invoking tools.
- [x] Select and record the Rust source parser and supported forms under the [Rust Engine dependency conventions](../design/rust-engine.md#rust-development-conventions). Evaluate ordinary Rust parsing (for example `syn`) and the additional parsing needed inside `verus!`; reject parser dependencies that defeat proof isolation.
- [x] Implement supported source associations and function identities, distinguishing tests, Verus executable/proof functions, and specification context. Report misplaced, duplicate, dangling, mismatched, ambiguous, and unsupported links.
- [x] Define target planning, report identity fields, normalized evidence, and per-method/criterion/anchor aggregation behind `verification`. Collect criteria/configuration content identities and Git revision/dirty state when available; code/lockfile collection and execution snapshots belong to milestones 1 and 2.
- [x] Expose typed reports and `anchorbook verify --config` scope/report behavior. Use independent fixture results and unavailable-runner cases; no native runner or speculative plugin API is required for this milestone.
- [x] Read coverage levels in the roadmap reader, rejecting a qualified Full entry with a located diagnostic; read claims from Complete milestones' Full coverage; fail the run for unsatisfied claims in scope and list unchecked ones.

### Completion criteria

#### Shared states {#verification-evidence-4-shared-states}

Independent evidence fixtures produce every method state and correct multi-method, criterion, and anchor aggregation, including linked proof failure versus target failures elsewhere. Only success for every required programmatic method satisfies a criterion.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: normalized state model; native test/proof results are in milestones 1 and 2.
- **Evidence:** Passed. `tests/verification.rs` `every_method_state_comes_from_links_and_this_runs_results` and `the_worst_linked_test_decides_and_reasons_name_the_function` evaluate a fixture plan against supplied results: passed, failed, skipped, and absent tests; unusable and unavailable runners; a linked proof failure outranking failures elsewhere and incompleteness, which are inconclusive; and a verified function without its proof guards as no-evidence. A criterion with test satisfied and proof not evaluated is partial; an unlinked criterion stays unlinked when no runner ran. `states_combine_as_the_design_orders_them` checks aggregation order.

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

**Identifier:** `verification-evidence-5`

**Dependencies:** Deferred until milestones 2, 3, and 6 are complete on this Book, and [Formal Verification milestone 4](formal-verification.md#formal-verification-4). Use a locally packaged release candidate; registry publication and contract delivery are not prerequisites for the fixtures.

**Design basis:** [Project verification](../design/project-integration.md#formal-verification-in-a-project), [A-project-verus-evidence](../design/project-integration.md#A-project-verus-evidence), and [verification responsibilities](../design/verification.md#responsibilities).

**Tasks**

- [ ] Prepare two independent packaged consumer fixtures with different Book/module layouts, a build toolchain separate from verification, and a consumer-selected `vstd` version different from the engine's proof workspace.
- [ ] Link a production executable contract and a supporting proof function to the consumer's criteria. Establish the runtime call path, preconditions, and excluded conversion/caller boundaries through tests and review.
- [ ] Exercise passing, deliberate proof failure and correction, incomplete/unsupported reports, unavailable tools, and trust-policy rejection through the real CLI. Add supported-version adapters only with native report fixtures.
- [ ] Run programmatic proof guards on the consumer's criteria: sensitivity with classified survivors and witnesses for any preconditions. Changing code or specifications requires fresh evidence.
- [ ] Document consumer setup, supported versions/forms, exact invocation, scope, assumptions, limits, and results for the tested package; retain partial states where executable evidence is missing.

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

Record the consumer criterion-to-contract assessment, production call path, satisfied preconditions, and test coverage of exclusions for the tested revisions. Reports disclose executed targets and unsatisfied executable obligations; proof success never implies all-method or whole-system verification.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `scope-and-limits` — Partial: consumer scope presentation. Fixture criterion fidelity and excluded boundaries are reviewed against the consumer's own Book.
- **Evidence:** Pending.

#### Consumer guidance {#verification-evidence-5-consumer-guidance}

A fresh fixture setup reproduces packaged adoption, tool selection, supported report/function forms, deliberate failure and recovery using the documented commands, without depending on this development checkout. Unsupported workspace forms and remaining release work are explicit.

- **Anchor coverage:** None — reproducible consumer setup and contributor guidance.
- **Evidence:** Pending.

### Completion record

Pending. Packaged fixture acceptance does not authorize publication or complete other project-integration capabilities.

## Milestone 6. Verify checks that proofs establish their criteria {#verification-evidence-6}

**Outcome:** Proof satisfaction requires current successful target verification,
verified witnesses, and sensitivity results accounted for by executable evidence.
Agents receive a criterion brief before developing a proof.

**Status:** Proposed

**Identifier:** `verification-evidence-6`

**Dependencies:** Milestones 1 and 2 and the scoped findings of [milestone 7](#verification-evidence-7).
Settle supported change operators, witness forms, and runtime budgets before native
implementation. Exclusion coverage uses the existing parsed proof-scope items.

**Design basis:** [Guards](../design/verification.md#guards), the
[workflow](../design/proof-guards.md), and the [brief](../design/verification-evidence.md#using-verify).

**Tasks**

- [ ] Generate changes only in linked executable bodies, verify complete targets in fresh copies, and run covering tests on survivors. Report unresolved, unviable, safety-only, and timed-out changes distinctly; reviewer equivalence judgments cannot clear survivors.
- [ ] Implement witness links and require a verified witness for each linked function with preconditions.
- [ ] Add a tool-free brief with scope, links, current input identities, and exclusions with their covering criteria and tests.
- [ ] Run the real CLI on `formal-authority` with every programmatic guard and record results and limits.

### Completion criteria

#### Sensitivity {#verification-evidence-6-sensitivity}

Native fixtures show an omitted rule as an unresolved survivor, the strengthened
specification rejecting that change, and excluded survivors classified only by
failing covering tests. Assert designed states for unviable, safety-only, and
timed-out changes; source specifications and proofs remain unchanged.

- **Anchor coverage:** [A-verify-proof-sensitive](../design/verification.md#A-verify-proof-sensitive) · `surviving-changes`, `classified-survivors`, `uninformative-changes` — Full
- **Evidence:** Pending.

#### Witnesses {#verification-evidence-6-witnesses}

Fixtures with unsatisfiable, unwitnessed, witnessed, and failing-witness
preconditions produce the designed proof states.

- **Anchor coverage:** [A-verify-proof-nonvacuous](../design/verification.md#A-verify-proof-nonvacuous) · `witness-required` — Full
- **Evidence:** Pending.

#### Brief {#verification-evidence-6-brief}

Fixtures assert scope, links, current identities, exclusions, and uncovered gaps
for criteria with and without proof requirements. The brief runs no tools.

- **Anchor coverage:** None — criterion guidance is assessed against the specified output; it adds no separate key correctness guarantee requiring another anchor.
- **Evidence:** Pending.

#### Pilot {#verification-evidence-6-pilot}

The engine's `formal-authority` method is satisfied with current programmatic
guards. A deliberate defect fails and correction passes; changed inputs require
fresh evidence. Record survivors, classifications, and the tested revision.

- **Anchor coverage:** [A-records-valid-authority](../design/rust-engine.md#A-records-valid-authority) · `formal-authority` — Full
- **Evidence:** Pending.

### Completion record

Pending. Simulated guard results exercise aggregation only; native witness and
sensitivity execution remain unimplemented.

## Milestone 7. Try the proof guards by hand on the authority proof {#verification-evidence-7}

**Outcome:** Recorded results show whether the proof guards find real weaknesses in an existing proof and what they cost, so the guard design is confirmed or revised before any tooling is built.

**Status:** Complete

**Identifier:** `verification-evidence-7`

**Dependencies:** The existing authority proof and the pinned Verus release documented in CONTRIBUTING. No engine tooling is required. This is a prototyping experiment: it changes no engine code or Book records, and its results do not verify any anchor.

**Design basis:** [Guards](../design/verification.md#guards), [proof guards](../design/verification-evidence.md#proof-guards), and the [authority proof](../design/rust-engine.md#authority-proof).

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

- **Anchor coverage:** None — design experiment; informs [A-verify-proof-nonvacuous](../design/verification.md#A-verify-proof-nonvacuous) and the retired specification-assessment proposal
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

## Milestone 9. The Book build returns current evidence to agents and readers {#verification-evidence-9}

**Outcome:** Agents developing from this Book receive current structural findings, criterion evidence, and completion-claim results through a shared report, and the rendered Book presents the same evaluated state. This capability is exercised here before Symposium relies on it to complete the full development workflow; its earlier current-tools baseline can inform the implementation.

**Status:** Proposed

**Identifier:** `verification-evidence-9`

**Dependencies:** Milestone 3's structured current-result interface and the existing structural checker and record renderer. Report retention is optional. Unsupported runners and unchecked claims remain explicit. The engine's proof gate remains separate.

**Design basis:** [Agent workflow](../design/system-overview.md#working-through-agents), [Book build evidence](../design/verification-evidence.md#book-build-evidence), and [Record Rendering](../design/record-rendering.md).

**Tasks**

- [x] Establish an initial coding-experience probe: use the current structured result to locate a deliberately failing criterion, correct its implementation or test from the Book definition, and rerun. Record where context, locations, or feedback need improvement before designing further integration.
- [ ] Use independent engine fixtures and this Book to settle the native build configuration, report handoff, artifact compatibility, input identities, and actionable diagnostics. Existing Symposium findings can inform the work; a new project test or coding trial is not a prerequisite. Implement the test-only path using existing runner outcomes, reconcile design and fixtures, and keep the engine usable with a broken Book.
- [ ] Compose structural findings, evidence states, and completion-claim results through shared library models. Expose stable record and criterion identities, source locations, scope, and limits through the CLI and versioned artifact so agents can act without HTML or handwritten result labels.
- [ ] Connect native project verification to rendering without duplicate test execution or recursive builds. Validate inputs before and after evaluation and rendering, including dirty working trees and specification changes; reject missing, invalid, unsupported, and mismatched handoffs.
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

### Initial coding probe {#verification-evidence-9-coding-probe}

On 2026-11-12, this coding session used engine revision
`108d87b8acfd59e43e760e4a613bc9b7f722e2ea` and native nextest 0.9.151 on a separate
Rust fixture with no Git repository. Its Book stated that incrementing inputs
0, 2, and 254 must return 1, 3, and 255. The initial implementation returned the
input unchanged. `verify --anchor A-increment --method test --format json`
reported `failed`, the criterion statement, the anchor definition location, and
the linked `tests::adds_one` location. Exit 0 correctly meant unclaimed progress,
not passing evidence. Reading the linked source and changing the implementation
to add one produced `satisfied` with a changed code identity. Recording that
fixture milestone Complete and rerunning produced one satisfied claim.

This establishes a small correction loop using current results, without retaining
report history. It is not a fresh-session usability study or evidence for the
renderer/build handoff. The result names the failing test but leaves detailed
assertion output in the native log; assessing that feedback and how clearly it
directs the next edit is the next coding-experience focus. The probe covered its
three stated examples and required no Symposium change.

### Completion record

Pending. The coding probe does not complete either shared-rendering criterion.

## Milestone 11. Anchor verification uses only programmatic evidence {#verification-evidence-11}

**Outcome:** The schema, CLI, reports, and proof aggregation accept only executable
verification. Legacy review declarations fail clearly; judgments cannot satisfy or
gate a method. The Book and working contract use the same policy.

**Status:** Complete

**Identifier:** `verification-evidence-11`

**Dependencies:** Existing record reader, test runner, proof target runner, and
simulated guard aggregation.

**Design basis:** [Programmatic verification](../design/verification.md#A-verify-programmatic-only),
[methods](../design/anchor-definitions.md#verification-methods), and
[review boundaries](../design/verification.md#review-and-verification).

**Tasks**

- [x] Remove review from the typed method set, record grammar, CLI selection, and default report scope.
- [x] Remove specification assessment from proof aggregation; retain current target checks, witnesses, and sensitivity obligations.
- [x] Reconcile the contract, Book criteria, proof design, and retired assessment work. Preserve historical evidence and identify missing executable checks.
- [x] Complete independent tests, supported-toolchain checks, Book checks, and evidence validation.

### Completion criteria

#### Programmatic methods {#verification-evidence-11-methods}

Independent fixtures and the actual CLI reject legacy review declarations and
selectors, retain supported methods, and establish proof states from programmatic
results without an approval record. Missing or failed checks remain outstanding.

- **Anchor coverage:** [A-verify-programmatic-only](../design/verification.md#A-verify-programmatic-only) · `methods`, `programmatic-states` — Full
- **Evidence:** Passed on Rust 1.88.0 and stable 1.99.1. The ordinary suite has 183 passing tests; fixtures cover grammar, CLI rejection, unchanged sources, programmatic guard aggregation, and completion claims. The pinned nextest run satisfies both criteria with no link findings.

#### Migration {#verification-evidence-11-migration}

Current Book records use executable criteria and render with valid references.
Historical review evidence is not recast as a test pass. Missing automation has
explicit work in milestone 12; proof guard execution remains in milestone 6.

- **Anchor coverage:** None — schema migration and current-Book acceptance.
- **Evidence:** Passed: structural checking, mdBook rendering, four current-Book tests, and rendered records/links acceptance. Browser acceptance is a separate unlinked criterion, so existing presentation tests do not hide its missing automation.

### Completion record

Complete locally on 2026-11-24, on `programmatic-verification` based on
`3bd59b7c1e352e94bafcf133dc05c9a7560a48df`. Rust 1.88.0 and stable 1.99.1 tests
and Clippy pass, as do formatting, Book checks, and engine-independence experiments.
The test-scoped evidence run has no findings and passes all evaluated completion
claims; proof remains outside that run. Native proof guard execution and migrated
acceptance checks remain in milestones 6 and 12. No new mutation campaign or
GitHub CI result is claimed by that local record. The implementation subsequently
merged in [PR #50](https://github.com/anchor-book/anchor-book/pull/50) as
`b809f76f107e1ed0d7edc9e66f18f516fb94e34d`.

## Milestone 12. Automate the migrated acceptance criteria {#verification-evidence-12}

**Outcome:** Core criteria formerly relying on review gain executable checks for the
behavior their design now specifies. Historical trials remain evidence of their
original scope until these checks run.

**Status:** Proposed

**Identifier:** `verification-evidence-12`

**Dependencies:** Milestone 11; agree independent expected results for each fixture
and scoped runtime budgets before agent or browser trials. Dependency audits
depend on the declared packaging and compatibility policy. Optional RFD support
is not a prerequisite.

**Design basis:** [Diagnostics](../design/check-diagnostics.md),
[Project Integration](../design/project-integration.md),
[Record Rendering](../design/record-rendering.md), and
[CI independence](../design/system-overview.md#A-check-independent-build).

**Tasks**

- [ ] Implement an executable audit of same-revision CI outcomes and dependency order for valid and seeded broken Books.
- [ ] Add agent trial harnesses with independently specified repairs, expected workflow choices and reported questions, preserved-record assertions, and repeat-adoption checks.
- [ ] Add the packaged dependency and feature audit, including a seeded undeclared pin and default verifier dependency.
- [ ] Add browser assertions for keyboard operation, collapsed/expanded state, light/dark/narrow presentation, print visibility, and record navigation under a project site prefix and in the combined print view.
- [ ] Link only checks covering these assertions, run them, and restore Full milestone coverage only for established scope.

### Completion criteria

#### Executable acceptance {#verification-evidence-12-acceptance}

Each core criterion covered below has linked checks derived from its specified expectations,
including negative cases. Checks assert observed behavior and never use a human or
agent judgment as the pass condition.

- **Anchor coverage:** [A-check-independent-build](../design/system-overview.md#A-check-independent-build) · `ci-order` — Full; [A-check-actionable-diagnostics](../design/check-diagnostics.md#A-check-actionable-diagnostics) · `agent-repair` — Full
- **Evidence:** Pending.

#### Adoption and presentation {#verification-evidence-12-adoption}

The dependency audit, bootstrap harness, and browser assertions exercise
their design criteria with independently specified results and seeded failures.

- **Anchor coverage:** [A-project-no-imposed-dependencies](../design/project-integration.md#A-project-no-imposed-dependencies) · `dependency-review` — Full; [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `fresh-session` — Full; [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `browser-presentation`, `site-prefix-navigation`, `print-navigation` — Full
- **Evidence:** Pending.

### Completion record

Pending. Existing passing parser, adapter, integration, and rendering fixtures
retain their original scope. They do not establish the new browser assertions or
replace the earlier human assessment of an agent trial. RFD incorporation and
agent-workflow checks remain in [RFD milestones 1 and 2](rfds.md); their draft
obligations are preserved without making that optional capability a completion
requirement here.

## Milestone 13. Known obligations remain visible until satisfied {#verification-evidence-13}

**Outcome:** Explicit criteria expose known verification gaps, and a CLI regression
protects all-criteria aggregation. Thinkwell supplies the consumer acceptance case.

**Status:** Complete

**Identifier:** `verification-evidence-13`

**Dependencies:** Milestone 11, the test runner, and Thinkwell's existing test links.

**Design basis:** [Independent criteria](../design/anchor-definitions.md#independently-verifiable-criteria),
[all-criteria aggregation](../design/verification-evidence.md#A-verify-criterion-states),
and the contract's declaration of every verification obligation.

**Tasks**

- [x] Clarify independent obligation authoring in the contract and design.
- [x] Add the CLI regression for required criteria and missing evidence.
- [x] Extract Thinkwell's uncovered obligations without changing product guarantees.
- [x] Run supported-toolchain, contract, Book, and consumer checks.

### Completion criteria

#### All criteria {#verification-evidence-13-all-criteria}

The CLI reports partial for a passing runtime criterion plus an unlinked, missing,
skipped, or failed compiler criterion. Only both current passing results satisfy
the anchor. Unavailable or excluded methods never satisfy it, and losing a link
cannot shrink the required criterion set.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `all-criteria` — Full
- **Evidence:** `cargo test --locked` passes 184 tests; eight opt-in checks are
  ignored. Verification and contract suites also pass on Rust 1.88.0. Formatting
  and all-target Clippy with warnings denied pass on stable and 1.88.0. The Book
  builds; four current-Book checks and rendered-record/link acceptance pass.
  `verify --method test --format json` reports this criterion and its Full claim
  satisfied, with no findings. The CLI fixture uses controlled runner outcomes;
  it supplies no consumer compiler evidence. Runtime implementation is unchanged,
  so no mutation campaign was added.

#### Consumer acceptance {#verification-evidence-13-consumer}

Thinkwell's original passing tests retain their results, while every known gap
has a criterion and prevents its anchor from becoming satisfied. No product code,
test assertion, or anchor property changes to obtain the result.

- **Anchor coverage:** None — consumer acceptance; product criteria retain their own evidence.
- **Evidence:** The same 38 linked tests pass before and after refinement. Seven
  runtime criteria pass; 15 new criteria are unlinked. Routing and cancellation
  anchors are satisfied; five anchors are partial. JSON and JUnit assertions
  confirm these results. Code identity remains `c5aec1dd9dd03591` and configuration
  identity `a9835f786b88232f`; criteria change from `430a13eaa269155e` to
  `3173bc7858e8742e`. Bootstrap and structural checks pass, as does rendered
  acceptance of 37 records and 1,262 local links. The existing pinned engine
  already enforces aggregation; the correction declares the missing obligations.
  Missing product checks remain work in Thinkwell's executable-verification roadmap.

### Evidence {#verification-evidence-13-evidence}

Local uncommitted trees: engine branch `complete-criteria` based on
`fb7badc8f3bb763f9ba89d6c15c8701faefe1e46`; Thinkwell branch `port-phase-adoption`
based on `4f13784f3b07cfee26f745901cafe1b0ca3b2cf6`. Consumer execution is on Linux.
Commands and runner setup follow each project's contributor guidance.

### Completion record

Complete for local implementation and consumer acceptance; integrated in
[PR #50](https://github.com/anchor-book/anchor-book/pull/50) as
`b809f76f107e1ed0d7edc9e66f18f516fb94e34d`. The recorded Thinkwell trial retains
its original criterion set and tested revisions. Compiler-evidence import and CI exit-policy changes remain
separate work.

## Retired work

### Mandatory CI report retention {#verification-evidence-3-ci-results}

The requirement to archive a whole-Book report for every PR and main revision is
retired. Current results can be consumed directly; retaining every run is not
required to enable coding. CI integration and avoiding duplicate test execution
remain in milestone 9 and Project Integration. Optional retained artifacts need
a concrete diagnostic or evidence purpose and an explicit expiration policy.
This criterion identifier is reserved and its former outcome is not claimed complete.


### Execution-report protocol {#verification-evidence-10}

`verification-evidence-10` is retired, not completed. The engineer clarified
that projects own their tests and outcomes, and Anchor Book may trust those
results, as [D-project-test-results-trusted](../authority/decisions.md#D-project-test-results-trusted)
records. No execution-report producer protocol, link qualifier, runtime-input
attestation, or special completion-claim rule is required. The proposed protocol
was not implemented. This identifier and its criterion identifiers are reserved
and will not be reused.

#### Execution design {#verification-evidence-10-execution-design}

Retired with the unaccepted producer/runner interface proposal.

#### Current execution {#verification-evidence-10-current-execution}

Retired. Existing runner identity, current-result handling, and evidence states
remain engine responsibilities; internal execution semantics belong to projects.

#### Repeated trial {#verification-evidence-10-repeated-trial}

Retired as engine work. Symposium decides whether to change the no-agent test
outcome and validates any change in its own tests.

### Criterion review records {#verification-evidence-8}

The proposed review-method reader is retired, not completed. Programmatic-only
verification supersedes it; no assessment directory or judgment format is needed.
`D-assessment-records-as-files` and `A-verify-proof-assessment-current` were draft
records for that abandoned design and are retired without reusing their IDs.
Git history retains the proposal. Review remains part of the development workflow.

#### Review states {#verification-evidence-8-review-states}

Retired with the review method; the identifier remains reserved.

#### Sufficiency assessed {#verification-evidence-8-sufficiency-assessed}

Retired as a verification outcome. Review findings inform design and test changes.

#### Specification assessments {#verification-evidence-6-assessments}

Retired. Assessment freshness, acceptance, and reviewer equivalence listings no
longer gate proof satisfaction. Current executable evidence is still required.
