# Formal Verification

**Roadmap identifier:** `formal-verification`

## Goal

Prove the authority-validation logic actually executed by the engine, and make
that proof repeatable locally and on GitHub. Start with
[`records-valid-authority`](../design/rust-engine.md#records-valid-authority), grounded
in [traceable properties](../authority/requirements.md#requirement-traceable-properties).
Rust Engine's [authority proof](../design/rust-engine.md#authority-proof) defines the
claim, production boundary, and trusted base, and [Verification](../design/verification.md)
the general method and evidence. Isolate
the engine's proof dependencies from consuming builds without losing the
connection between the verified functions and the packaged implementation.

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

Milestone 4 adds proposed packaging isolation and starts with a bounded
feasibility experiment. It does not depend on the consumer runner, but
[consumer acceptance](verification-evidence.md#verification-evidence-5) depends
on it. Existing proof evidence remains tied to its recorded revision; packaging
changes require new proof and integration evidence.

<a id="formal-verification-1"></a>

## Milestone 1. Prove production authority validation locally

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>Authority validation has a reproducible local proof over the executable functions used by the record API, with tested integration and explicit limits.</td></tr>
<tr><th scope="row">Status</th><td>In progress — local proof and integration verified; engineer review pending</td></tr>
<tr><th scope="row">Identifier</th><td><code>formal-verification-1</code></td></tr>
<tr><th scope="row">Design basis</th><td><a href="../design/rust-engine.html#authority-proof">Authority proof</a>, <a href="../design/authority-definitions.html#validating-authority-relationships">authority rules</a>, and <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a>.</td></tr>
<tr><th scope="row">Dependencies</th><td>The implemented record reader and renderer, existing independent tests, and reviewed mutation findings. Merge protection is deferred. Review the specification against the anchor before proving the extracted implementation.</td></tr>
<tr><th scope="row">Scope</th><td>One local proof of the authority validator used by <code>read_records</code>, covering every anchor authority and decision requirement. Parser correctness, diagnostic presentation proofs, GitHub proof jobs, merge protection, and generalized evidence tooling are outside this task.</td></tr>
<tr><th scope="row">Tasks</th><td><ol class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Check feasibility: pin compatible Verus, its Rust toolchain, and vstd; verify and compile a small representative validator slice using the required types and collection operations. Preserve Rust 1.85 and stable builds. Review any necessary package or minimum-version change before the full refactor.</label></li>
<li><label><input type="checkbox" checked disabled> Define the formal acceptance predicate from the authority rules and review it against the design's input matrix. Include invalid relationships in the domain; prove both acceptance and rejection. State assumptions and associate the specification with the anchor and criterion IDs.</label></li>
<li><label><input type="checkbox" checked disabled> Extract lookup and authority checks into <code>src/records/authority.rs</code> and develop the proof alongside the executable functions. Include required-authority checks currently in <code>schema.rs</code>. Keep function contracts beside the implementation; place reusable lemmas in supporting modules only when useful.</label></li>
<li><label><input type="checkbox" checked disabled> Connect <code>inventory.rs</code> to those same functions. Test faithful candidate conversion and failure propagation through the library and actual mdBook adapter, preserving identities, ambiguity, invalid candidates, status, references, and located diagnostics.</label></li>
<li><label><input type="checkbox" checked disabled> Verify the complete selected proof scope. Introduce one deliberate rule violation in a disposable copy and confirm a proof obligation fails; restore the code and verify again. Run focused integration and mutation checks, explicitly checking whether cargo-mutants reaches the annotated executable bodies.</label></li>
<li><label><input type="checkbox" checked disabled> Document the pinned local command and the proved claim, assumptions, tested boundaries, revision, and result. Reconcile the design and milestone evidence without claiming the entire engine or anchor is proved.</label></li>
<li><label><input type="checkbox" disabled> Assess the formal specification against the explicit proof scope (a specification assessment) and map production-integration tests and review to every excluded boundary. Record any remaining coverage gaps before accepting the milestone.</label></li>
</ol></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="formal-verification-1-authority-proof" class="roadmap-criterion-summary"><th scope="row">Authority proof</th><td><p class="roadmap-condition">The production validator verifies acceptance if and only if the defined authority rules hold over the complete typed input. Preconditions do not assume valid authority relationships. The run identifies the verified functions; a deliberate rule violation fails a proof obligation, and the restored code passes.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-authority-proof-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-1-authority-proof-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>authority-proof</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a> · <code>formal-authority</code> — Full. The claim is limited to the specified authority logic and recorded assumptions.</p>
<p><strong>Evidence:</strong> Passed locally; see milestone evidence. Engineer review pending.</p>
</td></tr>
<tr id="formal-verification-1-production-path" class="roadmap-criterion-summary"><th scope="row">Production path</th><td><p class="roadmap-condition">Review identifies the same verified functions on the normal read_records call path. Independent fixtures exercise conversion, empty and invalid authority, all resolution/status cases, and rejection through the real adapter. State which boundaries are tested rather than proved.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-production-path-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-1-production-path-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>production-path</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a> · <code>production-integration</code> — Full. Behavioral test coverage also contributes to <code>required-authority</code>, <code>resolved-authority</code>, and <code>compatible-status</code>; reconcile with Record Rendering milestone 3.</p>
<p><strong>Evidence:</strong> Passed locally; see milestone evidence. Engineer review pending.</p>
</td></tr>
<tr id="formal-verification-1-tooling" class="roadmap-criterion-summary"><th scope="row">Compatible tooling</th><td><p class="roadmap-condition">Pinned tools reproduce verification without rendering the Book. Ordinary builds, tests, formatting, and Clippy pass on supported toolchains. Record whether mutation testing reaches the annotated executable bodies and any gaps; a skipped or empty mutation scope is not coverage.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-tooling-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-1-tooling-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>tooling</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — developer tooling and compatibility; proof success alone does not complete the mutation roadmap.</p>
<p><strong>Evidence:</strong> Passed locally; see milestone evidence. Engineer review pending.</p>
</td></tr>
<tr><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence</button></td></tr>
<tr id="formal-verification-1-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Local result:</strong> Working tree based on <code>45125f6</code>, tested 2026-10-21. Verus <code>0.2026.10.18.7d82994</code>, verifier Rust 1.99.1, and bundled Z3 4.16.0 report 15 verification units passed, zero errors. All seven executable authority functions appear in the report. The native command and exactly pinned library are documented in CONTRIBUTING; no verification wrapper script is required.</p>
<p><strong>Scope and assumptions:</strong> The production <code>validate_authorities</code> function satisfies its acceptance biconditional over complete typed candidates. Project assumptions and trusted function bodies are prohibited by <code>--no-cheating</code>; standard-library specifications, verifier/solver, and compiler remain trusted. Parsing, conversion, diagnostics, and backlinks are tested boundaries. Review the specification's interpretation of the anchor before accepting the milestone.</p>
<p><strong>Failure demonstration:</strong> Reversing the accepted-target status condition in a disposable source copy caused a postcondition failure: 14 verification units passed, one failed. The original source was unchanged and its proof passed again. Independent tests cover all authority outcomes and an uncited decision failing through the real adapter. Tests and Clippy pass on Rust 1.85 and local stable 1.90.</p>
<p><strong>Mutation assessment:</strong> cargo-mutants 27.1.0 discovers no mutations inside <code>verus!</code>, so automatic mutation coverage of the validator is unavailable. The focused inventory integration run initially found four diagnostic assertion gaps; after strengthening message and related-location assertions, all 24 selected mutations were attempted: 22 caught, two unviable, no survivors or timeouts. Command: <code>target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/records/inventory.rs --output target/verus-mutations-fixed</code>. This is scoped evidence, not a new whole-engine campaign. Raw reports remain ignored under target. Milestone 2 records subsequent remote proof results.</p></td></tr>
<tr><th scope="row">Completion record</th><td>Local proof and integration checks pass; engineer review and acceptance remain pending.</td></tr>
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
<tr><th scope="row">Status</th><td>In progress — proof and integration pass on GitHub; failure demonstration and merge protection pending</td></tr>
<tr><th scope="row">Identifier</th><td><code>formal-verification-2</code></td></tr>
<tr><th scope="row">Design basis</th><td><a href="../design/rust-engine.html#authority-proof">Authority proof</a>, <a href="../design/authority-definitions.html#validating-authority-relationships">authority rules</a>, and <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a>.</td></tr>
<tr><th scope="row">Dependencies</th><td>Formal Verification milestone 1 and the existing PR workflows. Establish passing and failing proof runs first, then complete merge protection together with Mutation Testing milestone 2. Remote execution follows an authorized push.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Add a separate GitHub verification job using the pinned tools and full selected proof scope, alongside ordinary engine tests. Verify actual function coverage and retain public-safe results on failure.</label></li>
<li><label><input type="checkbox" disabled> Exercise a deliberate verification failure and correction through a PR; confirm that failed, skipped, or incomplete verification cannot count as success. The passing run is recorded below.</label></li>
<li><label><input type="checkbox" checked disabled> Record proof and production integration results for the same revision, including assumptions, tool versions, commands, function coverage, and evidence links.</label></li>
<li><label><input type="checkbox" disabled> Add the observed verification check to the intended merge rules and demonstrate that failure blocks merging; preserve existing required tests.</label></li>
<li><label><input type="checkbox" disabled> Demonstrate the agent correction loop without weakening the formal property, then update contributor instructions and roadmap evidence.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="formal-verification-2-remote-proof" class="roadmap-criterion-summary"><th scope="row">Remote proof</th><td><p class="roadmap-condition">Actual GitHub results show the intended production functions verified for the recorded revision and a deliberate defect rejected. Retained results identify the specification, inputs, assumptions, toolchain, and scope.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-remote-proof-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-2-remote-proof-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>remote-proof</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a> · <code>formal-authority</code> — Partial: remote reproduction of milestone 1 proof evidence.</p>
<p><strong>Evidence:</strong> Passing PR and main-branch runs are recorded below: 15 verified units, zero errors, all seven required functions present. The deliberate-failure GitHub run remains pending; the local negative experiment does not complete this criterion.</p>
</td></tr>
<tr id="formal-verification-2-same-revision" class="roadmap-criterion-summary"><th scope="row">Same revision</th><td><p class="roadmap-condition">Record passing integration tests and proof results for the same code revision; review evidence applicability when design, code, conversion, or dependencies change.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-same-revision-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-2-same-revision-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>same-revision</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a> · <code>production-integration</code> — Partial: remote integration evidence and revision alignment.</p>
<p><strong>Evidence:</strong> Passed: the retained artifact from <a href="https://github.com/anchor-book/anchor-book/actions/runs/39737287057">main verification</a> identifies <code>03fb98136809d30caba3b9c4a04e7617035a294a</code>, with successful proof and ordinary engine/integration-test commands for that checkout. Scope and integration-coverage review remain in milestone 1.</p>
</td></tr>
<tr id="formal-verification-2-merge-loop" class="roadmap-criterion-summary"><th scope="row">Merge and correction loop</th><td><p class="roadmap-condition">A real PR demonstrates a failed required verification check preventing merge, followed by an authorized correction and passing checks for the updated revision. Record the applied rule and any bypass policy.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-merge-loop-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-2-merge-loop-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>merge-loop</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — repository enforcement and agent workflow; a workflow definition is not enforcement.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence</button></td></tr>
<tr id="formal-verification-2-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Local checks:</strong> Working tree based on <code>95fa70b</code>, checked 2026-10-21. actionlint 1.7.12 accepts the workflow. The official Linux archive checksum and tool paths are confirmed. A fresh macOS verification using the same command options passes 15 verification units with zero errors; the report check confirms all seven required executable functions.</p>
<p><strong>Report rejection:</strong> Local experiments reject empty or malformed output, duplicate reports, a different verifier version, zero verified units, verification errors, partial-crate verification, and missing, failed, or non-executable required function results. Fresh dependency success summaries are handled without accepting arbitrary non-JSON output.</p>
<p><strong>Remote evidence:</strong> <a href="https://github.com/anchor-book/anchor-book/pull/2">PR #2</a> merged as <code>03fb981</code>. The <a href="https://github.com/anchor-book/anchor-book/actions/runs/39735753233">PR proof run</a> passed for head <code>687a4bf</code>, testing merge revision <code>4e0d6c1</code>; <a href="https://github.com/anchor-book/anchor-book/actions/runs/39737287057">main verification</a> passed for <code>03fb981</code>. Both report 15 verified units, zero errors, full-crate verification, and successful results for all seven required executable functions. The retained <code>authority-proof-results</code> artifacts identify revisions, commands, exit codes, tools, scope, and assumptions; ordinary engine/integration tests passed in the same jobs. Linux runs use the pinned Verus release, Rust 1.99.1 for verification and 1.85.0 for ordinary tests.</p>
<p><strong>Limits:</strong> The deliberate-failure/correction GitHub run and merge enforcement remain pending. Passing execution does not complete milestone 1 scope and integration-coverage review.</p></td></tr>
<tr><th scope="row">Completion record</th><td>Passing remote proof and integration are recorded; failure/correction demonstration, evidence review, and merge protection remain open.</td></tr>
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
<tr id="formal-verification-3-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Revision and integration:</strong> Uncommitted working tree on main based on <code>5c86d1a</code>, verified 2026-10-21. Schema version 1 is an unpublished draft revised together with its Book records and fixtures. The tested changes were subsequently committed as <code>7ab9616</code>.</p>
<p><strong>Strict schema:</strong> Independent record tests accept each supported method and combined methods in authored order. They reject missing, empty, duplicate, unknown, incorrectly typed, aliased, and duplicate-key forms on the first and a later criterion, with located invalid-definition diagnostics. No proof artifacts are required to parse a declared obligation.</p>
<p><strong>Presentation and migration:</strong> Actual-process adapter tests verify per-criterion labels, unchanged design status, and no success JSON for invalid methods. Current-Book acceptance checks the authority anchor's exact selections. The Book renders 25 complete records and its local links pass; browser review confirms readable required-method labels. Literal examples remain code. The working contract and verification design describe production-code proofs and reviewable assumptions without selecting a tool in the anchor.</p>
<p><strong>Commands and tools:</strong> Ordinary tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. The suite contains 14 record tests, 7 rendering/process tests, 2 CLI tests, and 1 documentation test. Formatting, separate current-Book acceptance, <code>scripts/check-engine-independence.sh</code>, mdBook 0.5.2 build, and the rendered acceptance script with Python 3.12 pass. The engine remains usable with an absent or invalid Book.</p>
<p><strong>Mutation scope:</strong> cargo-mutants 27.1.0 tested the changed method mapping, schema record validation, and record rendering functions with <code>target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/records/schema.rs --file src/records/model.rs --file src/rendering/html.rs --re 'VerificationMethod::as_str|::record -&gt;|in Context.*::record|in record$' --output target/mutation-verification-methods</code>. The initial 15-mutation run had 13 caught, 1 missed, 1 unviable, and no timeouts. It reproduced the existing heading-label survivor: tests did not distinguish an anchor's Authority heading from a decision's Requirements heading. Added assertions scoped to each record. The repeat run passed in 45 seconds: 14 caught, 1 unviable, zero missed or timed out. The unviable mutation attempts to construct a Record with Default, which that type does not implement; it is not counted as detection.</p>
<p><strong>Supporting records and limits:</strong> Tests and fixture changes accompany this implementation; raw mutation output remains ignored. This is a scoped campaign, not a rerun of the full engine baseline. Other findings remain in the Mutation Testing roadmap. No Verus proof, automatic evidence ingestion, remote result for this revision, or merge-readiness claim is established.</p></td></tr>
<tr><th scope="row">Completion record</th><td>Complete locally on 2026-10-21. Required-method declarations are parsed, rendered, and exercised with independent and current-Book inputs. The schema change is committed as <code>7ab9616</code>; proof implementation and subsequent remote evidence are tracked in milestones 1 and 2.</td></tr>
</tbody>
</table>
</div>

<a id="formal-verification-4"></a>

## Milestone 4. Engine proofs stay outside consuming builds

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>A packaged engine builds without its proof libraries or verification toolchain, while the implementation consumers execute remains derived from the same maintained source whose authority contracts are verified.</td></tr>
<tr><th scope="row">Status</th><td>Proposed</td></tr>
<tr><th scope="row">Identifier</th><td><code>formal-verification-4</code></td></tr>
<tr><th scope="row">Design basis</th><td><a href="../design/verified-code-packaging.html#proof-packaging-for-consumers">Proof packaging for consumers</a>, <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a>, and <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a>.</td></tr>
<tr><th scope="row">Dependencies</th><td>The existing production authority proof and engineer agreement on the isolation boundary. Begin with a small source-arrangement experiment and review its feasibility before assigning the full packaging change. Registry publication, contract delivery, and the consumer runner are not prerequisites.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Test one maintained source arrangement on a representative validator slice. A development proof workspace with reproducible erasure is a candidate; establish supported transformations, tool availability, drift detection, and runtime correspondence before choosing it.</label></li>
<li><label><input type="checkbox" disabled> Remove engine verification libraries from the ordinary packaged dependency graph. Keep the authority implementation on the production path; relocating a still-required proof crate or maintaining a second validator does not establish isolation.</label></li>
<li><label><input type="checkbox" disabled> Preserve <code>fixtures/consumers/verus-resolution</code> as the consumer-resolution regression fixture: its exact <code>vstd</code> pin fails against the current engine and must resolve and build after isolation. Repeat with the packaged candidate and an ordinary consumer without Verus installed.</label></li>
<li><label><input type="checkbox" disabled> Verify the source corresponding to that package, run ordinary tests on the minimum and stable toolchains, and reject a deliberate authority defect before restoring a passing proof. Reassess conversion, callers, diagnostics, and any new generation boundary.</label></li>
<li><label><input type="checkbox" disabled> Record package contents and dependency review, source correspondence, tools, commands, tested revisions, proof and integration results, and remaining limits. Reconcile contributor commands and the proof workflow with the chosen arrangement.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="formal-verification-4-runtime-correspondence" class="roadmap-criterion-summary"><th scope="row">Runtime correspondence</th><td><p class="roadmap-condition">Review identifies one maintained authority implementation and a reproducible relationship between verified and packaged functions. Independent tests and drift checks reject divergence; any erasure or conversion assumptions are explicit. A separately maintained proof-only validator cannot complete this criterion.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-4-runtime-correspondence-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-4-runtime-correspondence-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>runtime-correspondence</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a> · <code>formal-authority</code>, <code>production-integration</code> — Partial: correspondence across the packaging boundary; specification acceptance remains in milestone 1.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="formal-verification-4-consumer-resolution" class="roadmap-criterion-summary"><th scope="row">Consumer resolution</th><td><p class="roadmap-condition">The regression consumer keeps its exact <code>vstd = "=0.0.0-2026-10-11-0230"</code> pin: resolution fails against the baseline engine and resolution and build pass against the isolated packaged candidate. Independent consumers build without Verus installed; the engine's ordinary dependency tree contains no vstd or Verus dependency.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-4-consumer-resolution-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-4-consumer-resolution-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>consumer-resolution</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>independent-resolution</code> — Partial: engine proof dependency isolation. Other exact pins, supported toolchains, and renderer compatibility retain their separate integration and release scope.</p>
<p><strong>Baseline evidence:</strong> On 2026-11-08, Cargo 1.99.1 (<code>5f94df478</code>) reproduced the conflict against engine code at <code>fab05d6</code>, on the design branch for PR #19. The standalone consumer in <code>fixtures/consumers/verus-resolution</code> depends on the engine and exactly pins <code>vstd = "=0.0.0-2026-10-11-0230"</code>. From the repository root, <code>cargo metadata --offline --format-version 1 --manifest-path fixtures/consumers/verus-resolution/Cargo.toml</code> exits 101: the engine requires <code>=0.0.0-2026-10-18-0147</code>, and Cargo reports <code>failed to select a version for `vstd` which could resolve this conflict</code>. Compatible-version unification cannot satisfy both exact prerelease requirements; no <code>links</code> conflict is involved.</p>
<p><strong>Limits and remaining evidence:</strong> This measures dependency resolution against a checkout using cached registry metadata. It does not establish a build, native proof, or packaged-engine compatibility. Post-isolation resolution and build with the unchanged consumer pin remain Pending, as does the ordinary consumer without Verus.</p>
</td></tr>
<tr id="formal-verification-4-preserved-proof" class="roadmap-criterion-summary"><th scope="row">Preserved proof</th><td><p class="roadmap-condition">A native proof and ordinary integration tests pass for the same packaged candidate and recorded source revision. A deliberate authority defect fails verification and its correction passes. Review reassesses the claim, assumptions, production callers, and excluded boundaries after the packaging change; old proof reports do not establish this result.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-4-preserved-proof-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-4-preserved-proof-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>preserved-proof</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#records-valid-authority">records-valid-authority</a> · <code>formal-authority</code>, <code>production-integration</code> — Partial: revalidation of proof and integration after packaging changes, under the anchor's declared proof scope.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="formal-verification-4-dependency-review" class="roadmap-criterion-summary"><th scope="row">Dependency review</th><td><p class="roadmap-condition">A recorded review of the candidate's package contents, dependency set, and features confirms proof isolation and explains remaining exact requirements. The record identifies the package revision and explicitly limits its conclusion to verification dependencies.</p><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="formal-verification-4-dependency-review-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="formal-verification-4-dependency-review-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>dependency-review</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>dependency-review</code> — Partial: engine verification dependencies; general release dependency review remains separate.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr><th scope="row">Completion record</th><td>Pending. The source arrangement and native compatibility have not been established; source review or synthetic report checks are not packaging or proof acceptance evidence.</td></tr>
</tbody>
</table>
</div>

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
