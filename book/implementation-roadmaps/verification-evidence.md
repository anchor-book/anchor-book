# Verification Evidence

**Roadmap identifier:** `verification-evidence`

## Goal

`anchorbook verify` reports progress toward the goal design for each anchor and
criterion, running the linked tests and proofs, as defined in
[Verification Evidence](../design/verification-evidence.md). It serves agents in
the build loop and CI with the same command. This Book's own anchors are the first
acceptance case, starting with tests, then proofs, then CI.

None of this Book's criteria are linked to tests or proofs yet, so `verify` will
initially report them as unlinked. Linking criteria is part of the work.

<a id="verification-evidence-1"></a>

## Milestone 1. An agent sees test progress for an anchor in the build loop

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">An agent or engineer runs <code>anchorbook verify --anchor &lt;id&gt; --method test</code> after a change and sees that anchor's test criteria for the current files, with links resolved and drift reported; a whole-Book run reports progress by chapter.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">The record library, configuration loader, and navigation reader.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html">Verification Evidence</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/check-diagnostics.html">Diagnostics</a>, and <a href="../design/rust-engine.html">Rust Engine</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Add the <code>[verification]</code> configuration section, accepted by the loader and ignored by <code>check</code>.</label></li>
<li><label><input type="checkbox" disabled> Implement a <code>verification</code> capability: link scanning in configured sources, test selection, nextest invocation with an exact filter and JUnit output, test-name mapping, and per-method, per-criterion, and per-anchor states, returned as typed data.</label></li>
<li><label><input type="checkbox" disabled> Add <code>anchorbook verify</code> with scope options, the progress report, and exit statuses as designed.</label></li>
<li><label><input type="checkbox" disabled> Pin nextest and document installing it in CONTRIBUTING; ordinary <code>cargo test</code> and CI test jobs are unchanged.</label></li>
<li><label><input type="checkbox" disabled> Link the test criteria of the <code>verify-*</code> anchors and at least one existing checker anchor, and record a build-loop run.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="verification-evidence-1-test-states" class="roadmap-criterion-summary"><th scope="row">Test states</th><td><p class="roadmap-condition">Independent fixtures produce every state for the test method; criteria requiring several methods report partial or not evaluated correctly; scoped runs name what they exclude.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for test states" aria-expanded="true" aria-controls="verification-evidence-1-test-states-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-test-states-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>test-states</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: test method; <code>scope-and-limits</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-progress-report" class="roadmap-criterion-summary"><th scope="row">Progress report</th><td><p class="roadmap-condition">For a fixture Book with satisfied, partial, failing, and unlinked anchors, the library and actual CLI report every anchor by chapter with states, links, and summary counts and exit 0; invalid inputs make the run incomplete; an unbuildable or missing runner affects only its method.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for progress report" aria-expanded="true" aria-controls="verification-evidence-1-progress-report-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-progress-report-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>progress-report</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-progress-report">verify-progress-report</a> · <code>progress-output</code>, <code>incomplete-runs</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-links" class="roadmap-criterion-summary"><th scope="row">Links</th><td><p class="roadmap-condition">Fixtures with dangling, mismatched, misplaced, and duplicate links and unlinked criteria produce located findings; corrected fixtures resolve.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for links" aria-expanded="true" aria-controls="verification-evidence-1-links-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-links-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>links</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-links-resolve">verify-links-resolve</a> · <code>link-findings</code> — Partial: test links</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="verification-evidence-1-selected-runs" class="roadmap-criterion-summary"><th scope="row">Selected runs</th><td><p class="roadmap-condition">Recording fixture runners show that a scoped run invokes only nextest with exactly the linked tests in scope and reads no earlier reports; linked tests not executed, skipped, or missing have no evidence; Book and source files are unchanged.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for selected runs" aria-expanded="true" aria-controls="verification-evidence-1-selected-runs-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-1-selected-runs-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>selected-runs</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>selected-run</code> — Partial: test runner; <code>source-preservation</code> — Full</p>
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
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1"><code>verify</code> runs the Verus proof when a linked proof criterion is in scope, and <code>formal-authority</code> reports its state from links on the verified functions.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Verification Evidence — Milestone 1; the pinned Verus toolchain.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html">Verification Evidence</a> and <a href="../design/formal-verification.html">Formal Verification</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Invoke Verus with JSON output and interpret full-crate verification, errors, and per-function results.</label></li>
<li><label><input type="checkbox" disabled> Link the verified authority functions to <code>formal-authority</code>.</label></li>
<li><label><input type="checkbox" disabled> Exercise passing, failing, and unbuildable proofs, a missing Verus installation, and a deliberate proof defect in a disposable copy.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="verification-evidence-2-proof-states" class="roadmap-criterion-summary"><th scope="row">Proof states</th><td><p class="roadmap-condition">Fixture and real Verus runs produce satisfied, failed, inconclusive, and not-evaluated proof states as specified, and <code>formal-authority</code> is satisfied for the current code.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for proof states" aria-expanded="true" aria-controls="verification-evidence-2-proof-states-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="verification-evidence-2-proof-states-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>proof-states</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/verification-evidence.html#verify-criterion-states">verify-criterion-states</a> · <code>state-per-method</code> — Partial: proof method; <a href="../design/verification-evidence.html#verify-runs-linked-checks">verify-runs-linked-checks</a> · <code>selected-run</code> — Partial: proof runner; <a href="../design/verification-evidence.html#verify-links-resolve">verify-links-resolve</a> · <code>link-findings</code> — Partial: proof links</p>
<p><strong>Evidence:</strong> Pending.</p>
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
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Verification Evidence — Milestones 1 and 2.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/verification-evidence.html">Verification Evidence</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>verification-evidence-3</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Define this milestone's tasks and criteria after milestone 2, including the workflow job, toolchains, report retention, and whether any explicit option blocks on regressions.</label></li>
</ul></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

Review evidence and a rendered progress view remain
[open questions](../design/verification-evidence.md#open-questions) in the design;
they need design before they get milestones.
