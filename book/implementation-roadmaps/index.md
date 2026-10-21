# Implementation Roadmaps

Implementation roadmaps turn a sufficiently clear design into agreed work for
AI agents.

We develop the design with AI. When it is clear enough to implement a capability,
the AI proposes a roadmap from that design: the results to build, how to divide
the work, the dependencies, and how completion will be demonstrated. We review
and refine that proposal together and agree on what the AI agents will do before
they begin the affected implementation work.

The entire system's design does not need to be finished before a roadmap can be
created. The relevant part must be clear enough to define the work and its
completion criteria.

## Defining the work

Give each roadmap a short, descriptive name and a goal that states what becomes
possible when the roadmap is complete. Keep work outside that goal for a separate
roadmap rather than extending bootstrap to cover the entire product.

Group work into chunks that enable an outcome an engineer can take on with
their agents. Call these chunks milestones. Organize around what
becomes possible when the work is done, with tasks describing how to get there.

Each roadmap has an identifier, title, goal, and a sequence of milestone entries.
List the milestones directly, without a separate overview section or summary
table. Each milestone records:

- **Identifier:** a stable reference unique across the Book's roadmaps.
- **Outcome:** the capability or result the work enables.
- **Design basis:** the relevant design, governing authority, and anchors where defined.
- **Dependencies:** prerequisites and other chunks required for this result.
- **Tasks:** bounded work that advances the outcome.
- **Completion criteria:** stable criterion identifiers, observable
  completion conditions, and anchor-criterion references with planned coverage.
- **Evidence and completion:** results associated with those criteria, remaining
  gaps, and the milestone completion record.

Status is shown with the milestone identifier and dependencies. Completed
preparation can be described before the evidence section, without presenting it
as acceptance evidence. Tasks use checkboxes so completed work remains visible.

Each milestone appears in one table, leading with outcome and status, followed
by dependencies, design basis, identifier, tasks, completion criteria, progress
where recorded, and evidence and completion record. Tasks and complete-when
conditions remain visible. Expand anchor details and evidence to read the
criterion identifier, anchor references, planned coverage, partial-coverage scope,
and supporting evidence. Milestone evidence also expands within the table;
its completion record stays visible.
Details are included when printing and remain readable without JavaScript.
Direct criterion links open the relevant details. Evidence stays Pending until
results are recorded and assessed; the table does not infer success from tasks.

During prototyping we maintain this structure directly in Markdown. The engine
does not yet parse or render roadmap records. The intended structured source
will use `anchorbook-roadmap` and `anchorbook-milestone` blocks; their exact TOML
schemas and engine support must be designed together before migrating these
pages. The criteria here are the working model for those records.

Chunks may be decomposed into smaller outcome-based chunks when needed. Keep
detailed tasks and evidence in one place and link to them from the parent.
Different chunks can advance the same anchor, and one chunk can advance several
anchors. Anchor relationships provide traceability; they do not dictate work
boundaries.

An engineer's assignment carries ownership and coordination. Owner fields and
issue links are not required in the roadmap. Optional tracking can be added
when useful without moving the agreed scope and completion criteria out of the Book.

## Naming and references

Refer to work by roadmap name and milestone number, such as **Engine Bootstrap —
Milestone 1**. Its descriptive title states the outcome.

Assign a unique roadmap identifier, such as `engine-bootstrap`, and give each
milestone an identifier such as `engine-bootstrap-1`. Record it in the milestone
and provide a matching link target. Preserve identifiers and milestone numbers
when reordering work or refining titles; do not reuse a retired identifier.
Assign a new unused number to new work. These are work references, not anchors
that define system properties.

## Anchor criteria and planned coverage

Give each milestone completion criterion an identifier unique within that
milestone. Refer to it by milestone ID and criterion ID together. Preserve both
identifiers as titles and presentation change. Its page fragment combines them
as `<milestone-id>-<criterion-id>`.

An anchor-criterion reference contains the anchor ID and criterion ID, not a
copied verification requirement. The anchor remains the source of that
requirement. Each entry links to the anchor definition and names the
criteria it addresses. Several entries can address one anchor criterion, and
one completion criterion can address several anchor criteria.

State **Full** when the planned work addresses the entire referenced criterion.
State **Partial** with the precise portion when it addresses only part. This is
planned coverage, not a verification result. Combining partial entries does not
automatically establish full coverage; review their scope and evidence together.

Use **None** with a short explanation when a completion condition has no anchor
reference, such as design preparation or contributor setup. Do not invent a
system anchor just to give every task a reference. Ordinary design references
give context; requirements and decisions are reached through the anchors' own
authority references.

Changing a referenced anchor criterion requires reviewing the affected milestone
scope and evidence. Evidence for an earlier design revision must not silently
become evidence for a changed requirement. When the engine supports these
records, identity validation and derived backlinks will make these relationships
navigable without maintaining duplicate coverage lists.

## Recording progress

Use these statuses consistently:

| Status | Meaning |
| --- | --- |
| Proposed | Outcome and work are drafted for review |
| Ready | Scope and criteria are agreed, and prerequisites are satisfied |
| In progress | Assigned work has begun; remaining tasks and gaps are recorded |
| Blocked | A stated obstacle prevents further progress on the chunk |
| Complete | The outcome and all completion criteria have supporting evidence |

Update the Book as tasks progress. Finishing a task does not automatically
complete its chunk. When marking a chunk Complete, record the relevant revision,
checks or observations, their results, and any limits on the claim. Identify
whether evidence describes a branch or work integrated into main.

Keep code, tests, and corresponding Book updates together in the project's
chosen contribution workflow. The roadmap model does not mandate direct pushes
or pull requests. Scope expansions and changed completion criteria must be
reconciled with the engineer, rather than silently changed to match the work done.

