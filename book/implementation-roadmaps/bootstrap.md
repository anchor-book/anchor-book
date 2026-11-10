# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

## Goal

The checker's design rests on accepted governing records, and an agent starting a
fresh session can use the Book to take up an assigned milestone.

Bootstrap first prepared the Book so an engineer could assign structural-checker
work. The engineer then assigned the [Structural Checking](structural-checker.md)
milestones as scoped prototypes before that preparation was reviewed, and that
roadmap records the checker as built. Two parts of Bootstrap remain: the review
and acceptance of the checker's anchors, requirements, and decisions, which are
still drafts, and a demonstration that fresh agent sessions start from the
contract, project guidance, and Book.

This roadmap uses the contract as it is maintained in the Book today. Retrieving
it through `anchorbook contract` belongs to
[Project Integration milestone 1](project-integration.md#project-integration-1).
The [AI Working Contract](../ai-working-contract.md) defines how agents work.

## Milestone 1. The checker's governing records are reviewed and accepted {#engine-bootstrap-1}

**Outcome:** The engineer has reviewed the checker's anchors and the requirements and decisions they cite, and those records are accepted into the goal design.

**Status:** Complete — records reviewed and accepted; accepted by the engineer

**Dependencies:** None. Acceptance is a design judgment; the built checker and its evidence inform the review but do not establish it.

**Design basis:** [Structural Checking](../design/structural-checking.md), [Checking Diagnostics](../design/check-diagnostics.md), [Configuration](../design/configuration.md), [Rust Engine](../design/rust-engine.md#check-shared-typed-results), [System Overview](../design/system-overview.md#check-independent-build), the [initial checker requirements](../authority/requirements.md#initial-checker-requirements), and [Decisions](../authority/decisions.md).

**Identifier:** `engine-bootstrap-1`

**Tasks**

- [x] Draft the system and Rust engine design, configuration, navigation rules, and diagnostic behavior, with component and sequence diagrams.
- [x] Draft governing requirements and decisions in Authority.
- [x] Define a canonical structured format and draft the initial checker anchors, grounding them in authority and linking them to verification criteria and implementation work.
- [x] Draft outcome-based checker milestones with stable identifiers, dependencies, and completion criteria.
- [x] Settle the APIs, dependencies, and supported tool versions needed for structural checking; [Structural Checking milestones 1 and 2](structural-checker.md#structural-checking-2) record the choices.
- [x] Agree on the Structural Checking milestones; the engineer assigned them as scoped prototypes.
- [x] Review each of the nine checker anchors with the engineer as a guarantee supporting the check, repair, and recheck loop: its property, cited authority, retained constraints, and criteria. Make agreed revisions in the design and reconcile affected roadmaps.
- [x] Review the six checker requirements and the four decisions the anchors cite for whether they justify the properties: `decision-explicit-configuration`, `decision-mdbook-structure`, `decision-separate-check-and-repair`, and `decision-typed-check-results`.
- [x] Mark the agreed records `accepted`, each requirement and decision no later than the anchors that cite it, and run the structural check and Book build.

### Completion criteria

#### Traceable design {#engine-bootstrap-1-traceable-design}

The engineer has reviewed each checker anchor and the requirements and decisions it cites: the authority justifies the property, essential constraints are retained, and the criteria are testable. Revisions agreed in review are made in the design.

- **Anchor coverage:** None — design review
- **Evidence:** Passed: the engineer reviewed the anchors and their authority on 2026-11-10 and agreed these revisions: `ci-order` and `agent-repair` require `review`, since their evidence is recorded observation; `equivalent-callers` includes the `anchorbook md` preprocessor; required roles stay top-level chapters in every project; `check-configuration-validity` also cites `requirement-truthful-results`; `decision-separate-check-and-repair` is retitled; and `requirement-repair-feedback` states its product source. Affected roadmaps were reconciled in the same changes. Six anchors were accepted without revision.

#### Agreed scope {#engine-bootstrap-1-agreed-scope}

The engineer agrees on the checker scope and roadmap. Implementation choices are resolved or explicitly deferred outside that scope.

- **Anchor coverage:** None — work agreement
- **Evidence:** The engineer assigned all four Structural Checking milestones and accepted milestones 1–3 as complete; milestone 4 is recorded complete, pending engineer acceptance. Implementation choices are recorded in those milestones and the checker design. Passed: on 2026-11-10 the engineer confirmed that these assignments and acceptances constitute agreement on the checker scope and roadmap.

#### Accepted records {#engine-bootstrap-1-accepted-records}

The nine checker anchors and the requirements and decisions they cite have status `accepted`, and the structural check and Book build pass, so record validation confirms that every accepted anchor and decision cites only accepted authority.

- **Anchor coverage:** None — design acceptance; acceptance does not verify any anchor.
- **Evidence:** Passed: all nine checker anchors, six requirements, and four decisions have status `accepted`. `anchorbook check --config anchorbook.toml` passed, and `mdbook build`, whose record validation rejects accepted records citing draft authority, succeeded.

### Progress

The Book contains the checker design, six checker requirements, four governing decisions, and nine checker anchors [mapped to implementation work](structural-checker.md#authority-and-anchors), all accepted.

### Evidence {#engine-bootstrap-1-evidence}

*Milestone evidence — review, acceptance, and scope confirmation recorded*

**Revision:** Branch `book-upkeep`, based on `cea6ba3`; review revisions in `3962e7f` and `33ae5fd`, acceptance in the following commit. Not yet integrated into main.

**Checks:** The structural check, `mdbook build` with mdBook 0.5.4, current-Book acceptance, and rendered-Book acceptance passed after acceptance.

**Limitations:** Acceptance establishes the goal design only; it does not verify any anchor. The review was conducted by the agent that drafted the revisions, with the engineer deciding each change.

### Completion record

Complete on 2026-11-10, accepted by the engineer. The checker's governing records are reviewed and accepted, and the engineer confirmed the agreed scope. This completes design acceptance only; it does not verify any anchor.

## Milestone 2. A fresh agent session can take up an assigned milestone {#engine-bootstrap-2}

**Outcome:** An engineer can assign a roadmap milestone to an agent in a fresh session that has read the Book and can identify the applicable design, authority, anchors, and completion criteria before implementing it.

**Status:** Proposed

**Dependencies:** The contract and project guidance as maintained in the Book. Startup does not depend on [milestone 1](#engine-bootstrap-1): the records an agent reads can be drafts.

**Design basis:** [System Overview](../design/system-overview.md#contract-delivery) and [AI Working Contract](../ai-working-contract.md)

**Identifier:** `engine-bootstrap-2`

**Tasks**

- [ ] Exercise startup in fresh Codex and Claude Code sessions; record agent and model versions and the requests given.
- [ ] Assign a readiness walkthrough of a roadmap milestone: observe that the agent reads the contract, project guidance, and Book; identifies the current phase, authoring references, supported commands, assigned scope, and governing content; and does not begin implementation.
- [ ] Include a scoped design-change exercise: observe whether the agent updates the affected roadmap tasks, criteria, references, and evidence limits in the same change.
- [ ] Refine instructions where the agent misses context, fails to reconcile the roadmap, or starts unassigned work, and repeat the affected cases.

### Completion criteria

#### Agent startup {#engine-bootstrap-2-agent-startup}

Fresh Codex and Claude Code sessions demonstrably read the reusable contract, project guidance, and Book, and identify the project phase and supported commands from project guidance; record versions, configuration, and observations.

- **Anchor coverage:** None — workflow readiness; contract retrieval through the engine is covered by [Project Integration milestone 1](project-integration.md#project-integration-1-contract).
- **Evidence:** Pending.

#### Assigned scope {#engine-bootstrap-2-assigned-scope}

Each agent identifies the assigned milestone, its governing design, authority, anchors, and completion criteria without beginning unassigned implementation.

- **Anchor coverage:** None — workflow readiness
- **Evidence:** Pending.

#### Design reconciliation {#engine-bootstrap-2-design-reconciliation}

A fresh-session exercise changes a design criterion and updates the affected roadmap in the same change. The agent identifies stale evidence and proposed follow-up work without claiming completion or starting unassigned implementation.

- **Anchor coverage:** None — agent workflow readiness
- **Evidence:** Pending fresh-session demonstration; written contract guidance is preparation only.

#### Readiness record {#engine-bootstrap-2-readiness-record}

Record limitations and any instruction refinements and repeat affected cases. This demonstrates agent readiness, not any engine capability.

- **Anchor coverage:** None — workflow readiness
- **Evidence:** Pending.

### Preparation already done

`AGENTS.md` identifies the reusable [AI Working Contract](../ai-working-contract.md), [project guidance](../working-on-anchor-book.md), and Book index. The contract defines the working method and the readiness report; project guidance supplies the phase, paths, tools, authoring references, and public-content rules. [Structural Checking milestone 4](structural-checker.md#structural-checking-4) recorded fresh Codex sessions repairing seeded defects; those runs exercised repair, not the startup and assignment cases here.

### Evidence {#engine-bootstrap-2-evidence}

*Milestone evidence — pending*

Pending; use the [evidence format](index.md#evidence-records).

### Completion record

Pending. The preparation above does not establish that agents follow the instructions successfully.

## Scope changes and reserved identifiers

The engineer narrowed this roadmap on 2026-11-10 to the review and agent-startup
work that remained after Structural Checking was built. Milestone 1's completed
preparation tasks remain as recorded; its outcome now covers only record review
and acceptance. Milestone 2 applies to any assigned milestone rather than only
checker work.

The earlier draft included engine capabilities that belong after preparation.
Their milestone identifiers are retired here and will not be reused:

<a id="engine-bootstrap-3"></a>

- `engine-bootstrap-3`: checker implementation is wholly described by the
  Structural Checking roadmap.

<a id="engine-bootstrap-4"></a>

- `engine-bootstrap-4`: materialization is deferred to a later roadmap.

<a id="engine-bootstrap-5"></a>

- `engine-bootstrap-5`: engine-generated contract delivery is planned in
  [Project Integration milestone 1](project-integration.md#project-integration-1).
