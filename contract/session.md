# AI Working Contract

An Anchor Book defines the goal system: the system the engineer and AI agents
are working together to build. The design describes that system as a whole.
Its anchors define properties that must hold once the system is built, grounded
in requirements or decisions and linked to verification criteria.

Use the Book to understand the intended system and code and verification evidence
to understand its current state. When they differ, identify the gap; do not
silently redefine the goal to match the implementation.

The engineer and agents iterate on design, then the agents propose an
implementation roadmap. Once agreed with the engineer, it defines the work,
dependencies, and evidence needed to demonstrate completion. Design and anchors
establish the goal; the roadmap describes how to reach it.

This contract defines how agents read and use an Anchor Book, collaborate with
the engineer, and carry out assigned roadmap work. It is reusable guidance owned
by the Anchor Book engine and includes Rust development expectations. Project
state, paths, commands, and local policies belong in the project's Book, not in
this contract. Repository bootstrap instructions identify where to read that
project guidance.

## Start a session

Read this contract, then the project guidance identified by the repository's
bootstrap instructions. Use that guidance to locate the Book source, current
phase, authoring references, and development commands. Read the Book's `SUMMARY.md`
and every chapter it lists in full, using current Markdown sources so unpublished
changes are included; no Book build is needed.

Read the content, not just headings or filenames, and continue beyond truncated
tool output. Report missing or unreadable guidance and the resulting context gap;
do not guess project paths or claim to have read unavailable content.

Run the project's structural check, as described in
[Check the Book's structure](#check-the-books-structure), and include its result
in your readiness report. A Book that fails or cannot be checked is still read,
but its findings qualify what you report about it.

After reading, give the engineer a readiness report using the
[required template](#readiness-report-template), so they can see the project's
state and choose what to work on. Keep its headings, tables, and level of detail;
a brief summary does not replace the report.

When the request is only to read the Book, the readiness report is the whole
response: report check findings and the repairs you would make rather than
making them, and do not start self-selected work. When the request is to read the
Book and start named work, give the same report, then start that work. Identify
the milestone or design it belongs to and briefly state its scope and constraints
before changes. Resolve unmet prerequisites with the engineer, as
[Work with the engineer](#work-with-the-engineer) describes, before proceeding.
For any other request, state its scope and constraints before making
changes. Do not ask again for direction or agreement already given in the session.

### Readiness report template

Use the following Markdown structure, replacing bracketed instructions with the
project's facts. Repeat the roadmap section for every roadmap with a milestone
that is not Complete, and the milestone detail section for every such milestone.
Include every completion criterion, even when all are Pending. Keep wording
concise without omitting entries. If the report needs multiple messages, continue
it rather than silently dropping later roadmaps or criteria.

```markdown
# Readiness report

## Context

- Checkout: [project, branch and revision when available; note local changes]
- Read: [contract retrieval command, project guidance, configuration, Book index,
  and chapters read; identify the Book source and number of chapters]
- Context gaps: [missing or unreadable material and its effect, or None; distinguish
  references outside the Book that were not read]

## Current state

- Phase: [recorded phase and its source]
- Structural check: [command, result, evaluated scope, and remaining findings;
  state what the check does not establish]
- Book coverage: [design areas covered and acceptance status of governing records]
- Implemented: [what the Book records as implemented, with source references]
- Verification: [recorded evidence, coverage gaps and limitations, with source
  references and tested revisions where recorded; distinguish checks run now]
- Open questions: [unresolved design choices or obstacles, or None recorded]

## Open work

### [Roadmap title and identifier]

| Milestone | Title | Recorded status | What holds it or makes it assignable |
| --- | --- | --- | --- |
| [identifier] | [title] | [status] | [dependencies, agreement or obstacle; Ready if recorded and prerequisites hold] |

#### [Milestone identifier]: [title]

| Completion criterion | Recorded evidence and result | Remaining gap or limitation |
| --- | --- | --- |
| [identifier and short description] | [result and evidence reference, or Pending / Not recorded] | [gap or None recorded] |

- Tasks: [summarize completed and remaining tasks separately]
- To proceed: [unmet prerequisites or next step within an existing assignment]

### Ready milestones

[List the identifiers of milestones recorded as Ready, with any unmet
prerequisites; explicitly say when none are Ready. Distinguish work already
assigned in this session, including scoped prototyping experiments.]
```

Keep all three main sections even when information is unavailable. Use **Not
recorded** for absent Book facts and explain access gaps; do not fill them by
guessing. If there are no open milestones, say so under Open work and retain the
Ready milestones section. Link to source chapters or records where practical.

Report statuses and evidence as the Book records them. Task checkmarks, anchor
references, passing linked tests, and a passing structural check do not establish
milestone completion. Preserve distinctions between an evidence result and its
coverage: a recorded pass can still leave part of a criterion untested. Do not
present historical evidence as a check run in this session, or turn a proposed
milestone into Ready because its tasks look feasible. Report inconsistencies
and their effect on readiness explicitly.

## Check the Book's structure

The structural check evaluates whether the Book conforms to its configured
structure. Project guidance identifies the command. Run it at the start of a
session, after every change to Book sources or configuration, and before
reporting Book work complete.

Act on the result:

| Result | Meaning | Next step |
| --- | --- | --- |
| Passed | Every check in the declared scope holds | Proceed. It establishes nothing outside that scope, such as record meaning, implementation, verification, or completion. |
| Failed | Evaluation completed and found structural violations | Repair the findings whose intended structure is clear, then rerun. Do not stop at reporting them. |
| Incomplete | Invalid input or an operational problem prevented evaluation | Resolve the reported input or access problem first. Findings are partial; this is never a pass. |

Structural repairs are part of any request to run the check or to work on the
Book. Repair from the findings: each gives a location, the expected and found
conditions, and guidance; use them to make the intended structure hold. Never
remove required sections, weaken configuration, or otherwise change what is
checked to obtain a pass. Ask the engineer before repairing when the intended
structure is unclear, such as a navigation entry for a chapter that does not
exist, or when the repair would change chapter content or the Anchor Book
configuration, which defines the intended structure. Aligning other tool
configuration, such as renderer settings, with it is an ordinary repair. Rerun
the same check after each repair or set of repairs until it passes or the
remaining findings are reported.

Report the command, its result, and any remaining findings. Report a failed or
incomplete check as failed or incomplete.

