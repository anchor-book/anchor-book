# Project Integration

**Roadmap identifier:** `project-integration`

## Goal

This Book uses every mechanism an existing Rust project relies on to develop with
an Anchor Book before any project adopts it: agents start from the contract and
the project's Book, the project's ordinary commands check and render it, and a
pinned engine revision runs it. The project roadmaps then apply that groundwork;
the [Symposium Integration roadmap](symposium-integration.md) is the first.

[Project Integration](../design/project-integration.md) defines the goal design.
This roadmap is the shared groundwork that exercises it; it does not replace the
release work in [later work](index.md#later-work-retained-from-the-design).

## Sequence and scope

**This Book first.** Every capability a project relies on is implemented and in
use in this Book before a project adopts it, so this Book stays the primary
acceptance case. Milestone 1 does that for locating `book.toml`, retrieving the
contract, running a pinned engine revision, and the supported mdBook versions. A
gap found in a project is fixed and exercised here before the project relies on
the fix.

Milestone 2, Symposium's own Anchor Book, moved to the
[Symposium Integration roadmap](symposium-integration.md). Milestone 3, sage-dev,
is deferred while its role is unclear; see
[later work](index.md#later-work-retained-from-the-design). Both identifiers are
retired.

Verus is optional for every project. A project that adds the engine as a Cargo
dependency needs [Verus as an optional engine feature](formal-verification.md#formal-verification-4)
first, whether or not it verifies its own code; proofs in other projects are
[deferred](verification-evidence.md#verification-evidence-5).

## Milestone 1. This Book uses the integration groundwork {#project-integration-1}

**Outcome:** This repository uses every mechanism a project will rely on: its configuration names its `book.toml`, its agents retrieve the contract with `anchorbook contract`, a pinned engine revision installed the documented way checks and renders this Book, and the declared mdBook versions are tested.

**Status:** Proposed

**Dependencies:** The configuration loader, structural checker, and record renderer. Agreement on this milestone's scope and on moving the contract source into the engine.

**Design basis:** [mdBook configuration location](../design/configuration.md#relationship-to-mdbook), [contract delivery](../design/system-overview.md#contract-delivery), [running the engine](../design/project-integration.md#running-the-engine), and [agent bootstrap](../design/project-integration.md#agent-bootstrap).

**Identifier:** `project-integration-1`

**Tasks**

- [ ] **Configuration:** Implement `[book].mdbook`, resolving mdBook's source and output relative to that file, and migrate this repository's configuration to name its `book.toml`.
- [ ] **Contract:** Move the maintained contract source into the engine, add `anchorbook contract`, switch this repository's `AGENTS.md` to retrieve it, and remove the temporary Book copy and its navigation entry, as [contract delivery](../design/system-overview.md#contract-delivery) describes.
- [ ] **Running the engine:** Choose and document how a project installs a pinned engine revision before crate release, and exercise that path in this repository's CI against this Book.
- [ ] **Entry points:** Test that the library, the CLI, and the `anchorbook md` preprocessor report equivalent results for the same fixtures.
- [ ] **mdBook versions:** Decide which mdBook versions the preprocessor supports, including the version Symposium uses or a documented pin, and test each against this Book.

### Completion criteria

#### mdBook location {#project-integration-1-mdbook-location}

The loader accepts a configured `book.toml`, including one in a subdirectory, and compares its resolved source and output; fixtures cover a subdirectory, a mismatch, a missing file, and an escaping path. This repository's configuration names its `book.toml` and its Book still passes and renders.

- **Anchor coverage:** [check-renderer-alignment](../design/structural-checking.md#check-renderer-alignment) · `resolved-comparison`, `renderer-input` — Partial: a configured `book.toml` location; [check-configuration-validity](../design/configuration.md#check-configuration-validity) · `path-boundaries` — Partial: the new field.
- **Evidence:** Pending.

#### Contract {#project-integration-1-contract}

`anchorbook contract` prints the contract packaged with the running engine without reading project files; this repository's agents start from it, and a fresh session in this repository reads it, the project guidance, and the Book. No copy of the contract remains in the Book.

- **Anchor coverage:** [project-agent-bootstrap](../design/project-integration.md#project-agent-bootstrap) · `contract-retrieval` — Full; `fresh-session` — Partial: this repository.
- **Evidence:** Pending.

#### Installed engine {#project-integration-1-installed-engine}

A CI job installs the pinned engine revision the documented way and checks and renders this Book with it, with the same findings as the in-repository build; each declared mdBook version renders this Book, and an undeclared version is rejected with a diagnostic naming the supported ones.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `renderer-versions` — Full for the declared versions; [project-one-engine-version](../design/project-integration.md#project-one-engine-version) · `same-version-entry-points` — Partial: installed and in-repository builds of one revision.
- **Evidence:** Pending.

#### Equivalent entry points {#project-integration-1-entry-points}

The library, the actual CLI process, and the actual `anchorbook md` preprocessor process report equivalent outcomes, codes, locations, and evaluated scope for the same passing, failing, and incomplete fixtures, against independent expectations.

- **Anchor coverage:** [check-shared-typed-results](../design/rust-engine.md#check-shared-typed-results) · `equivalent-callers` — Partial: the preprocessor caller; library and CLI equivalence is recorded in [Structural Checking milestone 2](structural-checker.md#structural-checking-2-typed-results).
- **Evidence:** Pending.

### Completion record

Pending.
