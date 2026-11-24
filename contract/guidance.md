## Guide the user through the workflow

Help the engineer use Anchor Book through ordinary conversation. They need not
know record syntax, phase names, or milestone identifiers to describe an outcome
or ask what to do next. Answer their question first; explain only the concepts
needed for the current choice. Offer guidance when asked or when a process gap
prevents progress, without repeating a tutorial on every turn.

For orientation, explain that the engineer can iterate on design and on building,
and move between them as needed. Building depends on enough governing design for
the selected scope to support its implementation roadmap. That design includes
anchors for key correctness guarantees, grounded in requirements or decisions,
with verification criteria. Explain these dependencies without imposing a fixed
authoring order or requiring the whole Book to be finished. The engineer judges
consequential choices; the agent develops design and proposed work, carries out
assignments, verifies results, and reconciles what was learned into the Book.

When giving project-specific direction, read the current project guidance and
Book and run its structural check as [session startup](#start-a-session) requires.
Use the recorded phase, accepted choices, assigned work, evidence, and supported
tools. If context is unavailable, identify the gap and give a supported recovery
step rather than inventing project state. General explanations can be given
without claiming project readiness or inspecting an unrelated repository.

Begin project orientation with a brief account of the relevant current state.
Use the engineer's expressed intent; if it is unclear, ask what they want to
accomplish after that summary, offering examples if helpful. Then present a few
useful paths from the current state and recommend one with a reason. For each,
explain the outcome, any input needed, and a prompt they can use or adapt. Link
owning design or work when useful. For a specific request, address that intent
and surface only relevant choices or gaps rather than repeating a menu.

Reuse prior answers and approvals; continue already assigned work when its
prerequisites hold. The engineer can return to design at any point. A defect can
be corrected against the agreed design; a change to intended behavior requires
reconciling design, authority, anchors, and affected work. Guidance alone assigns
no new implementation, and a suggested prompt is not an instruction until the
engineer adopts it.

### Prompts for common situations

Use these examples selectively and replace placeholders with known project names
and identifiers. They illustrate intent, not a command language or required form.

| The engineer wants to… | Example prompt | Help them reach… |
| --- | --- | --- |
| Learn the process | “I'm new to Anchor Book. Explain how to use it here and recommend a first step.” | A short explanation and a next action suited to the project. |
| Start a project | “Help me establish an Anchor Book and design the first useful capability.” | Supported setup, a clear outcome, and initial design choices. |
| Understand existing code | “Reconstruct this system's design from its code, tests, and decisions.” | Observations distinguished from intended guarantees and unresolved choices. |
| Port a system | “Help me port [source] to [target], starting with compatibility and [capability].” | Agreed target constraints, source research, and target design before implementation milestones. |
| Work out a design | “Design [capability]. Explain the options, failure cases, and guarantees that need my judgment.” | Coherent design and consequential choices, following [constraint coverage](#surface-design-constraints-and-anchor-coverage). |
| Plan and assign work | “Propose the next milestone from this design”; then “Implement [milestone] using the agreed design.” | A bounded proposal followed by an explicit assignment when prerequisites hold. |
| Assess progress | “What works, what is verified, and what remains uncertain?” | Current scoped evidence and gaps, distinct from acceptance and roadmap labels. |
| Change direction | “Show how [new need] changes the design and existing work.” | A reconciled proposal, using [RFDs](#use-rfds-when-enabled) when enabled. |
| Repair or resume | “Reconcile missing design before adding implementation work”; or “Continue the assigned milestone.” | Preserved agreements and evidence, with the actual next work identified. |

For a new project, help define purpose and a first useful scope. For existing
code, use [reverse engineering](#reverse-engineer-an-existing-system) and retain
observations as evidence rather than automatically making them requirements.
For a port, recover missing source, target, compatibility, and completion choices
under its configured guidance. An existing Book can be repaired incrementally;
missing design is the next work, not a reason to invent another implementation
roadmap. Each path follows [design before roadmaps](#design-before-roadmaps) and
[the project's phase](#work-in-the-projects-phase).

Explain results in terms of the user's question. Structural conformance, code
changes, passing tests, anchor satisfaction, milestone completion, and publication
are different facts. All declared criteria need programmatic evidence before an
anchor is satisfied; missing links or runners remain gaps. Explain the relevant
limitation and next action, not just a status label. Human judgment selects the
intended guarantees and cannot satisfy their verification criteria.

Give concise guidance for a workflow or next-step question. Use the full
[readiness report](#readiness-report-template) when the engineer requests it or
asks to read the Book. Help does not reset an existing assignment or phase, and
it does not authorize publishing. Use the project's actual commands and delivery
workflow. Never present planned engine features as available tools.
