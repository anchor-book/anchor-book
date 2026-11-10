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

