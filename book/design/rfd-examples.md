# RFD Worked Examples

These examples illustrate [RFD format 1](rfd-format.md) in a fictional fixture
Book. They are code examples, not proposals or live records in this Book.
The repeated hexadecimal revisions identify hypothetical fixture snapshots;
they make no claim about this repository's history.

## Baseline and index

The fixture has design chapters `design/configuration.md` and `design/hooks.md`,
and roadmap `configuration` with milestone `configuration-1`. At its baseline,
these draft records govern configuration and hooks:

```markdown
> **`R-config-consistent`** (draft): **A run has consistent configuration**
> Statement: Commands and hooks in one run must use the same configuration snapshot.
> Source: A configuration edit during a run must not change its remaining behavior.

> **`A-config-single-read`** (draft): **Configuration is read once for a command**
> Property: A command reads its configuration once and reuses that snapshot.
> Authority: `R-config-consistent`
> - Criterion `single-read` [test]: A configuration edit after command startup does not change that command's snapshot.

> **`A-hooks-reload`** (draft): **Hooks reload configuration before dispatch**
> Property: Each hook reloads configuration before dispatch.
> Authority: `R-config-consistent`
> - Criterion `reload` [test]: Each hook sees the configuration file at its dispatch time.
```

The hook property conflicts with the requirement when a file changes mid-run.
A structural reference check alone cannot identify that contradiction. The RFD
below resolves it across both chapters instead of only extending command behavior.

The root configuration uses schema 2 and maps `rfds = "rfds/index.md"`. Its index
introduces the local review policy and links to `session-snapshot.md`. Navigation
contains this subtree alongside the three required roles:

```markdown
- [Requests for Discussion](rfds/index.md)
  - [One snapshot for a run](rfds/session-snapshot.md)
```

## Draft proposal

This is the complete proposal root. It adds a decision and an anchor, revises an
existing anchor, and retires a conflicting one. All definitions remain in this
RFD's scope; changing its status does not change any baseline record.

````markdown
# One snapshot for a run

```rfd-metadata
format-version = 1
id = "RFD-session-snapshot"
title = "One snapshot for a run"
status = "draft"
base-revision = "git:1111111111111111111111111111111111111111"
anchor-changes = "proposed"
affects = [
  { target = "design:design/configuration.md", scope = "current" },
  { target = "design:design/hooks.md", scope = "current" },
  { target = "requirement:R-config-consistent", scope = "current" },
  { target = "anchor:A-config-single-read", scope = "current" },
  { target = "anchor:A-hooks-reload", scope = "current" },
]
planned = []
milestones = [{ target = "milestone:configuration-1", scope = "current" }]
supersedes = []
sources = []
incorporation = []
```

## Summary

Use one configuration snapshot for a command and every hook in its run.

## Motivation

Reloading at hook dispatch can contradict a command's startup decisions after
the configuration file changes.

## Proposal

The run owns the snapshot. Commands and hooks receive the same snapshot, with
loading and error policy defined by Configuration and dispatch defined by Hooks.
Both chapters will describe the shared lifetime. Retire the hook reload guarantee.

## Proposed anchors

The decision and three anchor changes describe the whole affected guarantee.

```rfd-change
operation = "add"
id = "D-config-run-snapshot"
reason = "Choose one owner and lifetime for commands and hooks."
bindings = [{ target = "requirement:R-config-consistent", scope = "current" }]
definition = '''
> **`D-config-run-snapshot`** (draft): **The run owns one configuration snapshot**
> Choice: Load configuration once at run startup and pass the snapshot to commands and hooks.
> Requirements: `R-config-consistent`
> Rationale: Every operation in the run then observes the same configuration.
> Alternatives:
> - Reloading at each operation sees edits sooner but permits inconsistent behavior within a run.
> Reconsider when: Runs need explicitly coordinated live reconfiguration.
'''
```

```rfd-change
operation = "revise"
id = "A-config-single-read"
reason = "Extend the same snapshot lifetime across the complete run."
bindings = [
  { target = "requirement:R-config-consistent", scope = "current" },
  { target = "decision:D-config-run-snapshot", scope = "proposed" },
]
definition = '''
> **`A-config-single-read`** (draft): **Configuration is read once for a run**
> Property: A run reads configuration once and uses that snapshot for its command and every hook.
> Authority: `R-config-consistent`; `D-config-run-snapshot`
> - Criterion `single-read` [test]: Change configuration between command startup and hook dispatch; both retain the original snapshot and no second read occurs.
'''
```

