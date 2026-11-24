## Reverse engineer an existing system

When adopting, documenting, reworking, or porting an existing system, reconstruct
its relevant behavior and purpose within the assigned scope. Work incrementally;
a bounded investigation need not recover the whole system.

1. **Establish the evidence.** Record inspected source revisions and relevant
   documentation, interfaces, implementation, tests, and examples. Trace
   representative workflows through component responsibilities, data and control
   flow, state, lifecycle, failure paths, and external dependencies as relevant.
   Cite sources and distinguish documented intent, conclusions from source
   inspection, and executed observations. Record contradictions and unknowns;
   reading a test does not establish that it passes.
2. **Define the goal.** Use existing agreements to identify behavior to preserve,
   change, omit, or defer, with reasons. Observed behavior, including defects,
   does not automatically become a requirement. For documentation of an existing
   system, establish which behavior is intended to remain. Raise unresolved
   choices with the engineer; keep proposals distinct from accepted requirements.
3. **Write the coherent design.** Put the target system's purpose, interfaces,
   responsibilities, interactions, and constraints in their owning design
   chapters. Record governing requirements and decisions, with anchors and
   verification criteria for the resulting guarantees. Incorporate agreed design
   found in research or roadmaps. A reader must be able to understand the target
   without reconstructing it from source code or roadmap history.
4. **Derive and reconcile the work.** Compare implementation and evidence with
   that design to propose milestones. Follow [the project's phase](#work-in-the-projects-phase):
   scoped experiments can resolve design questions. As findings settle choices,
   update the owning design and affected records and roadmaps together. Replace
   superseded descriptions in place and duplicated roadmap definitions with
   links. Roadmaps retain tasks, remaining gaps, and completion evidence; research
   retains source observations and comparisons.

### Porting to a different implementation

Define the selected compatibility scope and source baseline explicitly. Choose
interfaces, architecture, and dependencies suited to the target environment;
source package boundaries and implementation mechanisms do not prescribe them.
Verify selected behavior and deliberate differences with representative scenarios,
including failures, against the target criteria. Source tests inform these checks
without defining the target's requirements.

Keep migration inventories and comparisons separate from durable target design.
Ongoing parity, source tracking, and dependencies on the original system require
explicit project scope. Define port completion and any cutover or retirement
separately. Before copying or adapting material, establish applicable permissions
and preserve required notices and provenance, even if migration notes are later
removed.

