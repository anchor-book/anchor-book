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