```rfd-change
operation = "add"
id = "A-hooks-shared-snapshot"
reason = "Make hook dispatch's dependency on the run snapshot explicit."
bindings = [{ target = "decision:D-config-run-snapshot", scope = "proposed" }]
definition = '''
> **`A-hooks-shared-snapshot`** (draft): **Hooks receive the run's snapshot**
> Property: Every dispatched hook receives the configuration snapshot owned by its run.
> Authority: `D-config-run-snapshot`
> - Criterion `same-snapshot` [test]: Dispatch multiple hooks after editing the file and check that each receives the original run snapshot.
'''
```

```rfd-change
operation = "retire"
id = "A-hooks-reload"
reason = "Reloading contradicts the shared snapshot; reconcile its references, roadmap coverage, and historical evidence."
```

## Alternatives

Reloading preserves freshness between operations but violates consistency within
one run. Coordinated live reconfiguration is outside this change.

## Open questions

None for the snapshot lifetime. Loading error policy stays with the existing
Configuration design and must be reviewed when adopting this proposal.

## Implementation

Adopt the definition in both design chapters and reconcile configuration-1 before
implementing its assigned work. Reassess the expanded single-read criterion;
retain earlier results as historical evidence and add the missing hook cases.
````

## Accepted and incorporated

Acceptance changes `status` to `accepted` and adds a review reference to the
already existing draft revision:

```toml
status = "accepted"
review = { revision = "git:2222222222222222222222222222222222222222", basis = "The project reviewer agreed to the complete snapshot change." }
```

At this point the proposal is agreed and the Book's baseline definitions are
still unchanged. The agent prepares adoption edits to both design chapters,
adds the decision and new anchor, revises the existing anchor under its same ID,
removes the retired definition, and reconciles its live references. It adjusts
roadmap coverage and reports code and tests still needed. The old hooks test can
remain in history; its pass cannot establish the new snapshot guarantee.

After those edits exist at a recorded Book revision, the metadata includes:

```toml
[[incorporation]]
proposal-revision = "git:2222222222222222222222222222222222222222"
book-revision = "git:3333333333333333333333333333333333333333"
changes = ["D-config-run-snapshot", "A-config-single-read", "A-hooks-shared-snapshot", "A-hooks-reload"]
destinations = [
  { target = "design:design/configuration.md", scope = "current" },
  { target = "design:design/hooks.md", scope = "current" },
  { target = "anchor:A-config-single-read", scope = "current" },
  { target = "anchor:A-hooks-shared-snapshot", scope = "current" },
]
note = "The system definition incorporates all four changes; implementation and hook coverage remain open."
```

This replaces the earlier empty incorporation array; it is not appended beside
that duplicate key. The retired `A-hooks-reload` entry in `affects` must become an explicit historical
reference to `base-revision`. The retained retirement operation still names that
baseline ID; it does not require a current record with that ID. Repeating adoption
finds the recorded definitions and remaining work, without adding new IDs or
resetting evidence. If a current requirement later retires, reconcile or qualify
the RFD's bindings explicitly and show the remaining historical validation limit.

## Other review cases

| Case | Expected interpretation |
| --- | --- |
| Another RFD revises A-config-single-read differently | Separate proposal identity and definition; show the overlap for semantic review, without duplicate current anchors |
| An authority binding names a missing current decision | Located reference failure; it cannot silently resolve to another RFD's proposed decision |
| Process-only RFD | anchor-changes is none with a nonempty reason, no anchor operations, and implementation work linked only when it exists |
| Newly proposed milestone | Declare milestone:configuration-2 in planned and use proposed scope in milestones until the roadmap contains that ID |
| Retirement leaves a live anchor or milestone referring to the removed record | Adoption is incomplete; repair the live references rather than treating them as historical implicitly |
| Superseding proposal | Draft successor names its predecessor; on agreement, accept the successor and mark its predecessor superseded together |
| Unreadable proposal or unsupported format | Evaluation is incomplete; no passing RFD check or complete Book view |

These authoring examples specify expected outcomes. Executed reader/checker
fixtures and the fresh-agent trial belong to the [RFD roadmap](../implementation-roadmaps/rfds.md).
