# Project Integration

**Roadmap identifier:** `project-integration`

## Goal

This Book first, then two existing Rust projects, Symposium and sage-dev, develop
with Anchor Books: agents start from the contract and the project's Book, the project's
ordinary commands check and render it, and the gaps found become engine work here.
Each project keeps its existing user Book and adds a separate Anchor Book, adapted
from its design notes and decision records. Provisional anchor conventions in
either project are dropped and replaced with Anchor Book records, not migrated.

[Project Integration](../design/project-integration.md) defines the goal design.
This roadmap is the trial that exercises it; it does not replace the release
work in [later work](index.md#later-work-retained-from-the-design).

## Sequence and scope

**This Book first.** Every capability a project relies on is implemented and in
use in this Book before a project adopts it, so this Book stays the primary
acceptance case. Milestone 1 does that for the shared groundwork: locating
`book.toml`, retrieving the contract, running a pinned engine revision, and the
supported mdBook versions. Milestone 2 then integrates Symposium, and milestone 3
sage-dev, adding its pinned nightly toolchain and other mdBook preprocessors. A gap
found in a project is fixed and exercised here before the project relies on the
fix. Project work happens on branches of the projects' forks; changes to the
projects themselves are their maintainers' decisions.

Formal verification in these projects is outside this roadmap until a project
needs it; it depends on [engine proof isolation](formal-verification.md#formal-verification-4)
and the [Verification Evidence roadmap](verification-evidence.md).

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

### Completion record

Pending.

## Milestone 2. Symposium develops with its own Anchor Book {#project-integration-2}

**Outcome:** On a Symposium branch, a separate Anchor Book beside the existing user Book passes the structural check and renders through Symposium's own commands, and a fresh agent session starts from the contract and that Book. Each gap found is fixed in the engine or recorded as engine work.

**Status:** Proposed

**Dependencies:** [Milestone 1](#project-integration-1) and agreement on this milestone's scope.

**Design basis:** [Project Integration](../design/project-integration.md), [mdBook configuration location](../design/configuration.md#relationship-to-mdbook), and [contract delivery](../design/system-overview.md#contract-delivery).

**Identifier:** `project-integration-2`

**Tasks**

- [ ] **Setup:** Install the engine at the pinned revision and configure rendering, as documented by milestone 1.
- [ ] **The Book:** Create the Anchor Book with its configuration, navigation, project guidance chapter, Authority adapted from Symposium's RFDs, one design area with anchors adapted from its design notes, a roadmap, and the `AGENTS.md` bootstrap. Drop the provisional anchor markup.
- [ ] **Use it:** Run the check and render from Symposium, and exercise a fresh agent session that reads the contract, guidance, and Book and identifies the phase and commands.
- [ ] **Record:** List every gap found. Fix each one in the engine and exercise it in this Book before Symposium relies on it, or add it as engine work in the relevant roadmap.

### Completion criteria

#### Native commands {#project-integration-2-native-commands}

On the Symposium branch, the documented commands check and render the Anchor Book with a pinned engine revision, leave the user Book unchanged, and refuse to render a deliberately broken copy.

- **Anchor coverage:** [project-native-commands](../design/project-integration.md#project-native-commands) · `consumer-fixture` — Partial: one real project with a pinned revision rather than a packaged crate; [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `renderer-versions` — Partial: the mdBook version Symposium uses.
- **Evidence:** Pending.

#### Agent start {#project-integration-2-agent-start}

A fresh agent session in the Symposium branch reads the contract, project guidance, and Book through the bootstrap, and identifies the phase, commands, and governing records of the adapted design area. Record agent versions and any assistance.

- **Anchor coverage:** [project-agent-bootstrap](../design/project-integration.md#project-agent-bootstrap) · `fresh-session` — Partial: a second project, using the contract command from milestone 1.
- **Evidence:** Pending.

#### Gaps recorded {#project-integration-2-gaps}

Every gap found has a fix with evidence or an entry in the relevant roadmap; the Anchor Book's adapted content is reviewed with the engineer.

- **Anchor coverage:** None — trial record.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 3. sage-dev develops with its own Anchor Book {#project-integration-3}

**Outcome:** On a sage-dev branch, a separate Anchor Book replaces the provisional anchor markup, passes the structural check, and renders through sage-dev's commands on its pinned nightly toolchain alongside its other mdBook preprocessors; a fresh agent session starts from it.

**Status:** Proposed

**Dependencies:** [Milestone 2](#project-integration-2), whose project setup it reuses.

**Design basis:** [Project Integration](../design/project-integration.md), including [what the crate must not impose](../design/project-integration.md#what-the-crate-must-not-impose).

**Identifier:** `project-integration-3`

**Tasks**

- [ ] Build and run the pinned engine revision with sage-dev's pinned nightly toolchain; fix or record any incompatibility.
- [ ] Create the Anchor Book from sage-dev's design material, as in milestone 2, and replace its provisional anchors with Anchor Book records. Removing the provisional tooling is the maintainers' decision.
- [ ] Render with the Anchor Book preprocessor in the Anchor Book's own `book.toml`; confirm the user Book and its preprocessors are unaffected.
- [ ] Exercise a fresh agent session and record gaps as in milestone 2; fix them in this Book first.

### Completion criteria

#### Nightly toolchain {#project-integration-3-nightly-toolchain}

The engine builds and checks the Anchor Book on sage-dev's pinned nightly toolchain without changing sage-dev's toolchain or dependencies.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `dependency-review` — Partial: one pinned nightly toolchain.
- **Evidence:** Pending.

#### Native commands {#project-integration-3-native-commands}

sage-dev's documented commands check and render the Anchor Book, its user Book and preprocessors are unaffected, and a deliberately broken copy is not rendered.

- **Anchor coverage:** [project-native-commands](../design/project-integration.md#project-native-commands) · `consumer-fixture` — Partial: a second project with other preprocessors.
- **Evidence:** Pending.

#### Agent start and gaps {#project-integration-3-agent-start}

A fresh agent session starts from the contract and the Anchor Book, and every gap found is fixed or recorded, as in milestone 2.

- **Anchor coverage:** [project-agent-bootstrap](../design/project-integration.md#project-agent-bootstrap) · `fresh-session` — Partial: second project.
- **Evidence:** Pending.

### Completion record

Pending.
