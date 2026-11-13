## Work with the engineer

Iterate on design together, distinguishing unresolved proposals from accepted
decisions. Use component and sequence diagrams where they clarify behavior.
When the relevant design is clear enough, propose a roadmap for agreement.
A draft roadmap is not agreed work; a written design is not implemented behavior.

For implementation, read the assigned part of the agreed roadmap, its referenced
design, governing requirements and decisions, and completion criteria. Implement
and verify that scope. A roadmap's existence does not authorize starting or
expanding it. Explain unresolved prerequisites and resolve the affected scope
with the engineer.

## Use RFDs when enabled

An RFD (Request for Discussion) is a coherent proposal for a substantial design
change, including its motivation, intended behavior, alternatives, unresolved
questions, and implementation approach. Apply this process when the project's
Book configuration enables RFD support through its optional section mapping.
Use project guidance for the supported format and commands. An RFD folder alone
does not enable the process. Projects with support disabled retain their ordinary
design workflow; do not create proposals or enable the feature without an
assignment requiring that change.

Read the RFD and its supporting material, then the affected design, authority,
anchors, criteria, and roadmap work. One RFD can span many capabilities, and a
capability can be affected by several RFDs. Keep the proposal as one discussion
context while recording every affected relationship; shared definitions stay in
one place. A proposed addition must remain distinguishable from an existing
governing record.

Include proposed anchor definitions in the RFD: new anchors, complete proposed
revisions to existing anchors, and explicit retirements with their rationale.
State properties, authority references, verification criteria, required methods,
and proof scope where applicable. Include proposed authority when needed and
distinguish it from current records. Preserve existing IDs for the same property
or criterion. If no anchor changes, say so and explain why.

Keep these definitions in the proposal scope specified by the supported format.
Marking an ordinary anchor draft does not isolate it from the current goal.
Review the actual proposed guarantees, then explicitly incorporate the agreed
definitions into their owning chapters and reassess affected evidence. Keep the
reviewed proposal identifiable; its retained definitions are not a second live
copy. Acceptance alone performs no incorporation or verification.

Develop and review the proposal with the engineer. Record disposition and its
basis through the project's review workflow; a draft, an accepted proposal, an
assigned implementation, and verified completion are different facts. RFD
acceptance does not accept linked records automatically, supply anchor authority,
or authorize unassigned work. Prototyping assignments retain their stated scope.

When the change is agreed, follow [the adoption procedure](#adopt-an-rfd-into-the-system-definition)
to reconcile the live system definition and affected work in one reviewed change.
Use linked milestones for tasks, dependencies, and evidence; the RFD presents
their progress without maintaining a second completion checklist. If findings
change an accepted proposal's scope, seek renewed agreement or a successor RFD
and preserve the earlier decision and its revision. Keep rejected, withdrawn,
and superseded proposals identifiable without treating them as current goals.

Report proposal questions and affected work separately from implementation and
verification results. Use only checks the project actually supports; written
RFD guidance does not establish that the engine validates its format or links.

### Adopt an RFD into the system definition

For an assigned adoption, prepare the resulting system definition so an engineer
can review what the system is meant to be after the change. Follow these steps:

1. Establish the RFD revision, agreed scope, and current Book baseline. Use
   agreement already provided; do not infer it solely from a merge or navigation
   label when other records disagree. Identify unresolved choices without asking
   again for decisions already made.
2. Read the affected design and its dependents, including requirements, decisions,
   anchors, criteria, linked work, and related proposals. Examine effects beyond
   the RFD's stated links: shared data, ownership, ordering, failure behavior,
   terminology, and guarantees. Report missing design coverage and conflicting
   assumptions; valid links alone do not establish semantic consistency.
3. Map each agreed claim to its design destinations and proposed anchor additions,
   revisions, or retirements. Identify changed authority and verification
   obligations. Resolve intervening changes to the target against the reviewed
   proposal; raise choices that would change the agreement with the engineer.
4. Prepare the coherent design, authority, and anchor edits together, preserving
   identities and one canonical definition per property. Keep unresolved
   alternatives in proposal scope. Readers must be able to understand the
   resulting system without reconstructing it from successive RFDs.
5. Reconcile linked roadmap work and evidence for every affected area. Separate
   definition updates from remaining implementation work; use code and tests to
   assess that gap, not to redefine the goal. Preserve historical evidence while
   reassessing changed criteria. Adoption alone authorizes no unrelated coding.
6. Run supported structural and rendering checks and relevant verification for
   changed criteria or links. Review the resulting Book and its interactions;
   report structural results separately from semantic assessment and test
   coverage. Present the concrete change through the project's review workflow.
7. Record the reviewed source and incorporation references. Report changed
   guarantees, conflicts resolved or outstanding, coverage limits, checks, and
   implementation work still needed. An already incorporated proposal should
   require no duplicate definitions, new IDs, or evidence resets on repetition.

Do not claim full adoption while an unresolved conflict changes its meaning or
part of its agreed scope is unmapped. Preserve the RFD's coherent change context
and distinguish review acceptance, design incorporation, and implementation
completion.

## Work in the project's phase

The project phase sets working expectations for the current development iteration.
Read the phase from `[project].phase` in the project's `anchorbook.toml` when
configuration is in use. Otherwise, use the explicit phase declaration in the
project's Book. Keep one authoritative phase setting; do not infer it from the
presence of draft records or unfinished code.

Every phase repeats **design, agree on a roadmap, build and verify, reconcile
what was learned**. Phase, record acceptance, milestone progress, and verification
evidence describe different things; one does not automatically establish another.

| Phase | Working expectations |
| --- | --- |
| **Prototyping** | Draft designs can guide assigned experiments. Evolve design, schemas, roadmaps, and implementation together. Even a provisional roadmap states the experiment's scope, intended learning or outcome, and assessment. Record results, changes, and unresolved questions. |
| **Building** | Before assigned implementation starts, its governing design, requirements, decisions, and anchors must be accepted. Later design changes may remain drafts. Agree on changes to governing design before implementing them. |
| **Complete** | The defined goal is implemented for a recorded scope and revision, with evidence satisfying its verification criteria. Preserve that baseline; further development starts another Prototyping or Building iteration. |

Prototyping does not waive record formats, valid references, assigned scope, or
truthful reporting. Successful experiments do not automatically accept designs,
verify anchors, or complete milestones.

The engineer directs phase transitions. Never change phase to bypass a
prerequisite. Entering Building does not accept draft records; recording Complete
requires scope, revision, completed work, and verification evidence in the
roadmap. Configuration or a structural check alone cannot establish completion.

A scoped prototype can coexist with an accepted system. State its boundaries in
the agreed roadmap and assignment, preserving the accepted baseline and its
obligations. Do not change the entire project's phase to loosen unrelated work.

