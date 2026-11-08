# Record Line Grammar

**Roadmap identifier:** `record-line-grammar`

## Goal

The engine and this Book use the goal [record syntax](../design/anchor-definitions.md#record-syntax),
a strict line grammar, as their only record format, with validation at least as
strict as today. Measure the syntax with agents first, so the design is confirmed
or revised on evidence before the migration.

## Gap

The engine and all of this Book's records use the earlier
[TOML form](../design/anchor-definitions.md#current-implementation). Searches
against this Book's sources show what that costs agents:

| Question | Result with TOML records |
| --- | --- |
| What cites `requirement-observation-only`? | 6 lines in three shapes: the definition, an anchor citation, a decision citation, and three lines from literal schema examples that look the same without reading the surrounding block |
| Which records are drafts? | 29 lines of `status = "draft"`, none stating the record's kind or identity |
| Which criteria require proof? | `verification = ["proof"]`, without its criterion or anchor identifier |

Proof scope is a writing convention inside a statement string that the engine
cannot check. The renderer needs no change beyond the inline-text
[open question](../design/anchor-definitions.md#open-questions): it replaces each
record's source range with HTML from the typed inventory and never reads TOML.

<a id="record-line-grammar-1"></a>

## Milestone 1. Measure how agents search each record syntax

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">Recorded measurements show how many searches, lines read, and wrong matches agents need to answer fixed record questions in the goal line grammar, the current TOML form, and TOML with kind, identity, and status on the fence line, so the goal syntax is confirmed or revised on evidence.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Engineer agreement on the task set, variants, and assessment below.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/anchor-definitions.html#record-syntax">Record syntax</a>, <a href="../design/authority-definitions.html">Authority Definitions</a>, and the <a href="#gap">gap</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>record-line-grammar-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Produce three disposable copies of this Book's records and literal examples: current TOML, the goal line grammar, and TOML with kind, identity, and status on the fence line. Convert mechanically and check each copy preserves every identity, field, reference, and method.</label></li>
<li><label><input type="checkbox" disabled> Fix the task set: find a record's definition; list every reference to a requirement; list draft anchors; list criteria requiring proof with their anchor; change one criterion's methods; add one authority reference.</label></li>
<li><label><input type="checkbox" disabled> For the four search tasks, record the minimal searches for each variant, the lines matched, wrong matches, and further lines read to answer.</label></li>
<li><label><input type="checkbox" disabled> Run the full task set in fresh Claude Code and Codex sessions against each variant, without engine tools. Record tool versions, tool calls, lines read, correctness, and edits that would fail validation.</label></li>
<li><label><input type="checkbox" disabled> Record results and limits, answer or carry forward the design's open questions, and revise the record syntax in the design if the results warrant it, for engineer review.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="record-line-grammar-1-faithful-variants" class="roadmap-criterion-summary"><th scope="row">Faithful variants</th><td><p class="roadmap-condition">Each variant represents the same records: a recorded comparison shows identical identities, kinds, statuses, fields, references, criteria, and methods across all three.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for faithful variants" aria-expanded="true" aria-controls="record-line-grammar-1-faithful-variants-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-line-grammar-1-faithful-variants-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>faithful-variants</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — experiment preparation</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="record-line-grammar-1-search-measurements" class="roadmap-criterion-summary"><th scope="row">Search measurements</th><td><p class="roadmap-condition">For every search task and variant, the recorded searches, matched lines, wrong matches, and further reads are reproducible from committed scripts or listed commands.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for search measurements" aria-expanded="true" aria-controls="record-line-grammar-1-search-measurements-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-line-grammar-1-search-measurements-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>search-measurements</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — design experiment; informs <a href="../design/anchor-definitions.html#records-searchable-lines">records-searchable-lines</a></p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="record-line-grammar-1-agent-trials" class="roadmap-criterion-summary"><th scope="row">Agent trials</th><td><p class="roadmap-condition">Fresh-session results for each agent and variant record tool calls, lines read, answer correctness, and edit validity, with versions and any human assistance. Inconclusive or failed sessions remain in the record.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent trials" aria-expanded="true" aria-controls="record-line-grammar-1-agent-trials-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-line-grammar-1-agent-trials-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-trials</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — design experiment</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="record-line-grammar-1-assessment" class="roadmap-criterion-summary"><th scope="row">Assessment</th><td><p class="roadmap-condition">A written assessment compares the variants on searches, reads, wrong matches, and edit validity, states its limits, and recommends confirming or revising the goal syntax. Any revision is made in the design chapters, not here.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for assessment" aria-expanded="true" aria-controls="record-line-grammar-1-assessment-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="record-line-grammar-1-assessment-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>assessment</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — decision input</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="record-line-grammar-2"></a>

## Milestone 2. The engine and this Book use the chosen syntax

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">The record reader, renderer, fixtures, and this Book use the goal record syntax as the only record format, with validation at least as strict as today.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">Record Line Grammar — Milestone 1, with any resulting design revision agreed.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/anchor-definitions.html">Anchor Definitions</a>, <a href="../design/authority-definitions.html">Authority Definitions</a>, <a href="../design/record-rendering.html">Record Rendering</a>, and <a href="../design/rust-engine.html">Rust Engine</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>record-line-grammar-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Replace the TOML record reader with the line scanner, keeping record types and diagnostic codes; decide and implement inline-field presentation.</label></li><li><label><input type="checkbox" disabled> Enforce the <a href="../design/anchor-definitions.html#identifiers">identifier forms</a> and the kind codes each reference field accepts. Rename every identifier to its kind-coded form in the same change, updating references, link fragments, evidence links, code comments, and workflow metadata that name them.</label></li>
<li><label><input type="checkbox" disabled> Migrate this Book's records, literal examples, and test fixtures in the same change, and reject TOML record blocks as unsupported syntax.</label></li>
<li><label><input type="checkbox" disabled> Repeat the authority proof's production-integration tests and review for the new conversion, and run mutation testing on the reader.</label></li>
<li><label><input type="checkbox" disabled> Define completion criteria against <a href="../design/anchor-definitions.html#records-searchable-lines">records-searchable-lines</a>, <a href="../design/anchor-definitions.html#records-every-line">records-every-line</a>, <a href="../design/anchor-definitions.html#records-named-identifiers">records-named-identifiers</a>, and the existing record anchors before the milestone is assigned.</label></li>
</ul></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>
