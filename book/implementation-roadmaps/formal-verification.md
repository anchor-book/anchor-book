# Formal Verification

**Roadmap identifier:** `formal-verification`

**Status:** Proof milestones 1 and 2 remain proposed. Milestone 3 is complete locally for
criterion-method schema preparation; the uncommitted change awaits engineer review.

## Goal

Prove the authority-validation logic actually executed by the engine, and make
that proof repeatable locally and on GitHub. Start with
[`engine-valid-authority`](../design/rust-engine.md#engine-valid-authority), grounded
in [traceable properties](../authority/requirements.md#requirement-traceable-properties).
The [Formal Verification design](../design/formal-verification.md) defines the
specification, production integration, trusted boundaries, and evidence.

The roadmap uses the engine as the first experiment in associating anchor
criteria with formal specifications, production functions, and proof evidence.
A general source-association syntax remains to be designed from that experience.

This roadmap adds `formal-authority` and `production-integration` coverage to the
anchor. [Record Rendering milestone 3](record-rendering.md#record-rendering-3-authority-coverage)
retains the three existing behavioral test criteria. Earlier rendering completion
does not establish these new proof criteria. Draft status remains design status.

The schema preparation in milestone 3 is assigned independently of the proof
refactor. For that refactor, the current implementation sequence places [Mutation Testing milestone 2](mutation-testing.md#mutation-testing-2)
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
<li><label><input type="checkbox" disabled> Review the formal conditions against the anchor; enumerate valid and invalid inputs, preconditions, and trusted boundaries. Associate specifications with anchor and criterion IDs beside the code, and give the engineer a readable account of their meaning and limits.</label></li>
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

<a id="formal-verification-3"></a>

## Milestone 3. Criteria declare their required verification methods

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>Engineers and agents can read explicit proof, test, and review obligations on each criterion; the engine validates and renders those selections without claiming results.</td></tr>
<tr><th scope="row">Status</th><td>Complete — local schema implementation and verification</td></tr>
<tr><th scope="row">Identifier</th><td><code>formal-verification-3</code></td></tr>
<tr><th scope="row">Design basis</th><td><a href="../design/anchor-definitions.html#verification-methods">Criterion methods</a>, <a href="../design/record-rendering.html#reader-presentation">record presentation</a>, and <a href="../ai-working-contract.html#develop-verification-with-the-code">agent verification guidance</a>.</td></tr>
<tr><th scope="row">Dependencies</th><td>The existing record parser and renderer. This scoped schema change is assigned before the proof milestones; it does not assign the Verus refactor or settle its prerequisites.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Define required criterion methods and clarify authoring and code/proof association guidance.</label></li>
<li><label><input type="checkbox" checked disabled> Add typed methods, strict parsing, and required-evidence labels; migrate Book records, examples, and fixtures.</label></li>
<li><label><input type="checkbox" checked disabled> Verify positive and negative cases, rendering, current-Book acceptance, toolchain compatibility, and mutation sensitivity; record results and limits.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="formal-verification-3-strict-schema" class="roadmap-criterion-summary"><th scope="row">Strict schema</th><td><p class="roadmap-condition">The record API accepts every supported method and combinations in authored order; it rejects missing, empty, repeated, unknown, or incorrectly typed methods on any criterion with located diagnostics.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-3-strict-schema-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-3-strict-schema-details" class="roadmap-criterion-details"><td colspan="2"><p><strong>Criterion:</strong> <code>strict-schema</code></p><p><strong>Anchor criteria and planned coverage:</strong> None — criterion schema preparation; no proof or test result is inferred from a method.</p><p><strong>Evidence:</strong> Passed locally; see the milestone evidence below for checks and limits.</p></td></tr>
<tr id="formal-verification-3-presentation" class="roadmap-criterion-summary"><th scope="row">Presentation</th><td><p class="roadmap-condition">The actual adapter displays every criterion’s methods as required verification, preserves design acceptance status, and rejects invalid methods without success JSON. The current Book and print view preserve the authored criteria and links.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-3-presentation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-3-presentation-details" class="roadmap-criterion-details"><td colspan="2"><p><strong>Criterion:</strong> <code>presentation</code></p><p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-records-in-context">render-records-in-context</a> · <code>record-presentation</code> — Partial: the new required-method labels.</p><p><strong>Evidence:</strong> Passed locally; see the milestone evidence below for checks and limits.</p></td></tr>
<tr id="formal-verification-3-migration" class="roadmap-criterion-summary"><th scope="row">Migration and guidance</th><td><p class="roadmap-condition">All Book definitions, literal examples, and fixtures use the same required field. The contract distinguishes obligations from results and guides production-code proof associations. Tests, supported-toolchain checks, rendered acceptance, and scoped mutation findings are recorded.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-3-migration-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-3-migration-details" class="roadmap-criterion-details"><td colspan="2"><p><strong>Criterion:</strong> <code>migration</code></p><p><strong>Anchor criteria and planned coverage:</strong> None — schema migration and contributor guidance.</p><p><strong>Evidence:</strong> Passed locally; see the milestone evidence below for checks and limits.</p></td></tr>
<tr><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-3-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Schema preparation evidence</button></td></tr>
<tr id="formal-verification-3-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Revision and integration:</strong> Uncommitted working tree on main based on <code>5c86d1a</code>, verified 2026-10-21. Schema version 1 is an unpublished draft revised together with its Book records and fixtures. Engineer review and a follow-up commit remain pending.</p>
<p><strong>Strict schema:</strong> Independent record tests accept each supported method and combined methods in authored order. They reject missing, empty, duplicate, unknown, incorrectly typed, aliased, and duplicate-key forms on the first and a later criterion, with located invalid-definition diagnostics. No proof artifacts are required to parse a declared obligation.</p>
<p><strong>Presentation and migration:</strong> Actual-process adapter tests verify per-criterion labels, unchanged design status, and no success JSON for invalid methods. Current-Book acceptance checks the authority anchor's exact selections. The Book renders 25 complete records and its local links pass; browser review confirms readable required-method labels. Literal examples remain code. The working contract and verification design describe production-code proofs and reviewable assumptions without selecting a tool in the anchor.</p>
<p><strong>Commands and tools:</strong> Ordinary tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. The suite contains 14 record tests, 7 rendering/process tests, 2 CLI tests, and 1 documentation test. Formatting, separate current-Book acceptance, <code>scripts/check-engine-independence.sh</code>, mdBook 0.5.2 build, and the rendered acceptance script with Python 3.12 pass. The engine remains usable with an absent or invalid Book.</p>
<p><strong>Mutation scope:</strong> cargo-mutants 27.1.0 tested the changed method mapping, schema record validation, and record rendering functions with <code>target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/records/schema.rs --file src/records/model.rs --file src/rendering/html.rs --re 'VerificationMethod::as_str|::record -&gt;|in Context.*::record|in record$' --output target/mutation-verification-methods</code>. The initial 15-mutation run had 13 caught, 1 missed, 1 unviable, and no timeouts. It reproduced the existing heading-label survivor: tests did not distinguish an anchor's Authority heading from a decision's Requirements heading. Added assertions scoped to each record. The repeat run passed in 45 seconds: 14 caught, 1 unviable, zero missed or timed out. The unviable mutation attempts to construct a Record with Default, which that type does not implement; it is not counted as detection.</p>
<p><strong>Supporting records and limits:</strong> Tests and fixture changes accompany this implementation; raw mutation output remains ignored. This is a scoped campaign, not a rerun of the full engine baseline. Other findings remain in the Mutation Testing roadmap. No Verus proof, automatic evidence ingestion, remote result for this revision, or merge-readiness claim is established.</p></td></tr>
<tr><th scope="row">Completion record</th><td>Complete locally on 2026-10-21. Required-method declarations are parsed, rendered, and exercised with independent and current-Book inputs. The schema change remains uncommitted for engineer review; proof implementation is still proposed.</td></tr>
</tbody>
</table>
</div>

## Later work

Automatic evidence ingestion, proof-result schemas, anchor progress views, and
support for other verifiers remain in the [later-work list](index.md#later-work-retained-from-the-design).
The first proof uses manual evidence records. Neither milestone adds verifier
execution to structural checking or mdBook preprocessing, proves the whole engine,
or changes the acceptance status of an anchor automatically.
