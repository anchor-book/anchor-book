# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

## Goal

The checker's design rests on accepted governing records.

Bootstrap first prepared the Book so an engineer could assign structural-checker
work. The engineer then assigned the [Structural Checking](structural-checker.md)
milestones as scoped prototypes before that preparation was reviewed, and that
roadmap records the checker as built. What remained of Bootstrap was the review
and acceptance of the checker's anchors, requirements, and decisions.

Retrieving the contract through `anchorbook contract` belongs to
[Project Integration milestone 1](project-integration.md#project-integration-1).
The [AI Working Contract](../ai-working-contract.md) defines how agents work.

## Milestone 1. The checker's governing records are reviewed and accepted {#engine-bootstrap-1}

**Outcome:** The checker's anchors and the requirements and decisions they cite are reviewed and accepted into the goal design.

**Status:** Complete

**Dependencies:** None. Acceptance is a design judgment; the built checker and its evidence inform the review but do not establish it.

**Design basis:** [Structural Checking](../design/structural-checking.md), [Checking Diagnostics](../design/check-diagnostics.md), [Configuration](../design/configuration.md), [Rust Engine](../design/rust-engine.md#check-shared-typed-results), [System Overview](../design/system-overview.md#check-independent-build), the [initial checker requirements](../authority/requirements.md#initial-checker-requirements), and [Decisions](../authority/decisions.md).

**Identifier:** `engine-bootstrap-1`

**Tasks**

- [x] Draft the system and Rust engine design, configuration, navigation rules, and diagnostic behavior, with component and sequence diagrams.
- [x] Draft governing requirements and decisions in Authority.
- [x] Define a canonical structured format and draft the initial checker anchors, grounding them in authority and linking them to verification criteria and implementation work.
- [x] Draft outcome-based checker milestones with stable identifiers, dependencies, and completion criteria.
- [x] Settle the APIs, dependencies, and supported tool versions needed for structural checking; [Structural Checking milestones 1 and 2](structural-checker.md#structural-checking-2) record the choices.
- [x] Assign the Structural Checking milestones as scoped prototypes.
- [x] Review each of the nine checker anchors as a guarantee supporting the check, repair, and recheck loop: its property, cited authority, retained constraints, and criteria. Make agreed revisions in the design and reconcile affected roadmaps.
- [x] Review the six checker requirements and the four decisions the anchors cite for whether they justify the properties: `decision-explicit-configuration`, `decision-mdbook-structure`, `decision-separate-check-and-repair`, and `decision-typed-check-results`.
- [x] Mark the agreed records `accepted`, each requirement and decision no later than the anchors that cite it, and run the structural check and Book build.

### Completion criteria

#### Accepted records {#engine-bootstrap-1-accepted-records}

The nine checker anchors and the requirements and decisions they cite have status `accepted`, and the structural check and Book build pass, so record validation confirms that every accepted anchor and decision cites only accepted authority.

- **Anchor coverage:** None — design acceptance; acceptance does not verify any anchor.
- **Evidence:** Passed: all nine checker anchors, six requirements, and four decisions have status `accepted`. `anchorbook check --config anchorbook.toml` passed, and `mdbook build`, whose record validation rejects accepted records citing draft authority, succeeded.

### Progress

The Book contains the checker design, six checker requirements, four governing decisions, and nine checker anchors [mapped to implementation work](structural-checker.md#authority-and-anchors), all accepted.

### Evidence {#engine-bootstrap-1-evidence}

*Milestone evidence — review and acceptance recorded*

**Revision:** Branch `book-upkeep`, based on `cea6ba3`; review revisions in `3962e7f` and `33ae5fd`, acceptance in the following commit. Not yet integrated into main.

**Review revisions:** `ci-order` and `agent-repair` require `review`, since their evidence is recorded observation; `equivalent-callers` includes the `anchorbook md` preprocessor; required roles stay top-level chapters in every project; `check-configuration-validity` also cites `requirement-truthful-results`; `decision-separate-check-and-repair` is retitled; and `requirement-repair-feedback` states its product source. Affected roadmaps were reconciled in the same changes. Six anchors were accepted without revision.

**Checks:** The structural check, `mdbook build` with mdBook 0.5.4, current-Book acceptance, and rendered-Book acceptance passed after acceptance.

**Limitations:** Acceptance establishes the goal design only; it does not verify any anchor.

### Completion record

Complete on 2026-11-10. The checker's governing records are reviewed and accepted. This completes design acceptance only; it does not verify any anchor.

## Scope changes and reserved identifiers

This roadmap was narrowed on 2026-11-10 to the record review that remained after
Structural Checking was built. Milestone 1's completed preparation tasks remain as
recorded; its outcome now covers only record review and acceptance.

The earlier draft included engine capabilities that belong after preparation.
Their milestone identifiers are retired here and will not be reused:

<a id="engine-bootstrap-2"></a>

- `engine-bootstrap-2`: a staged demonstration of fresh agent sessions taking up
  assigned work. Agents start from the contract, project guidance, and Book in
  ordinary sessions, so observing that is a review step rather than an outcome.
  Contract retrieval and fresh sessions in other projects remain in
  [Project Integration milestone 1](project-integration.md#project-integration-1-contract)
  and [Symposium Integration milestone 3](symposium-integration.md#symposium-integration-3).

<a id="engine-bootstrap-3"></a>

- `engine-bootstrap-3`: checker implementation is wholly described by the
  Structural Checking roadmap.

<a id="engine-bootstrap-4"></a>

- `engine-bootstrap-4`: materialization is deferred to a later roadmap.

<a id="engine-bootstrap-5"></a>

- `engine-bootstrap-5`: engine-generated contract delivery is planned in
  [Project Integration milestone 1](project-integration.md#project-integration-1).

Milestone 1's criteria `engine-bootstrap-1-traceable-design` and
`engine-bootstrap-1-agreed-scope` recorded engineer review and agreement rather
than observable outcomes. They are retired and will not be reused;
`engine-bootstrap-1-accepted-records` shows the result.
