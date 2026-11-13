# Requests for Discussion

An RFD is a coherent proposal for a substantial design change. It carries the
problem, motivation, proposed behavior, alternatives, unresolved questions, and
relationships needed to discuss and implement that change across capabilities.
It remains identifiable through review and implementation. Design chapters
describe the goal system; an RFD explains a proposed or agreed change to it.

## Anchors

> **`A-rfd-optional-structure`** (draft): **An RFD section brings a defined process without making every Book use RFDs**
> Property: RFD support is enabled explicitly by the optional book.sections.rfds mapping. Without it, a Book retains its existing structure and process even if an rfds folder exists. When enabled, the configured section must satisfy its navigation and proposal rules; missing, ambiguous, escaping, or unreadable RFD structure cannot produce a successful RFD check. Directory names never activate the capability or determine its role.
> Authority: `R-coherent-design-changes`; `D-explicit-configuration`; `D-mdbook-structure`
> - Criterion `optional-section` [test]: Omitted configuration leaves RFD support off with or without an rfds folder, preserving ordinary Book checks. A configured section with a local top-level index and valid RFD navigation passes; missing or nested indexes, duplicate mappings, unlisted proposal roots, and unreadable or escaping paths produce located findings or incomplete results. A malformed mapping is invalid input, not disabled support; omitted support never reports RFD checks as evaluated.
> - Criterion `alternative-layout` [test]: Equivalent RFD Books under conventional and explicitly mapped alternative directories produce equivalent roles and findings regardless of caller working directory or display titles.
> - Criterion `schema-compatibility` [test]: Schema 1 retains its existing outcomes and rejects an RFD mapping. Schema 2 enables RFDs only when mapped. Unsupported configuration or RFD format versions are incomplete input, never ignored or rewritten; removing the mapping preserves proposal sources and existing current-record behavior.

> **`A-rfd-shared-relationships`** (draft): **A design change connects to every affected part of the Book**
> Property: RFDs participate in the engine's shared typed Book relationship model with stable identities and located references. One RFD may affect several design sections, anchors, criteria, requirements, decisions, roadmaps, and milestones, and each may relate to several RFDs. All consumers use the same validated relationships and derived backlinks. Proposal relationships confer neither authority nor verification, and invalid current-Book references never enter a successfully validated model.
> Authority: `R-coherent-design-changes`; `R-traceable-properties`; `D-typed-check-results`
> - Criterion `many-to-many` [test]: Independent fixtures connect two RFDs to overlapping design sections, records, criteria, and milestones. Forward links and backlinks agree across input ordering and callers; moving or retitling an RFD preserves its identity and relationships.
> - Criterion `typed-boundaries` [test]: Reject duplicate RFD IDs, missing, ambiguous, or wrong-kind current targets, and self-supersession or supersession cycles with located findings. Explicit planned additions and revision-qualified historical citations remain distinguishable from resolved current targets. An RFD reference cannot replace required authority, satisfy an anchor criterion, or weaken existing authority validation.

> **`A-rfd-proposed-anchors`** (draft): **Reviewers can assess proposed guarantees before changing the goal**
> Property: An RFD contains the proposed definitions of new anchors and revisions to existing anchors, including authority and verification criteria, and explicitly identifies proposed retirements. These changes are scoped to the proposal and remain distinct from the current Book's anchor definitions, including draft definitions. Several RFDs may propose alternatives for one current anchor without creating duplicate governing records. Changing proposal disposition alone cannot alter the goal or its evidence; incorporation is an explicit reviewed change preserving record identity and reassessing changed criteria.
> Authority: `R-coherent-design-changes`; `R-traceable-properties`; `R-truthful-results`
> - Criterion `complete-proposals` [test]: Read canonical metadata and proposed additions and full replacement definitions with their properties, explicitly bound current or proposed authority references, criteria, required methods, and any proof scope. Diagnose malformed definitions, misplaced or unclosed blocks, ambiguous references, and conflicting operations within a proposal. A retirement names the baseline anchor and its rationale; an RFD with no anchor changes says so explicitly. Retained proposals with explicit historical references remain readable with their validation limits, without requiring retired targets in the current inventory.
> - Criterion `proposal-isolation` [test]: Fixtures contain a current anchor and two RFDs proposing different replacements under its ID, plus proposed anchors and authorities. Each proposal is independently addressable and resolvable; the current inventory, authority validation, verification selection, and completion claims remain unchanged. A current anchor cannot cite proposal-only authority as if it were current.
> - Criterion `explicit-incorporation` [test, review]: Accepting or rejecting a proposal leaves current definitions untouched. In a reviewed fixture change, incorporate the selected additions, revisions, and retirements into the goal and roadmap, preserve existing identities, and reassess affected evidence. Retain the proposal as a versioned account of what was reviewed; it never becomes a second live definition. Review checks the proposal against the current target and resolves intervening changes before incorporation.

