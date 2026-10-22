# Mutation Testing

**Roadmap identifier:** `mutation-testing`

**Status:** Milestone 1 complete locally; milestone 2 in progress. Full campaigns
locally and on GitHub found the same unresolved survivors. Corrections, the PR
workflow demonstration, and required-check enforcement remain.

## Goal

Before merging implementation into `main`, agents can run mutation tests,
investigate findings, improve tests or implementation, and repeat verification.
GitHub independently runs the checks on the pull request; required checks protect
the merge. Tool setup alone does not establish that the engine's tests are adequate.

This work precedes further engine implementation. It applies the
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
<tr><th scope="row">Status</th><td>In progress — local and remote baselines recorded; findings and merge workflow remain</td></tr>
<tr><th scope="row">Identifier</th><td><code>mutation-testing-2</code></td></tr>
<tr><th scope="row">Design basis</th><td>Working-contract testing policy and Rust Engine testing guidance.</td></tr>
<tr><th scope="row">Dependencies</th><td>Mutation Testing milestone 1; publication of the workflow and access to repository merge rules.</td></tr>
<tr><th scope="row">Tasks</th><td><ul class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Run the initial unfiltered engine campaign and record its scope, results, and limits.</label></li>
<li><label><input type="checkbox" checked disabled> Manually run the workflow on the published prototype checkpoint; verify failure reporting and retained evidence.</label></li>
<li><label><input type="checkbox" disabled> Review survivors and unviable mutations. Correct test gaps or defects, justify any equivalent mutations, and repeat affected checks and the full campaign.</label></li>
<li><label><input type="checkbox" disabled> Exercise the workflow on a PR and record its revision, results, artifacts, and runtime. Tune or split the 45-minute budget if needed without dropping coverage.</label></li>
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
<p><strong>Engine campaign:</strong> Working tree based on <code>d66184b</code> with the milestone 1 test corrections, tested 2026-10-21 using cargo-mutants 27.1.0 and Rust 1.90.0 on macOS. Command: <code>target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --output target/mutation-full</code>. No source filters or exclusions. The baseline passed; all 226 generated mutations were attempted in approximately 10 minutes: 154 caught, 28 missed, 44 unviable, zero timeouts. Exit status 2 correctly reports failure.</p>
<p><strong>Outstanding review:</strong> Survivors occur in Markdown recognition (10), record diagnostics (8), rendering errors (3), mdBook adapter errors (3), inventory behavior (2), schema source locations (1), and HTML presentation (1). Review their diffs against the specified behavior before choosing tests or implementation corrections. Parser-option mutations may include equivalent changes; none has been excluded or accepted as equivalent. The 44 unviable mutations are not counted as test detection.</p>
<p><strong>Supporting records and limits:</strong> Reproduce with the command above; local output remains ignored under <code>target/mutation-full/mutants.out/</code>. Source and dependency files are unchanged. Findings have not all been corrected or classified, so this is a baseline, not a passing campaign or anchor verification.</p>
<p><strong>Remote campaign:</strong> Setup commit <code>46f47c8</code> was published to main with the engineer's authorization. The <a href="https://github.com/anchor-book/anchor-book/actions/runs/39662175193">manual GitHub mutation run</a> completed on 2026-10-21 using Ubuntu 24.04, Rust 1.99.1, and cargo-mutants 27.1.0. Ordinary tests and the mutation baseline passed. All 226 mutations were attempted in approximately 3 minutes: 154 caught, 28 missed, 44 unviable, zero timeouts. The surviving mutations match the local run exactly. The mutation step exited 2 and the job failed, as required; the subsequent artifact upload succeeded. The <code>mutation-results</code> artifact contains outcomes, mutation descriptions, diffs, and logs, with 14-day retention. Total job time, including installation and upload, was approximately 4 minutes.</p>
<p><strong>Integration and limits:</strong> The <a href="https://github.com/anchor-book/anchor-book/actions/runs/39662145649">Engine checks</a> and <a href="https://github.com/anchor-book/anchor-book/actions/runs/39662146023">Book build and Pages deployment</a> passed for <code>46f47c8</code>. The initial GitHub inspection found no rulesets or classic branch protection on main; no merge rule was changed. This manual run demonstrates remote execution, failure propagation, and artifact retention. It does not resolve the survivors, demonstrate the PR correction loop, or establish merge enforcement.</p>
</td></tr>
<tr><th scope="row">Completion record</th><td>Pending.</td></tr>
</tbody>
</table>
</div>

Keep the engine's independent tests usable with an invalid Book. Mutation testing
does not invoke mdBook, structural checking, or the ignored current-Book test.
The roadmap retains unresolved findings until addressed; passing a scoped run
does not complete milestone 2 or authorize merging.

## Scoped follow-up during criterion-method work

The schema preparation in [Formal Verification milestone 3](formal-verification.md#formal-verification-3)
reproduced the HTML heading-label survivor from the baseline above. Assertions now
check Authority on anchor records and Requirements on decisions. The local scoped
rerun caught that mutation; its complete scope, commands, and results are recorded
there. This correction was committed as `7ab9616`.
The historical full-campaign results remain unchanged; a new full campaign and
review of the remaining findings are still required by milestone 2.
