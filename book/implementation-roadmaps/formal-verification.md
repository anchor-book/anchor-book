# Formal Verification

**Roadmap identifier:** `formal-verification`

**Status:** Proposed implementation; design and roadmap drafted for review.

## Goal

Prove the authority-validation logic actually executed by the engine, and make
that proof repeatable locally and on GitHub. Start with
[`engine-valid-authority`](../design/rust-engine.md#engine-valid-authority), grounded
in [traceable properties](../authority/requirements.md#requirement-traceable-properties).
The [Formal Verification design](../design/formal-verification.md) defines the
specification, production integration, trusted boundaries, and evidence.

This roadmap adds `formal-authority` and `production-integration` coverage to the
anchor. [Record Rendering milestone 3](record-rendering.md#record-rendering-3-authority-coverage)
retains the three existing behavioral test criteria. Earlier rendering completion
does not establish these new proof criteria. Draft status remains design status.

The current implementation sequence places [Mutation Testing milestone 2](mutation-testing.md#mutation-testing-2)
before further engine changes. Design preparation can proceed now; agree on any
change to that sequence before starting the Verus refactor. Proofs do not resolve
unreviewed mutation findings or establish merge protection.

<a id="formal-verification-1"></a>

## Milestone 1. The engine uses authority logic verified by Verus

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>Authority validation has a reproducible local proof over the executable functions used by the record API, with tested integration and explicit limits.</td></tr>
<tr><th scope="row">Status</th><td>Proposed</td></tr>
<tr><th scope="row">Identifier</th><td><code>formal-verification-1</code></td></tr>
<tr><th scope="row">Design basis</th><td><a href="../design/formal-verification.html">Formal Verification</a>, <a href="../design/authority-definitions.html#validating-authority-relationships">authority rules</a>, and <a href="../design/rust-engine.html#engine-valid-authority">engine-valid-authority</a>.</td></tr>
<tr><th scope="row">Dependencies</th><td>Record Rendering milestones 1 and 2; Mutation Testing milestone 2 under the current sequence. Review this prototype scope and formal specification before implementation.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Review the formal conditions against the anchor; enumerate valid and invalid inputs, preconditions, and trusted boundaries.</label></li>
<li><label><input type="checkbox" disabled> Pin Verus, its Rust toolchain, and vstd. Confirm normal Cargo builds on the minimum and stable Rust versions; resolve compatibility or package changes before adopting them.</label></li>
<li><label><input type="checkbox" disabled> Refactor authority lookup and validation into the production records capability and prove its executable functions. Preserve invalid candidates, identity ambiguity, all reference edges, and existing failure behavior.</label></li>
<li><label><input type="checkbox" disabled> Exercise input conversion and failure propagation through read_records and the actual mdBook adapter. Keep the verified functions on the normal production call path.</label></li>
<li><label><input type="checkbox" disabled> Demonstrate proof failure for a deliberate behavioral defect in a disposable copy, restore it, and rerun. Run relevant unit/integration tests and mutation checks; document any mutation-tool coverage limits.</label></li>
<li><label><input type="checkbox" disabled> Document exact local verification commands in CONTRIBUTING, reconcile the implemented design, and record proof and integration evidence.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="formal-verification-1-authority-proof" class="roadmap-criterion-summary"><th scope="row">Authority proof</th><td><p class="roadmap-condition">The reviewed specification and executable functions verify over the declared input domain. Valid inputs satisfy the preconditions; invalid relationships are not excluded by them. A deliberate violation fails verification, and the restored code passes.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-authority-proof-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-1-authority-proof-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>authority-proof</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#engine-valid-authority">engine-valid-authority</a> · <code>formal-authority</code> — Full. The claim is limited to the specified authority logic and recorded assumptions.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="formal-verification-1-production-path" class="roadmap-criterion-summary"><th scope="row">Production path</th><td><p class="roadmap-condition">Review identifies the same verified functions on the normal read_records call path. Independent fixtures exercise conversion, empty and invalid authority, all resolution/status cases, and rejection through the real adapter. State which boundaries are tested rather than proved.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-production-path-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-1-production-path-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>production-path</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#engine-valid-authority">engine-valid-authority</a> · <code>production-integration</code> — Full. Behavioral test coverage also contributes to <code>required-authority</code>, <code>resolved-authority</code>, and <code>compatible-status</code>; reconcile with Record Rendering milestone 3.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="formal-verification-1-tooling" class="roadmap-criterion-summary"><th scope="row">Compatible tooling</th><td><p class="roadmap-condition">Pinned tools reproduce verification without rendering the Book. Ordinary builds, tests, formatting, and Clippy pass on supported toolchains. Record mutation results and any unsupported annotated code; unresolved findings remain visible.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-tooling-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-1-tooling-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>tooling</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — developer tooling and compatibility; proof success alone does not complete the mutation roadmap.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence</button></td></tr>
<tr id="formal-verification-1-evidence" class="roadmap-criterion-details"><td colspan="2">Pending. Use the <a href="index.html#evidence-records">evidence fields</a> and include the <a href="../design/formal-verification.html#evidence-and-later-integration">proof scope and assumptions</a>.</td></tr>
<tr><th scope="row">Completion record</th><td>Pending; no proof or implementation is claimed.</td></tr>
</tbody>
</table>
</div>

<a id="formal-verification-2"></a>

## Milestone 2. GitHub verifies the production authority logic

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>Changes receive repeatable remote verification with revision-specific proof and integration evidence, and the required check protects implementation merges.</td></tr>
<tr><th scope="row">Status</th><td>Proposed</td></tr>
<tr><th scope="row">Identifier</th><td><code>formal-verification-2</code></td></tr>
<tr><th scope="row">Design basis</th><td><a href="../design/formal-verification.html">Formal Verification</a>, <a href="../design/authority-definitions.html#validating-authority-relationships">authority rules</a>, and <a href="../design/rust-engine.html#engine-valid-authority">engine-valid-authority</a>.</td></tr>
<tr><th scope="row">Dependencies</th><td>Formal Verification milestone 1 and the merge workflow from Mutation Testing milestone 2. Remote execution follows an authorized push.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Add a separate GitHub verification job using the pinned tools and full selected proof scope, alongside ordinary engine tests. Verify actual function coverage and retain public-safe results on failure.</label></li>
<li><label><input type="checkbox" disabled> Exercise a passing revision and a deliberate verification failure through a PR; confirm that failed, skipped, or incomplete verification cannot count as success.</label></li>
<li><label><input type="checkbox" disabled> Record proof and production integration results for the same revision, including assumptions, tool versions, commands, function coverage, and evidence links.</label></li>
<li><label><input type="checkbox" disabled> Add the observed verification check to the intended merge rules and demonstrate that failure blocks merging; preserve existing required tests.</label></li>
<li><label><input type="checkbox" disabled> Demonstrate the agent correction loop without weakening the formal property, then update contributor instructions and roadmap evidence.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="formal-verification-2-remote-proof" class="roadmap-criterion-summary"><th scope="row">Remote proof</th><td><p class="roadmap-condition">Actual GitHub results show the intended production functions verified for the recorded revision and a deliberate defect rejected. Retained results identify the specification, inputs, assumptions, toolchain, and scope.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-remote-proof-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-2-remote-proof-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>remote-proof</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#engine-valid-authority">engine-valid-authority</a> · <code>formal-authority</code> — Partial: remote reproduction of milestone 1 proof evidence.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="formal-verification-2-same-revision" class="roadmap-criterion-summary"><th scope="row">Same revision</th><td><p class="roadmap-condition">Record passing integration tests and proof results for the same code revision; review evidence applicability when design, code, conversion, or dependencies change.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-same-revision-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-2-same-revision-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>same-revision</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#engine-valid-authority">engine-valid-authority</a> · <code>production-integration</code> — Partial: remote integration evidence and revision alignment.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="formal-verification-2-merge-loop" class="roadmap-criterion-summary"><th scope="row">Merge and correction loop</th><td><p class="roadmap-condition">A real PR demonstrates a failed required verification check preventing merge, followed by an authorized correction and passing checks for the updated revision. Record the applied rule and any bypass policy.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-merge-loop-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-2-merge-loop-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>merge-loop</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — repository enforcement and agent workflow; a workflow definition is not enforcement.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence</button></td></tr>
<tr id="formal-verification-2-evidence" class="roadmap-criterion-details"><td colspan="2">Pending. Use the <a href="index.html#evidence-records">evidence fields</a> and include the <a href="../design/formal-verification.html#evidence-and-later-integration">proof scope and assumptions</a>.</td></tr>
<tr><th scope="row">Completion record</th><td>Pending; no proof or implementation is claimed.</td></tr>
</tbody>
</table>
</div>

## Later work

Automatic evidence ingestion, proof-result schemas, anchor progress views, and
support for other verifiers remain in the [later-work list](index.md#later-work-retained-from-the-design).
The first proof uses manual evidence records. Neither milestone adds verifier
execution to structural checking or mdBook preprocessing, proves the whole engine,
or changes the acceptance status of an anchor automatically.
