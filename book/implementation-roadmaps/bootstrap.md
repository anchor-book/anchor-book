# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

**Status:** In progress. Design and working-contract drafts are in place;
review and agent-readiness checks remain.

## Goal

Prepare this Book so an engineer can assign structural-checker implementation
and an agent can understand the intended behavior, applicable requirements and
decisions, anchors, and completion criteria before starting.

The Book needs a reviewed checker design, an agreed implementation roadmap, and
a working contract that guides agents to read and use that context. Bootstrap
completes when both milestones demonstrate that readiness. A working checker is
the outcome of the [Structural Checking roadmap](structural-checker.md);
materialization and engine-generated contracts are later work.

[Record Rendering](record-rendering.md) proceeds alongside this preparation under
its own scoped design and roadmap. Its first two milestones are complete locally;
authority-criterion coverage and CI integration remain in milestone 3.

The [System Overview](../design/system-overview.md) provides system context;
the [AI Working Contract](../ai-working-contract.md) defines how agents work.

<a id="engine-bootstrap-1"></a>

## Milestone 1. The Book defines the checker and the agreed work to build it

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">The Book contains an agreed checker design, its governing authority and anchors, and a roadmap for building and demonstrating it.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">In progress</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">current design drafts.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/system-overview.html">System Overview</a> and <a href="../ai-working-contract.html">AI Working Contract</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>engine-bootstrap-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled checked> Draft the system and Rust engine design, configuration, navigation rules, and diagnostic behavior, with component and sequence diagrams.</label></li><li><label><input type="checkbox" disabled checked> Draft governing requirements and design decisions in Authority.</label></li><li><label><input type="checkbox" disabled checked> Define a canonical structured format and draft the initial checker anchors, grounding them in authority and linking them to verification criteria and implementation work.</label></li><li><label><input type="checkbox" disabled checked> Draft outcome-based checker milestones with stable identifiers, dependencies, and completion criteria.</label></li><li><label><input type="checkbox" disabled> Review the checker design, governing records, and verification criteria with the engineer.</label></li><li><label><input type="checkbox" disabled> Review the crate and record model established by the rendering prototype; settle the additional APIs, dependencies, and supported tool versions needed for structural checking.</label></li><li><label><input type="checkbox" disabled> Reconcile and agree on the <a href="structural-checker.html">Structural Checking</a> chunks with those choices.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="engine-bootstrap-1-traceable-design" class="roadmap-criterion-summary"><th scope="row">Traceable design</th><td><p class="roadmap-condition">The checker design and governing authority are reviewed; anchors are linked to the implementation roadmap and its completion criteria are testable.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for traceable design" aria-expanded="true" aria-controls="engine-bootstrap-1-traceable-design-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-1-traceable-design-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>traceable-design</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — design preparation</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="engine-bootstrap-1-agreed-scope" class="roadmap-criterion-summary"><th scope="row">Agreed scope</th><td><p class="roadmap-condition">The engineer agrees on the checker scope and roadmap. Implementation choices are resolved or explicitly deferred outside that scope.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agreed scope" aria-expanded="true" aria-controls="engine-bootstrap-1-agreed-scope-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-1-agreed-scope-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agreed-scope</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — work agreement</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Progress</th><td colspan="1">The Book contains the design drafts, six checker requirements, four governing engine decisions, and nine checker anchors <a href="structural-checker.html#authority-and-anchors">mapped to implementation work</a>. The current design baseline is <code>b9df14a</code>. Record parsing and rendering are implemented locally. The new authority traceability requirement and Rust Engine anchor are covered by the <a href="record-rendering.html#record-rendering-3-authority-coverage">rendering follow-up</a>; they do not expand the first structural check. Local Book builds and record/link checks have passed. These checks do not establish checker implementation or acceptance of the governing records. Completed rendering evidence remains in its own roadmap.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="engine-bootstrap-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="engine-bootstrap-1-evidence" class="roadmap-criterion-details"><td colspan="2">Pending; use the <a href="index.html#evidence-records">evidence format</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending review and scope agreement. The rendering prototype has established the initial crate and parser choices; checker-specific choices remain open.</td></tr>
</tbody>
</table>
</div>

<a id="engine-bootstrap-2"></a>

