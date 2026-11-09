# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

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
its own scoped design and roadmap.

The [System Overview](../design/system-overview.md) provides system context;
the [AI Working Contract](../ai-working-contract.md) defines how agents work.

## Milestone 1. The Book defines the checker and the agreed work to build it {#engine-bootstrap-1}

**Outcome:** The Book contains an agreed checker design, its governing authority and anchors, and a roadmap for building and demonstrating it.

**Status:** In progress

**Dependencies:** current design drafts.

**Design basis:** [System Overview](../design/system-overview.md) and [AI Working Contract](../ai-working-contract.md)

**Identifier:** `engine-bootstrap-1`

**Tasks**

- [x] Draft the system and Rust engine design, configuration, navigation rules, and diagnostic behavior, with component and sequence diagrams.
- [x] Draft governing requirements and decisions in Authority.
- [x] Define a canonical structured format and draft the initial checker anchors, grounding them in authority and linking them to verification criteria and implementation work.
- [x] Draft outcome-based checker milestones with stable identifiers, dependencies, and completion criteria.
- [ ] Review the checker anchors as guarantees supporting the check, repair, and recheck workflow; confirm their governing authority, retained constraints, and testable completion criteria with the engineer.
- [ ] Review the crate and record model established by the rendering prototype; settle the additional APIs, dependencies, and supported tool versions needed for structural checking.
- [ ] Reconcile and agree on the [Structural Checking](structural-checker.md) chunks with those choices.

### Completion criteria

#### Traceable design {#engine-bootstrap-1-traceable-design}

The checker design and governing authority are reviewed; anchors are linked to the implementation roadmap and its completion criteria are testable.

- **Anchor coverage:** None — design preparation
- **Evidence:** Pending.

#### Agreed scope {#engine-bootstrap-1-agreed-scope}

The engineer agrees on the checker scope and roadmap. Implementation choices are resolved or explicitly deferred outside that scope.

- **Anchor coverage:** None — work agreement
- **Evidence:** Pending.

### Progress

The Book contains the design drafts, six checker requirements, four governing engine decisions, and nine checker anchors [mapped to implementation work](structural-checker.md#authority-and-anchors). Book builds and record and link checks pass; they do not establish acceptance of the governing records.

### Evidence {#engine-bootstrap-1-evidence}

*Milestone evidence — pending*

Pending; use the [evidence format](index.md#evidence-records).

### Completion record

Pending. The engineer agreed the checker design as a prototyping starting point and assigned Structural Checking milestones as scoped prototypes. Review of the checker anchors and scope, and agreement on the full roadmap, remain.

## Milestone 2. An agent can use the Book to take up assigned checker work {#engine-bootstrap-2}

**Outcome:** An engineer can assign checker work to an agent that has read the Book and can identify the applicable design, authority, anchors, and completion criteria before implementing it.

**Status:** Proposed

**Dependencies:** [Engine Bootstrap — Milestone 1](#engine-bootstrap-1) and the existing working contract. Startup experiments can begin earlier; final acceptance uses the reviewed Book.

**Design basis:** [System Overview](../design/system-overview.md) and [AI Working Contract](../ai-working-contract.md)

**Identifier:** `engine-bootstrap-2`

**Tasks**

- [ ] Exercise startup in fresh Codex and Claude Code sessions.
- [ ] Assign a readiness walkthrough of a checker milestone: observe retrieval of the contract, project guidance, and Book, including identification of the current phase, authoring references, supported commands, assigned scope, and governing content, without beginning implementation.
- [ ] Include a scoped design-change exercise: observe whether the agent updates the affected roadmap tasks, criteria, references, and evidence limits in the same change.
- [ ] Refine instructions where the agent misses context, fails to reconcile the roadmap, or starts unassigned work.

### Completion criteria

#### Agent startup {#engine-bootstrap-2-agent-startup}

Fresh Codex and Claude Code sessions demonstrably read the reusable contract, project guidance, and reviewed Book, and identify the project phase and supported commands from project guidance; record versions, configuration, and observations.

- **Anchor coverage:** None — workflow readiness
- **Evidence:** Pending.

#### Assigned scope {#engine-bootstrap-2-assigned-scope}

Each agent identifies the assigned checker milestone, governing design, authority, anchors, and completion criteria without beginning unassigned implementation.

- **Anchor coverage:** None — workflow readiness
- **Evidence:** Pending.

#### Design reconciliation {#engine-bootstrap-2-design-reconciliation}

A fresh-session exercise changes a design criterion and updates the affected roadmap in the same change. The agent identifies stale evidence and proposed follow-up work without claiming completion or starting unassigned implementation.

- **Anchor coverage:** None — agent workflow readiness
- **Evidence:** Pending fresh-session demonstration; written contract guidance is preparation only.

#### Readiness record {#engine-bootstrap-2-readiness-record}

Record limitations and any instruction refinements and repeat affected cases. This demonstrates readiness, not a working checker.

- **Anchor coverage:** None — workflow readiness
- **Evidence:** Pending.

### Preparation already done

`AGENTS.md` identifies the reusable [AI Working Contract](../ai-working-contract.md), [project guidance](../working-on-anchor-book.md), and Book index. The contract defines the working method; project guidance supplies the phase, paths, tools, authoring references, and public-content rules. Fresh-session verification of that separation remains pending.

### Evidence {#engine-bootstrap-2-evidence}

*Milestone evidence — pending*

Pending; use the [evidence format](index.md#evidence-records).

### Completion record

Pending. Fresh-session startup and the assigned checker readiness walkthrough have not been demonstrated. The preparation above does not establish that agents follow the instructions successfully.

## Handoff to Structural Checking

The engineer has explicitly assigned Structural Checking milestone 1 as a scoped
prototype while Bootstrap review and fresh-session exercises remain open. This
assignment does not complete Bootstrap or the later checker milestones.

For the full readiness handoff, after both milestones complete the engineer can assign
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
