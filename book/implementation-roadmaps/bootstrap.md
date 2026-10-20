# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

**Status:** In progress. Design and working-contract drafts are in place;
review and agent-readiness checks remain.

## Goal

Prepare this Book to guide building the structural checker. It must contain
the checker design, governing requirements and decisions, initial design anchors,
an agreed implementation roadmap, and a working contract that enables agents
to read the Book and carry out assigned work.

Bootstrap prepares that working context. It does not implement the checker,
materialization, or engine-generated contracts.

The [Record Rendering prototype](record-rendering.md) is now the next engine
implementation increment. It makes this Book's records readable while bootstrap
preparation continues. Its own scoped design and roadmap govern that experiment;
it does not depend on completing all checker preparation. Bootstrap's goal and
milestone identifiers remain focused on readiness to build the checker.

The [System Overview](../design/system-overview.md) supplies the system context;
the [AI Working Contract](../ai-working-contract.md) defines how agents work.
Each milestone enables an outcome with tasks and evidence for completion.

## Already working: Book rendering and publishing

The initial Book was built locally and published through GitHub Actions and
Pages. Subsequent design, diagram, and contract changes have been checked
locally; these later revisions have not yet been verified on GitHub. Fresh
agent startup has not yet been tested. The engine is not implemented.

## Milestones

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#engine-bootstrap-1) | The Book defines the checker and the agreed work to build it | In progress | Current design drafts |
| [2](#engine-bootstrap-2) | An agent can use the Book to take up assigned checker work | Proposed | Milestone 1 and the working contract |

Bootstrap is complete when both milestones are complete: the engineer can use
this Book to assign checker implementation, and an agent can load the intended
behavior, governing authority, anchors, and completion criteria from it. A
working checker is the outcome of the subsequent Structural Checking roadmap,
not a prerequisite for completing Bootstrap.

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
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="engine-bootstrap-1-agreed-scope" class="roadmap-criterion-summary"><th scope="row">Agreed scope</th><td><p class="roadmap-condition">The engineer agrees on the checker scope and roadmap. Implementation choices are resolved or explicitly deferred outside that scope.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agreed scope" aria-expanded="true" aria-controls="engine-bootstrap-1-agreed-scope-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-1-agreed-scope-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agreed-scope</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — work agreement</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Progress</th><td colspan="1">The Book contains the design drafts, six checker requirements, five checker decisions, and nine checker anchors <a href="structural-checker.html#authority-and-anchors">mapped to implementation work</a>. The committed draft baseline is <code>f396bcf</code>; subsequent rendering-design changes bring record parsing ahead of checker implementation. Local Book builds and record/link checks have passed. These are documentation checks, not engine verification or acceptance of the governing records.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="engine-bootstrap-1-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="engine-bootstrap-1-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending review and scope agreement. The rendering prototype will establish the initial crate and parser choices; checker-specific choices remain open.</td></tr>
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
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks"><li><label><input type="checkbox" disabled> Exercise startup in fresh Codex and Claude Code sessions.</label></li><li><label><input type="checkbox" disabled> Assign a readiness walkthrough of a checker milestone: observe contract and Book retrieval and identification of its scope and governing content, without beginning implementation.</label></li><li><label><input type="checkbox" disabled> Refine instructions where the agent misses context or starts unassigned work.</label></li></ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="engine-bootstrap-2-agent-startup" class="roadmap-criterion-summary"><th scope="row">Agent startup</th><td><p class="roadmap-condition">Fresh Codex and Claude Code sessions demonstrably read the current contract and reviewed Book; record versions, configuration, and observations.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent startup" aria-expanded="true" aria-controls="engine-bootstrap-2-agent-startup-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-agent-startup-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-startup</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — workflow readiness</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="engine-bootstrap-2-assigned-scope" class="roadmap-criterion-summary"><th scope="row">Assigned scope</th><td><p class="roadmap-condition">Each agent identifies the assigned checker milestone, governing design, authority, anchors, and completion criteria without beginning unassigned implementation.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for assigned scope" aria-expanded="true" aria-controls="engine-bootstrap-2-assigned-scope-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-assigned-scope-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>assigned-scope</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — workflow readiness</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr id="engine-bootstrap-2-readiness-record" class="roadmap-criterion-summary"><th scope="row">Readiness record</th><td><p class="roadmap-condition">Record limitations and any instruction refinements and repeat affected cases. This demonstrates readiness, not a working checker.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for readiness record" aria-expanded="true" aria-controls="engine-bootstrap-2-readiness-record-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="engine-bootstrap-2-readiness-record-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>readiness-record</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — workflow readiness</p>
<p><strong>Evidence:</strong> Not recorded. Record the tested revision, checks, results, supporting records, limitations, and integration context here.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Preparation already done</th><td colspan="1"><code>AGENTS.md</code> directs agents to the manually maintained <a href="../ai-working-contract.html">AI Working Contract</a>, which requires reading the Book before taking up assigned work. The contract now covers the goal system, authority and anchors, project phases, roadmap creation, scoped implementation, and progress updates.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Evidence</th><td colspan="1"><button type="button" class="roadmap-toggle" aria-expanded="true" aria-controls="engine-bootstrap-2-evidence"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Milestone evidence — pending</button></td></tr>
<tr id="engine-bootstrap-2-evidence" class="roadmap-criterion-details"><td colspan="2">No milestone acceptance evidence recorded. See <a href="index.html#evidence-records">evidence fields</a> when recording results.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending. Fresh-session startup and the assigned checker readiness walkthrough have not been demonstrated. The preparation above does not establish that agents follow the instructions successfully.</td></tr>
</tbody>
</table>
</div>

## Handoff to Structural Checking

When Bootstrap is complete, the engineer can assign work from
[Structural Checking](structural-checker.md). That roadmap delivers the agent's
check, repair, and recheck loop. Bootstrap completion does not itself assign
or start that implementation.

The initial anchors are now drafted alongside the checker designs, grounded in
requirements and design decisions. Review their properties and criteria together
with the [canonical definition format](../design/anchor-definitions.md). Defining
those properties does not require an implemented anchor parser or cross-Book
authority.

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