> **`A-rfd-distinct-progress`** (draft): **Proposal acceptance and implementation evidence remain distinct**
> Property: An RFD's review disposition describes agreement on its proposal, while linked roadmap milestones and criterion results describe implementation and evidence. Accepting, rejecting, withdrawing, or superseding an RFD does not change linked record statuses or evidence. Its progress view derives from linked work and reports missing, partial, unchecked, and stale results without maintaining a second completion checklist.
> Authority: `R-coherent-design-changes`; `R-truthful-results`; `D-typed-check-results`
> - Criterion `independent-states` [test]: Fixtures include an accepted RFD with proposed work, partially complete linked milestones, a draft with prototype evidence, and rejected or superseded proposals. Views preserve each state independently; no review label or missing evaluation becomes a completion claim or passing criterion.
> - Criterion `consistent-views` [test]: Library, agent-facing output, and rendered RFD views agree on proposal identity, affected scope, proposed anchor changes, linked progress, evidence applicability, and limits for identical inputs. Current and proposed definitions remain visibly distinct, without modifying sources or rerunning verification during rendering.

> **`A-rfd-agent-workflow`** (draft): **Agents use RFDs as the shared context for design changes**
> Property: When a Book enables RFDs, the packaged AI Working Contract directs agents to read their proposal context, report pending decisions and linked work, and adopt agreed changes into the live system definition by reconciling all affected design, authority, anchors, and roadmaps. The engine supplies templates matching its supported RFD format without creating project content. Disabled RFD support imposes no proposal ceremony. Contract and template retrieval remain project-independent; authors and reviewers apply conditional guidance from the Book's explicit configuration.
> Authority: `R-coherent-design-changes`; `R-project-adoption`
> - Criterion `conditional-guidance` [test, review]: The actual contract command returns the same packaged guidance with RFD support enabled or disabled. Fresh agent sessions use configuration to identify whether the workflow applies, including disabled support with an rfds folder present and enabled support at an alternative location. They distinguish proposal acceptance from assigned implementation and report affected scope and unresolved decisions. A cross-capability assignment preserves the coherent proposal while reconciling every named affected area; an ordinary change with RFD support disabled requires no invented RFD.
> - Criterion `design-adoption` [review]: In a fresh agent assignment, a reviewed RFD changes several capabilities and has a seeded conflict outside its initial affected list. The agent identifies that conflict, prepares a coherent resulting system definition for review, preserves stable identities, reconciles criteria and work, and reports unresolved scope and implementation gaps. Repeat against an already incorporated revision without duplicate definitions or evidence resets; distinguish declared relationship checks from semantic review and record assistance.
> - Criterion `template-delivery` [test]: The actual template commands return the packaged index and proposal templates identically without project configuration, a Book build, network access, or file writes. Unknown names and extra arguments return usage errors. Filling templates with independent fixture values yields a valid enabled Book through the real checker; unfilled placeholders do not pass as finished input.

## Optional Book structure

Design, Authority, and Implementation Roadmaps remain required. RFDs are an
optional Book capability, enabled and located by one configuration setting:

```toml
[book.sections]
rfds = "rfds/index.md"
```

This optional mapping in `anchorbook.toml` is the switch; no separate enabled
boolean, Cargo feature, or plugin package is required. Omit it to leave RFD
support off. Folder presence alone does not activate the process or RFD checks.
Ordinary navigation and record rules still apply, and disabled RFD checks must
not be reported as passed. A malformed declaration is an error rather than an
implicit request to disable support.