## Milestone 2. An agent can use the Book to take up assigned checker work

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">An engineer can assign checker work to an agent that has read the Book and can identify the applicable design, authority, anchors, and completion criteria before implementing it.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="#engine-bootstrap-1">Engine Bootstrap — Milestone 1</a> and the existing working contract. Startup experiments can begin earlier; final acceptance uses the reviewed Book.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/system-overview.html">System Overview</a> and <a href="../ai-working-contract.html">AI Working Contract</a></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>engine-bootstrap-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Exercise startup in fresh Codex and Claude Code sessions.</label></li><li><label><input type="checkbox" disabled> Assign a readiness walkthrough of a checker milestone: observe contract and Book retrieval and identification of its scope and governing content, without beginning implementation.</label></li><li><label><input type="checkbox" disabled> Include a scoped design-change exercise: observe whether the agent updates the affected roadmap tasks, criteria, references, and evidence limits in the same change.</label></li><li><label><input type="checkbox" disabled> Refine instructions where the agent misses context, fails to reconcile the roadmap, or starts unassigned work.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="engine-bootstrap-2-agent-startup" class="roadmap-criterion-summary"><th scope="row">Agent startup</th><td><p class="roadmap-condition">Fresh Codex and Claude Code sessions demonstrably read the current contract and reviewed Book; record versions, configuration, and observations.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent startup" aria-expanded="true" aria-controls="engine-bootstrap-2-agent-startup-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-agent-startup-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-startup</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — workflow readiness</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="engine-bootstrap-2-assigned-scope" class="roadmap-criterion-summary"><th scope="row">Assigned scope</th><td><p class="roadmap-condition">Each agent identifies the assigned checker milestone, governing design, authority, anchors, and completion criteria without beginning unassigned implementation.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for assigned scope" aria-expanded="true" aria-controls="engine-bootstrap-2-assigned-scope-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-assigned-scope-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>assigned-scope</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — workflow readiness</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="engine-bootstrap-2-design-reconciliation" class="roadmap-criterion-summary"><th scope="row">Design reconciliation</th><td><p class="roadmap-condition">A fresh-session exercise changes a design criterion and updates the affected roadmap in the same change. The agent identifies stale evidence and proposed follow-up work without claiming completion or starting unassigned implementation.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for design reconciliation" aria-expanded="true" aria-controls="engine-bootstrap-2-design-reconciliation-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-design-reconciliation-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>design-reconciliation</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — agent workflow readiness</p>
<p><strong>Evidence:</strong> Pending fresh-session demonstration; written contract guidance is preparation only.</p>
</td></tr>
<tr id="engine-bootstrap-2-readiness-record" class="roadmap-criterion-summary"><th scope="row">Readiness record</th><td><p class="roadmap-condition">Record limitations and any instruction refinements and repeat affected cases. This demonstrates readiness, not a working checker.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for readiness record" aria-expanded="true" aria-controls="engine-bootstrap-2-readiness-record-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-readiness-record-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>readiness-record</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — workflow readiness</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Preparation already done</th><td colspan="1"><code>AGENTS.md</code> directs agents to the manually maintained <a href="../ai-working-contract.html">AI Working Contract</a>, which requires reading the Book before taking up assigned work. The contract now covers the goal system, authority and anchors, project phases, roadmap creation, scoped implementation, progress updates, anchor placement, authority reasoning, and design-to-roadmap reconciliation.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="engine-bootstrap-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="engine-bootstrap-2-evidence" class="roadmap-criterion-details"><td colspan="2">Pending; use the <a href="index.html#evidence-records">evidence format</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. Fresh-session startup and the assigned checker readiness walkthrough have not been demonstrated. The preparation above does not establish that agents follow the instructions successfully.</td></tr>
</tbody>
</table>
</div>

## Handoff to Structural Checking

After both milestones complete, the engineer can assign
[Structural Checking](structural-checker.md) work. Bootstrap completion does not
itself assign that work. Review the drafted anchors and their criteria with the
[canonical format](../design/anchor-definitions.md); cross-Book authority is not
needed for this local design.

## Scope changes and reserved identifiers

The earlier draft included engine capabilities that belong after preparation.
Their milestone identifiers are retired here and will not be reused:

<a id="engine-bootstrap-3"></a>

- `engine-bootstrap-3`: checker implementation is now wholly described by the
  Structural Checking roadmap, which follows Bootstrap.

<a id="engine-bootstrap-4"></a>

- `engine-bootstrap-4`: materialization is deferred to a later roadmap.

<a id="engine-bootstrap-5"></a>

- `engine-bootstrap-5`: engine-generated contract delivery is deferred to a later
  roadmap. Bootstrap uses the existing manually maintained contract.

Automated anchor reporting, verification evidence, cross-Book authority, and
release preparation also remain outside this roadmap. Cross-Book authority is
not needed to define local design anchors or to build the first checker.
