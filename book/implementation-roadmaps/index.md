# Implementation Roadmaps

Implementation roadmaps turn a sufficiently clear design into agreed work for
AI agents. The engineer and agents develop the design together. Agents then propose
the outcomes to build, how to divide the work, dependencies, and how completion
will be demonstrated. The engineer reviews and agrees on that work before the
agents begin its implementation.

The relevant design must be clear enough to define the work and its completion
criteria; the entire system design need not be finished.

## Defining the work

Each roadmap has a stable identifier, a short descriptive title, and a goal
stating what becomes possible when it is complete. Its milestones group work
into outcomes an engineer can take on with their agents. Tasks describe the work;
completion criteria describe how to demonstrate that its outcome has been reached.

List milestones directly, without a separate overview table. Each records:

- **Identifier:** unique across the Book's roadmaps.
- **Outcome:** the capability or result enabled.
- **Status:** progress using the states below.
- **Design basis:** governing design, authority, and anchors where defined.
- **Dependencies:** prerequisites and required milestones.
- **Tasks:** bounded work, with checkboxes for progress.
- **Completion criteria:** stable identifiers, observable conditions, and planned
  anchor-criterion coverage.
- **Evidence and completion:** results, gaps, and the completion record.

Use one table per milestone, with outcome and status first. Keep tasks,
completion conditions, and the completion record visible; expand anchor details
and evidence within the table. Details include criterion IDs, anchor references,
coverage scope, and supporting results. Details remain readable without JavaScript
and are included when printing. Following a direct criterion link opens its details.
Completed preparation may be recorded separately, but neither preparation nor
task checkmarks establish acceptance evidence.

Milestones may be split into smaller outcomes. Keep detailed tasks and evidence
in one place and link from the parent. Anchors provide traceability, not work
boundaries: several milestones may advance one anchor or vice versa. Put work
outside a roadmap's goal in a separate roadmap. Ownership belongs to assignments;
owner fields and issue links are optional.

During prototyping these tables are maintained in Markdown. The engine does not
yet parse or render roadmap records. Proposed `anchorbook-roadmap` and
`anchorbook-milestone` blocks need agreed TOML schemas and engine support before
migration; these pages provide their working model.

## Naming and references

Refer to **Engine Bootstrap — Milestone 1**, for example, using roadmap ID
`engine-bootstrap` and milestone ID `engine-bootstrap-1`. Record each milestone
ID with a matching link target. Preserve IDs and numbers when reordering work or
refining titles. Retired IDs are not reused; new work receives an unused number.
These identify work, not system properties.

## Anchor criteria and planned coverage

Give each completion criterion an ID unique within its milestone, with fragment
`<milestone-id>-<criterion-id>`. Preserve both IDs through title or presentation
changes.

Link to the anchor and name its criterion ID; keep the verification requirement
in the anchor rather than copying it. One completion criterion may address several
anchor criteria, and several entries may address the same anchor criterion.

| Planned coverage | Meaning |
| --- | --- |
| Full | The work addresses the entire referenced criterion. |
| Partial | The work addresses a stated portion of the criterion. |
| None | No anchor applies; give a reason, such as design preparation or contributor setup. |

Planned coverage is not verification. Review the scope and evidence of partial
entries before claiming combined full coverage. Do not invent anchors to cover
preparation tasks. Design links supply context; authority is traced through the
anchors' own references.

When anchor criteria change, reassess milestone scope and evidence against the
new design. Future engine support will validate identities and derive backlinks
without duplicate coverage lists.

## Recording progress

| Status | Meaning |
| --- | --- |
| Proposed | Outcome and work are drafted for review |
| Ready | Scope and criteria are agreed, and prerequisites are satisfied |
| In progress | Assigned work has begun; remaining tasks and gaps are recorded |
| Blocked | A stated obstacle prevents further progress on the chunk |
| Complete | The outcome and all completion criteria have supporting evidence |

Update code, tests, Book progress, and evidence together through the project's
contribution workflow. This model does not mandate direct pushes or pull requests.
Agree on scope and criterion changes with the engineer; do not weaken them to
match completed work.

## Evidence records

Until results exist, record **Pending**, meaning no acceptance evidence has been
recorded for that criterion. When a check or review has run, record what it
established and its limits under the relevant criterion using these fields:

