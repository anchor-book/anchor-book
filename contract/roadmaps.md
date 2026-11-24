## Design before roadmaps

Before proposing milestones to implement a capability, establish its coherent
target design. Explain the intended behavior, interfaces, interactions,
constraints, and relevant failure cases in the owning design chapters, with the
governing requirements, decisions, anchors, and verification criteria. A design
basis must explain the milestone's intended outcome; a heading, record inventory,
source description, or list of tasks is insufficient.

Research and design milestones may establish that basis. Give them an agreed
question or outcome, scope, and assessment. Scoped experiments may use draft
design under [the project's phase](#work-in-the-projects-phase). Incorporate
findings and agreed choices into the owning design before proposing implementation
milestones. Work capability by capability; the whole system's design need not be
finished first.

When RFDs are enabled, proposed work may reference the design in its RFD. Follow
[the adoption procedure](#adopt-an-rfd-into-the-system-definition) to incorporate
agreed definitions before that work is ready for implementation.

Keep source inventories and exploratory notes in the research location named by
project guidance. If none is defined, propose a suitable location and record it
in that guidance; ask the engineer when the choice affects agreed structure or
policy. Roadmaps summarize scoped findings and link evidence; research does not
substitute for target design.

## Create and maintain implementation roadmaps

Give each roadmap a simple name and an explicit goal. Divide the intended design
into milestones: coherent chunks an engineer can take on with their agents and
complete by demonstrating an outcome. Do not divide solely by file, component,
or anchor.

Each milestone states its outcome, relevant design and authority, dependencies,
tasks, and observable completion criteria. Write roadmaps in the engine's roadmap
format, in plain Markdown without HTML; presentation is the renderer's job. Read and follow the roadmap conventions
identified in project guidance for stable milestone and criterion identifiers,
anchor references, coverage, and evidence.
Link affected anchors and specific criteria; explain the scope of partial coverage.
Keep verification requirements in the anchors rather than copying them into
milestones. Preparation criteria may have no anchor reference; do not invent
anchors merely to organize work.

Make the next milestone assignable; leave unsettled later work explicitly
proposed. Review changes with the engineer before treating them as agreed.
Ownership belongs with the assignment; owner fields and issue tracking are optional.

When work implements an RFD, link the proposal and the affected milestones.
One proposal can coordinate several roadmaps, and a milestone can address more
than one proposal. Keep tasks and completion evidence here; the RFD links to
their current state and preserves the change's rationale and review context.

Update Book progress, remaining work, and evidence alongside code and tests.
Record evidence against the milestone's completion criteria, including tested
revisions, checks or observations, results, supporting records, limitations, and
whether the work is on a branch or integrated into main. Mark Complete only when
the outcome and every criterion hold. Report gaps rather than weakening criteria.
Milestone completion does not automatically verify linked anchors.

The Book tracks progress toward the goal system; it is not a journal. Update
existing milestone tasks, status, and evidence in place rather than adding
session summaries or follow-up narratives. Keep only the evidence and context
needed to assess completion, understand remaining work, and distinguish tested
results from pending claims. Use Git history for the chronology of changes.

Follow the project's contribution workflow. Roadmap work does not itself
authorize pushing, merging, or unrelated changes.

