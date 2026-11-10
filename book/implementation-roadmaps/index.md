# Implementation Roadmaps

Implementation roadmaps turn a sufficiently clear design into the work that
reaches it: the outcomes to build, how the work is divided, dependencies, and how
completion will be demonstrated. The
AI Working Contract (`anchorbook contract`, “Work with the engineer”) defines
how agents propose roadmaps and when work may begin.

The relevant design must be clear enough to define the work and its completion
criteria; the entire system design need not be finished.

## Defining the work

Each roadmap has a stable identifier, a short descriptive title, and a goal
stating what becomes possible when it is complete. Its milestones group work
into outcomes an engineer can take on with their agents. Tasks describe the work;
completion criteria describe how to demonstrate that its outcome has been reached.
A completion criterion is an observable condition shown by recorded evidence, not
someone's approval or confirmation; a milestone is Complete when its evidence
shows every criterion holds.

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

Write milestones in plain Markdown, without HTML, so agents read only content:

```markdown
## Milestone 1. Outcome stated as a title {#roadmap-id-1}

**Outcome:** The capability or result enabled.

**Status:** Proposed

**Identifier:** `roadmap-id-1`

**Dependencies:** Prerequisites and required milestones.

**Design basis:** [Linked design](../design/chapter.md#section) and authority.

**Tasks**

- [ ] Bounded work, checked when done.

### Completion criteria

#### Criterion title {#roadmap-id-1-criterion-id}

The observable condition that shows the outcome is reached.

- **Anchor coverage:** `anchor-id` · `criterion-id` — Full, Partial with its scope, or None with a reason.
- **Evidence:** Pending, or the recorded result.

### Evidence {#roadmap-id-1-evidence}

Milestone-level results, if any.

### Completion record

Pending, or the completion statement.
```

Fields are separate paragraphs, in this order, with outcome and status first.
The milestone heading and each criterion heading carry their stable fragment as a
heading attribute; these fragments are the link targets. Completed preparation may
be recorded separately, but neither preparation nor task checkmarks establish
acceptance evidence.

Milestones may be split into smaller outcomes. Keep detailed tasks and evidence
in one place and link from the parent. Anchors provide traceability, not work
boundaries: several milestones may advance one anchor or vice versa. Put work
outside a roadmap's goal in a separate roadmap. Ownership belongs to assignments;
owner fields and issue links are optional.

