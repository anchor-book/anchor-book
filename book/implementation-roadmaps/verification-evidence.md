# Verification Evidence

**Roadmap identifier:** `verification-evidence`

## Goal

`anchorbook verify` reports progress toward the goal design for each anchor and
criterion, running the linked tests and proofs and checking that proofs establish
their criteria, as defined in [Verification](../design/verification.md) and
[Running Verification](../design/verification-evidence.md). It serves agents in
the build loop and CI with the same command. This Book's own anchors are the first
pilot; independent consumers establish that their own tools and code can supply
evidence for their criteria.

None of this Book's criteria are linked to tests or proofs yet, so `verify` will
initially report them as unlinked. Linking criteria is part of the work.

## Sequence and scope

Start with the shared foundation in milestone 4. Milestones 1 and 2 then provide
test and proof runners independently. Milestone 5 exercises consumer proofs after
[engine proof isolation](formal-verification.md#formal-verification-4); milestone 3
adds CI and versioned retained reports. Milestone 6 adds the proof guards, assessment
records, and the agent brief once both runners exist; it starts with a bounded
experiment on the engine's authority proof. Existing milestone and criterion IDs stay
stable. All work here remains Proposed until its scope is agreed and assigned.

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

<a id="verification-evidence-1"></a>

## Milestone 1. An agent sees test progress for an anchor in the build loop

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">An agent or engineer runs <code>anchorbook verify --anchor &lt;id&gt; --method test</code> after a change and sees that anchor's test criteria for the current files, with links resolved and drift reported; a whole-Book run reports progress by chapter.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="#verification-evidence-4">Milestone 4</a> and agreement on nextest support.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html">Running Verification</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/check-diagnostics.html">Diagnostics</a>, and <a href="../design/rust-engine.html">Rust Engine</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Use milestone 4's configuration, link inventory, states, and report API; implement nextest selection, an exact filter with ignored tests included, JUnit interpretation, and test-name mapping.</label></li>
<li><label><input type="checkbox" disabled> Connect the runner to <code>anchorbook verify --config</code>, preserving the shared scope and exit statuses. Collect code and lockfile content identities and compare input snapshots before and after execution.</label></li>
<li><label><input type="checkbox" disabled> Exercise changed-input and stale-report cases with the actual runner; neither can produce satisfied evidence.</label></li>
<li><label><input type="checkbox" disabled> Pin nextest and document installing it in CONTRIBUTING; ordinary <code>cargo test</code> and CI test jobs are unchanged.</label></li>
<li><label><input type="checkbox" disabled> Link the test criteria of the <code>verify-*</code> anchors and at least one existing checker anchor, and record a build-loop run.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="verification-evidence-1-test-states" class="roadmap-criterion-summary"><th scope="row">Test states</th><td><p class="roadmap-condition">Independent fixtures produce every state for the test method; criteria requiring several methods report partial or not evaluated correctly; scoped runs name what they exclude.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for test states" aria-expanded="true" aria-controls="verification-evidence-1-test-states-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-test-states-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>test-states</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code>, <code>scope-and-limits</code> — Partial: test method and its execution scope; proof scope is in milestone 2.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-progress-report" class="roadmap-criterion-summary"><th scope="row">Progress report</th><td><p class="roadmap-condition">For a fixture Book with satisfied, partial, failing, and unlinked anchors, the library and actual CLI report every anchor by chapter with states, links, and summary counts and exit 0; invalid inputs make the run incomplete; an unbuildable or missing runner affects only its method.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for progress report" aria-expanded="true" aria-controls="verification-evidence-1-progress-report-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-progress-report-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>progress-report</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-progress-report">verify-progress-report</a> · <code>progress-output</code>, <code>incomplete-runs</code> — Partial: native test progress; shared reporting is in milestone 4, proof results in milestone 2, and retained CI reports in milestone 3.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-links" class="roadmap-criterion-summary"><th scope="row">Links</th><td><p class="roadmap-condition">Fixtures with dangling, mismatched, misplaced, and duplicate links and unlinked criteria produce located findings; corrected fixtures resolve.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for links" aria-expanded="true" aria-controls="verification-evidence-1-links-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-links-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>links</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-links-resolve">verify-links-resolve</a> · <code>link-findings</code> — Partial: test links</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-selected-runs" class="roadmap-criterion-summary"><th scope="row">Selected runs</th><td><p class="roadmap-condition">Recording runners and native nextest runs select exactly the linked tests in scope and identify code and lockfile contents. Before-and-after input snapshots detect changes and make affected evidence inconclusive; earlier reports never count. Linked tests not executed, skipped, or missing have no evidence; the runner preserves Book and source files.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for selected runs" aria-expanded="true" aria-controls="verification-evidence-1-selected-runs-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-selected-runs-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>selected-runs</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>selected-run</code>, <code>source-preservation</code> — Partial: native test selection, current inputs, and preservation; proof execution is in milestone 2.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-build-loop" class="roadmap-criterion-summary"><th scope="row">Build loop</th><td><p class="roadmap-condition">In this repository, <code>anchorbook verify --anchor</code> for a linked anchor reports its criteria after a passing change, a deliberately failing change, and the restored code, and a whole-Book run reports the remaining unlinked criteria; record the commands and output.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for build loop" aria-expanded="true" aria-controls="verification-evidence-1-build-loop-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-build-loop-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>build-loop</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — acceptance against this Book</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="verification-evidence-2"></a>

## Milestone 2. Verify runs and reports proofs

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1"><code>verify</code> runs the configured project's Verus over complete linked targets and returns criterion evidence from a supported, mode-aware report adapter. The engine's <code>formal-authority</code> criterion is the first pilot.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="#verification-evidence-4">Milestone 4</a>, the existing engine proof, and a scoped native compatibility experiment that settles configuration fields and supported function/target forms before the complete runner is assigned. Nextest completion is not required.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html#verus-runner">Verus runner</a>, <a href="../design/rust-engine.html#authority-proof">authority proof</a>, and <a href="../design/project-integration.html#project-verus-evidence">project-verus-evidence</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> <strong>Setup:</strong> Settle installation/version/toolchain and manifest/package/target/feature fields with the pinned native verifier. Capture executable and proof-function reports; declare supported forms.</label></li>
<li><label><input type="checkbox" disabled> <strong>Adapter:</strong> Normalize one supported Verus report format into the shared model. Match identities and modes uniquely using resolved links, replacing the engine's hard-coded function inventory.</label></li>
<li><label><input type="checkbox" disabled> <strong>Execution:</strong> Run each required target once with locked dependencies, the trust policy, and fresh target/report directories. Collect code and lockfile content identities, before-and-after input snapshots, actual tools, invocation, and scope.</label></li>
<li><label><input type="checkbox" disabled> <strong>Pilot:</strong> Link the authority validator to <code>formal-authority</code> and exercise the real CLI, retaining its specification assessment and excluded integration obligations.</label></li>
<li><label><input type="checkbox" disabled> <strong>Failure cases:</strong> Exercise missing, duplicate, skipped, and wrong-mode functions; unsupported versions; empty, malformed, or partial reports; and zero coverage.</label></li>
<li><label><input type="checkbox" disabled> <strong>Failure cases:</strong> In a native fixture, distinguish a linked function's failure from a passing linked caller whose unlinked supporting lemma fails. Report failed and inconclusive respectively, list failing obligations, then restore full-target success.</label></li>
<li><label><input type="checkbox" disabled> <strong>Failure cases:</strong> Exercise cached runs with no new report, changed code or lockfile, absent tools, and build failures through the actual runner.</label></li>
<li><label><input type="checkbox" disabled> <strong>Trust policy:</strong> Demonstrate native rejection of deliberate assumptions and trusted project bodies for the supported version.</label></li>
<li><label><input type="checkbox" disabled> <strong>Acceptance:</strong> Record native failure/correction results, report fixtures, commands, runtime, scope, assumptions, and limits.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="verification-evidence-2-proof-states" class="roadmap-criterion-summary"><th scope="row">Proof states</th><td><p class="roadmap-condition">Fixtures cover every proof-method state. Native runs demonstrate a linked failure as failed, a passing linked caller with a failing unlinked lemma as inconclusive, correction as satisfied, and build failure and unavailable tools in their designed states. The engine's <code>formal-authority</code> proof method is satisfied for the tested inputs; test and review obligations retain their own states.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for proof states" aria-expanded="true" aria-controls="verification-evidence-2-proof-states-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-2-proof-states-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>proof-states</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: proof method; <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>selected-run</code> — Partial: proof runner; <a href="../design/verification-evidence.html#verify-links-resolve">verify-links-resolve</a> · <code>link-findings</code> — Partial: proof links</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-2-report-integrity" class="roadmap-criterion-summary"><th scope="row">Report integrity</th><td><p class="roadmap-condition">Real and independent fixture reports establish mode-aware, unique function matching and full-target success. Missing, duplicate, skipped, wrong-mode, partial, malformed, zero-coverage, or unsupported-version evidence never satisfies a method. Failures outside a criterion's links make its proof evidence inconclusive and retain the failing obligations and locations.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for report integrity" aria-expanded="true" aria-controls="verification-evidence-2-report-integrity-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-2-report-integrity-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>report-integrity</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-links-resolve">verify-links-resolve</a> · <code>link-findings</code> — Partial: proof modes and result identities; <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: proof report interpretation.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-2-current-runs" class="roadmap-criterion-summary"><th scope="row">Current runs</th><td><p class="roadmap-condition">Recording runners and native runs establish exactly the required targets, actual features and tools, fresh reports, and code and lockfile content identities including dirty sources. Before-and-after input snapshots detect changes and make affected evidence inconclusive. Earlier reports and successful commands without new verification output never count as passing evidence; the runner preserves Book and source files.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for current runs" aria-expanded="true" aria-controls="verification-evidence-2-current-runs-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-2-current-runs-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>current-runs</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>selected-run</code>, <code>source-preservation</code> — Partial: native proof execution, completing the test-runner scope in milestone 1; <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>scope-and-limits</code> — Partial: proof target scope.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-2-trust-policy" class="roadmap-criterion-summary"><th scope="row">Trust policy</th><td><p class="roadmap-condition">For the supported verifier, deliberate assumptions and trusted project bodies fail the configured policy. Reports retain trusted dependencies and excluded boundaries; zero errors or a successful linked function cannot override partial-target verification or an unsupported report version.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for trust policy" aria-expanded="true" aria-controls="verification-evidence-2-trust-policy-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-2-trust-policy-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>trust-policy</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-verus-evidence">project-verus-evidence</a> · <code>trusted-assumptions</code> — Partial: the first supported adapter and engine pilot; independent consumers and their tool selections are in milestone 5.</p>
<p><strong>Evidence:</strong> Pending. Existing engine proof runs do not establish consumer-runner behavior.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="verification-evidence-3"></a>

## Milestone 3. CI reports progress for every revision

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Each pull request and main revision runs <code>anchorbook verify</code> for the whole Book and retains its report.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Verification Evidence — Milestones 1, 2, and 5.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html">Running Verification</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-3</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Define a versioned artifact schema from the typed report, with current inputs, executed scope, tools, states, assumptions, and limits; review retained fields against public-content guidance.</label></li>
<li><label><input type="checkbox" disabled> Run the whole-Book command in CI with supported tools and retain reports for passing, unsatisfied, and incomplete runs. Preserve the progress command's exit policy.</label></li>
<li><label><input type="checkbox" disabled> Demonstrate an authorized failure/correction cycle and record actual revision-specific workflow results, retention, and runtime.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="verification-evidence-3-retained-report" class="roadmap-criterion-summary"><th scope="row">Retained report</th><td><p class="roadmap-condition">A versioned artifact preserves the typed report's scope and states, input identities, tools, and limits, with private paths and raw source/proof payloads omitted. Invalid or stale artifacts never become evidence for another run.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for retained report" aria-expanded="true" aria-controls="verification-evidence-3-retained-report-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-3-retained-report-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>retained-report</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-progress-report">verify-progress-report</a> · <code>progress-output</code>, <code>incomplete-runs</code> — Partial: retained CI presentation; <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>scope-and-limits</code> — Partial: artifact scope and limits.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-3-ci-results" class="roadmap-criterion-summary"><th scope="row">CI results</th><td><p class="roadmap-condition">Actual runs identify the tested checkout and demonstrate passing, failing-evidence, and incomplete outcomes, followed by correction at a new revision. Unsatisfied evidence remains progress; merge enforcement and rendered progress are separately agreed work.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for CI results" aria-expanded="true" aria-controls="verification-evidence-3-ci-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-3-ci-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — CI execution, artifact retention, and correction workflow.</p>
<p><strong>Evidence:</strong> Pending; local results alone cannot complete this criterion.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="verification-evidence-4"></a>

## Milestone 4. Both runners share criterion evidence and report behavior

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Configuration, function associations, target planning, report identity fields, evidence states, and library/CLI reports are usable before either native runner.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">The record library, configuration loader, navigation reader, and engineer agreement on this bounded foundation. Existing record identifiers remain usable.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html">Running Verification</a>, including <a href="../design/verification-evidence.html#evidence-links">links</a>, <a href="../design/verification-evidence.html#run-identity">run identity</a>, states, and command behavior.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-4</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Add the common <code>[verification]</code> fields to the loader, ignored by <code>check</code>; validate sources and separate output without invoking tools.</label></li>
<li><label><input type="checkbox" disabled> Select and record the Rust source parser and supported forms under the <a href="../design/rust-engine.html#rust-development-conventions">Rust Engine dependency conventions</a>. Evaluate ordinary Rust parsing (for example <code>syn</code>) and the additional parsing needed inside <code>verus!</code>; reject parser dependencies that defeat proof isolation.</label></li>
<li><label><input type="checkbox" disabled> Implement supported source associations and function identities, distinguishing tests, Verus executable/proof functions, and specification context. Report misplaced, duplicate, dangling, mismatched, ambiguous, and unsupported links.</label></li>
<li><label><input type="checkbox" disabled> Define target planning, report identity fields, normalized evidence, and per-method/criterion/anchor aggregation behind <code>verification</code>. Collect criteria/configuration content identities and Git revision/dirty state when available; code/lockfile collection and execution snapshots belong to milestones 1 and 2.</label></li>
<li><label><input type="checkbox" disabled> Expose typed reports and <code>anchorbook verify --config</code> scope/report behavior. Use independent fixture results and unavailable-runner cases; no native runner or speculative plugin API is required for this milestone.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="verification-evidence-4-shared-states" class="roadmap-criterion-summary"><th scope="row">Shared states</th><td><p class="roadmap-condition">Independent evidence fixtures produce every method state and correct multi-method, criterion, and anchor aggregation, including linked proof failure versus target failures elsewhere. The review method remains not evaluated until milestone 6, and only all-required-method success satisfies a criterion.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for shared states" aria-expanded="true" aria-controls="verification-evidence-4-shared-states-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-4-shared-states-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>shared-states</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: normalized state model; native test/proof results are in milestones 1 and 2.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-4-function-links" class="roadmap-criterion-summary"><th scope="row">Function links</th><td><p class="roadmap-condition">A recorded parser/dependency review identifies supported Rust and Verus forms. Independent source fixtures resolve identities and methods against real inventory IDs, reject invalid links, and distinguish same-named functions by module/package/target. A specification declaration alone supplies no successful proof evidence; unsupported forms have located findings.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for function links" aria-expanded="true" aria-controls="verification-evidence-4-function-links-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-4-function-links-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>function-links</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-links-resolve">verify-links-resolve</a> · <code>link-findings</code> — Partial: source discovery and inventory resolution; result matching is in each runner milestone.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-4-plans-and-reports" class="roadmap-criterion-summary"><th scope="row">Plans and reports</th><td><p class="roadmap-condition">Fixture plans identify requested criteria, selected tests/proof targets, excluded scope, criteria/configuration content identities, and Git revision/dirty state, remaining usable without Git. Library and actual CLI reports agree for valid, invalid, and unavailable-runner inputs, preserve sources, and use the designed progress/incomplete exit behavior. The report model includes execution identity fields; native collection and snapshots remain in milestones 1 and 2.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for plans and reports" aria-expanded="true" aria-controls="verification-evidence-4-plans-and-reports-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-4-plans-and-reports-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>plans-and-reports</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-progress-report">verify-progress-report</a> · <code>progress-output</code>, <code>incomplete-runs</code>; <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>scope-and-limits</code>; <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>selected-run</code>, <code>source-preservation</code> — Partial: planning and reporting before native execution; milestones 1 and 2 supply native results.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. This milestone provides common machinery, not passing native proof or test evidence.</td></tr>
</tbody>
</table>
</div>

<a id="verification-evidence-5"></a>

## Milestone 5. Consumers verify their own production code

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Independent Rust consumers use a packaged engine and their own Verus installations to obtain criterion evidence for their production code, with tested scope and dependency boundaries.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Milestones 2 and 6 and <a href="formal-verification.html#formal-verification-4">Formal Verification milestone 4</a>. Use a locally packaged release candidate; registry publication and contract delivery are not prerequisites for the fixtures.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/project-integration.html#formal-verification-in-a-project">Project verification</a>, <a href="../design/project-integration.html#project-verus-evidence">project-verus-evidence</a>, and <a href="../design/verification.html#responsibilities">verification responsibilities</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-5</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Prepare two independent packaged consumer fixtures with different Book/module layouts, a build toolchain separate from verification, and a consumer-selected <code>vstd</code> version different from the engine's proof workspace.</label></li>
<li><label><input type="checkbox" disabled> Link a production executable contract and a supporting proof function to the consumer's criteria. Establish the runtime call path, preconditions, and excluded conversion/caller boundaries through tests and review.</label></li>
<li><label><input type="checkbox" disabled> Exercise passing, deliberate proof failure and correction, incomplete/unsupported reports, unavailable tools, and trust-policy rejection through the real CLI. Add supported-version adapters only with native report fixtures.</label></li>
<li><label><input type="checkbox" disabled> Run the proof guards on the consumer's criteria: sensitivity with classified survivors, witnesses for any preconditions, and a recorded specification assessment, including a stale review after a specification change.</label></li>
<li><label><input type="checkbox" disabled> Document consumer setup, supported versions/forms, exact invocation, scope, assumptions, limits, and results for the tested package; retain partial states where review evidence is unsupported.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="verification-evidence-5-consumer-tools" class="roadmap-criterion-summary"><th scope="row">Consumer tools</th><td><p class="roadmap-condition">Both consumers build the packaged engine without its verifier dependencies and run their configured Verus installations using verification Rust separate from build Rust. A different consumer <code>vstd</code> resolves and builds; reports identify the actual tools and supported adapter used.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for consumer tools" aria-expanded="true" aria-controls="verification-evidence-5-consumer-tools-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-5-consumer-tools-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>consumer-tools</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-verus-evidence">project-verus-evidence</a> · <code>separate-toolchain</code> — Full; <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>independent-resolution</code> — Partial: proof dependencies and consumer <code>vstd</code> choice. Other exact pins and general release compatibility remain separate work.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-5-consumer-proof" class="roadmap-criterion-summary"><th scope="row">Consumer proof</th><td><p class="roadmap-condition">The consumer's own linked functions produce satisfied proof evidence, a deliberate defect fails, and correction passes at new input identities. Assumed/trusted project bodies, partial verification, unsupported versions, and engine proof results cannot satisfy a consumer criterion.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for consumer proof" aria-expanded="true" aria-controls="verification-evidence-5-consumer-proof-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-5-consumer-proof-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>consumer-proof</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-verus-evidence">project-verus-evidence</a> · <code>trusted-assumptions</code> — Full; <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: independent consumer proof states.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-5-scope-review" class="roadmap-criterion-summary"><th scope="row">Scope review</th><td><p class="roadmap-condition">Record the consumer criterion-to-contract assessment, production call path, satisfied preconditions, and test/review coverage of exclusions for the tested revisions. Reports disclose executed targets and any unsatisfied review method; proof success never implies all-method or whole-system verification.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for scope review" aria-expanded="true" aria-controls="verification-evidence-5-scope-review-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-5-scope-review-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>scope-review</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>scope-and-limits</code> — Partial: consumer scope presentation. Fixture criterion fidelity and excluded boundaries are reviewed against the consumer's own Book.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-5-consumer-guidance" class="roadmap-criterion-summary"><th scope="row">Consumer guidance</th><td><p class="roadmap-condition">A fresh fixture setup reproduces packaged adoption, tool selection, supported report/function forms, deliberate failure and recovery using the documented commands, without depending on this development checkout. Unsupported workspace forms, review ingestion, and remaining release work are explicit.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for consumer guidance" aria-expanded="true" aria-controls="verification-evidence-5-consumer-guidance-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-5-consumer-guidance-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>consumer-guidance</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — reproducible consumer setup and contributor guidance.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. Packaged fixture acceptance does not authorize publication or complete other project-integration capabilities.</td></tr>
</tbody>
</table>
</div>

<a id="verification-evidence-6"></a>

## Milestone 6. Verify checks that proofs establish their criteria

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">A proof method is satisfied only when its specification constrains the linked code, its preconditions can be met, and a current specification assessment accepts it; review criteria are evaluated from assessment records; and agents get a brief of each criterion before writing its proof.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Milestones 1 and 2. Begin with a bounded experiment on the engine's authority proof that settles the change operators, the witness form, the fingerprint inputs, and the runtime per change, and review it before assigning the full milestone. Exclusion coverage in the brief needs the proof-scope items of <a href="record-line-grammar.html#record-line-grammar-2">Record Line Grammar milestone 2</a>; before then the brief shows the statement text.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification.html#guards">Guards</a>, <a href="../design/verification.html#assessment-records">assessment records</a>, <a href="../design/verification-evidence.html#proof-guards">proof guards</a>, and the <a href="../design/verification-evidence.html#using-verify">brief</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-6</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> <strong>Experiment:</strong> On a disposable copy of the authority validator, apply candidate change operators to its seven executable functions and record which changes fail verification, which survive, and why. Include the accepted-status change, which must fail, and a failure-classification change, which the specification deliberately leaves to tests. Measure time per change; try a witness for <code>validate_reference</code>.</label></li>
<li><label><input type="checkbox" disabled> <strong>Sensitivity:</strong> Generate changes inside <code>verus!</code> executable bodies with the milestone 4 parser, never in specifications or proofs. Verify each in a fresh copy; classify survivors through the covering criteria's linked tests and the review's equivalent listing; report unviable, safety-only, and timed-out changes separately.</label></li>
<li><label><input type="checkbox" disabled> <strong>Witnesses:</strong> Implement the <code>witness</code> link form and require a verified witness for each linked function with preconditions.</label></li>
<li><label><input type="checkbox" disabled> <strong>Fingerprints and assessments:</strong> Compute token-based specification fingerprints over the criterion, linked contracts, and reachable specification functions and types. Add the <code>reviews</code> configuration field and the assessment record schema; read records, report malformed and stale ones, and evaluate the review method.</label></li>
<li><label><input type="checkbox" disabled> <strong>Brief:</strong> Add <code>--brief</code>, running no tools, with each criterion's scope, links, fingerprint, assessment status, and exclusions with their covering criteria and tests.</label></li>
<li><label><input type="checkbox" disabled> <strong>Pilot:</strong> Record a specification assessment of <code>formal-authority</code> and run every guard on it through the real CLI; record survivors and their classification.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="verification-evidence-6-sensitivity" class="roadmap-criterion-summary"><th scope="row">Sensitivity</th><td><p class="roadmap-condition">Native fixtures show an omitted rule as an unclassified survivor that fails the proof method, the strengthened specification rejecting the same change, survivors classified only by failing covering tests or a current equivalent listing, and unviable, safety-only, and timed-out changes in their designed outcomes. Specifications and proofs are never changed.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for sensitivity" aria-expanded="true" aria-controls="verification-evidence-6-sensitivity-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-6-sensitivity-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>sensitivity</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification.html#verify-proof-sensitive">verify-proof-sensitive</a> · <code>surviving-changes</code>, <code>classified-survivors</code>, <code>uninformative-changes</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-6-witnesses" class="roadmap-criterion-summary"><th scope="row">Witnesses</th><td><p class="roadmap-condition">Fixtures with unsatisfiable, unwitnessed, witnessed, and failing-witness preconditions produce the designed proof states.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for witnesses" aria-expanded="true" aria-controls="verification-evidence-6-witnesses-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-6-witnesses-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>witnesses</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification.html#verify-proof-nonvacuous">verify-proof-nonvacuous</a> · <code>witness-required</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-6-assessments" class="roadmap-criterion-summary"><th scope="row">Assessments</th><td><p class="roadmap-condition">Fixtures show which changes alter the fingerprint and which do not; missing, stale, rejecting, and current assessment records produce the designed proof and review method states; malformed and duplicate records are located findings; <code>verify</code> never writes a record.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for assessments" aria-expanded="true" aria-controls="verification-evidence-6-assessments-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-6-assessments-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>assessments</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification.html#verify-proof-assessment-current">verify-proof-assessment-current</a> · <code>fingerprint-coverage</code>, <code>assessment-states</code> — Full; <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: the review method; <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>source-preservation</code> — Partial: assessment records are read only.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-6-brief" class="roadmap-criterion-summary"><th scope="row">Brief</th><td><p class="roadmap-condition">For fixture criteria with and without proof scope, the brief lists scope, links, fingerprint, assessment status, and exclusions with their covering criteria and tests, reports uncovered exclusions as gaps, and runs no tools.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for brief" aria-expanded="true" aria-controls="verification-evidence-6-brief-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-6-brief-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>brief</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — agent guidance output; whether the brief needs its own anchor is open with the engineer.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-6-pilot" class="roadmap-criterion-summary"><th scope="row">Pilot</th><td><p class="roadmap-condition">The engine's <code>formal-authority</code> proof method is satisfied with every guard and a recorded specification assessment for the tested revision, with its survivors and their classification recorded; a deliberate specification change makes the assessment stale.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for pilot" aria-expanded="true" aria-controls="verification-evidence-6-pilot-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-6-pilot-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>pilot</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — acceptance against this Book; the authority proof's own evidence stays in the Formal Verification roadmap.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

A rendered progress view remains an
[open question](../design/verification-evidence.md#open-questions) in the design;
it needs design before it gets a milestone.
