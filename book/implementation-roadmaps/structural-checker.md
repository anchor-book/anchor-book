# Structural Checking

**Roadmap identifier:** `structural-checking`

**Status:** Proposed; implementation not started.

This roadmap follows [Engine Bootstrap](bootstrap.md). Bootstrap prepares the
Book, authority, anchors, working contract, and agreed implementation scope used
here. It extends the crate established in [Record Rendering — Milestone 1](record-rendering.md#record-rendering-1).
The planned delivery order brings rendering first, but checker execution never
depends on rendered output. Review the governing records before marking this
implementation ready.

## Goal

An agent can use the checker's feedback to bring a Book into conformance with
its configured structure: run the check, understand the findings, make appropriate
changes within its assignment, and recheck until the structure passes.

The checker supplies information through a CLI and a reusable Rust library.
It stays read-only: it does not render the Book, modify files, fetch dependencies,
or run project tests. In this first increment, the agent makes the repairs.
Later engine operations may support those changes, but are not required here.

A pass must result from correcting the structure according to the intended
configuration and rules, not from bypassing checks or weakening the requirements.

The design is defined in:

- [Rust Engine](../design/rust-engine.md)
- [Configuration](../design/configuration.md)
- [Structural Checking](../design/structural-checking.md)
- [Checking Diagnostics](../design/check-diagnostics.md)

The roadmap sequences that design without maintaining a second configuration
schema or rule specification.

## Authority and anchors

The initial [requirements](../authority/requirements.md#initial-checker-requirements),
[design decisions](../authority/design-decisions.md#initial-checker-decisions),
and anchors below are drafts for review. Their links establish traceability,
not acceptance or implementation evidence. Each anchor carries its governing
authority and verification criteria in the [structured format](../design/anchor-definitions.md).

Each milestone’s completion criteria below identify the exact anchor criteria
it addresses and whether the planned coverage is full or partial.

Some criteria span milestones. Completing one milestone does not establish every
criterion of an associated anchor. Record parsing and authority resolution now
belong to the preceding rendering prototype. They do not become additional rules
of the initial layout/navigation check. Evidence aggregation remains later work.

## Milestones

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#structural-checking-1) | The engine understands this Book's configured layout | Proposed | Engine Bootstrap complete and Record Rendering — Milestone 1 |
| [2](#structural-checking-2) | Developers receive actionable structural findings | Proposed | [1](#structural-checking-1) |
| [3](#structural-checking-3) | Continuous checks protect the current Book | Proposed | [2](#structural-checking-2) |
| [4](#structural-checking-4) | An agent can repair Book structure using checker feedback | Proposed | [2](#structural-checking-2) |

Milestones 3 and 4 can proceed independently after milestone 2. All four are
required to complete this roadmap; milestone 4 demonstrates the agent workflow.

<a id="structural-checking-1"></a>

## Milestone 1. The engine understands this Book's configured layout

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Rust code can resolve this project&#x27;s Book configuration and reject invalid inputs independently of rendering or creating Book files.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="bootstrap.html">Engine Bootstrap</a> complete and <a href="record-rendering.html#record-rendering-1">Record Rendering — Milestone 1</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Extend the prototype&#x27;s library/CLI package; confirm or update Rust/dependency versions for configuration and navigation checking.</label></li><li><label><input type="checkbox" disabled> Implement typed configuration parsing, validation, and path resolution.</label></li><li><label><input type="checkbox" disabled> Validate and expose the configured project phase without changing structural rules or claiming automated design-readiness or completion checks.</label></li><li><label><input type="checkbox" disabled> Add the real configuration for this repository once the reader works.</label></li><li><label><input type="checkbox" disabled> Exercise other layouts, invocation directories, and invalid inputs.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-1-book-selection" class="roadmap-criterion-summary"><th scope="row">Book selection</th><td><p class="roadmap-condition">Resolve the actual Book and equivalent docs/ and md/ layouts correctly, including different invocation directories and display titles.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for book selection" aria-expanded="true" aria-controls="structural-checking-1-book-selection-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-book-selection-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>book-selection</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-book-selection">check-book-selection</a> · <code>invocation-directory</code>, <code>alternative-layout</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-1-configuration-validation" class="roadmap-criterion-summary"><th scope="row">Configuration validation</th><td><p class="roadmap-condition">Reject invalid fields, versions, phases, mappings, escapes, overlaps, and symlink aliases; allow a resolvable separate nonexistent output and distinguish missing from unresolvable paths.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for configuration validation" aria-expanded="true" aria-controls="structural-checking-1-configuration-validation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-configuration-validation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>configuration-validation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/configuration.html#check-configuration-validity">check-configuration-validity</a> · <code>schema-rejection</code>, <code>path-boundaries</code>, <code>missing-paths</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-1-configuration-preservation" class="roadmap-criterion-summary"><th scope="row">Configuration preservation</th><td><p class="roadmap-condition">Configuration operations preserve files on success and failure.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for configuration preservation" aria-expanded="true" aria-controls="structural-checking-1-configuration-preservation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-configuration-preservation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>configuration-preservation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: configuration operations only</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-1-independent-build" class="roadmap-criterion-summary"><th scope="row">Independent build</th><td><p class="roadmap-condition">Cargo build, formatting, Clippy, and configuration/API tests pass without rendering or validating the current Book.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent build" aria-expanded="true" aria-controls="structural-checking-1-independent-build-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-1-independent-build-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-build</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>broken-book</code> — Partial: engine build and fixture tests; current-Book checking follows</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-1-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. No operational checker is claimed by this milestone.</td></tr>
</tbody>
</table>
</div>

<a id="structural-checking-2"></a>

## Milestone 2. Developers receive actionable structural findings

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">A developer can use the library or CLI to identify structural defects in this Book and distinguish those findings from an incomplete evaluation.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Structural Checking — Milestone 1.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/structural-checking.html">Checker design</a>, <a href="../design/configuration.html">Configuration</a>, <a href="../design/rust-engine.html">Rust Engine</a>, and <a href="../design/check-diagnostics.html">Diagnostics</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>structural-checking-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Read renderer settings, navigation, and chapter metadata without executing preprocessors or creating missing files.</label></li><li><label><input type="checkbox" disabled> Evaluate the initial structural rules against those observations.</label></li><li><label><input type="checkbox" disabled> Expose typed findings and evaluated scope through the library and CLI, including the affected location, expected structure, and useful next action.</label></li><li><label><input type="checkbox" disabled> Explain missing prerequisites and incomplete evaluation so an agent knows whether it needs to repair structure or resolve an input/operational problem.</label></li><li><label><input type="checkbox" disabled> Exercise the current Book and independently specified failure cases.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-2-navigation" class="roadmap-criterion-summary"><th scope="row">Navigation</th><td><p class="roadmap-condition">The actual Book passes; independent fixtures cover chapter targets, required roles, and supported and unsupported navigation.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for navigation" aria-expanded="true" aria-controls="structural-checking-2-navigation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-navigation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>navigation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-navigation-conformance">check-navigation-conformance</a> · <code>chapter-inventory</code>, <code>required-roles</code>, <code>grammar-boundary</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-2-renderer-alignment" class="roadmap-criterion-summary"><th scope="row">Renderer alignment</th><td><p class="roadmap-condition">Report renderer drift without repair; handle defaults, equivalent paths, missing configuration, and malformed or unreadable input.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for renderer alignment" aria-expanded="true" aria-controls="structural-checking-2-renderer-alignment-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-renderer-alignment-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>renderer-alignment</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-renderer-alignment">check-renderer-alignment</a> · <code>resolved-comparison</code>, <code>renderer-input</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-2-typed-results" class="roadmap-criterion-summary"><th scope="row">Typed results</th><td><p class="roadmap-condition">Library and actual CLI results agree against independent expectations; library errors preserve causes without printing, exiting, or panicking.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for typed results" aria-expanded="true" aria-controls="structural-checking-2-typed-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-typed-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>typed-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/rust-engine.html#check-shared-typed-results">check-shared-typed-results</a> · <code>equivalent-callers</code>, <code>library-boundary</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-2-truthful-results" class="roadmap-criterion-summary"><th scope="row">Truthful results</th><td><p class="roadmap-condition">Verify documented exit statuses, incomplete and mixed failures, blocked checks, evaluated scope, and stdout/stderr behavior.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for truthful results" aria-expanded="true" aria-controls="structural-checking-2-truthful-results-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-truthful-results-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>truthful-results</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-truthful-results">check-truthful-results</a> · <code>outcomes-and-precedence</code>, <code>blocked-evaluation</code>, <code>report-scope</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-2-diagnostics" class="roadmap-criterion-summary"><th scope="row">Diagnostics</th><td><p class="roadmap-condition">Verify diagnostic fields, locations, next actions, stable order, and readable output without styling.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for diagnostics" aria-expanded="true" aria-controls="structural-checking-2-diagnostics-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-diagnostics-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>diagnostics</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-actionable-diagnostics">check-actionable-diagnostics</a> · <code>diagnostic-content</code>, <code>stable-presentation</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-2-observation-only" class="roadmap-criterion-summary"><th scope="row">Observation only</th><td><p class="roadmap-condition">Passing, failing, and incomplete checks preserve contents and directory entries; execution traps establish no subprocess, network, preprocessor, or verifier activity.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for observation only" aria-expanded="true" aria-controls="structural-checking-2-observation-only-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-observation-only-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>observation-only</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code>, <code>no-execution</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-2-acceptance-cases" class="roadmap-criterion-summary"><th scope="row">Acceptance cases</th><td><p class="roadmap-condition">Record results for the design’s <a href="../design/structural-checking.html#acceptance-against-this-book">acceptance cases</a>, including alternate layouts and deliberate failures.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for acceptance cases" aria-expanded="true" aria-controls="structural-checking-2-acceptance-cases-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-2-acceptance-cases-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>acceptance-cases</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — acceptance suite record</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-2-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. This result establishes local checking; Structural Checking — Milestone 3 owns continuous integration.</td></tr>
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
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Extend the independent engine and supported-toolchain checks introduced by rendering with configuration and structural-rule tests.</label></li><li><label><input type="checkbox" disabled> Add a separate current-Book check followed by mdBook rendering.</label></li><li><label><input type="checkbox" disabled> Exercise successful and deliberately broken Book cases.</label></li><li><label><input type="checkbox" disabled> Document local commands and record CI evidence for the implemented revision.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-3-independent-acceptance" class="roadmap-criterion-summary"><th scope="row">Independent acceptance</th><td><p class="roadmap-condition">With deliberately broken navigation and no rendered output, engine compilation and fixture tests pass while current-Book acceptance reports the known failure.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent acceptance" aria-expanded="true" aria-controls="structural-checking-3-independent-acceptance-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-independent-acceptance-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-acceptance</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>broken-book</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-3-ci-order" class="roadmap-criterion-summary"><th scope="row">CI order</th><td><p class="roadmap-condition">Record actual CI showing engine checks, Book acceptance, then rendering for a valid revision; a broken Book fails acceptance usefully.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for ci order" aria-expanded="true" aria-controls="structural-checking-3-ci-order-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-ci-order-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-order</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/system-overview.html#check-independent-build">check-independent-build</a> · <code>ci-order</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-3-no-repair" class="roadmap-criterion-summary"><th scope="row">No repair</th><td><p class="roadmap-condition">The acceptance check performs no source repair; a valid Book renders after acceptance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for no repair" aria-expanded="true" aria-controls="structural-checking-3-no-repair-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-no-repair-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>no-repair</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: CI acceptance invocation</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-3-ci-evidence" class="roadmap-criterion-summary"><th scope="row">CI evidence</th><td><p class="roadmap-condition">Record the tested revision, actual workflow results, and local commands; local success alone is not CI evidence.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for ci evidence" aria-expanded="true" aria-controls="structural-checking-3-ci-evidence-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-3-ci-evidence-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>ci-evidence</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — integration evidence</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-3-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-3-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
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
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Prepare disposable copies of this Book with known structural defects and independently specified expected findings and valid repairs.</label></li><li><label><input type="checkbox" disabled> Give an agent the Book, the checker, and an explicit repair assignment; have it check, interpret the findings, edit the permitted files, and recheck.</label></li><li><label><input type="checkbox" disabled> Include missing chapters, misplaced required sections, and renderer-setting drift. Exercise an invalid-input case that must be resolved before evaluation.</label></li><li><label><input type="checkbox" disabled> Improve unclear diagnostics or workflow guidance and repeat affected cases.</label></li><li><label><input type="checkbox" disabled> Record the initial defects, findings, edits, final result, and any human help.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="structural-checking-4-agent-repair" class="roadmap-criterion-summary"><th scope="row">Agent repair</th><td><p class="roadmap-condition">Recorded agent runs repair seeded defects and reach a passing check. Independent review confirms intended repairs, unrelated content preservation, no weakened requirements, and any human assistance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent repair" aria-expanded="true" aria-controls="structural-checking-4-agent-repair-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-4-agent-repair-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-repair</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-actionable-diagnostics">check-actionable-diagnostics</a> · <code>agent-repair</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-4-separate-edits" class="roadmap-criterion-summary"><th scope="row">Separate edits</th><td><p class="roadmap-condition">Each checker invocation remains read-only; agent edits are separately identifiable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for separate edits" aria-expanded="true" aria-controls="structural-checking-4-separate-edits-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-4-separate-edits-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>separate-edits</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-observation-only">check-observation-only</a> · <code>file-preservation</code> — Partial: invocations during the agent workflow</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="structural-checking-4-unresolved-cases" class="roadmap-criterion-summary"><th scope="row">Unresolved cases</th><td><p class="roadmap-condition">Cases outside the assignment or the agent’s ability produce an explicit explanation rather than a false completion claim; record versions and assistance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for unresolved cases" aria-expanded="true" aria-controls="structural-checking-4-unresolved-cases-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="structural-checking-4-unresolved-cases-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>unresolved-cases</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/check-diagnostics.html#check-truthful-results">check-truthful-results</a> · <code>report-scope</code> — Partial: reported scope and limits during the workflow</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="structural-checking-4-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="structural-checking-4-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

## Updating and completing the work

Use disposable copies for deliberate failures and public-safe reusable fixtures.
Exercise the actual Book as each capability becomes usable; Structural Checking — Milestone 3 is not the first
self-use experiment. Do not add success-returning placeholder commands.

Update task progress and remaining work in the Book during implementation.
When all completion criteria hold, mark the milestone Complete and replace its
pending record with the revision, checks, results, and integration context.
Reconcile design changes before claiming completion against revised criteria.

This roadmap is complete when all four milestones are complete: the current
Book and independent fixtures validate the checker, CI exercises it, and recorded
agent runs demonstrate the check, repair, and recheck loop. A successful structural
check does not establish full anchor verification or completion of other roadmaps.
Materialization and contract generation remain separate later work.
