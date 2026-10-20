# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

**Status:** Proposed preparation work; milestone details remain subject to review.

## Goal

Prepare this Book to guide building the structural checker. It must contain
the checker design, governing requirements and decisions, initial design anchors,
an agreed implementation roadmap, and a working contract that enables agents
to read the Book and carry out assigned work.

Bootstrap prepares that working context. It does not implement the checker,
materialization, or engine-generated contracts.

The [System Overview](../design/system-overview.md) supplies the system context;
the [AI Working Contract](../ai-working-contract.md) defines how agents work.
Each milestone enables an outcome with tasks and evidence for completion.

## Already working: Book rendering and publishing

The initial Book was built locally and published through GitHub Actions and
Pages. Subsequent design, diagram, and contract changes have been checked
locally; their updated workflow has not yet been verified on GitHub. Fresh
agent startup has not yet been tested. The engine is not implemented.

## Milestones

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#engine-bootstrap-1) | The Book defines the checker and the agreed work to build it | Proposed | Current design drafts |
| [2](#engine-bootstrap-2) | An agent can use the Book to take up assigned checker work | Proposed | Milestone 1 and the working contract |

Bootstrap is complete when both milestones are complete: the engineer can use
this Book to assign checker implementation, and an agent can load the intended
behavior, governing authority, anchors, and completion criteria from it. A
working checker is the outcome of the subsequent Structural Checking roadmap,
not a prerequisite for completing Bootstrap.

<a id="engine-bootstrap-1"></a>

## Milestone 1. The Book defines the checker and the agreed work to build it

**Identifier:** `engine-bootstrap-1`

**Status:** Proposed. **Depends on:** current design drafts.

**Outcome:** The Book contains an agreed checker design, its governing authority
and anchors, and a roadmap for building and demonstrating it.

**Tasks:**

- Review configuration, navigation rules, and diagnostic behavior.
- Record governing requirements and design decisions in Authority.
- Define initial anchors for the current designs, grounding them in that authority
  and linking the relevant properties to the checker work.
- Settle the Rust package/API boundaries, initial dependencies, and supported
  tool versions needed for the first increment.
- Reconcile and review the [Structural Checking](structural-checker.md)
  chunks with those choices.

**Complete when:** the governing records and relevant design anchors are linked
from the checker roadmap, its completion criteria are testable, and the engineer
has agreed on its scope.
Unresolved choices that affect implementation are resolved or explicitly deferred
outside that scope.

**Completion record:** Pending. Initial requirements, decisions, and structured
checker anchors are drafted and [mapped to implementation work](structural-checker.md#authority-and-anchors).
They still need review. Dependencies, supported tool versions, and remaining API
choices are not settled; this milestone is not complete.

<a id="engine-bootstrap-2"></a>

## Milestone 2. An agent can use the Book to take up assigned checker work

**Identifier:** `engine-bootstrap-2`

**Status:** Proposed. **Depends on:** [Engine Bootstrap — Milestone 1](#engine-bootstrap-1)
and the existing working contract. Startup experiments can begin earlier; final
acceptance uses the reviewed Book.

**Outcome:** An engineer can assign checker work to an agent that has read the
Book and can identify the applicable design, authority, anchors, and completion
criteria before implementing it.

**Tasks:**

- Exercise startup in fresh Codex and Claude Code sessions.
- Assign a readiness walkthrough of a checker milestone: observe contract and
  Book retrieval and identification of its scope and governing content, without
  beginning implementation.
- Refine instructions where the agent misses context or starts unassigned work.

**Complete when:** each supported agent demonstrably reads the current contract
and reviewed Book, identifies the assigned checker scope and its governing
content, and distinguishes that assignment from unassigned work. Record the
tested versions/configuration, observations, and remaining limitations. This
checks readiness to use the Book, not whether the checker has been implemented.

**Completion record:** Pending. File presence alone is not completion evidence.

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
