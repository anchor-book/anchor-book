# Implementation Roadmaps

Implementation roadmaps turn a sufficiently clear design into agreed work for
AI agents.

We develop the design with AI. When it is clear enough to implement a capability,
the AI proposes a roadmap from that design: the results to build, how to divide
the work, the dependencies, and how completion will be demonstrated. We review
and refine that proposal together and agree on what the AI agents will do before
they begin the affected implementation work.

The entire system's design does not need to be finished before a roadmap can be
created. The relevant part must be clear enough to define the work and its
completion criteria.

## Defining the work

Give each roadmap a short, descriptive name and a goal that states what becomes
possible when the roadmap is complete. Keep work outside that goal for a separate
roadmap rather than extending bootstrap to cover the entire product.

Group work into chunks that enable an outcome an engineer can take on with
their agents. Call these chunks milestones. Organize around what
becomes possible when the work is done, with tasks describing how to get there.

Each chunk records:

- **Identifier:** a stable reference unique across the Book's roadmaps.
- **Outcome:** the capability or result the work enables.
- **Design basis:** the relevant design, governing authority, and anchors where defined.
- **Dependencies:** prerequisites and other chunks required for this result.
- **Tasks:** bounded work that advances the outcome.
- **Complete when:** observable criteria that establish the outcome.
- **Status and completion record:** current progress, followed by evidence when complete.

Chunks may be decomposed into smaller outcome-based chunks when needed. Keep
detailed tasks and evidence in one place and link to them from the parent.
Different chunks can advance the same anchor, and one chunk can advance several
anchors. Anchor relationships provide traceability; they do not dictate work
boundaries.

An engineer's assignment carries ownership and coordination. Owner fields and
issue links are not required in the roadmap. Optional tracking can be added
when useful without moving the agreed scope and completion criteria out of the Book.

## Naming and references

Refer to work by roadmap name and milestone number, such as **Engine Bootstrap —
Milestone 1**. Its descriptive title states the outcome.

Assign a unique roadmap identifier, such as `engine-bootstrap`, and give each
milestone an identifier such as `engine-bootstrap-1`. Record it in the milestone
and provide a matching link target. Preserve identifiers and milestone numbers
when reordering work or refining titles; do not reuse a retired identifier.
Assign a new unused number to new work. These are work references, not anchors
that define system properties.

## Recording progress

Use these statuses consistently:

| Status | Meaning |
| --- | --- |
| Proposed | Outcome and work are drafted for review |
| Ready | Scope and criteria are agreed, and prerequisites are satisfied |
| In progress | Assigned work has begun; remaining tasks and gaps are recorded |
| Blocked | A stated obstacle prevents further progress on the chunk |
| Complete | The outcome and all completion criteria have supporting evidence |

Update the Book as tasks progress. Finishing a task does not automatically
complete its chunk. When marking a chunk Complete, record the relevant revision,
checks or observations, their results, and any limits on the claim. Identify
whether evidence describes a branch or work integrated into main.

Keep code, tests, and corresponding Book updates together in the project's
chosen contribution workflow. The roadmap model does not mandate direct pushes
or pull requests. Scope expansions and changed completion criteria must be
reconciled with the engineer, rather than silently changed to match the work done.

## Keeping the agreement current

An AI-generated roadmap is a proposal until we agree on it. If the design changes
or implementation reveals a new constraint, the AI proposes the corresponding
roadmap changes for review. The agreed roadmap remains the basis for agent work.

Verification evidence establishes whether a capability works; agreement on a
planned milestone alone does not establish completion.

## Current roadmaps

- [Engine Bootstrap](bootstrap.md) prepares the Book's design, authority, anchors,
  working contract, and agreed roadmap so an agent can use it to build the checker.
- [Structural Checking](structural-checker.md) enables an agent to use checker
  feedback to repair a Book's structure and recheck until it passes, supported
  by independent engine tests and continuous checks on this Book.

Both are proposals for review. Later capabilities need their relevant design
and detailed work agreed before implementation begins. The Book owns these
roadmaps now; engine automation is not a prerequisite for maintaining them.
