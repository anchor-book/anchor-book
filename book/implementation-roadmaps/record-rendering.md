# Record Rendering

**Roadmap identifier:** `record-rendering`

**Status:** Milestone 1 complete locally; milestones 2 and 3 proposed.

## Goal

Read and review this Book's anchors, requirements, and decisions as formatted,
linked content while keeping their structured TOML as the source. Use the
prototype to exercise and refine the schemas during Prototyping.

This is the first engine implementation increment. It can proceed alongside
[Engine Bootstrap](bootstrap.md), which still prepares the Book for checker
implementation. Prototype work does not require the whole checker design or
bootstrap readiness exercise to be complete. The engineer has selected this
direction and assigned milestone 1. Milestones 2 and 3 remain proposed work.

## Design basis

- [Record Rendering](../design/record-rendering.md) defines the adapter, reader
  presentation, validation behavior, and source preservation.
- [Anchor Definitions](../design/anchor-definitions.md) and
  [Authority Definitions](../design/authority-definitions.md) define the current schemas.
- [Rust Engine](../design/rust-engine.md) supplies package and API boundaries.
- [Readable records requirement](../authority/requirements.md#requirement-readable-records)
  and [shared rendering decision](../authority/design-decisions.md#decision-shared-record-rendering)
  govern [rendered records](../design/record-rendering.md#render-records-in-context)
  and [validated inventory](../design/record-rendering.md#render-validated-records).

All records remain drafts during this prototype. Assignment of an experiment
does not automatically accept its design or prove its properties.

## Preparation already done

The rendering design, component and sequence diagrams, presentation rules,
failure behavior, and acceptance criteria are drafted. One requirement, one
design decision, and two anchors establish the prototype's intended behavior.
The milestones below record implementation progress, and the checker roadmap
builds on the shared crate established by this prototype.

The Book builds locally with these drafts. The Rust library now reads and
validates its records. Records still display as TOML; the adapter and renderer
remain to be built. Toolchain, dependencies, and API boundaries are documented
in [Rust Engine](../design/rust-engine.md#implemented-record-api).

## Module organization follow-up

Following milestone 1, the record implementation is grouped under the `records`
capability with private model, Markdown, schema, inventory, and diagnostic modules.
The public capability namespace is `anchor_book::records`; initial crate-root
imports remain supported. This refactor prepares the boundary for milestone 2's
separate rendering and mdBook adapter code. It does not implement that hook.

The module refactor was verified locally before amending this milestone commit:
independent tests and Clippy pass on Rust 1.85 and 1.90; the current-Book acceptance
test, API documentation, formatting, and Book build/link checks pass. The refactor
is included in this commit; push and remote verification are pending. Record
behavior and the milestone's scope are unchanged.

## Milestones

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#record-rendering-1) | Engine code can read and validate the Book's records | Complete locally | Current schema drafts and agreed prototype scope |
| [2](#record-rendering-2) | Readers can understand and navigate the rendered records | Proposed | Milestone 1 |
| [3](#record-rendering-3) | Local preview and CI use the same rendering path | Proposed | Milestone 2 |

<a id="record-rendering-1"></a>

## Milestone 1. Engine code can read and validate the Book's records

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">A Rust library constructs a typed record inventory and reports invalid definitions and relationships independently of rendering.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Complete — local implementation and verification</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">current schemas and agreement on prototype scope.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="#design-basis">Rendering design and governing records</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>record-rendering-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled checked> Establish the single library/CLI crate. Choose and record the edition, supported Rust version, mdBook protocol version, and focused parser dependencies.</label></li><li><label><input type="checkbox" disabled checked> Implement Markdown block recognition with offsets, strict TOML decoding, typed records, global identity indexing, and authority-reference validation.</label></li><li><label><input type="checkbox" disabled checked> Return located diagnostics and derive backlinks from the same inventory.</label></li><li><label><input type="checkbox" disabled checked> Exercise the actual Book&#x27;s records and independent positive/negative fixtures.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="record-rendering-1-valid-inventory" class="roadmap-criterion-summary"><th scope="row">Valid inventory</th><td><p class="roadmap-condition">Read the full current inventory with expected kinds, fields, and edges; chapter order affects neither resolution nor diagnostic ordering.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for valid inventory" aria-expanded="true" aria-controls="record-rendering-1-valid-inventory-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-1-valid-inventory-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>valid-inventory</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — inventory acceptance</p>
<p><strong>Evidence:</strong> Passed: the separate <code>current_book</code> acceptance test reads all chapters listed in SUMMARY and checks 24 records (11 anchors, 7 requirements, 6 decisions), their expected identities, 33 authority edges, criterion IDs, backlinks, source ranges, and chapter-order independence. Independent fixtures check every record kind and field. See the milestone evidence for the tested work and commands.</p>
</td></tr>
<tr id="record-rendering-1-invalid-inventory" class="roadmap-criterion-summary"><th scope="row">Invalid inventory</th><td><p class="roadmap-condition">Reject the canonical-format violations and invalid relationships specified by the anchor; literal examples are not collected.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for invalid inventory" aria-expanded="true" aria-controls="record-rendering-1-invalid-inventory-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-1-invalid-inventory-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>invalid-inventory</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-validated-records">render-validated-records</a> · <code>invalid-inventory</code> — Full</p>
<p><strong>Evidence:</strong> Passed: <code>tests/records.rs</code> rejects noncanonical fences, schema violations, duplicate identities, wrong-kind, missing, invalid, ambiguous, repeated, and incompatible-status references. Literal code examples are excluded. Tests check stable diagnostics, parser causes, Unicode positions, and truncation safety. This covers record validation; it does not verify rendering or the adapter.</p>
</td></tr>
<tr id="record-rendering-1-independent-build" class="roadmap-criterion-summary"><th scope="row">Independent build</th><td><p class="roadmap-condition">Compile and run fixture tests with an invalid or unrendered Book; formatting, Clippy, and library tests pass.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent build" aria-expanded="true" aria-controls="record-rendering-1-independent-build-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-1-independent-build-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-build</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-validated-records">render-validated-records</a> · <code>protocol-and-independence</code> — Partial: compilation and fixture-test independence</p>
<p><strong>Evidence:</strong> Passed: formatting on Rust 1.90; Clippy, independent tests, and documentation tests on Rust 1.85 and 1.90. <code>scripts/check-engine-independence.sh</code> runs a temporary source copy with no Book, then an invalid Book: independent tests pass both times, while explicit Book acceptance fails as expected. No renderer or mdBook build is invoked.</p>
</td></tr>
<tr id="record-rendering-1-declared-scope" class="roadmap-criterion-summary"><th scope="row">Declared scope</th><td><p class="roadmap-condition">Document available operations without claiming a structural checker or configuration reader.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for declared scope" aria-expanded="true" aria-controls="record-rendering-1-declared-scope-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-1-declared-scope-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>declared-scope</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — delivery scope</p>
<p><strong>Evidence:</strong> Passed: Rust Engine and CONTRIBUTING describe the available library API and unsupported operations. Actual-process CLI tests verify help/version, exit status 2 for unsupported commands, and no misleading successful output. Configuration and structural checking are not implemented.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="record-rendering-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — local checks passed</button></td></tr>
<tr id="record-rendering-1-evidence" class="roadmap-criterion-details"><td colspan="2"><p><strong>Tested work:</strong> Record Rendering milestone 1 working tree based on <code>7f45b14</code>, verified 2026-10-20. Implementation, fixtures, workflow, and Book updates are included together in this milestone commit. Checks were run locally before committing; remote verification is pending.</p>
<p><strong>Checks:</strong> <code>cargo fmt --all -- --check</code>; <code>cargo clippy --all-targets --locked -- -D warnings</code>; <code>cargo test --locked</code>; <code>cargo test --locked --test current_book -- --ignored</code>; and <code>scripts/check-engine-independence.sh</code>. Toolchain checks cover Rust 1.85.0 and 1.90.0. Independent tests include 12 record tests, 2 CLI tests, and 1 documentation example; Book acceptance is a separate test.</p>
<p><strong>Result and supporting records:</strong> Local checks pass. Tests live in <code>tests/records.rs</code>, <code>tests/cli.rs</code>, and <code>tests/current_book.rs</code>; the isolation experiment is reproducible with the script above. The Engine workflow is configured for Rust 1.85 and stable.</p>
<p><strong>Limitations and integration:</strong> No remote Actions result yet. The mdBook adapter, rendered records, configuration reader, and structural checker remain unimplemented. This milestone establishes the parsing and validation portion of the rendering anchors, not their complete verification or design acceptance.</p></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Complete locally on 2026-10-20. The library reads this Book and independent fixtures, rejects invalid input with located diagnostics, and builds independently of Book validity. Push and remote verification are pending.</td></tr>
</tbody>
</table>
</div>

<a id="record-rendering-2"></a>

## Milestone 2. Readers can understand and navigate the rendered records

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">mdBook displays records in their chapters with readable content, status labels, permalinks, linked authorities, and derived backlinks.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Record Rendering — Milestone 1.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="#design-basis">Rendering design and governing records</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>record-rendering-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Add the thin <code>anchorbook md</code> protocol adapter and library rendering operation.</label></li><li><label><input type="checkbox" disabled> Style the three kinds, status labels, criteria, and expandable decision details.</label></li><li><label><input type="checkbox" disabled> Generate safe relative links and stable fragments, including print navigation.</label></li><li><label><input type="checkbox" disabled> Remove the temporary record targets, duplicate titles, and manually maintained relationship lists when their generated replacements are in place.</label></li><li><label><input type="checkbox" disabled> Exercise complete-input success, malformed protocol input, invalid records, and renderer capability probes through the actual CLI process.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="record-rendering-2-record-content" class="roadmap-criterion-summary"><th scope="row">Record content</th><td><p class="roadmap-condition">Every authored record appears once per chapter with expected fields and links; literal examples remain code.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for record content" aria-expanded="true" aria-controls="record-rendering-2-record-content-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-2-record-content-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>record-content</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-records-in-context">render-records-in-context</a> · <code>record-presentation</code> — Partial: inventory and field checks</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-2-accessible-presentation" class="roadmap-criterion-summary"><th scope="row">Accessible presentation</th><td><p class="roadmap-condition">Review light/dark themes, narrow layout, keyboard access, and printed details.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for accessible presentation" aria-expanded="true" aria-controls="record-rendering-2-accessible-presentation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-2-accessible-presentation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>accessible-presentation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-records-in-context">render-records-in-context</a> · <code>record-presentation</code> — Partial: visual and accessibility review</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-2-relationship-navigation" class="roadmap-criterion-summary"><th scope="row">Relationship navigation</th><td><p class="roadmap-condition">Links resolve across chapter depths, the project site prefix, and the combined print view.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for relationship navigation" aria-expanded="true" aria-controls="record-rendering-2-relationship-navigation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-2-relationship-navigation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>relationship-navigation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-records-in-context">render-records-in-context</a> · <code>relationship-navigation</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-2-source-preservation" class="roadmap-criterion-summary"><th scope="row">Source preservation</th><td><p class="roadmap-condition">Passing and failing preprocessing preserve sources; field text stays inert and unrelated prose and Mermaid diagrams retain their behavior.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for source preservation" aria-expanded="true" aria-controls="record-rendering-2-source-preservation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-2-source-preservation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>source-preservation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-records-in-context">render-records-in-context</a> · <code>preservation-and-escaping</code> — Full</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-2-adapter-protocol" class="roadmap-criterion-summary"><th scope="row">Adapter protocol</th><td><p class="roadmap-condition">Exercise the actual process with valid and malformed input and renderer probes. Success emits complete Book JSON; failure emits diagnostics and no successful partial Book.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for adapter protocol" aria-expanded="true" aria-controls="record-rendering-2-adapter-protocol-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-2-adapter-protocol-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>adapter-protocol</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-validated-records">render-validated-records</a> · <code>protocol-and-independence</code> — Partial: adapter protocol; independent build is addressed in milestone 1</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="record-rendering-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="record-rendering-2-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. Local rendering alone does not establish CI success.</td></tr>
</tbody>
</table>
</div>

<a id="record-rendering-3"></a>

## Milestone 3. Local preview and CI use the same rendering path

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Contributors can build and preview the readable Book, and the GitHub workflow builds the same output using the tested crate and locked dependencies.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Record Rendering — Milestone 2.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="#design-basis">Rendering design and governing records</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>record-rendering-3</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Configure <code>book.toml</code> and preprocessor ordering; document local setup and preview.</label></li><li><label><input type="checkbox" disabled> Extend the existing engine checks with record rendering in CI, preserving the existing Pages output path. Do not add a placeholder structural check.</label></li><li><label><input type="checkbox" disabled> Exercise a clean build and a live source-edit/rebuild cycle.</label></li><li><label><input type="checkbox" disabled> Record actual GitHub Actions results for the implemented revision when that revision is pushed through the engineer&#x27;s chosen contribution workflow.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="record-rendering-3-local-preview" class="roadmap-criterion-summary"><th scope="row">Local preview</th><td><p class="roadmap-condition">Clean builds and live source-edit/rebuild cycles use the record adapter successfully.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for local preview" aria-expanded="true" aria-controls="record-rendering-3-local-preview-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-3-local-preview-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>local-preview</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — contributor workflow</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-3-independent-ci" class="roadmap-criterion-summary"><th scope="row">Independent CI</th><td><p class="roadmap-condition">Engine compilation and fixture tests remain runnable with an invalid Book; the separate Book build reports invalid input usefully.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for independent ci" aria-expanded="true" aria-controls="record-rendering-3-independent-ci-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-3-independent-ci-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>independent-ci</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/record-rendering.html#render-validated-records">render-validated-records</a> · <code>protocol-and-independence</code> — Partial: independence and failure reporting in CI</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-3-published-build" class="roadmap-criterion-summary"><th scope="row">Published build</th><td><p class="roadmap-condition">Record a successful GitHub Actions run for the valid implementation revision using the same rendering path as local builds. A local pass is insufficient.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for published build" aria-expanded="true" aria-controls="record-rendering-3-published-build-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-3-published-build-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>published-build</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — CI integration</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="record-rendering-3-documented-limits" class="roadmap-criterion-summary"><th scope="row">Documented limits</th><td><p class="roadmap-condition">Document setup, supported versions, and schema/rendering limits. Pushing remains a separately directed action.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for documented limits" aria-expanded="true" aria-controls="record-rendering-3-documented-limits-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-rendering-3-documented-limits-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>documented-limits</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — contributor documentation</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="record-rendering-3-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="record-rendering-3-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

## Handoff and later work

Record what the prototype teaches us about authoring, presentation, and schema
changes. Reconcile these changes with the design and records during the work.
Do not mark a milestone complete solely because it renders the happy-path Book.

[Structural Checking](structural-checker.md) will extend the same crate with the
configuration reader and layout/navigation checks. It can reuse record-model
infrastructure where applicable without invoking the renderer. Materialization,
generated contracts, cross-Book authority, evidence pages, and generated progress
indexes remain separate increments. No existing checker milestone is completed
or renumbered by this roadmap.
