# Structural Checking

**Roadmap identifier:** `structural-checking`

**Status:** Proposed; implementation not started.

This roadmap follows [Engine Bootstrap](bootstrap.md). Bootstrap prepares the
Book, authority, anchors, working contract, and agreed implementation scope used
here. Link the governing records here before marking implementation ready.

## Goal

An agent can use the checker's feedback to bring a Book into conformance with
its configured structure: run the check, understand the findings, make appropriate
changes within its assignment, and recheck until the structure passes.

The checker supplies information through a CLI and a reusable Rust library.
It stays read-only: it does not render the Book, modify files, fetch dependencies,
or run project tests. In this first increment, the agent makes the repairs.
Later engine operations may support those changes, but are not required here.

A pass must result from correcting the structure according to the intended
configuration and rules, not from bypassing checks or weakening the requirements.

The design is defined in:

- [Rust Engine](../design/rust-engine.md)
- [Configuration](../design/configuration.md)
- [Structural Checking](../design/structural-checking.md)
- [Checking Diagnostics](../design/check-diagnostics.md)

The roadmap sequences that design without maintaining a second configuration
schema or rule specification.

## Authority and anchors

The initial [requirements](../authority/requirements.md#initial-checker-requirements),
[design decisions](../authority/design-decisions.md#initial-checker-decisions),
and anchors below are drafts for review. Their links establish traceability,
not acceptance or implementation evidence. Each anchor carries its governing
authority and verification criteria in the [structured format](../design/anchor-definitions.md).

| Milestone | Relevant anchors |
| --- | --- |
| [1](#structural-checking-1) | [Book selection](../design/configuration.md#check-book-selection), [configuration validity](../design/configuration.md#check-configuration-validity), [shared typed results](../design/rust-engine.md#check-shared-typed-results), [independent build](../design/system-overview.md#check-independent-build) |
| [2](#structural-checking-2) | [Observation-only checking](../design/structural-checking.md#check-observation-only), [navigation conformance](../design/structural-checking.md#check-navigation-conformance), [renderer alignment](../design/structural-checking.md#check-renderer-alignment), [shared typed results](../design/rust-engine.md#check-shared-typed-results), [truthful results](../design/check-diagnostics.md#check-truthful-results), [actionable diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics) |
| [3](#structural-checking-3) | [Independent build](../design/system-overview.md#check-independent-build), [truthful results](../design/check-diagnostics.md#check-truthful-results), [observation-only checking](../design/structural-checking.md#check-observation-only) |
| [4](#structural-checking-4) | [Actionable diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics), [truthful results](../design/check-diagnostics.md#check-truthful-results), [observation-only checking](../design/structural-checking.md#check-observation-only) |

Some criteria span milestones. Completing one milestone does not establish every
criterion of an associated anchor. Parsing anchor definitions, resolving their
authority automatically, and aggregating evidence are later engine work, not
additional requirements for the first layout and navigation checker.

## Milestones

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#structural-checking-1) | The engine understands this Book's configured layout | Proposed | Engine Bootstrap complete |
| [2](#structural-checking-2) | Developers receive actionable structural findings | Proposed | [1](#structural-checking-1) |
| [3](#structural-checking-3) | Continuous checks protect the current Book | Proposed | [2](#structural-checking-2) |
| [4](#structural-checking-4) | An agent can repair Book structure using checker feedback | Proposed | [2](#structural-checking-2) |

Milestones 3 and 4 can proceed independently after milestone 2. All four are
required to complete this roadmap; milestone 4 demonstrates the agent workflow.

<a id="structural-checking-1"></a>

## Milestone 1. The engine understands this Book's configured layout

**Identifier:** `structural-checking-1`

**Status:** Proposed. **Depends on:** [Engine Bootstrap](bootstrap.md) complete.

**Outcome:** Rust code can resolve this project's Book configuration and reject
invalid inputs independently of rendering or creating Book files.

**Tasks:**

- Establish the library/CLI Cargo package and agreed Rust/dependency versions.
- Implement typed configuration parsing, validation, and path resolution.
- Validate and expose the configured project phase without changing structural
  rules or claiming automated design-readiness or completion checks.
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
- Expose typed findings and evaluated scope through the library and CLI, including
  the affected location, expected structure, and useful next action.
- Explain missing prerequisites and incomplete evaluation so an agent knows
  whether it needs to repair structure or resolve an input/operational problem.
- Exercise the current Book and independently specified failure cases.

**Complete when:**

- The current Book passes the declared structural scope.
- Missing source, summary, chapter, or mapped roles yield the specified findings;
  renderer drift is reported without repair.
- Malformed, unreadable, and unsupported inputs cannot produce a clean result.
- Library and CLI findings agree, and the CLI implements the documented statuses.
- Diagnostic cases demonstrate the designed codes, locations, expectations,
  repair guidance, blocked checks, report scope, and stream behavior.
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

<a id="structural-checking-4"></a>

## Milestone 4. An agent can repair Book structure using checker feedback

**Identifier:** `structural-checking-4`

**Status:** Proposed. **Depends on:** Structural Checking — Milestone 2.

**Outcome:** An agent can use findings to make scoped structural repairs and
reach a passing check while preserving unrelated content.

**Tasks:**

- Prepare disposable copies of this Book with known structural defects and
  independently specified expected findings and valid repairs.
- Give an agent the Book, the checker, and an explicit repair assignment; have it
  check, interpret the findings, edit the permitted files, and recheck.
- Include missing chapters, misplaced required sections, and renderer-setting
  drift. Exercise an invalid-input case that must be resolved before evaluation.
- Improve unclear diagnostics or workflow guidance and repeat affected cases.
- Record the initial defects, findings, edits, final result, and any human help.

**Complete when:**

- Recorded agent runs reach a passing structural check by correcting the known
  defects according to the intended configuration and rules.
- Review of the diff and independent expected outcomes confirms the repairs;
  the checker agreeing with itself is not the sole evidence.
- Unrelated content is preserved and the agent does not disable checks, remove
  required roles, or change the intended configuration merely to get a pass.
- Each checker invocation remains read-only; the agent's edits are identifiable
  separately from the check operation.
- Cases the agent cannot resolve within its scope produce an explicit explanation
  rather than a false completion claim. Record tool versions and assistance so
  the result does not imply that every agent will repair every Book unaided.

**Completion record:** Pending.

## Updating and completing the work

Use disposable copies for deliberate failures and public-safe reusable fixtures.
Exercise the actual Book as each capability becomes usable; Structural Checking — Milestone 3 is not the first
self-use experiment. Do not add success-returning placeholder commands.

Update task progress and remaining work in the Book during implementation.
When all completion criteria hold, mark the milestone Complete and replace its
pending record with the revision, checks, results, and integration context.
Reconcile design changes before claiming completion against revised criteria.

This roadmap is complete when all four milestones are complete: the current
Book and independent fixtures validate the checker, CI exercises it, and recorded
agent runs demonstrate the check, repair, and recheck loop. A successful structural
check does not establish full anchor verification or completion of other roadmaps.
Materialization and contract generation remain separate later work.
