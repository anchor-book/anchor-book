## Work in the port phase

Port an existing system into an agreed target implementation using the
[reverse-engineering workflow](#reverse-engineer-an-existing-system). The Book
defines the target; source behavior supplies evidence for design choices.

Read available project material and prior agreements first. Ask the engineer for
missing intent, constraints, references, and known problems: why port, which source
and baseline to use, the target environment, compatibility expectations, priority
capabilities, and what constitutes completion. Explain consequential choices with
a recommendation. Reuse answers already given and record agreed answers in the
Book. Continue investigation that does not depend on unresolved choices.

Identify the source and target in project guidance and record inspected revisions
in research evidence. Define the selected compatibility scope explicitly. Choose
interfaces, architecture, and dependencies suited to the target environment;
source package boundaries and implementation mechanisms do not prescribe them.
Follow [design before roadmaps](#design-before-roadmaps): write the target design
for a capability from research and agreed choices before proposing its
implementation milestones. Source behavior recorded only in a roadmap or
migration inventory is not target design.

Scoped experiments may use draft designs to resolve questions. Assigned
implementation requires accepted governing design and authority, as in Building;
Port does not treat every task as an experiment. Apply the implementation testing
requirements to that work. Verify selected behavior and deliberate differences
with representative scenarios, including failures, against the target criteria.
Source tests inform these checks without defining the target's requirements.

Keep migration inventories and comparisons separate from durable target design.
Ongoing parity, source tracking, and dependencies on the original system require
explicit project scope. Before copying or adapting material, establish applicable
permissions and preserve required notices and provenance, even if migration notes
are later removed.

Agree on the port's completion criteria. Account for selected capabilities,
deliberate differences and deferrals, remaining work, and verification evidence;
the target design must be understandable without reconstructing the source or
roadmap history. The engineer directs leaving Port and selects the next phase.
Entering Complete requires its usual evidence; cutover and source retirement are
separate decisions. A milestone's completion alone does not end the port.
