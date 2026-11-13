# Requests for Discussion

**Roadmap identifier:** `rfds`

## Goal

Make RFDs an optional, understood part of the Book's structure and development
process. Agents and people can follow one design change across all affected
capabilities and implementation work through a shared typed relationship model.
[Requests for Discussion](../design/rfds.md) defines the intended behavior.

## Current scope

The engineer assigned the RFD design, conditional contract guidance, source
format, templates, and relationship-model plan. These are prepared here. The current schema and readers
still recognize only the three required roles and existing authority records;
they do not implement an RFD role, typed proposal references, or RFD progress
views or template commands. No RFD configuration field is added to this Book until it is supported.
The contract defines how agents use explicitly enabled support. Automated RFD
checks and the configuration switch remain to be implemented; scoped trials can
assess the guidance without claiming that current tools support the setting.

Preparation validation on local branch `codex/rfd-design`, based on merged
revision `76f415e4934734f29776dda15df8e5d060c8fbe7`: the structural check
(`structural/v1`), Book build, four packaged-contract tests, and six source/rendered
Book acceptance tests passed. These check the authored design and delivered
guidance; they establish neither RFD-specific engine behavior nor fresh-agent
compliance. The changes are not yet integrated into main.

Source-format authoring validation used a temporary local program with the
checkout's locked dependencies: all nine TOML example/template blocks parsed,
four embedded definitions and the baseline/resulting record sets passed the
existing record reader, and the filled proposal template contributed no current
records. The selected rfd-metadata/rfd-change markers preserve the reader's
rejection of legacy anchorbook-prefixed fences. This validates example syntax
and existing fence isolation, not the planned RFD schema, scoped resolver,
template commands, or actual adoption behavior.

