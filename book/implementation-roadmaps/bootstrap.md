# Engine Bootstrap

**Roadmap identifier:** `engine-bootstrap`

**Status:** Proposed implementation work; milestone scope remains subject to review.

## Goal

Build enough of the engine to check and manage its own Book and deliver its
AI Working Contract. This roadmap ends when those capabilities support daily
development through the same interfaces offered to consuming projects.

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
| [1](#engine-bootstrap-1) | The first engine implementation has an agreed scope | Proposed | Current design drafts |
| [2](#engine-bootstrap-2) | An agent can start with the Book's working context | Proposed | Existing bootstrap files and contract |
| [3](#engine-bootstrap-3) | Developers can check this Book through Rust and the CLI | Proposed | Engine Bootstrap — Milestone 1 |
| [4](#engine-bootstrap-4) | The engine can maintain this Book's managed structure | Proposed | Engine Bootstrap — Milestone 3 and ownership design |
| [5](#engine-bootstrap-5) | Agents can retrieve the effective contract from the engine | Proposed | Structural Checking — Milestone 1 and contract-generation design |

Refer to work by roadmap and milestone, for example **Engine Bootstrap —
Milestone 1**. Each milestone also has a stable identifier for links and future
tooling. These identifiers identify work, not system-property anchors.

Milestone 2 can proceed alongside milestone 1. Contract delivery need not wait
for materialization. The status table summarizes the records below; completing
this roadmap requires all five milestones and evidence that the combined
workflow works on this repository's current Book.

<a id="engine-bootstrap-1"></a>

## Milestone 1. The first engine implementation has an agreed scope

**Identifier:** `engine-bootstrap-1`

**Status:** Proposed. **Depends on:** current design drafts.

**Outcome:** An engineer can assign the first checker work without leaving its
required behavior or acceptance criteria for agents to infer.

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

**Completion record:** Pending.

<a id="engine-bootstrap-2"></a>

## Milestone 2. An agent can start with the Book's working context

**Identifier:** `engine-bootstrap-2`

**Status:** Proposed. **Depends on:** existing bootstrap files and contract.

**Outcome:** An engineer can start a fresh agent session, have it read the Book,
and then direct design work or an assigned roadmap chunk.

**Tasks:**

- Exercise startup in fresh Codex and Claude Code sessions.
- Observe contract and Book retrieval and identification of the assigned scope.
- Refine instructions where the agent misses context or starts unassigned work.

**Complete when:** each supported agent demonstrably reads the current contract
and Book and identifies the requested work without treating all roadmap work as
assigned. Record the tested versions/configuration and remaining limitations.

**Completion record:** Pending. File presence alone is not completion evidence.

<a id="engine-bootstrap-3"></a>

## Milestone 3. Developers can check this Book through Rust and the CLI

**Identifier:** `engine-bootstrap-3`

**Status:** Proposed. **Depends on:** [Engine Bootstrap — Milestone 1](#engine-bootstrap-1).

**Outcome:** The current Book receives useful structural findings through the
same public interfaces used by consuming projects.

**Tasks:** Complete milestones 1–3 in [Structural Checking](structural-checker.md).
That page owns the detailed tasks and acceptance evidence for this chunk.

**Complete when:** Structural Checking milestones 1–3 are complete, including acceptance against the current
Book, deliberate failure cases, file preservation, and independent engine checks
in CI. A structural pass makes no claim about full anchor verification.

**Completion record:** Pending; reference the Structural Checking completion records when complete.

<a id="engine-bootstrap-4"></a>

## Milestone 4. The engine can maintain this Book's managed structure

**Identifier:** `engine-bootstrap-4`

**Status:** Proposed. **Depends on:** [Engine Bootstrap — Milestone 3](#engine-bootstrap-3) and reviewed ownership/conflict design.

**Outcome:** An engineer can inspect and apply managed Book changes while
preserving project-authored content.

**Tasks:**

- Define owned content, shared settings, conflicts, and stale/interrupted plans.
- Plan and apply selected mdBook settings and the initial required scaffold/core.
- Exercise this Book, an empty initialization fixture, and an existing layout.

**Complete when:** drift in a disposable copy is repaired and confirmed by shared
checks; a second application makes no changes; authored content survives; and
conflicting edits, stale plans, and interrupted writes follow documented behavior.

**Completion record:** Pending.

<a id="engine-bootstrap-5"></a>

## Milestone 5. Agents can retrieve the effective contract from the engine

**Identifier:** `engine-bootstrap-5`

**Status:** Proposed. **Depends on:** [Structural Checking — Milestone 1](structural-checker.md#structural-checking-1)
and reviewed contract-generation design. Managed writes are not required.

**Outcome:** Agent startup obtains current, project-specific working guidance
without a maintained copy in the Book or a saved generated artifact.

**Tasks:**

- Move the contract source into the engine and specialize it from configuration.
- Expose command retrieval and exercise successful and failed retrieval.
- Switch startup instructions once retrieval works; remove the temporary Book copy.

**Complete when:** a fresh agent reads the correct command output; a configuration
change affects subsequent output; invalid configuration fails clearly; and an
incomplete Book receives accurate repair guidance. Retrieval saves no artifact.

**Completion record:** Pending.

## Using this Book throughout

Exercise the current Book whenever a capability becomes usable. Supplement it
with independent expected results and disposable failure cases. As assigned
tasks progress, update milestone status, evidence, and remaining work. Record
the tested revision and whether results describe branch work or integration
into main. Ownership and optional issue tracking live with the assignment.

## Next design work and later roadmaps

After the roadmap structure is committed, define the first anchors for the
designs already in this Book, with their governing authority and verification
criteria. This is part of milestone 1's design work; it does not require an
implemented anchor parser or evidence engine.

Automated anchor reporting, verification evidence, cross-Book authority, and
release preparation will have separate goal-based roadmaps when their designs
are ready. Cross-Book authority is deferred beyond bootstrap. Defining local
design anchors now does not bring those later capabilities into this scope.

The next proposed milestone is **Engine Bootstrap — Milestone 1**. Milestone 2
can be assigned independently while the checker scope is being reviewed.