[Configuration schema 2](rfd-format.md#configuration-and-compatibility) introduces
the mapping; schema 1 retains its existing meaning. The
[source format](rfd-format.md) defines index ownership, metadata, typed changes,
version handling, and the engine's template delivery.

Resolve the configured index relative to the Book source under the ordinary path
rules. It is a distinct top-level local chapter in `SUMMARY.md`; proposal roots
and supporting chapters appear beneath it. Any contained directory can be mapped;
titles and filenames do not define identity. An RFD directory outside the mapped
section remains ordinary documentation or migration input, not another implicitly
enabled collection.

An index can contain no proposals. Each proposal has one root document and may
have supporting chapters, examples, or proposed user documentation. The section
inventory distinguishes these from proposal roots and diagnoses unlisted roots.
Enabling support establishes a process, not permission to accept a proposal or
change the goal. Checking does not create directories, move content, or run tools.

## Proposal content and lifecycle

Symposium's existing RFD template is the authoring starting point: summary,
motivation, change overview, detailed design with supporting chapters, questions
and alternatives, and implementation approach. Preserve that coherent narrative
while adding explicit identity, disposition, and relationship fields. Link the
maintained roadmap for implementation tasks and status, and include a Proposed
anchors section with the actual changes described below. The RFD roadmap records
the reference template and implementation work.

An RFD has a stable meaningful identity, title, disposition, problem and
motivation, proposed change, affected scope, alternatives, unresolved questions,
and links to implementation work where defined. Review outcomes identify their
basis and scope. Existing source RFD identities and URLs remain traceable during
adoption. [RFD format 1](rfd-format.md#proposal-metadata) defines the canonical
encoding and explicit proposal scope, with versioned migration rather than
extracting relationships from prose. [Worked examples](rfd-examples.md) show
draft, accepted, and incorporated forms.

The review dispositions are draft, accepted, rejected, withdrawn, and superseded.
Discussion happens while draft. Supersession names the succeeding proposal;
it is not implicit from a newer file date. Acceptance is scoped to the reviewed
change, not blanket acceptance of every linked record or permission to implement
unassigned work. A substantive change to an accepted proposal needs renewed review
or a successor RFD; preserve the earlier scope and outcome through revision links.
Snapshot validation cannot establish that a human reviewed a change or reconstruct
unrecorded transitions.

The process is:

1. Discuss the problem and proposed change as one unit, including effects across
   capabilities and any conditional work or compatibility concerns.
2. Identify the affected design sections, authority, anchors, criteria, and work.
   Write the proposed anchor definitions and changes, including their authority
   and verification criteria; they do not enter the current goal by implication.
3. On agreement, reconcile the goal and roadmap in the same reviewed change.
   Shared definitions have one owner and links from every affected area.
4. Implement assigned milestones, using the RFD for rationale and the roadmap for
   tasks and evidence. Revisit the proposal and affected goal when findings change
   the design; do not silently narrow criteria to complete the RFD.
5. Present implementation progress through linked roadmap outcomes and evidence.
   Keep the RFD as a coherent change record after implementation and when superseded.

RFDs are useful for substantial Rust system changes such as API, protocol,
ownership, or lifecycle changes. Their presence does not require one for every
bug fix. Projects define review authority and when a proposal is warranted in
their working guidance. This process requires neither a skill nor another agent.

## Adopting a reviewed RFD

Adoption updates the live system definition so readers can understand the agreed
system without reconstructing it from a sequence of RFDs. The agent follows the
contract's adoption procedure and prepares a reviewable change to that definition.

It identifies the reviewed proposal revision and current Book baseline, reads
the affected definitions and their dependents, and examines interactions beyond
the RFD's declared links. Existing guarantees, shared terminology, data ownership,
ordering, failure behavior, and other active proposals can expose missing effects
or conflicts. Relationship checks validate declared links; AI and human review
assess semantic consistency and whether the impact map is complete.

Map every agreed claim to its design destination and proposed anchor changes.
Resolve contradictions under the existing agreement, raising new choices with
the engineer without asking again for settled decisions. Prepare the resulting
design, authority, anchors, criteria, and roadmap changes together. Shared
definitions remain canonical; RFDs retain the change context and provenance.
Unresolved alternatives stay in proposal scope and prevent claiming full adoption.

Report the resulting definition, changed guarantees, reviewed conflicts,
unresolved coverage, checks performed, and implementation work still outstanding.
Current code and tests help assess implementation gaps; they do not redefine the
agreed goal. Adoption can precede implementation. A status edit or merge alone
does not perform it, and an adoption assignment does not authorize unrelated code
changes. Repeating adoption against already incorporated inputs makes no duplicate
records and resets no evidence. Record source and incorporated revisions so later
changes can be reconciled against what was actually reviewed.

## Proposed anchors

The RFD's Proposed anchors section gives reviewers the guarantees they are being
asked to agree to. It covers three operations:

| Operation | Required proposal content |
| --- | --- |
| Add | The complete new anchor: intended ID, title, property, authority references, criteria, required verification methods, and proof scope where required |
| Revise | The existing anchor ID and reviewed base revision, the complete proposed replacement, and an explanation of changed guarantees or criteria; keep IDs for unchanged properties and criteria |
| Retire | The existing ID and reviewed base revision, why the property is removed or superseded, and affected references, work, and evidence needing reconciliation |

An RFD affecting several capabilities includes all its proposed anchor changes,
in the root or linked proposal-owned chapters. When no anchor changes, state that
and why; do not invent a guarantee for a process-only proposal. A not-yet-resolved
property is an explicit design question, not a complete proposed anchor.

Use the canonical anchor field meanings within an explicit proposal scope.
Proposed requirements or decisions can be carried in that same scope when the
new anchors need them; distinguish those targets from current Book authority.
Check proposed definitions and their references without inserting them into the
current record inventory. A `draft` status alone does not provide this isolation:
ordinary draft anchors are still current goal records.

The model identifies a proposed definition by its RFD and intended anchor ID,
with source location and review revision. Two RFDs can offer different versions
of the same anchor. A replacement links to its current target as well as carrying
the proposed content. Current verification and authority references resolve only
against current records; prototype evidence for a proposal must remain explicit
about that proposal's scope and revision.

When the reviewed change is incorporated, add or revise the canonical definitions
in their owning design chapters and reconcile retirements, authority, roadmap
work, and affected evidence. Resolve any intervening changes to the target first.
Keep the reviewed proposal and its connection to the incorporated definitions;
its historical copy is not another current definition. RFD acceptance does not
perform this incorporation or mark criteria verified automatically.

## Shared relationship model

The Book model includes typed identities for RFDs alongside design sections,
anchors and their criteria, requirements, decisions, roadmaps, and milestones.
RFDs have their own payload and disposition type. They are not encoded as
requirements or decisions, nor added to the allowed authority kinds of an anchor.

| Relationship | Meaning and constraint |
| --- | --- |
| RFD affects a current Book entity | A located, typed link to every affected section, record, criterion, or work item; the target must resolve in the current inventory |
| RFD proposes an addition | An explicitly scoped planned entity, separate from current records; it cannot satisfy current authority or evidence references |
| RFD proposes an anchor change | A typed add/revise/retire operation carrying the proposed definition or retirement rationale, its intended ID, and the current/base target where applicable; proposal payloads are isolated from the current inventory |
| RFD is implemented by milestones | Links to the maintained tasks and evidence; progress is derived from those milestones |
| RFD supersedes an RFD | Explicit successor relationship; reject self-links and cycles |
| Record or section derives from an RFD | Provenance for the choice or change; it does not replace an anchor's authority edges |
| RFD cites a historical revision | A revision-qualified source citation, displayed as historical and outside current-target validation; no implicit current evidence |

The shared model retains source locations and separates authored edges from
derived backlinks. Several proposals can affect the same record without
duplicating its definition. Conflicting proposals are shown together for review;
the engine cannot determine semantic agreement from links alone. Retired or moved
material must preserve identity or change a link explicitly to historical
provenance; missing current targets cannot silently become historical ones.

Compose this model from the existing record and roadmap readers and a distinct
RFD reader. The record inventory remains responsible for authority validation;
the broader index resolves relationships across capabilities. Adapters derive
their views from that index, without separate parsers or a persistent graph store.
Extending the relationship model does not extend the scope of the existing
authority proof. Tests and review cover the new relationships unless a future
criterion explicitly requires proof.

## Agent and rendered views

The contract is the same packaged text for every project. It includes conditional
RFD guidance; agents apply it when the configuration enables the capability. The
readiness report shows proposal disposition, affected scope, unresolved decisions,
and linked roadmap status separately. It reports unsupported engine checks as
unavailable, never as passed. No project-sensitive contract command is required.

An RFD view presents the proposal and its supporting material, affected entities,
proposed anchor definitions and changes, related proposals, and derived work
progress together. Current and proposed definitions are labeled distinctly.
The same model supplies
agent-readable output and human navigation. Evidence still comes from existing
verification results for the evaluated inputs; rendering performs no verification.

## Adoption

Adopt existing RFDs as coherent proposals with their source identity, review
history, supporting material, and mapped relationships. Reconcile their design
effects across the Book and move active implementation tracking into linked
roadmaps. Drafts can live in the RFD section without becoming agreed goal design.
Source acceptance, imported-record review, and current evidence remain distinct.

Repeat imports against exact source revisions, preserving deliberate Book edits,
identities, and evidence. Resolve later source changes across every affected
destination. At final adoption, agree ownership and preserve RFD URLs and
discussion; do not reduce the RFD to an archive entry merely because design text
has destinations. The Book's project guidance defines its contribution workflow.