| Field | Meaning |
| --- | --- |
| Completion criterion | Local criterion ID; include the milestone ID in references from elsewhere |
| Revision | Tested code and Book revisions, identifying both if different |
| Check or observation | What ran or was reviewed, including versions and scope |
| Result | Passed, failed, or inconclusive, with the observed result |
| Supporting record | Test report, CI run, review, or reproducible observation |
| Limitations | Missing cases, partial coverage, assistance, or other limits; state none when appropriate |
| Integration context | Branch work or integrated into main |

Keep failed and inconclusive results visible. Complete requires evidence for the
outcome and every completion criterion; task checkmarks, anchor references, and
structural passes are insufficient. Linked anchors may span several milestones
or need other evidence, so milestone completion does not automatically verify them.

## Keeping the agreement current

Reconcile roadmaps in the same change as design. Review outcomes, dependencies,
tasks, criteria, anchor references, and evidence; add work or an explicit deferral.
New scope remains proposed until agreed. Prior evidence stays tied to its tested
revision. Follow the [AI Working Contract](../ai-working-contract.md#keep-roadmaps-aligned-with-the-goal-design)
for phase-specific responsibilities.

## Current roadmaps

- [Record Rendering](record-rendering.md): milestones 1 and 2 complete locally.
  Milestone 3 covers authority criteria, CI integration, and remote verification.
- [Engine Bootstrap](bootstrap.md): drafts are in place; review, checker-specific
  technical choices, and fresh-agent readiness remain. This can proceed alongside
  rendering.
- [Structural Checking](structural-checker.md): proposed, not implemented. Delivers
  the agent's check, repair, and recheck loop with independent tests and CI.

Rendering establishes the crate; structural checking extends it without depending
on rendered output. Drafting is preparation, not milestone completion. Unassigned
work remains proposed; engine automation is not needed to maintain these roadmaps.

## Later work retained from the design

These capabilities are part of the goal system but are outside the three current
roadmaps. They remain deferred, not omitted or assigned. Each needs a sufficiently
clear design and outcome-based milestones before implementation.

| Capability | Design basis | Work to define before implementation |
| --- | --- | --- |
| Pull-request testing readiness | [Testing policy](../ai-working-contract.md#test-before-opening-a-pull-request) | Before adopting the Building workflow, agree on mutation tooling, scope, runtime budget, survivor/exclusion handling, and local/CI commands alongside unit, integration, and acceptance checks. Demonstrate the workflow and document it for contributors; the initial contract section is a draft, not evidence of completed setup. |
| Initialize and adopt Books | [Operations](../design/system-overview.md#operations) | New-project setup and mapping existing layouts; exercise different consuming projects after configuration and materialization are usable. |
| Materialize managed content | [Operations](../design/system-overview.md#operations) | Ownership boundaries, planned changes, conflict detection, repeatable reconciliation, and preservation of authored content. Depends on configured structure and checking. |
| Deliver the working contract | [Contract delivery](../design/system-overview.md#contract-delivery) | Engine-owned reusable content, command retrieval, and agent bootstrap that separately locates project guidance and configuration. Remove the temporary Book copy when that path works; keep project-specific content in the Book. Language-specific instruction extensions remain a possible later addition requiring their own design. |
| Structured roadmaps and proposal support | [Roadmap model](#defining-the-work), [Design Proposals](../design/proposals/index.md) | Define roadmap/milestone schemas and criterion references before parsing or rendering them. Define any proposal tooling beyond the manual workflow; preserve the distinction between approval, design reconciliation, and implementation. |
| Anchor progress and verification evidence | [Book concepts](../design/system-overview.md#what-a-book-describes), [Build](../design/system-overview.md#operations) | Source/test associations, provided test surfaces, evidence ingestion, and progress views. Include formal verification assumptions, bounds, and inconclusive results; distinguish existing tests from passing executions and keep verification independent of its consuming Book build. |
| Cross-Book authority | [Book concepts](../design/system-overview.md#what-a-book-describes) | Provider/consumer identity, revisions, distribution, change review, and conflict handling. Local authority validation does not require this. |
| Automated phase and readiness checks | [Project phase](../design/configuration.md#project-phase) | Define what can establish design readiness or completion and how further iterations preserve a completed baseline. The first configuration reader only exposes the phase. |
| Crate release | [Rust Engine](../design/rust-engine.md#rust-development-conventions) | Registry naming, package contents, public API and schema compatibility, bundled resources when introduced, and release verification. Publishing is separate from local prototyping. |

The prototype's mdBook-version, output-format, and plain-text-field limits remain
documented constraints. Expanding them needs its own design and roadmap change;
they are not implicit tasks in the current rendering milestone.