The Anchor Book preprocessor reads this format in the chapters of the
`implementation-roadmaps` role: it validates milestone and criterion identifiers
and anchor-coverage references and renders each criterion as a collapsible
section, as [Roadmap rendering](../design/record-rendering.md#roadmap-rendering)
describes. A malformed milestone fails the Book build with a located diagnostic.

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

| Planned coverage | Meaning | Written |
| --- | --- | --- |
| Full | The work addresses the entire referenced criterion, as it currently reads. | `— Full`, with no qualifier; notes go in the evidence |
| Partial | The work addresses a stated portion of the criterion. | `— Partial: <scope>` |
| None | No anchor applies; give a reason, such as design preparation or contributor setup. | `None — <reason>` |

A Full entry with a qualifier, such as "Full for the declared versions", is a
Partial entry; write it as one. The roadmap reader rejects a qualified Full, and
an entry with no level, as `roadmap.invalid-coverage`. When a criterion grows after its milestone is
Complete, change that milestone's Full entry to Partial with the scope it covered.

Planned coverage is not verification by itself. Once a milestone is Complete,
its Full entries become [claims](../design/verification-evidence.md#claims-of-completion)
that `verify` checks against current evidence; Partial entries claim nothing.
Review the scope and evidence of partial entries before claiming combined full
coverage. Do not invent anchors to cover
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
Do not weaken scope or criteria to match completed work.

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

Record the latest applicable result for each criterion. Leave superseded
revisions and the chronology of changes to Git history, unless the criterion
concerns the correction itself, such as a mutation survivor and its fix. Status
belongs in each milestone's Status and Completion record; design pages and other
roadmaps link to it rather than restating it.

Keep failed and inconclusive results visible. Complete requires evidence for the
outcome and every completion criterion; task checkmarks, anchor references, and
structural passes are insufficient. Linked anchors may span several milestones
or need other evidence, so milestone completion does not automatically verify them.

## Keeping roadmaps current

Reconcile roadmaps in the same change as design. Review outcomes, dependencies,
tasks, criteria, anchor references, and evidence; add work or an explicit deferral.
Prior evidence stays tied to its tested
revision. Follow the AI Working Contract (`anchorbook contract`, “Keep roadmaps aligned with the goal design”)
for phase-specific responsibilities.

## Current roadmaps

Each roadmap's milestones record their own status and evidence.

- [Engine Bootstrap](bootstrap.md): reviewing and accepting the checker's governing records.
- [Record Rendering](record-rendering.md): readable, linked records in the rendered Book.
- [Structural Checking](structural-checker.md): read-only checking that guides agents through format repairs.
- [Mutation Testing](mutation-testing.md): mutation testing and its correction loop before merge.
- [Formal Verification](formal-verification.md): proving the engine's own authority validation, and making Verus an optional feature so no project needs it.
- [Record Line Grammar](record-line-grammar.md): measuring whether a line-oriented record syntax serves agents better than TOML.
- [Verification Evidence](verification-evidence.md): shared criterion evidence, test and proof runners, proof guards and assessment records, consumer-owned Verus, and retained CI reports.
- [Project Integration](project-integration.md): this Book uses the groundwork every project relies on before any project adopts it.
- [Symposium Integration](symposium-integration.md): Symposium develops with its own Anchor Book, without Verus, and the gaps it exposes become engine work.

Drafting is preparation, not milestone completion. Unassigned work remains
proposed; engine automation is not needed to maintain these roadmaps.

## Later work retained from the design

These capabilities are part of the goal system but are outside the current
roadmaps. They remain deferred, not omitted or assigned. Each needs a sufficiently
clear design and outcome-based milestones before implementation.

| Capability | Design basis | Work to define before implementation |
| --- | --- | --- |
| Broader pull-request testing readiness | Testing policy (`anchorbook contract`, “Test before opening a pull request”) | The active [Mutation Testing roadmap](mutation-testing.md) owns mutation tooling, correction, and merge checks. Before adopting Building, review the combined unit, integration, acceptance, and mutation workflow and any remaining contribution-policy gaps. |
| Initialize and adopt Books | [Operations](../design/system-overview.md#operations) | New-project setup and mapping existing layouts; exercise different consuming projects after configuration and materialization are usable. |
| Materialize managed content | [Operations](../design/system-overview.md#operations) | Ownership boundaries, planned changes, conflict detection, repeatable reconciliation, and preservation of authored content. Depends on configured structure and checking. |
| Language-specific contract extensions | [Contract delivery](../design/system-overview.md#contract-delivery) | `anchorbook contract` delivers the engine-owned contract through [Project Integration milestone 1](project-integration.md#project-integration-1). Its Rust guidance is built in; extensions for other languages need their own design. |
| Structured roadmaps | [Roadmap model](#defining-the-work) | Validation and rendering of the Markdown roadmap format are implemented in [Record Rendering milestone 4](record-rendering.md#record-rendering-4), which is Complete; references between roadmaps beyond fragments remain later work. |
| Remaining mdBook summary input | [Navigation parser](../design/structural-checking.md#navigation-parser) | Accept ordered lists, HTML or images in link labels, and content mdBook ignores, reporting links mdBook would drop as structural findings. Deferred until a Book needs a construct; each is added with regression tests against mdBook's parser. |
| Anchor progress views | [Book concepts](../design/system-overview.md#what-a-book-describes), [Build](../design/system-overview.md#operations) | Provided test surfaces and progress views combining anchor development, `verify` results, and roadmap completion. The [Verification Evidence roadmap](verification-evidence.md) builds evidence evaluation itself; keep verification independent of its consuming Book build. |
| Cross-Book authority | [Book concepts](../design/system-overview.md#what-a-book-describes) | Provider/consumer identity, revisions, distribution, change review, and conflict handling. Local authority validation does not require this. |
| Automated phase and readiness checks | [Project phase](../design/configuration.md#project-phase) | Define what can establish design readiness or completion and how further iterations preserve a completed baseline. The first configuration reader only exposes the phase. |
| Crate release | [Rust Engine](../design/rust-engine.md#rust-development-conventions), [Project Integration](../design/project-integration.md) | Registry naming, package contents including the contract, public API and schema compatibility, the declared engine, configuration, and record-syntax versions, and release verification. The package delivers what consuming Books need at run time, as it now does for the contract and the record and milestone presentation, and should leave out repository-only files such as workflows. Publishing is separate from local prototyping. |
| Remaining project integration | [Project Integration](../design/project-integration.md) | The [Project Integration](project-integration.md) and [Symposium Integration](symposium-integration.md) roadmaps build the groundwork and trial it in one project; their findings refine this work. sage-dev's integration is deferred while its role is unclear. The library CLI entry point and exact engine-version declaration, including validation of exact requirements and rejection of mismatched entry points; other exact dependency pins; declared mdBook versions; the agent bootstrap template; and broader adoption fixtures. Verus is an optional engine feature through [Formal Verification milestone 4](formal-verification.md#formal-verification-4); project proof execution and consumer fixtures in [Verification Evidence milestones 2 and 5](verification-evidence.md#verification-evidence-2). Those fixtures can use a locally packaged candidate; general adoption still needs crate release and contract delivery. Found in [Symposium Integration milestone 1](symposium-integration.md#symposium-integration-1-gaps): projects need versioned authoring references for the record and roadmap forms their engine reads, rather than this Book's chapters at a pinned revision. |

The prototype's mdBook-version, output-format, and plain-text-field limits remain
documented constraints. Expanding them needs its own design and roadmap change;
they are not implicit tasks in the current rendering milestone.
