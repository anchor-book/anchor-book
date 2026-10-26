# Mutation Testing

**Roadmap identifier:** `mutation-testing`

**Status:** Milestone 1 complete locally; milestone 2 in progress. Local test gaps are corrected
and three reviewed equivalent mutations have narrow exclusions. PR mutation
execution has passed; merge protection and the remote
failure/correction demonstration remain open. The local and GitHub authority
proofs now pass, satisfying the earlier prerequisite for that follow-up.

## Goal

Before merging implementation into `main`, agents can run mutation tests,
investigate findings, improve tests or implementation, and repeat verification.
GitHub independently runs the checks on the pull request; required checks protect
the merge. Tool setup alone does not establish that the engine's tests are adequate.

Local findings are resolved. Local and GitHub authority proofs in the
[Formal Verification roadmap](formal-verification.md) now pass. The earlier
deferral of merge protection has reached its prerequisite; enforcement and the
remote correction demonstration remain unfinished. Existing CI results must still be
reviewed before merging. This roadmap applies the
[testing and correction policy](../ai-working-contract.md#test-before-opening-a-pull-request)
to the existing Rust crate. Project choices and commands are documented in
`CONTRIBUTING.md` at the repository root and the [Rust Engine design](../design/rust-engine.md#mutation-testing).
Mutation outcomes assess tests, not design acceptance or anchor verification.

<a id="mutation-testing-1"></a>

## Milestone 1. Agents can run mutation tests and act on findings locally

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>Repeatable local mutation testing and documented correction steps, with a pull-request workflow ready for remote validation.</td></tr>
<tr><th scope="row">Status</th><td>Complete — local setup and correction demonstration</td></tr>
<tr><th scope="row">Identifier</th><td><code>mutation-testing-1</code></td></tr>
<tr><th scope="row">Design basis</th><td>Working-contract testing policy and Rust Engine testing guidance.</td></tr>
<tr><th scope="row">Dependencies</th><td>The independent test suite from Record Rendering milestones 1 and 2.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Pin cargo-mutants, configure temporary source copies and timeouts, and document full and scoped commands.</label></li>
<li><label><input type="checkbox" checked disabled> Add pull-request and manual CI execution, preserving failed outcomes and evidence.</label></li>
<li><label><input type="checkbox" checked disabled> Demonstrate a scoped local campaign, investigate survivors, strengthen the affected tests, and rerun.</label></li>
<li><label><input type="checkbox" checked disabled> Document agent iteration, equivalent-mutation review, inconclusive results, and the separate merge-rule requirement.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="mutation-testing-1-local-run"><th scope="row">Local run</th><td>Ordinary tests pass; the pinned tool reads the shared configuration and tests a stated nonempty scope in temporary copies, preserving source files.</td></tr>
<tr id="mutation-testing-1-correction-loop"><th scope="row">Correction loop</th><td>Record original survivors, their meaning, the corrections, and a repeat run. Tests check specified behavior; exclusions are not used to hide gaps.</td></tr>
<tr id="mutation-testing-1-workflow"><th scope="row">Workflow definition</th><td>PR and manual workflows use the same configuration after ordinary tests, fail on missed mutations or timeouts, and retain evidence. Source review establishes configuration only; milestone 2 owns remote proof and enforcement.</td></tr>
<tr><th scope="row">Anchor coverage</th><td>None — testing infrastructure and agent workflow. The demonstration exercises existing behavior but does not verify an entire anchor.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="mutation-testing-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Local demonstration and limits</button></td></tr>
<tr id="mutation-testing-1-evidence" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Revision and scope:</strong> Working tree based on <code>d66184b</code>, tested 2026-10-21 with cargo-mutants 27.1.0 and Rust 1.90.0 on macOS. Command: <code>target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/rendering/links.rs</code>, using the checkout-local tool installation documented in CONTRIBUTING. This selects 20 of the 226 mutations listed for the crate, not a full campaign.</p>
<p><strong>Local run and correction loop:</strong> Ordinary tests and the unmutated baseline passed. The first mutation run took 65 seconds: 18 caught, 2 missed, no timeouts or unviable mutations. One survivor weakened route validation; existing invalid URLs also failed another condition, masking the missing assertion. Added isolated scheme and control-character cases. The other changed relative-path arithmetic; a substring assertion accepted an extra parent-directory segment. Changed the assertions to require the complete link destination.</p>
<p><strong>Result:</strong> After the test changes, ordinary tests passed and the same 20 mutations were all caught in 62 seconds, with no missed, timed-out, or unviable mutations. Reviewed logs confirm the two former survivors now fail the intended protocol and navigation tests. No exclusions were added; engine source and dependencies are unchanged.</p>
<p><strong>Supporting records:</strong> <code>tests/rendering.rs</code>, shared mutation configuration, and documented reproduction commands. Raw local logs remain ignored. Formatting, Clippy, current-Book acceptance, and Book build/rendered-link checks pass; rendering tests and Clippy also pass on Rust 1.85.0. Workflow YAML parses and was reviewed for triggers, failure propagation, and artifact retention.</p>
<p><strong>Limitations and integration:</strong> Verified locally before the setup commit on main. This demonstration covered only link-rendering mutations; milestone 2 records the subsequent full campaign. No remote mutation workflow result or branch-rule enforcement is claimed by this demonstration.</p>
</td></tr>
<tr><th scope="row">Completion record</th><td>Complete locally on 2026-10-21. Scoped findings led to stronger tests and a passing repeat run. Full campaign and merge enforcement remain in milestone 2.</td></tr>
</tbody>
</table>
</div>

<a id="mutation-testing-2"></a>

## Milestone 2. Mutation testing protects implementation merges

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr><th scope="row">Outcome</th><td>The complete engine campaign has reviewed results, and GitHub requires passing tests before merging into main.</td></tr>
<tr><th scope="row">Status</th><td>In progress — PR execution verified; merge protection and remote correction demonstration pending</td></tr>
<tr><th scope="row">Identifier</th><td><code>mutation-testing-2</code></td></tr>
<tr><th scope="row">Design basis</th><td>Working-contract testing policy and Rust Engine testing guidance.</td></tr>
<tr><th scope="row">Dependencies</th><td>Mutation Testing milestone 1; publication of the workflow and access to repository merge rules.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Run the initial unfiltered engine campaign and record its scope, results, and limits.</label></li>
<li><label><input type="checkbox" checked disabled> Manually run the workflow on the published prototype checkpoint; verify failure reporting and retained evidence.</label></li>
<li><label><input type="checkbox" checked disabled> Review survivors and unviable mutations. Correct test gaps or defects, justify any equivalent mutations, and repeat affected checks and the full campaign.</label></li>
<li><label><input type="checkbox" checked disabled> Exercise the workflow on a PR and record its revision, results, artifacts, and runtime. Tune or split the 45-minute budget if needed without dropping coverage.</label></li>
<li><label><input type="checkbox" disabled> Configure the main branch rule to require Mutation tests and ordinary engine checks, with results current for the revision being merged. Verify that a failed check prevents merging under the intended contributor policy.</label></li>
<li><label><input type="checkbox" disabled> Demonstrate an agent reading a remote failure, correcting it locally, and obtaining passing checks on an updated PR through authorized pushes.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr id="mutation-testing-2-engine-campaign"><th scope="row">Engine campaign</th><td>Full results have no unexplained survivors or inconclusive required runs. Any equivalent mutation or exclusion has a specific reviewed rationale. Record scope, counts, versions, revision, and limits.</td></tr>
<tr id="mutation-testing-2-remote-loop"><th scope="row">Remote correction loop</th><td>Actual PR results demonstrate local correction and remote revalidation for the updated revision, with accessible evidence and a workable runtime budget.</td></tr>
<tr id="mutation-testing-2-merge-rule"><th scope="row">Merge enforcement</th><td>Record the applied rule, required check names, stale-result behavior, and observed prevention of merging with failed checks. A workflow YAML file or local success alone is insufficient.</td></tr>
<tr><th scope="row">Anchor coverage</th><td>None — test readiness and repository integration; no automatic claim of anchor verification.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="mutation-testing-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Initial campaign and remote assessment</button></td></tr>
<tr id="mutation-testing-2-evidence" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Baseline:</strong> The full campaign at <code>46f47c8</code> attempted 226 mutations: 154 caught, 28 missed, 44 unviable, no timeouts. Local Rust 1.90.0 and the <a href="https://github.com/anchor-book/anchor-book/actions/runs/39662175193">GitHub run</a> on Rust 1.99.1 produced the same survivors with cargo-mutants 27.1.0.</p>
<p><strong>Local correction evidence:</strong> Tests in <code>tests/errors.rs</code> and <code>tests/records.rs</code> now check error chains and presentation, diagnostic codes/messages/locations, duplicate-target locations, nonempty inventories, and Markdown boundaries and extensions. The HTML heading assertion is in <code>7ab9616</code>. No production engine behavior changed. The full unfiltered rerun on the working tree based on <code>1fa9b3e</code> completed on 2026-10-21: 230 attempted, 183 caught, 44 unviable, three equivalent survivors, no timeouts. All 25 non-equivalent survivors from the baseline now fail relevant tests; their failure logs were reviewed. Command: <code>target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --output target/mutation-findings</code>, cargo-mutants 27.1.0, Rust 1.90.0, approximately 16 minutes. Exit 2 reflects the three equivalent cases. The subsequent exclusion listing selects 227 mutations; that configured campaign was not rerun. Ordinary tests and Clippy pass on Rust 1.85 and 1.90; formatting, absent/invalid-Book independence, and current/rendered-Book acceptance pass.</p>
<p><strong>Equivalent mutations:</strong> The three OR-to-XOR changes at <code>src/records/markdown.rs:24–26</code> produce the same option mask: the locked parser's four flags occupy distinct bits. Exact exclusions and their review conditions are recorded in <code>.cargo/mutants.toml</code>. Listing checks confirm exactly three exclusions and retain all flag-removal mutations. The previous unviable cases were reviewed: generated replacements call unavailable constructors or require missing trait implementations; they are not counted as detected faults.</p>
<p><strong>Integration and limits:</strong> The <a href="https://github.com/anchor-book/anchor-book/actions/runs/39735753188">PR #2 mutation run</a> for head <code>687a4bf</code> passed with 228 attempted, 184 caught, 44 unviable, no survivors or timeouts, using cargo-mutants 27.1.0 and the reviewed exclusions. That work merged as <code>03fb981</code>. The tool does not discover bodies inside <code>verus!</code>; those functions have separate proof evidence. Merge protection and a remote failure/correction demonstration remain open. Passing local and GitHub authority proofs now satisfy the prerequisite for that follow-up; passing mutation runs alone do not demonstrate correction or enforcement. No ruleset rules applied to main when checked on 2026-10-24; merge enforcement has not been demonstrated.</p>
</td></tr>
<tr><th scope="row">Completion record</th><td>Pending.</td></tr>
</tbody>
</table>
</div>

Keep the engine's independent tests usable with an invalid Book. Mutation testing
does not invoke mdBook, structural checking, or the ignored current-Book test.
The roadmap retains unresolved findings until addressed; passing a scoped run
does not complete milestone 2 or authorize merging.