## Evidence records

Record evidence under the relevant milestone using these fields. Until results
exist, say that no acceptance evidence is recorded rather than adding empty or
successful-looking entries.

| Field | Meaning |
| --- | --- |
| Completion criterion | The local completion criterion ID; include the milestone ID when referencing it elsewhere |
| Revision | The tested code and Book revisions, identifying both if different |
| Check or observation | What ran or was reviewed, including relevant versions and scope |
| Result | Passed, failed, or inconclusive, with the observed result |
| Supporting record | A test report, CI run, review, or reproducible observation record |
| Limitations | Missing cases, partial coverage, assistance, or other limits; explicitly state none when appropriate |
| Integration context | Branch work or integrated into main |

Keep failed and inconclusive results visible. A milestone is complete only when
its outcome and every completion criterion have supporting evidence. A task
checkmark, a reference to an anchor, or a successful structural check cannot
establish this. Milestone completion does not automatically mark linked anchors
verified; their criteria may span several milestones or require other evidence.

## Keeping the agreement current

An AI-generated roadmap is a proposal until we agree on it. If the design changes
or implementation reveals a new constraint, the AI proposes the corresponding
roadmap changes for review. The agreed roadmap remains the basis for agent work.

Reconcile affected roadmaps in the same change as the design. Review outcomes,
dependencies, tasks, completion criteria, anchor references, and evidence;
record new work or an explicit deferral. Completed evidence remains tied to its
tested revision. The [AI Working Contract](../ai-working-contract.md#keep-roadmaps-aligned-with-the-goal-design)
defines the agent's responsibilities for this reconciliation.

Verification evidence establishes whether a capability works; agreement on a
planned milestone alone does not establish completion.

## Current roadmaps

- [Record Rendering](record-rendering.md) has completed its record-library and
  readable-rendering milestones locally. The book uses the adapter in local
  preview; milestone 3 includes explicit coverage of the new authority anchor,
  CI integration, and remote verification.
- [Engine Bootstrap](bootstrap.md) is in progress. The design, authority, anchors,
  working contract, and checker roadmap are drafted. Review, remaining technical
  choices, and fresh-agent readiness checks are still outstanding. That
  preparation can proceed alongside the rendering prototype.
- [Structural Checking](structural-checker.md) has a drafted design and proposed
  milestones; implementation has not started. It enables an agent to use checker
  feedback to repair a Book's structure and recheck until it passes, supported
  by independent engine tests and continuous checks on this Book.

The planned implementation order is record rendering, then structural checking.
The checker extends the crate established by rendering; it does not require a
rendered Book to run. Roadmap and milestone identifiers remain unchanged.

Completed drafting tasks are recorded separately from milestone completion.
Unassigned implementation milestones remain proposals for review. Later capabilities need their relevant design
and detailed work agreed before implementation begins. The Book owns these
roadmaps now; engine automation is not a prerequisite for maintaining them.

## Later work retained from the design

These capabilities are part of the goal system but are outside the three current
roadmaps. They remain deferred, not omitted or assigned. Each needs a sufficiently
clear design and outcome-based milestones before implementation.

| Capability | Design basis | Work to define before implementation |
| --- | --- | --- |
| Pull-request testing readiness | [Testing policy](../ai-working-contract.md#test-before-opening-a-pull-request) | Before adopting the Building workflow, agree on mutation tooling, scope, runtime budget, survivor/exclusion handling, and local/CI commands alongside unit, integration, and acceptance checks. Demonstrate the workflow and document it for contributors; the initial contract section is a draft, not evidence of completed setup. |
| Initialize and adopt Books | [Operations](../design/system-overview.md#operations) | New-project setup and mapping existing layouts; exercise different consuming projects after configuration and materialization are usable. |
| Materialize managed content | [Operations](../design/system-overview.md#operations) | Ownership boundaries, planned changes, conflict detection, repeatable reconciliation, and preservation of authored content. Depends on configured structure and checking. |
| Deliver the working contract | [Contract transition](../ai-working-contract.md#transition-to-engine-delivery) | Engine-owned content, configuration specialization, command retrieval, and agent bootstrap; remove the temporary Book copy when that path works. |
| Structured roadmaps and proposal support | [Roadmap model](#defining-the-work), [Design Proposals](../design/proposals/index.md) | Define roadmap/milestone schemas and criterion references before parsing or rendering them. Define any proposal tooling beyond the manual workflow; preserve the distinction between approval, design reconciliation, and implementation. |
| Anchor progress and verification evidence | [Book concepts](../design/system-overview.md#what-a-book-describes), [Build](../design/system-overview.md#operations) | Source/test associations, provided test surfaces, evidence ingestion, and progress views. Include formal verification assumptions, bounds, and inconclusive results; distinguish existing tests from passing executions and keep verification independent of its consuming Book build. |
| Cross-Book authority | [Book concepts](../design/system-overview.md#what-a-book-describes) | Provider/consumer identity, revisions, distribution, change review, and conflict handling. Local authority validation does not require this. |
| Automated phase and readiness checks | [Project phase](../design/configuration.md#project-phase) | Define what can establish design readiness or completion and how further iterations preserve a completed baseline. The first configuration reader only exposes the phase. |
| Crate release | [Rust Engine](../design/rust-engine.md#rust-development-conventions) | Registry naming, package contents, public API and schema compatibility, bundled resources when introduced, and release verification. Publishing is separate from local prototyping. |

The prototype's mdBook-version, output-format, and plain-text-field limits remain
documented constraints. Expanding them needs its own design and roadmap change;
they are not implicit tasks in the current rendering milestone.