The source format draws on Symposium's
[RFD template](https://github.com/symposium-dev/symposium/blob/b55d74c9577668c251a384c3d9e8e5e44626bf7d/md/rfds/TEMPLATE/README.md)
and [process](https://github.com/symposium-dev/symposium/blob/b55d74c9577668c251a384c3d9e8e5e44626bf7d/md/rfds/rfd-process/README.md).
These are design inputs. [RFD Source Format](../design/rfd-format.md) now specifies
configuration schema 2 and RFD format 1, and [worked examples](../design/rfd-examples.md)
show a cross-capability draft and its acceptance and incorporation. Template
sources live in templates/rfds; packaging and command delivery remain work.

Design review should assess the explicit configuration upgrade, proposal scope,
historical-reference limits, and adoption example. The governing RFD requirement
and anchors remain draft; milestone status remains Proposed pending agreement or
an explicit implementation prototype assignment. No parser design decision is
left as an implicit prerequisite for milestone 1.

This engine work precedes Symposium's broad RFD import. It uses independent
fixtures and this Book first; Symposium's tests and migration do not block it.
The coding experience remains the goal: proposals make the intended change and
affected scope clear before agents implement and verify it.

## Milestone 1. Books declare RFDs and resolve their relationships {#rfds-1}

**Outcome:** Books can opt into the RFD section, and a shared typed model validates proposal identity and relationships across existing Book entities without weakening authority validation.

**Status:** Proposed

**Identifier:** `rfds-1`

**Dependencies:** Existing configuration, navigation, record, and roadmap readers. Agreement on the specified RFD design or a scoped implementation prototype assignment; canonical format, proposal scope, section ownership, and version boundaries are now defined.

**Design basis:** [Optional structure](../design/rfds.md#optional-book-structure), [source format and templates](../design/rfd-format.md), [worked examples](../design/rfd-examples.md), [proposal lifecycle](../design/rfds.md#proposal-content-and-lifecycle), and [shared relationships](../design/rfds.md#shared-relationship-model).

**Tasks**

- [x] Define optional structure, proposal lifecycle, relationship boundaries, and conditional AI Working Contract guidance.
- [x] Specify canonical proposal blocks, Proposed anchors, typed references, stable IDs, section ownership, and schema compatibility. Author add/revise/retire examples with draft, accepted, and incorporated states.
- [x] Prepare engine-owned index and proposal template sources matching the format.
- [ ] Implement schema-1 compatibility and the schema-2 extension, and reserved format-1 block parsing with precise locations. Cover malformed and unclosed fences, misplaced blocks, unknown fields, unfilled prompts, orphan roots, and supporting-chapter ownership.
- [ ] Add the optional book.sections.rfds mapping as the explicit capability switch and section location. Extend role and navigation checks for enabled conventional and alternative locations, disabled support with or without an RFD folder, and located failure/incomplete cases. Malformed declarations must not silently disable support.
- [ ] Implement an RFD reader and shared Book index using existing record and roadmap readers. Distinguish current targets, planned additions, and historical citations; validate typed edges and supersession cycles.
- [ ] Validate proposal-scoped anchor and authority definitions without admitting them to the current record inventory. Exercise competing revisions of one anchor, current/proposed authority resolution, reviewed incorporation, and evidence reassessment.
- [ ] Exercise retained add/revise/retire payloads after incorporation and target retirement, explicit historical references, partial incorporation records, and supersession chains. Report history and semantic adoption as unevaluated rather than rejecting retained history or claiming it was checked.
- [ ] Package both template sources and implement the read-only template commands. Fill them in independent fixtures and check the resulting Book through actual entry points; test retrieval without a project and invalid invocations.
- [ ] Test many-to-many relationships, shared decisions, stable identity under moves, invalid targets, deterministic backlinks, and unchanged authority rules through independent fixtures.
- [ ] Add this Book's own RFD section and a coherent proposal exercising multiple design chapters through the implemented model. Keep independent engine tests usable with a broken Book.

### Completion criteria

#### Optional structure {#rfds-1-structure}

Books omitting the optional mapping retain their prior outcomes, including when
an RFD folder exists, and report no RFD checks as evaluated. Enabled conventional
and alternative sections resolve explicitly; malformed, unlisted, missing,
escaping, and unreadable inputs produce the expected located outcomes through
actual callers. An invalid declaration cannot be treated as disabled support.

- **Anchor coverage:** [A-rfd-optional-structure](../design/rfds.md#A-rfd-optional-structure) · `optional-section`, `alternative-layout`, `schema-compatibility` — Full.
- **Evidence:** Pending. The current checker has no RFD-specific rules.

#### Shared relationships {#rfds-1-relationships}

The validated index preserves all authored cross-capability relationships and
their backlinks, distinguishes current/planned/historical targets, and rejects
invalid typed references and supersession cycles. Existing authority validation
and its tested/proved boundary remain intact. Independent fixtures and this Book
exercise the actual implementation.

- **Anchor coverage:** [A-rfd-shared-relationships](../design/rfds.md#A-rfd-shared-relationships) · `many-to-many`, `typed-boundaries` — Full.
- **Evidence:** Pending. Existing record and roadmap inventories are groundwork, not RFD support.

#### Proposed anchors {#rfds-1-proposed-anchors}

RFDs contain checkable proposed additions, revisions, and retirements with their
authority and criteria. Competing proposals remain separately addressable and
do not affect current authority or verification. Reviewed incorporation preserves
identities and explicitly reconciles intervening changes and affected evidence;
changing proposal disposition alone changes no current definitions.

- **Anchor coverage:** [A-rfd-proposed-anchors](../design/rfds.md#A-rfd-proposed-anchors) · `complete-proposals`, `proposal-isolation`, `explicit-incorporation` — Full.
- **Evidence:** Pending. The current reader has no proposal-definition scope.

#### Authoring templates {#rfds-1-templates}

Packaged template commands work identically without project context or writes,
reject invalid invocations, and produce the checked-in template text. Independent
filled templates form a valid schema-2 Book with a format-1 RFD through the actual
checker. Unfilled prompts produce useful findings rather than valid proposals.

- **Anchor coverage:** [A-rfd-agent-workflow](../design/rfds.md#A-rfd-agent-workflow) · `template-delivery` — Full.
- **Evidence:** Pending. Template source files exist; command delivery and RFD validation are not implemented.

### Completion record

Pending. Design and contract preparation do not establish implemented structure
or relationship checks.

## Milestone 2. Agents and readers follow RFDs through implementation {#rfds-2}

**Outcome:** RFD views and agent guidance preserve a coherent change through discussion, agreement, assigned implementation, and reconciliation, showing linked work and current evidence without confusing acceptance and completion.

**Status:** Proposed

**Identifier:** `rfds-2`

**Dependencies:** Milestone 1. Reuse current verification results; showing shared rendered evidence depends on [Verification Evidence milestone 9](verification-evidence.md#verification-evidence-9). Conditional contract guidance can be assessed earlier without claiming the rendered workflow complete.

**Design basis:** [Agent and rendered views](../design/rfds.md#agent-and-rendered-views), [adopting a reviewed RFD](../design/rfds.md#adopting-a-reviewed-rfd), and the contract's RFD process.

**Tasks**

- [ ] Present proposal scope, actual proposed anchor definitions and changes, review disposition, relationships, and progress derived from linked milestones through shared agent and rendered views. Distinguish proposed and current definitions.
- [ ] Exercise accepted proposals with incomplete work, draft prototypes with evidence, rejected/withdrawn/superseded proposals, and missing or stale evaluation. Do not introduce a second authored completion checklist.
- [ ] Run fresh agent sessions with RFD support enabled and disabled, including an RFD folder with support disabled and an enabled alternative location. Demonstrate a design change spanning chapters, its roadmap assignment, and reconciliation after implementation findings; record assistance and scope.
- [x] Define the contract's adoption procedure for reconciling an agreed RFD with the live system definition, affected authority, anchors, criteria, work, and evidence.
- [ ] Assess adoption in a fresh agent assignment with a cross-capability proposal and a seeded conflict outside its declared impact list. Review the resulting definition and implementation gaps, then repeat against the incorporated revision to assess preservation of identities and evidence.
- [ ] Use the full workflow in this Book before importing Symposium's RFDs. Carry source identity and review history forward, retaining draft proposals without making them governing authority.

### Completion criteria

#### Consistent progress {#rfds-2-progress}

Actual library, agent, and rendered views agree on proposal disposition, affected
scope, linked milestone progress, and current evidence limits. Acceptance and
verification remain independent; views change no sources and rendering runs no tests.

- **Anchor coverage:** [A-rfd-distinct-progress](../design/rfds.md#A-rfd-distinct-progress) · `independent-states`, `consistent-views` — Full.
- **Evidence:** Pending.

#### Agent workflow {#rfds-2-agents}

Fresh sessions apply the conditional contract correctly, report pending RFD
decisions and affected work, and reconcile a cross-capability change through its
proposal and linked roadmap. Configuration determines whether the workflow
applies: an existing RFD folder does not activate it, and an explicitly mapped
alternative location does. Disabled support requires no invented proposal.
Record agent versions, inputs, commands, results, and assistance.

- **Anchor coverage:** [A-rfd-agent-workflow](../design/rfds.md#A-rfd-agent-workflow) · `conditional-guidance` — Full.
- **Evidence:** Pending. Packaged-contract tests alone do not establish agent behavior.

#### Design adoption {#rfds-2-adoption}

A fresh agent adopts a reviewed RFD spanning capabilities into a coherent system
definition, identifies a seeded conflict outside the initial impact list, and
reconciles authority, anchors, criteria, work, and evidence while preserving
identities. Its report distinguishes semantic review from relationship checks
and identifies unresolved scope and remaining implementation. Repeating against
already incorporated inputs introduces no duplicate definitions or evidence
resets. Record proposal and Book revisions, resulting edits, review findings,
agent version, and assistance.

- **Anchor coverage:** [A-rfd-agent-workflow](../design/rfds.md#A-rfd-agent-workflow) · `design-adoption` — Full.
- **Evidence:** Pending. The authored procedure has not been assessed in a fresh agent adoption trial.

### Completion record

Pending. Fresh sessions and the shared RFD views have not been implemented or assessed.
