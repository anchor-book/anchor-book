# Structural Checking

**Roadmap identifier:** `structural-checking`

This roadmap follows [Engine Bootstrap](bootstrap.md), which prepares the Book's
checker design, requirements, decisions, anchors, working contract, and agreed
scope. It extends the crate from
[Record Rendering — Milestone 1](record-rendering.md#record-rendering-1).
Milestones are assigned as scoped prototypes while Bootstrap review continues
separately. The checker runs independently of rendering and does not require
generated Book output.

## Goal

An agent can use checker feedback to bring a Book into conformance with its
configured structure: run the check, understand the findings, make appropriate
changes within its assignment, and recheck until it passes. Diagnostics must
identify the problem, its location, and the expected structure. Repairs must
satisfy the intended rules without weakening them.

The checker provides a read-only CLI and Rust library. It does not render, edit
files, fetch dependencies, or run project tests. Agents make repairs in this
increment; later engine operations may support them.

The design is defined in:

- [Rust Engine](../design/rust-engine.md)
- [Configuration](../design/configuration.md)
- [Structural Checking](../design/structural-checking.md)
- [Checking Diagnostics](../design/check-diagnostics.md)

The roadmap sequences that design without maintaining a second configuration
schema or rule specification.

## Authority and anchors

The [requirements](../authority/requirements.md#initial-checker-requirements),
[decisions](../authority/decisions.md), and referenced anchors remain drafts.
Milestone criteria link to specific anchor criteria with full or partial planned
coverage, following the [roadmap convention](index.md#anchor-criteria-and-planned-coverage).

Record parsing and authority resolution belong to rendering, including the
[authority-coverage follow-up](record-rendering.md#record-rendering-3-authority-coverage).
They do not expand this layout/navigation check. Evidence aggregation is later work.

Milestone 1 prepares configuration; milestone 2 delivers a usable local checker
whose findings explain format repairs and whose rerun confirms the corrected
structure. Its fixture tests use independently specified corrections. Milestone 4
then demonstrates that an agent can use those findings to choose and make repairs.
Milestones 3 and 4 can proceed independently after milestone 2. All four are
required to complete this roadmap.

Each milestone uses its linked criteria's declared verification methods. Navigation
grammar behavior is tested against the documented supported syntax and version;
it does not require a manual review of each Book. The agent-repair criterion
retains review of the resulting edits and preserved obligations.

<a id="structural-checking-1"></a>

## Milestone 1. The engine selects the intended Book and validates its configuration

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">The library resolves the intended Book and its required structure from explicit configuration. Configuration problems identify what needs correcting before structural evaluation; no files are created or changed.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Complete — merged with passing CI</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="record-rendering.html#record-rendering-1">Record Rendering — Milestone 1</a> and the assigned configuration prototype design. Remaining <a href="bootstrap.html">Bootstrap</a> review and fresh-session exercises proceed separately for this explicitly assigned milestone.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" checked disabled> Extend the prototype&#x27;s library/CLI package; retain Rust 1.85 and the existing TOML/Serde dependencies for configuration. Navigation-parser selection belongs to milestone 2.</label></li><li><label><input type="checkbox" checked disabled> Implement typed configuration parsing, validation, and path resolution in a separate <code>configuration</code> capability; preserve the record, rendering, and adapter boundaries.</label></li><li><label><input type="checkbox" checked disabled> Validate and expose the configured project phase without changing structural rules or claiming automated design-readiness or completion checks.</label></li><li><label><input type="checkbox" checked disabled> Add the real configuration for this repository once the reader works, with source <code>book</code>, output <code>target/anchorbook</code>, and phase <code>prototyping</code>. Replace the temporary phase declaration in <a href="../working-on-anchor-book.html#current-phase">Working on Anchor Book</a> with a pointer to configuration as the single source; keep the working contract independent of this project's current phase.</label></li><li><label><input type="checkbox" checked disabled> Exercise this repository and alternative layouts from different invocation directories. Use independently specified invalid configuration and corrected fixtures to verify errors identify the problem and expected form, and that corrected inputs resolve successfully.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-1-book-selection" class="roadmap-criterion-summary"><th scope="row">Book selection</th><td><p class="roadmap-condition">Resolve the actual Book and equivalent docs/ and md/ layouts correctly, including different invocation directories and display titles.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for book selection" aria-expanded="true" aria-controls="structural-checking-1-book-selection-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-book-selection-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>book-selection</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-book-selection">check-book-selection</a> · <code>invocation-directory</code>, <code>alternative-layout</code> — Partial: configuration selection, resolved paths, and roles. Milestone 2 verifies equivalent structural findings across invocation directories and layouts.</p>
<p><strong>Evidence:</strong> Passed locally: <code>tests/configuration.rs</code> loads book/docs/md layouts and every required role from independent fixtures. Separate subprocesses use different working directories to select the same configuration and resolved paths; a symlinked configuration uses the actual file location. Current-project acceptance loads the real repository configuration.</p>
</td></tr>
<tr id="structural-checking-1-configuration-validation" class="roadmap-criterion-summary"><th scope="row">Configuration validation</th><td><p class="roadmap-condition">Reject invalid fields, versions, phases, mappings, escapes, overlaps, and symlink aliases with located findings and expected conditions. Corrected configuration permits evaluation; allow a resolvable separate nonexistent output and distinguish missing from unresolvable paths.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for configuration validation" aria-expanded="true" aria-controls="structural-checking-1-configuration-validation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-configuration-validation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>configuration-validation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-configuration-validity">check-configuration-validity</a> · <code>schema-rejection</code>, <code>path-boundaries</code> — Full for configuration loading. <code>missing-paths</code> — Partial: missing-source observations and resolution errors; milestone 2 turns those observations into structural findings.</p>
<p><strong>Evidence:</strong> Passed locally: schema unit tests cover required and unknown fields, invalid types, duplicate TOML keys, unsupported versions, and all phase values. Filesystem tests cover escapes, overlaps, resolved duplicate roles, symlink aliases/loops/dangling links, non-directory ancestors, and inaccessible descendants. Corrected fixtures resolve successfully; missing and non-directory sources remain explicit observations for milestone 2.</p>
</td></tr>
<tr id="structural-checking-1-configuration-preservation" class="roadmap-criterion-summary"><th scope="row">Configuration preservation</th><td><p class="roadmap-condition">Configuration operations preserve file contents and directory entries on success and failure, including nonexistent output paths; tests make any attempted write observable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for configuration preservation" aria-expanded="true" aria-controls="structural-checking-1-configuration-preservation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-configuration-preservation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>configuration-preservation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: configuration operations only</p>
<p><strong>Evidence:</strong> Passed locally: fixture snapshots compare all contents and directory entries before and after successful and failing loads. Missing output/source directories are not created. Source/path review confirms only read/metadata operations; permission and error-classification cases exercise failure handling.</p>
</td></tr>
<tr id="structural-checking-1-phase-handoff" class="roadmap-criterion-summary"><th scope="row">Phase handoff</th><td><p class="roadmap-condition">The reader exposes the phase from the real repository configuration. Project guidance directs agents to that value and no longer declares a separate bootstrap phase; the working contract describes phase behavior without project-specific values. Valid phase choices do not alter structural rules or imply design acceptance or system completion.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for phase handoff" aria-expanded="true" aria-controls="structural-checking-1-phase-handoff-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-phase-handoff-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>phase-handoff</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — configuration and working-contract integration; <a href="../design/configuration.html#project-phase">project phase design</a></p>
<p><strong>Evidence:</strong> Passed locally: <code>anchorbook.toml</code> declares the phase; Working on Anchor Book now directs agents there without repeating the value. Unit tests accept all three phases and reject missing/invalid values. The ignored current-project acceptance test confirms the real configured phase and layout; loading makes no structural or completion claim.</p>
</td></tr>
<tr id="structural-checking-1-independent-build" class="roadmap-criterion-summary"><th scope="row">Independent build</th><td><p class="roadmap-condition">Cargo build, formatting, Clippy, and configuration/API tests pass without rendering or validating the current Book.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent build" aria-expanded="true" aria-controls="structural-checking-1-independent-build-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-independent-build-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-build</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>broken-book</code> — Partial: engine build and fixture tests; current-Book checking follows</p>
<p><strong>Evidence:</strong> Passed locally: tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. The independence experiment passes with no Book and with an invalid Book; explicit Book acceptance rejects the invalid input. Configuration fixtures do not read the repository Book or configuration.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — local checks passed</button></td></tr>
<tr id="structural-checking-1-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Revision and scope:</strong> Working tree based on <code>2359d59</code>, verified locally on 2026-10-24. The configuration capability, repository configuration, tests, and phase handoff are included together.</p>
<p><strong>Checks:</strong> Formatting, ordinary tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. Four configuration unit tests cover decoding and observation-error classification; nine filesystem integration tests cover selection, paths, aliases, errors, preservation, and separate-process working directories. <code>cargo test --locked --test current_book -- --ignored</code> loads the real configuration and Book. The absent/invalid-Book independence experiment, mdBook build, and rendered acceptance pass. The existing authority proof still verifies 15 units with zero errors; it does not prove configuration loading.</p>
<p><strong>Mutation evidence:</strong> cargo-mutants 27.1.0 on <code>src/configuration.rs</code> and <code>src/configuration/*.rs</code> first found three survivors in missing-path versus I/O-error handling; independent error and permission cases now cover them. Final runs: 65 mutations (34 caught, 31 unviable) and a 14-mutation retest of <code>paths.rs</code> (12 caught, 2 unviable), with no survivors, timeouts, or exclusions. Unviable mutations are not counted as detection.</p>
<p><strong>Limits:</strong> Commands are in CONTRIBUTING. Filesystem tests ran on macOS; symlink and permission fixtures are Unix-specific, privileged runners can bypass permission bits, and native Windows behavior is not claimed.</p></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Integration</th><td colspan="1"><a href="https://github.com/anchor-book/anchor-book/pull/3">PR #3</a> merged as <code>7d795bb</code>. Its engine, Book, mutation, and authority-proof checks passed. The merged revision also passed <a href="https://github.com/anchor-book/anchor-book/actions/runs/40262112290">Engine</a>, <a href="https://github.com/anchor-book/anchor-book/actions/runs/40262112281">Book</a>, and <a href="https://github.com/anchor-book/anchor-book/actions/runs/40262112289">authority proof</a> workflows.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Complete and merged: configuration loading, validation, preservation, and phase handoff have local and CI evidence. This establishes configuration loading, not an operational structural checker.</td></tr>
</tbody>
</table>
</div>

<a id="structural-checking-2"></a>

## Milestone 2. The checker identifies format repairs and confirms conformance

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">An engineer or agent can run the library or CLI to locate format defects, understand the expected structure, make separate repairs, and recheck. A pass confirms the evaluated format rules; incomplete evaluation never counts as success.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Complete — merged with passing CI; accepted by the engineer</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Structural Checking — Milestone 1.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" checked disabled> Integrate the selected <code>mdbook-summary = &quot;=0.5.2&quot;</code> parser behind a private adapter. Add supported-input validation and source-location mapping using Markdown events, including constructs the parser silently ignores; expose engine-owned results and errors. Read renderer settings, navigation, and chapter metadata using the documented supported syntax and version. Cover source and summary prerequisites, all linked chapter targets, configured section roles, and renderer alignment without executing preprocessors or creating missing files.</label></li><li><label><input type="checkbox" checked disabled> Evaluate the initial structural rules in a separate <code>checking</code> capability. Keep rules independent of rendering and protocol handling; share diagnostic primitives only where both implemented capabilities need them.</label></li><li><label><input type="checkbox" checked disabled> Expose the same findings and evaluated scope through the Rust library and <code>anchorbook check --config path/to/anchorbook.toml</code>. Include locations, expected and observed structure, and useful next actions; retain the documented result states, exit statuses, and output streams. Review the public API boundary and test actual CLI/library equivalence.</label></li><li><label><input type="checkbox" checked disabled> Explain missing prerequisites and incomplete evaluation so an agent knows whether it needs to repair structure or resolve an input/operational problem.</label></li><li><label><input type="checkbox" checked disabled> Exercise the current Book and independent passing, failing, and incomplete fixtures. Assert repair-relevant findings, apply independently specified corrections outside the checker, and recheck successfully. Verify each invocation preserves files and does not execute project tools; include failures after partial findings have been collected.</label></li><li><label><input type="checkbox" checked disabled> Complete mutation verification and final checks on GitHub, investigate any findings, and record evidence for the final revision before marking this milestone complete.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-2-navigation" class="roadmap-criterion-summary"><th scope="row">Navigation</th><td><p class="roadmap-condition">The actual Book passes. Independent fixtures cover source and summary prerequisites, chapter targets, required roles, and supported and unsupported navigation. Assert located findings and expected structure; correcting seeded format defects produces a pass. Grammar-boundary tests use the supported mdBook version (0.5.4) and include ignored prose/code/headings, valid comments and reference links, duplicates, drafts, external destinations, encoded spaces, and source locations. Parser success alone cannot satisfy this criterion.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for navigation" aria-expanded="true" aria-controls="structural-checking-2-navigation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-navigation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>navigation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-navigation-conformance">check-navigation-conformance</a> · <code>chapter-inventory</code>, <code>required-roles</code>, <code>grammar-boundary</code> — Full</p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-configuration-validity">check-configuration-validity</a> · <code>missing-paths</code> — Partial: a resolvable missing source produces the expected structural violation; resolution failures prevent a pass. Configuration-resolution evidence is in milestone 1.</p>
<p><strong>Evidence:</strong> Local tests pass in <code>checking::navigation::tests</code> and <code>tests/checking.rs</code>: hierarchy, reference links, drafts, roles, malformed/ignored input, missing files, and correction/recheck cases. The current Book passes its explicit structural acceptance test.</p>
</td></tr>
<tr id="structural-checking-2-renderer-alignment" class="roadmap-criterion-summary"><th scope="row">Renderer alignment</th><td><p class="roadmap-condition">Report differing renderer settings and expected locations without editing files. Handle defaults, equivalent paths, missing configuration, and malformed or unreadable input; aligning seeded mismatches produces a pass.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for renderer alignment" aria-expanded="true" aria-controls="structural-checking-2-renderer-alignment-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-renderer-alignment-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>renderer-alignment</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-renderer-alignment">check-renderer-alignment</a> · <code>resolved-comparison</code>, <code>renderer-input</code> — Full</p>
<p><strong>Evidence:</strong> Local tests cover mdBook defaults, equivalent relative and absolute paths, missing/invalid renderer input, and corrected mismatches. A CLI test sets an environment override and confirms only file settings are checked. The implementation uses pure mdbook-core deserialization; it never runs the renderer or configured commands.</p>
</td></tr>
<tr id="structural-checking-2-typed-results" class="roadmap-criterion-summary"><th scope="row">Typed results</th><td><p class="roadmap-condition">Library and actual CLI results agree against independent expectations, including equivalent configured layouts and invocations from different working directories. Library errors preserve causes without printing, exiting, or panicking.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for typed results" aria-expanded="true" aria-controls="structural-checking-2-typed-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-typed-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>typed-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#check-shared-typed-results">check-shared-typed-results</a> · <code>equivalent-callers</code>, <code>library-boundary</code> — Full</p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-book-selection">check-book-selection</a> · <code>invocation-directory</code>, <code>alternative-layout</code> — Partial: equivalent structural findings across layouts and caller directories, completing milestone 1 selection evidence.</p>
<p><strong>Evidence:</strong> Actual-process tests compare CLI output with library reports for pass, structural failure, and incomplete evaluation, plus independent expected fields and a complete passing report. Alternative layouts and caller directories are exercised. Source review confirms the CLI calls the shared library, findings retain typed locations and causes, and parser types remain private. The final GitHub mutation campaign has no survivors; see milestone evidence.</p>
</td></tr>
<tr id="structural-checking-2-truthful-results" class="roadmap-criterion-summary"><th scope="row">Truthful results</th><td><p class="roadmap-condition">Verify documented exit statuses, incomplete and mixed failures, blocked checks, evaluated scope, and stdout/stderr behavior.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for truthful results" aria-expanded="true" aria-controls="structural-checking-2-truthful-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-truthful-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>truthful-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-truthful-results">check-truthful-results</a> · <code>outcomes-and-precedence</code>, <code>blocked-evaluation</code>, <code>report-scope</code> — Full</p>
<p><strong>Evidence:</strong> Local tests verify exit 0/1/2, complete stdout versus incomplete stderr, preserved partial findings, explicit blockers, and operational-error precedence. Model tests reject an empty unevaluated report as a pass. Missing prerequisites do not generate invented missing-role findings.</p>
</td></tr>
<tr id="structural-checking-2-diagnostics" class="roadmap-criterion-summary"><th scope="row">Diagnostics</th><td><p class="roadmap-condition">For each seeded format defect, assert that findings identify the affected location, expected and observed structure, and useful repair guidance without weakening requirements. Verify documented fields, stable order, and readable output without styling; do not invent unavailable observations.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for diagnostics" aria-expanded="true" aria-controls="structural-checking-2-diagnostics-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-diagnostics-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>diagnostics</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-actionable-diagnostics">check-actionable-diagnostics</a> · <code>diagnostic-content</code>, <code>stable-presentation</code> — Full</p>
<p><strong>Evidence:</strong> Local tests assert independently specified codes, locations, related declarations, expected/found fields, guidance, and deterministic ordering. Unicode positions and terminal-control escaping are exercised. Guidance preserves intended configuration and required roles; the real agent usability demonstration remains in milestone 4.</p>
</td></tr>
<tr id="structural-checking-2-observation-only" class="roadmap-criterion-summary"><th scope="row">Observation only</th><td><p class="roadmap-condition">Passing, failing, and incomplete checks preserve contents and directory entries; execution traps establish no subprocess, network, preprocessor, or verifier activity.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for observation only" aria-expanded="true" aria-controls="structural-checking-2-observation-only-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-observation-only-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>observation-only</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code>, <code>no-execution</code> — Full</p>
<p><strong>Evidence:</strong> Fixture snapshots preserve all contents and directory entries across pass, failure, and incomplete runs. Missing chapters remain absent even when mdBook create-missing is enabled. Configured preprocessor/renderer command traps remain unexecuted, and external URLs produce no local-target failures. Source review confirms inspection uses file reads/metadata and pure parser calls, with no process or network-fetch path.</p>
</td></tr>
<tr id="structural-checking-2-acceptance-cases" class="roadmap-criterion-summary"><th scope="row">Acceptance cases</th><td><p class="roadmap-condition">Record results for the design’s <a href="../design/structural-checking.html#acceptance-against-this-book">acceptance cases</a>, including alternate layouts and deliberate failures.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for acceptance cases" aria-expanded="true" aria-controls="structural-checking-2-acceptance-cases-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-acceptance-cases-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>acceptance-cases</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — acceptance suite record</p>
<p><strong>Evidence:</strong> The current Book and independent fixtures exercise missing prerequisites, alternate layouts, section placement, renderer drift, configuration errors, inaccessible targets, and corrected reruns. A real agent repair demonstration is deliberately reserved for milestone 4; passing fixture tests do not establish it.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — local, CI, and mutation checks passed</button></td></tr>
<tr id="structural-checking-2-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Implementation:</strong> Based on <code>e02f48b</code>: structural/v1 in the library and CLI, with separate observation, navigation, rule, report-model, and presentation modules. The parser crates are now pinned to mdbook-summary and mdbook-core 0.5.4 (PR #6); that revision's full mutation campaign is recorded below. Checking invokes no write operation, project test runner, or verifier.</p>
<p><strong>Local evidence:</strong> Ordinary tests and Clippy pass on Rust 1.88.0 and stable 1.99.1: 16 library tests and 19 checker integration tests covering actual CLI streams and statuses, fixture corrections, source preservation, path and role rules, renderer settings, unreadable input, errors, and source positions. The current Book passes <code>anchorbook check --config anchorbook.toml</code>.</p>
<p><strong>Mutation correction:</strong> The <a href="https://github.com/anchor-book/anchor-book/actions/runs/40274238323">campaign for <code>e65cc0c</code></a> found four survivors in 464 mutations: standalone inline code was admitted, metadata errors after resolution were treated as missing files, and known top-level roles were blocked in two ways. Regression tests now cover them, without production or exclusion changes. The campaigns for <a href="https://github.com/anchor-book/anchor-book/actions/runs/40348199734"><code>fbdc073</code></a> and <a href="https://github.com/anchor-book/anchor-book/actions/runs/40425789096"><code>322a102</code></a> each tested 464 mutations: 354 caught, 110 unviable, none missed or timed out (cargo-mutants 27.1.0).</p>
<p><strong>Integration and limits:</strong> <a href="https://github.com/anchor-book/anchor-book/pull/5">PR #5</a> merged as <code>3d197c0</code>; its engine, Book, mutation, and authority-proof checks passed, as did the <a href="https://github.com/anchor-book/anchor-book/actions/runs/40368530912">Engine</a>, <a href="https://github.com/anchor-book/anchor-book/actions/runs/40368530949">Book</a>, and <a href="https://github.com/anchor-book/anchor-book/actions/runs/40368530948">authority proof</a> workflows on main. The checker is test-validated, not formally proved. Unix symlink/permission cases have no native Windows result; privileged runners can bypass permission tests.</p></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Complete on 2026-10-26, accepted by the engineer. Merged in <code>3d197c0</code> with passing CI and a full mutation campaign with no survivors. On <code>a1cf336</code>, the checker passed this Book and reported a missing chapter and a <code>book.toml</code> source mismatch in a disposable broken copy, exiting 1 without changing files. Milestone 3 owns CI gating; milestone 4 owns the real agent repair demonstration.</td></tr>
</tbody>
</table>
</div>

<a id="structural-checking-3"></a>

## Milestone 3. Continuous checks protect the current Book

The shared rendering and publication gate also addresses Record Rendering
milestone 3; its other criteria remain separate.

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Changes to this repository receive independent engine validation and an acceptance check against its current Book before rendering.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Complete — merged with GitHub evidence; accepted by the engineer</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Structural Checking — Milestone 2.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-3</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" checked disabled> Retain configuration and structural-rule tests in the independent engine and supported-toolchain checks introduced by rendering.</label></li>
<li><label><input type="checkbox" checked disabled> Run the structural check inside the Anchor Book preprocessor so every <code>mdbook build</code> refuses a failed or incomplete check, and make rendering depend on successful engine checks and current-Book acceptance for the same revision. Reuse rendered-output acceptance and reconcile shared workflow gating with <a href="record-rendering.html#record-rendering-3">Record Rendering milestone 3</a>.</label></li>
<li><label><input type="checkbox" checked disabled> Exercise valid, structurally broken, and incomplete-input Book cases locally and on GitHub; failed or incomplete acceptance must prevent rendering while independent engine tests remain runnable.</label></li>
<li><label><input type="checkbox" checked disabled> Document the preprocessor check and the shared workflow dependency chain.</label></li>
<li><label><input type="checkbox" checked disabled> Record actual CI results and job ordering for the implemented revision, including failure cases.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-3-independent-acceptance" class="roadmap-criterion-summary"><th scope="row">Independent acceptance</th><td><p class="roadmap-condition">With deliberately broken navigation and no rendered output, engine compilation and fixture tests pass while current-Book acceptance reports the known failure.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent acceptance" aria-expanded="true" aria-controls="structural-checking-3-independent-acceptance-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-independent-acceptance-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-acceptance</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>broken-book</code> — Full</p>
<p><strong>Evidence:</strong> Passed locally and on GitHub. <code>scripts/check-engine-independence.sh</code> runs independent tests in a disposable checkout without rendered output, with no Book, an invalid record, a missing navigation target, and unsupported summary prose; separate acceptance rejects each with its expected diagnostic. The PR #9 runs below repeat the missing-chapter and unsupported-navigation cases on GitHub.</p>
</td></tr>
<tr id="structural-checking-3-ci-order" class="roadmap-criterion-summary"><th scope="row">CI order</th><td><p class="roadmap-condition">Record actual CI showing engine checks, structural Book acceptance, then rendering for a valid revision. Broken or unprocessable Book input produces useful findings and prevents rendering; independent engine tests remain runnable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for ci order" aria-expanded="true" aria-controls="structural-checking-3-ci-order-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-ci-order-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-order</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>ci-order</code> — Full</p>
<p><strong>Evidence:</strong> Passed on GitHub: valid revisions ran both Engine jobs, then Build mdBook, then Publish Book on main; broken and unprocessable Books failed current-Book acceptance with useful findings, independent tests passed, and rendering was skipped. Locally, <code>mdbook build</code> also refuses a structural failure (exit 1) and incomplete input (exit 2) through the preprocessor. See milestone evidence.</p>
</td></tr>
<tr id="structural-checking-3-no-repair" class="roadmap-criterion-summary"><th scope="row">No repair</th><td><p class="roadmap-condition">The acceptance check performs no source repair; a valid Book renders after acceptance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for no repair" aria-expanded="true" aria-controls="structural-checking-3-no-repair-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-no-repair-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>no-repair</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: CI acceptance invocation</p>
<p><strong>Evidence:</strong> Passed locally: source trees, files, and directory entries are unchanged by passing, failing, and incomplete acceptance invocations, and acceptance creates no rendered output. <code>build_gate</code> renders a valid fixture Book and produces no rendered JSON for rejected ones. On GitHub, valid revisions render and publish after acceptance; file preservation was not observed in CI.</p>
</td></tr>
<tr id="structural-checking-3-ci-evidence" class="roadmap-criterion-summary"><th scope="row">CI evidence</th><td><p class="roadmap-condition">Record the tested revision, actual workflow results, and local commands; local success alone is not CI evidence.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for ci evidence" aria-expanded="true" aria-controls="structural-checking-3-ci-evidence-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-ci-evidence-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-evidence</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — integration evidence</p>
<p><strong>Evidence:</strong> Recorded in the milestone evidence: PR, main, and deliberate-failure runs with revisions, job order, and results.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-3-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — local experiments and GitHub runs passed</button></td></tr>
<tr id="structural-checking-3-evidence" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Revision and integration:</strong> Implementation <code>7e81db1</code>, merged as <code>9ecd950</code> (PR #8). PR #10 (<code>613cb4d</code>, merged as <code>2aaf496</code>) runs current-Book acceptance before the failure experiments so a broken Book is reported directly.</p>
<p><strong>Implementation:</strong> <code>book.yml</code> calls <code>engine.yml</code> as a local reusable workflow; Build mdBook requires both Rust jobs, which include current-Book acceptance. <code>[preprocessor.anchorbook] config</code> names <code>anchorbook.toml</code>, and <code>anchorbook md</code> stops on a failed or incomplete check before rendering any chapter. Rendered-output acceptance precedes upload and publication. The <a href="../authority/decisions.html#decision-native-tool-integration">native tool integration decision</a> records why the check runs in the preprocessor. Proof and mutation workflows and merge rules are unchanged.</p>
<p><strong>Local failure experiments:</strong> <code>build_gate</code> runs <code>anchorbook md</code> on fixture Books: broken navigation (exit 1) and unsupported navigation, invalid configuration, or missing configuration (exit 2) produce the check report, no rendered JSON, and unchanged snapshots. Real <code>mdbook build</code> runs on disposable copies of this Book reject unsupported summary prose (<code>input.unsupported</code>) and a renderer path mismatch (<code>structure.renderer-path-mismatch</code>) and render nothing; mdBook itself rejects a missing chapter before preprocessing because <code>create-missing</code> is false. Under <code>mdbook watch</code>, a failing rebuild keeps the previous output and a restored summary rebuilds.</p>
<p><strong>Local checks:</strong> Formatting, ordinary tests, Clippy, current-Book acceptance, and the independence experiments pass on Rust 1.88.0 and stable 1.99.1. <code>mdbook build</code> passes with mdBook 0.5.4 and mdbook-mermaid 0.17.1, leaving sources unchanged; rendered acceptance checks 26 records and 1,066 local links. cargo-mutants 27.1.0 on the changed <code>src/adapters/mdbook.rs</code> and <code>src/main.rs</code>: 45 mutants, 42 caught, 3 unviable, none missed or timed out.</p>
<p><strong>GitHub results:</strong> Valid revisions: the <a href="https://github.com/anchor-book/anchor-book/actions/runs/40453654594">PR #8 Book run</a> for <code>7e81db1</code> passed Engine checks on Rust 1.88.0 and stable, then Build mdBook (Publish skipped on a PR); the <a href="https://github.com/anchor-book/anchor-book/actions/runs/40457351906">PR #10 Book run</a> for <code>613cb4d</code> passed. On main, Book runs for <a href="https://github.com/anchor-book/anchor-book/actions/runs/40455815967"><code>9ecd950</code></a>, <a href="https://github.com/anchor-book/anchor-book/actions/runs/40458337088"><code>2aaf496</code></a>, and <a href="https://github.com/anchor-book/anchor-book/actions/runs/40459873774"><code>ed2ade0</code></a> passed with job order Engine (both toolchains), Build mdBook, Publish Book; mutation and authority-proof workflows also passed for the PR heads. Deliberate failures on throwaway <a href="https://github.com/anchor-book/anchor-book/pull/9">PR #9</a>: a <a href="https://github.com/anchor-book/anchor-book/actions/runs/40457441522">missing chapter (<code>e520765</code>)</a> failed current-Book acceptance on both toolchains with <code>structure.chapter-missing</code> (result failed); <a href="https://github.com/anchor-book/anchor-book/actions/runs/40457892139">unsupported navigation prose (<code>de73f3e</code>)</a> failed with <code>input.unsupported</code> (result incomplete). In both, the independent suites passed (16 library and 18 integration tests per toolchain), Build mdBook and Publish Book were skipped; PR #9 was closed unmerged.</p>
<p><strong>Limits:</strong> Commands are in CONTRIBUTING. <code>build_gate</code> tests the preprocessor process boundary, not mdBook itself. On GitHub, broken Books are stopped by current-Book acceptance before Build mdBook runs, so the preprocessor's own refusal was demonstrated only in the local <code>mdbook build</code> experiments. File preservation was compared locally, not in CI.</p></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Complete on 2026-11-07, accepted by the engineer. Merged in <code>9ecd950</code> and <code>2aaf496</code>; local and GitHub evidence covers every criterion within the limits above.</td></tr>
</tbody>
</table>
</div>

<a id="structural-checking-4"></a>

## Milestone 4. An agent can repair Book structure using checker feedback

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">An agent can use findings to make scoped structural repairs and reach a passing check while preserving unrelated content.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Structural Checking — Milestone 2.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-4</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Prepare disposable copies of this Book with known structural defects and independently specified expected findings and valid repairs.</label></li><li><label><input type="checkbox" disabled> Give an agent the Book, the checker, and an explicit repair assignment; have it check, interpret the findings, edit the permitted files, and recheck.</label></li><li><label><input type="checkbox" disabled> Include missing chapters, misplaced required sections, and renderer-setting drift. Exercise invalid or unsupported input that must be resolved before evaluation. The agent must distinguish these results from a successful format check and preserve the intended configuration and required roles.</label></li><li><label><input type="checkbox" disabled> Improve unclear diagnostics or workflow guidance and repeat affected cases.</label></li><li><label><input type="checkbox" disabled> Record the initial defects, findings, edits, final result, and any human help.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-4-agent-repair" class="roadmap-criterion-summary"><th scope="row">Agent repair</th><td><p class="roadmap-condition">Recorded agent runs repair seeded defects and reach a passing check. Independent review confirms intended repairs, unrelated content preservation, no weakened requirements, and any human assistance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent repair" aria-expanded="true" aria-controls="structural-checking-4-agent-repair-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-4-agent-repair-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-repair</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-actionable-diagnostics">check-actionable-diagnostics</a> · <code>agent-repair</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-4-separate-edits" class="roadmap-criterion-summary"><th scope="row">Separate edits</th><td><p class="roadmap-condition">Each checker invocation remains read-only; agent edits are separately identifiable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for separate edits" aria-expanded="true" aria-controls="structural-checking-4-separate-edits-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-4-separate-edits-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>separate-edits</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: invocations during the agent workflow</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-4-unresolved-cases" class="roadmap-criterion-summary"><th scope="row">Unresolved cases</th><td><p class="roadmap-condition">Cases outside the assignment or the agent’s ability produce an explicit explanation rather than a false completion claim; record versions and assistance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for unresolved cases" aria-expanded="true" aria-controls="structural-checking-4-unresolved-cases-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-4-unresolved-cases-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>unresolved-cases</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-truthful-results">check-truthful-results</a> · <code>report-scope</code> — Partial: reported scope and limits during the workflow</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-4-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-4-evidence" class="roadmap-criterion-details"><td colspan="2">Pending; use the <a href="index.html#evidence-records">evidence format</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

## Updating and completing the work

Use disposable copies for deliberate failures and public-safe reusable fixtures.
Exercise the actual Book as each capability becomes usable; Structural Checking — Milestone 3 is not the first
self-use experiment. Do not add success-returning placeholder commands.

Follow the [working contract](../ai-working-contract.md) for progress, evidence,
and design-to-roadmap updates.

This roadmap is complete when all four milestones are complete: the current
Book and independent fixtures validate the checker, CI exercises it, and recorded
agent runs demonstrate the check, repair, and recheck loop. A successful structural
check does not establish full anchor verification or completion of other roadmaps.
Materialization and contract generation remain separate later work.
