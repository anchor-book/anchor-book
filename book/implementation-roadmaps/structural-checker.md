# Structural Checking

**Roadmap identifier:** `structural-checking`

**Status:** Proposed implementation plan for the agreed prototype design direction;
implementation not started.

This roadmap follows [Engine Bootstrap](bootstrap.md), which prepares the Book's
checker design, requirements, decisions, anchors, working contract, and agreed
scope. It extends the crate from
[Record Rendering — Milestone 1](record-rendering.md#record-rendering-1).
The engineer has agreed that the design is a good starting point for prototyping.
Bootstrap still owns the remaining API/dependency choices, scope review, and
agent-readiness checks. The checker runs independently of rendering and does not
require generated Book output.

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
[decisions](../authority/design-decisions.md), and referenced anchors remain drafts.
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
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="bootstrap.html">Engine Bootstrap</a> complete and <a href="record-rendering.html#record-rendering-1">Record Rendering — Milestone 1</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Extend the prototype&#x27;s library/CLI package; confirm or update Rust/dependency versions for configuration and navigation checking.</label></li><li><label><input type="checkbox" disabled> Implement typed configuration parsing, validation, and path resolution in a separate <code>configuration</code> capability; preserve the record, rendering, and adapter boundaries.</label></li><li><label><input type="checkbox" disabled> Validate and expose the configured project phase without changing structural rules or claiming automated design-readiness or completion checks.</label></li><li><label><input type="checkbox" disabled> Add the real configuration for this repository once the reader works, with source <code>book</code>, output <code>target/anchorbook</code>, and phase <code>prototyping</code>. Replace the temporary phase declaration in <a href="../working-on-anchor-book.html#current-phase">Working on Anchor Book</a> with a pointer to configuration as the single source; keep the working contract independent of this project's current phase.</label></li><li><label><input type="checkbox" disabled> Exercise this repository and alternative layouts from different invocation directories. Use independently specified invalid configuration and corrected fixtures to verify errors identify the problem and expected form, and that corrected inputs resolve successfully.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-1-book-selection" class="roadmap-criterion-summary"><th scope="row">Book selection</th><td><p class="roadmap-condition">Resolve the actual Book and equivalent docs/ and md/ layouts correctly, including different invocation directories and display titles.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for book selection" aria-expanded="true" aria-controls="structural-checking-1-book-selection-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-book-selection-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>book-selection</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-book-selection">check-book-selection</a> · <code>invocation-directory</code>, <code>alternative-layout</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-1-configuration-validation" class="roadmap-criterion-summary"><th scope="row">Configuration validation</th><td><p class="roadmap-condition">Reject invalid fields, versions, phases, mappings, escapes, overlaps, and symlink aliases with located findings and expected conditions. Corrected configuration permits evaluation; allow a resolvable separate nonexistent output and distinguish missing from unresolvable paths.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for configuration validation" aria-expanded="true" aria-controls="structural-checking-1-configuration-validation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-configuration-validation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>configuration-validation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-configuration-validity">check-configuration-validity</a> · <code>schema-rejection</code>, <code>path-boundaries</code>, <code>missing-paths</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-1-configuration-preservation" class="roadmap-criterion-summary"><th scope="row">Configuration preservation</th><td><p class="roadmap-condition">Configuration operations preserve file contents and directory entries on success and failure, including nonexistent output paths; tests make any attempted write observable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for configuration preservation" aria-expanded="true" aria-controls="structural-checking-1-configuration-preservation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-configuration-preservation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>configuration-preservation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: configuration operations only</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-1-phase-handoff" class="roadmap-criterion-summary"><th scope="row">Phase handoff</th><td><p class="roadmap-condition">The reader exposes the phase from the real repository configuration. Project guidance directs agents to that value and no longer declares a separate bootstrap phase; the working contract describes phase behavior without project-specific values. Valid phase choices do not alter structural rules or imply design acceptance or system completion.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for phase handoff" aria-expanded="true" aria-controls="structural-checking-1-phase-handoff-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-phase-handoff-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>phase-handoff</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — configuration and working-contract integration; <a href="../design/configuration.html#project-phase">project phase design</a></p>
<p><strong>Evidence:</strong> Pending configuration reader and repository migration.</p>
</td></tr>
<tr id="structural-checking-1-independent-build" class="roadmap-criterion-summary"><th scope="row">Independent build</th><td><p class="roadmap-condition">Cargo build, formatting, Clippy, and configuration/API tests pass without rendering or validating the current Book.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent build" aria-expanded="true" aria-controls="structural-checking-1-independent-build-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-independent-build-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-build</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>broken-book</code> — Partial: engine build and fixture tests; current-Book checking follows</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-1-evidence" class="roadmap-criterion-details"><td colspan="2">Pending; use the <a href="index.html#evidence-records">evidence format</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. No operational checker is claimed by this milestone.</td></tr>
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
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Structural Checking — Milestone 1.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Read renderer settings, navigation, and chapter metadata using the documented supported mdBook syntax and version. Cover source and summary prerequisites, all linked chapter targets, configured section roles, and renderer alignment without executing preprocessors or creating missing files.</label></li><li><label><input type="checkbox" disabled> Evaluate the initial structural rules in a separate <code>checking</code> capability. Keep rules independent of rendering and protocol handling; share diagnostic primitives only where both implemented capabilities need them.</label></li><li><label><input type="checkbox" disabled> Expose the same findings and evaluated scope through the Rust library and <code>anchorbook check --config path/to/anchorbook.toml</code>. Include locations, expected and observed structure, and useful next actions; retain the documented result states, exit statuses, and output streams. Review the public API boundary and test actual CLI/library equivalence.</label></li><li><label><input type="checkbox" disabled> Explain missing prerequisites and incomplete evaluation so an agent knows whether it needs to repair structure or resolve an input/operational problem.</label></li><li><label><input type="checkbox" disabled> Exercise the current Book and independent passing, failing, and incomplete fixtures. Assert repair-relevant findings, apply independently specified corrections outside the checker, and recheck successfully. Verify each invocation preserves files and does not execute project tools; include failures after partial findings have been collected.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-2-navigation" class="roadmap-criterion-summary"><th scope="row">Navigation</th><td><p class="roadmap-condition">The actual Book passes. Independent fixtures cover source and summary prerequisites, chapter targets, required roles, and supported and unsupported navigation. Assert located findings and expected structure; correcting seeded format defects produces a pass. Grammar-boundary verification uses tests against the documented supported syntax and version.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for navigation" aria-expanded="true" aria-controls="structural-checking-2-navigation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-navigation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>navigation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-navigation-conformance">check-navigation-conformance</a> · <code>chapter-inventory</code>, <code>required-roles</code>, <code>grammar-boundary</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-2-renderer-alignment" class="roadmap-criterion-summary"><th scope="row">Renderer alignment</th><td><p class="roadmap-condition">Report differing renderer settings and expected locations without editing files. Handle defaults, equivalent paths, missing configuration, and malformed or unreadable input; aligning seeded mismatches produces a pass.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for renderer alignment" aria-expanded="true" aria-controls="structural-checking-2-renderer-alignment-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-renderer-alignment-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>renderer-alignment</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-renderer-alignment">check-renderer-alignment</a> · <code>resolved-comparison</code>, <code>renderer-input</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-2-typed-results" class="roadmap-criterion-summary"><th scope="row">Typed results</th><td><p class="roadmap-condition">Library and actual CLI results agree against independent expectations; library errors preserve causes without printing, exiting, or panicking.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for typed results" aria-expanded="true" aria-controls="structural-checking-2-typed-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-typed-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>typed-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#check-shared-typed-results">check-shared-typed-results</a> · <code>equivalent-callers</code>, <code>library-boundary</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-2-truthful-results" class="roadmap-criterion-summary"><th scope="row">Truthful results</th><td><p class="roadmap-condition">Verify documented exit statuses, incomplete and mixed failures, blocked checks, evaluated scope, and stdout/stderr behavior.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for truthful results" aria-expanded="true" aria-controls="structural-checking-2-truthful-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-truthful-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>truthful-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-truthful-results">check-truthful-results</a> · <code>outcomes-and-precedence</code>, <code>blocked-evaluation</code>, <code>report-scope</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-2-diagnostics" class="roadmap-criterion-summary"><th scope="row">Diagnostics</th><td><p class="roadmap-condition">For each seeded format defect, assert that findings identify the affected location, expected and observed structure, and useful repair guidance without weakening requirements. Verify documented fields, stable order, and readable output without styling; do not invent unavailable observations.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for diagnostics" aria-expanded="true" aria-controls="structural-checking-2-diagnostics-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-diagnostics-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>diagnostics</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-actionable-diagnostics">check-actionable-diagnostics</a> · <code>diagnostic-content</code>, <code>stable-presentation</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-2-observation-only" class="roadmap-criterion-summary"><th scope="row">Observation only</th><td><p class="roadmap-condition">Passing, failing, and incomplete checks preserve contents and directory entries; execution traps establish no subprocess, network, preprocessor, or verifier activity.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for observation only" aria-expanded="true" aria-controls="structural-checking-2-observation-only-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-observation-only-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>observation-only</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code>, <code>no-execution</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-2-acceptance-cases" class="roadmap-criterion-summary"><th scope="row">Acceptance cases</th><td><p class="roadmap-condition">Record results for the design’s <a href="../design/structural-checking.html#acceptance-against-this-book">acceptance cases</a>, including alternate layouts and deliberate failures.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for acceptance cases" aria-expanded="true" aria-controls="structural-checking-2-acceptance-cases-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-acceptance-cases-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>acceptance-cases</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — acceptance suite record</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-2-evidence" class="roadmap-criterion-details"><td colspan="2">Pending; use the <a href="index.html#evidence-records">evidence format</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. Completion delivers a usable local checker with tested correction/recheck cases. Milestone 3 owns CI integration; milestone 4 owns the agent repair demonstration.</td></tr>
</tbody>
</table>
</div>

<a id="structural-checking-3"></a>

## Milestone 3. Continuous checks protect the current Book

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Changes to this repository receive independent engine validation and an acceptance check against its current Book before rendering.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Structural Checking — Milestone 2.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-3</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Extend the independent engine and supported-toolchain checks introduced by rendering with configuration and structural-rule tests.</label></li><li><label><input type="checkbox" disabled> Add a separate current-Book structural check and make rendering depend on successful engine checks and structural acceptance for the same revision. Reuse the existing rendered-output acceptance step and reconcile shared workflow gating with <a href="record-rendering.html#record-rendering-3">Record Rendering milestone 3</a>.</label></li><li><label><input type="checkbox" disabled> Exercise valid, structurally broken, and incomplete-input Book cases; failed or incomplete structural acceptance must prevent rendering while independent engine tests remain runnable.</label></li><li><label><input type="checkbox" disabled> Document local commands and record CI evidence for the implemented revision.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-3-independent-acceptance" class="roadmap-criterion-summary"><th scope="row">Independent acceptance</th><td><p class="roadmap-condition">With deliberately broken navigation and no rendered output, engine compilation and fixture tests pass while current-Book acceptance reports the known failure.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent acceptance" aria-expanded="true" aria-controls="structural-checking-3-independent-acceptance-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-independent-acceptance-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-acceptance</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>broken-book</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-3-ci-order" class="roadmap-criterion-summary"><th scope="row">CI order</th><td><p class="roadmap-condition">Record actual CI showing engine checks, structural Book acceptance, then rendering for a valid revision. Broken or unprocessable Book input produces useful findings and prevents rendering; independent engine tests remain runnable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for ci order" aria-expanded="true" aria-controls="structural-checking-3-ci-order-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-ci-order-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-order</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>ci-order</code> — Full</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-3-no-repair" class="roadmap-criterion-summary"><th scope="row">No repair</th><td><p class="roadmap-condition">The acceptance check performs no source repair; a valid Book renders after acceptance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for no repair" aria-expanded="true" aria-controls="structural-checking-3-no-repair-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-no-repair-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>no-repair</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: CI acceptance invocation</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="structural-checking-3-ci-evidence" class="roadmap-criterion-summary"><th scope="row">CI evidence</th><td><p class="roadmap-condition">Record the tested revision, actual workflow results, and local commands; local success alone is not CI evidence.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for ci evidence" aria-expanded="true" aria-controls="structural-checking-3-ci-evidence-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-ci-evidence-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-evidence</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — integration evidence</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-3-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-3-evidence" class="roadmap-criterion-details"><td colspan="2">Pending; use the <a href="index.html#evidence-records">evidence format</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
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
