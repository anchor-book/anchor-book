# Structural Checking

**Roadmap identifier:** `structural-checking`

**Status:** Proposed; implementation not started.

This roadmap supplies the outcome of
[Engine Bootstrap — Milestone 3](bootstrap.md#engine-bootstrap-3).
Governing requirements, decisions, and relevant design anchors must be recorded
and linked here during [Engine Bootstrap — Milestone 1](bootstrap.md#engine-bootstrap-1)
before this implementation work is ready.

## Goal

A developer or agent can point the CLI at this Book's configuration and receive
structural findings. Rust callers use the same library operation. Neither path
renders the Book, changes files, fetches dependencies, or runs project tests.

The design is defined in:

- [Rust Engine](../design/rust-engine.md)
- [Configuration](../design/configuration.md)
- [Structural Checking](../design/structural-checking.md)

The roadmap sequences that design without maintaining a second configuration
schema or rule specification.

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#structural-checking-1) | The engine understands this Book's configured layout | Proposed | Engine Bootstrap — Milestone 1 |
| [2](#structural-checking-2) | Developers receive actionable structural findings | Proposed | [1](#structural-checking-1) |
| [3](#structural-checking-3) | Continuous checks protect the current Book | Proposed | [2](#structural-checking-2) |

<a id="structural-checking-1"></a>

## Milestone 1. The engine understands this Book's configured layout

**Identifier:** `structural-checking-1`

**Status:** Proposed. **Depends on:** [Engine Bootstrap — Milestone 1](bootstrap.md#engine-bootstrap-1).

**Outcome:** Rust code can resolve this project's Book configuration and reject
invalid inputs independently of rendering or creating Book files.

**Tasks:**

- Establish the library/CLI Cargo package and agreed Rust/dependency versions.
- Implement typed configuration parsing, validation, and path resolution.
- Add the real configuration for this repository once the reader works.
- Exercise other layouts, invocation directories, and invalid inputs.

**Complete when:**

- The real Book's source, output, and section roles resolve as designed.
- Equivalent `docs/` and `md/` layouts work without special-case rules.
- Invalid fields/versions, path overlap, escapes, and symlink aliases receive
  useful errors; a separate nonexistent output directory is allowed.
- Configuration tests demonstrate file preservation on success and failure.
- Cargo build, formatting, Clippy, and meaningful configuration/API tests pass
  without rendering or validating the current Book.

**Completion record:** Pending. No operational checker is claimed by this milestone.

<a id="structural-checking-2"></a>

## Milestone 2. Developers receive actionable structural findings

**Identifier:** `structural-checking-2`

**Status:** Proposed. **Depends on:** Structural Checking — Milestone 1.

**Outcome:** A developer can use the library or CLI to identify structural defects
in this Book and distinguish those findings from an incomplete evaluation.

**Tasks:**

- Read renderer settings, navigation, and chapter metadata without executing
  preprocessors or creating missing files.
- Evaluate the initial structural rules against those observations.
- Expose typed findings and evaluated scope through the library and CLI.
- Exercise the current Book and independently specified failure cases.

**Complete when:**

- The current Book passes the declared structural scope.
- Missing source, summary, chapter, or mapped roles yield the specified findings;
  renderer drift is reported without repair.
- Malformed, unreadable, and unsupported inputs cannot produce a clean result.
- Library and CLI findings agree, and the CLI implements the documented statuses.
- Passing and failing runs leave file contents and directory entries unchanged.
- The [structural-checking acceptance cases](../design/structural-checking.md#acceptance-against-this-book)
  have recorded results, including alternate layouts and failure cases.

**Completion record:** Pending. This result establishes local checking; Structural Checking — Milestone 3 owns
continuous integration.

<a id="structural-checking-3"></a>

## Milestone 3. Continuous checks protect the current Book

**Identifier:** `structural-checking-3`

**Status:** Proposed. **Depends on:** Structural Checking — Milestone 2.

**Outcome:** Changes to this repository receive independent engine validation
and an acceptance check against its current Book before rendering.

**Tasks:**

- Add engine build/tests and supported-toolchain checks to CI.
- Add a separate current-Book check followed by mdBook rendering.
- Exercise successful and deliberately broken Book cases.
- Document local commands and record CI evidence for the implemented revision.

**Complete when:**

- CI runs the documented engine checks and the real current-Book acceptance case.
- A broken Book fails its acceptance step usefully while engine compilation and
  independent fixture tests remain runnable.
- The passing path renders the Book, with no automatic source repair.
- Evidence identifies the tested revision and actual workflow results; a local
  run alone is not recorded as a successful GitHub Actions run.

**Completion record:** Pending.

## Updating and completing the work

Use disposable copies for deliberate failures and public-safe reusable fixtures.
Exercise the actual Book as each capability becomes usable; Structural Checking — Milestone 3 is not the first
self-use experiment. Do not add success-returning placeholder commands.

Update task progress and remaining work in the Book during implementation.
When all completion criteria hold, mark the milestone Complete and replace its
pending record with the revision, checks, results, and integration context.
Reconcile design changes before claiming completion against revised criteria.

Engine Bootstrap — Milestone 3 is complete when all three milestones here are complete. That does not establish anchor semantics,
evidence adequacy, managed writes, or completion of later bootstrap milestones.
Materialization and contract generation remain separately agreed work.
