# Symposium Integration

**Roadmap identifier:** `symposium-integration`

## Goal

Symposium develops with its own Anchor Book. Its agents start from the contract
and that Book, its ordinary commands and CI check and render it, its tests are
evidence for its anchors' criteria, and the gaps it exposes become engine work in
this Book. Symposium does not use Verus.

Symposium keeps its existing user Book in `md/`. The Anchor Book is separate, with
its own `book.toml`, and is adapted from Symposium's design chapters, tenets, and
requests for discussion (RFDs). Any provisional anchor markup is dropped and
replaced with Anchor Book records, not migrated.

[Project Integration](../design/project-integration.md) defines the goal design;
the [Project Integration roadmap](project-integration.md) builds the shared
groundwork in this Book first.

## Sequence and scope

**This Book first.** Each mechanism Symposium relies on is implemented and in use
in this Book before a Symposium milestone depends on it: the shared groundwork in
[Project Integration milestone 1](project-integration.md#project-integration-1),
the test runner in [Verification Evidence milestone 1](verification-evidence.md#verification-evidence-1),
and retained reports in [milestone 3](verification-evidence.md#verification-evidence-3).
A gap found in Symposium is fixed and exercised here before Symposium relies on
the fix, or recorded as engine work in the relevant roadmap.

Milestone 1 runs an installed engine against a first Anchor Book, so it needs no
Cargo dependency on the engine. Milestones 2 and 3 complete the Book and the agent
start. Milestone 4 links Symposium's tests to its criteria. Milestone 5 adds the
library check to `cargo test` and the Book to CI; adding the engine as a Cargo
dependency first needs [Verus as an optional engine feature](formal-verification.md#formal-verification-4).
Milestone 6, delivering the contract through a Symposium plugin, is proposed for
agreement and is not a prerequisite for the others.

Work happens on branches of the engineer's Symposium fork. Changes to Symposium
itself, including where its Anchor Book lives and which files it adds, are its
maintainers' decisions.

## Milestone 1. Symposium checks and renders a first Anchor Book {#symposium-integration-1}

**Outcome:** On a Symposium branch, a separate Anchor Book with one adapted design area passes the structural check and renders through Symposium's own commands with a pinned engine revision, and the user Book is unchanged.

**Status:** Complete

**Identifier:** `symposium-integration-1`

**Dependencies:** [Project Integration milestone 1](project-integration.md#project-integration-1).

**Design basis:** [Adding Anchor Book](../design/project-integration.md#adding-anchor-book), [running the engine](../design/project-integration.md#running-the-engine), and [mdBook configuration location](../design/configuration.md#relationship-to-mdbook).

**Tasks**

- [x] **Location:** The Anchor Book lives in `anchor-book/` with its own `anchor-book/book.toml`; `anchorbook.toml` stays at the repository root with `mdbook = "anchor-book/book.toml"`. The user Book (`md/`, root `book.toml`) is untouched. Work is on a branch of the fork; whether and how it goes upstream is the maintainers' decision.
- [x] **Setup:** Install the pinned engine revision the way Project Integration milestone 1 documents, and configure `anchorbook.toml` and the Anchor Book's `book.toml` with the `anchorbook md` preprocessor and the `mdbook-mermaid` preprocessor Symposium already uses.
- [x] **First Book:** Create the configuration, navigation, project guidance chapter, an Authority section with the requirements and decisions the first design area needs, that design area with its anchors, and a roadmap. Choose a small, well-described area, such as configuration loading.
- [x] **mdBook version:** Symposium installs the newest mdBook without a pin, currently 0.5.4, the only version the preprocessor supports. The Anchor Book's guidance pins 0.5.4 for local use; pinning it in Symposium's CI belongs to [milestone 5](#symposium-integration-5).
- [x] **Gaps:** List every gap found and fix it in the engine and this Book, or add it to the relevant roadmap.

### Completion criteria

#### Native commands {#symposium-integration-1-native-commands}

On the Symposium branch, the documented commands check and render the Anchor Book with a pinned engine revision, leave the user Book and its build unchanged, and refuse to render a deliberately broken copy.

- **Anchor coverage:** [project-native-commands](../design/project-integration.md#project-native-commands) · `consumer-fixture` — Partial: one real project using an installed engine rather than a package dependency; [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `renderer-versions` — Partial: the mdBook version Symposium uses.
- **Evidence:** Passed locally on 2026-11-10, Symposium branch `anchor-book` (commit `98219876`, based on upstream `cb858e53`), not pushed. The engine was installed with `cargo install --git https://github.com/anchor-book/anchor-book --rev 40b0098fcbcc9948115680f7884ca709b9df18fc --locked anchor-book` (into a separate `--root`). With mdBook 0.5.4 and mdbook-mermaid 0.17.1, `anchorbook check --config anchorbook.toml` passed and `mdbook build anchor-book` rendered the three anchors, seven authority records, and one milestone card into `target/anchorbook`. The root `mdbook build` of the user Book still succeeded, and the commit adds files only, changing no tracked file. In disposable copies, a broken navigation target failed in mdBook before preprocessing, an unresolved authority reference failed with `record.invalid-reference`, and a renderer output mismatch failed with `structure.renderer-path-mismatch` (check exit 1); none produced output. Limits: one design area; installed engine only, not a package dependency; Symposium's CI does not build the Anchor Book yet. Setup note: mdBook copies `additional-js` paths with `../` outside the output directory, so a Book in a subdirectory carries its own copies of Symposium's Mermaid files.

#### Gaps recorded {#symposium-integration-1-gaps}

Every gap found has a fix with evidence in this Book or an entry in the relevant roadmap.

- **Anchor coverage:** None — trial record.
- **Evidence:** Recorded. (1) Cargo does not resolve a shortened `--rev`; fixed in [running the engine](../design/project-integration.md#running-the-engine), which now asks for the full commit hash. (2) `anchorbook --version` printed only the package version, so installations of different revisions were indistinguishable; fixed: it now prints the commit the binary was built from, as [running the engine](../design/project-integration.md#running-the-engine) describes. `tests/cli.rs` compares it with Git's `HEAD` for this checkout; by hand, a packaged crate reported its `.cargo_vcs_info.json` commit, and the same source without Git, or nested inside another repository, reported the revision as unknown. (3) The record and milestone styles and print script lived only in this repository's theme, so Symposium's cards rendered unstyled and its details did not expand when printing; fixed: the engine compiles them in and `anchorbook md` appends them to each chapter it renders records or milestones in, as [mdBook integration](../design/record-rendering.md#mdbook-integration) describes. This Book no longer lists them in its `book.toml` and renders with the delivered copies; `tests/rendering.rs` and `tests/roadmaps.rs` check the block appears once, last, in rendered chapters and not in others. Both fixes are on branch `project-integration`; Symposium keeps its pin at `40b0098` until they reach `main`, and a browser review of the delivered styles is pending. (4) A project's authoring references are this Book's chapters at a pinned revision, which describe the goal record syntax alongside the earlier form the engine reads, with no versioned reference delivered with the engine; this is an entry in [remaining project integration](index.md#later-work-retained-from-the-design). (5) Symposium's `AGENTS.md` asks agents to add a `Co-authored-by` line, which conflicts with the engineer's practice of no commit attribution; it is left for [milestone 3](#symposium-integration-3)'s reconciliation.

### Completion record

Complete on 2026-11-10, on fork branch `anchor-book`, not pushed. Symposium's Anchor Book checks and renders through its own commands with a pinned installed engine, the user Book is unchanged, broken copies are refused, and every gap found is fixed here or recorded. Its records serve both as a test case for the engine and as the start of Symposium's own Book: the two configuration decisions are reconstructed from Symposium's design and code, with alternatives written in the trial, and Symposium's maintainers review the content when it is integrated.

## Milestone 2. Symposium's design and authority are anchored {#symposium-integration-2}

**Outcome:** Symposium's Anchor Book covers its design: requirements and decisions adapted from its tenets and accepted and completed RFDs, and anchors for each design area with criteria and required methods, reviewed with the engineer.

**Status:** Proposed

**Identifier:** `symposium-integration-2`

**Dependencies:** [Milestone 1](#symposium-integration-1).

**Design basis:** [Anchor Definitions](../design/anchor-definitions.md), [Authority Definitions](../design/authority-definitions.md), and [choosing methods](../design/verification.md#choosing-methods).

**Tasks**

- [ ] **Authority:** Adapt Symposium's tenets into requirements, and its accepted and completed RFDs into the decisions and requirements they establish. Record each source; draft RFDs stay out until accepted.
- [ ] **Design areas:** Adapt each design chapter, such as configuration loading, sync, hooks, plugin distribution, state, and the report layer, into a design chapter whose anchors state the properties Symposium relies on.
- [ ] **Methods:** Give each criterion its required methods, `test` or `review`; no criterion requires `proof`.
- [ ] **Roadmap:** Write Symposium's own roadmap for work its design identifies, kept in its Anchor Book.
- [ ] **Gaps:** Record record-format, authority, or checking gaps as in milestone 1.

### Completion criteria

#### Design coverage {#symposium-integration-2-design-coverage}

Every design area in Symposium's contribution guide is either represented by a design chapter with anchors or listed with a reason for leaving it out, and the Book passes the structural check.

- **Anchor coverage:** None — project content; the engine's anchors are exercised by the check.
- **Evidence:** Pending.

#### Authority traced {#symposium-integration-2-authority-traced}

Each anchor cites at least one requirement or decision that resolves; each adapted requirement and decision names its tenet or RFD; a recorded review confirms that the cited authority justifies each property.

- **Anchor coverage:** None — project content and review.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 3. Symposium's agents start from the contract and its Anchor Book {#symposium-integration-3}

**Outcome:** A fresh agent session in Symposium reads the contract, the project guidance, and the Anchor Book through the `AGENTS.md` bootstrap, and Symposium's existing agent instructions live in its project guidance without conflicting with the contract.

**Status:** Proposed

**Identifier:** `symposium-integration-3`

**Dependencies:** [Milestone 1](#symposium-integration-1) and the contract command from [Project Integration milestone 1](project-integration.md#project-integration-1).

**Design basis:** [Agent bootstrap](../design/project-integration.md#agent-bootstrap) and [contract delivery](../design/system-overview.md#contract-delivery).

**Tasks**

- [ ] Add the bootstrap section to Symposium's `AGENTS.md`.
- [ ] Move Symposium's own instructions, such as checking common issues, keeping design chapters current, and its commit conventions, into the project guidance chapter, and record any that conflict with the contract for the engineer and maintainers to resolve.
- [ ] Run fresh agent sessions and record what they read and report.

### Completion criteria

#### Agent start {#symposium-integration-3-agent-start}

A fresh agent session in the Symposium branch reads the contract, project guidance, and Book through the bootstrap, and identifies the phase, commands, and governing records of an adapted design area. Record agent versions and any assistance.

- **Anchor coverage:** [project-agent-bootstrap](../design/project-integration.md#project-agent-bootstrap) · `fresh-session` — Partial: a second project, after this Book.
- **Evidence:** Pending.

#### Instructions reconciled {#symposium-integration-3-instructions}

Every instruction in Symposium's previous `AGENTS.md` is in its project guidance or listed as a conflict with its resolution.

- **Anchor coverage:** None — project guidance.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 4. Symposium's tests are evidence for its criteria {#symposium-integration-4}

**Outcome:** Symposium's tests carry evidence links to its anchors' criteria, and `anchorbook verify --method test` reports each criterion's state across its workspace in the build loop.

**Status:** Proposed

**Identifier:** `symposium-integration-4`

**Dependencies:** [Milestone 2](#symposium-integration-2) and the test runner working on this Book in [Verification Evidence milestone 1](verification-evidence.md#verification-evidence-1).

**Design basis:** [Running Verification](../design/verification-evidence.md) and [evidence links](../design/verification-evidence.md#evidence-links).

**Tasks**

- [ ] Link existing tests to the criteria they address and record the criteria that have none.
- [ ] Run `verify` over Symposium's workspace, which has several member crates and excludes its evaluation fixtures.
- [ ] Agent-mode tests report success without running anything when no agent is configured. Make sure `verify` reports such a run as not evaluated, never as satisfied; fix this in the engine and this Book first if it does not.
- [ ] Symposium runs `cargo test`, and the runner uses nextest. Confirm nextest runs Symposium's tests with the same results on its supported platforms, or record the gap.

### Completion criteria

#### Linked tests {#symposium-integration-4-linked-tests}

Every link in Symposium's sources resolves to a criterion requiring `test`, misplaced or dangling links are reported at their location, and unlinked criteria are listed.

- **Anchor coverage:** [verify-links-resolve](../design/verification-evidence.md#verify-links-resolve) · `link-findings` — Partial: a multi-crate project workspace.
- **Evidence:** Pending.

#### Honest test states {#symposium-integration-4-honest-states}

A whole-Book run reports every anchor by chapter with its criteria's states; a passing, a deliberately failing, and a restored test change the expected criterion; an agent-mode test that ran no agent is not satisfied evidence.

- **Anchor coverage:** [verify-criterion-states](../design/verification-evidence.md#verify-criterion-states) · `state-per-method` — Partial: tests that skip themselves at run time; [verify-progress-report](../design/verification-evidence.md#verify-progress-report) · `progress-output` — Partial: a second project.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 5. Symposium's tests and CI check its Anchor Book {#symposium-integration-5}

**Outcome:** Symposium's `cargo test` checks its Anchor Book through the library, its CI checks and renders the Book on every pull request, and CI retains a `verify` report for its tests.

**Status:** Proposed

**Identifier:** `symposium-integration-5`

**Dependencies:** [Milestones 1](#symposium-integration-1) and [4](#symposium-integration-4); [Verus as an optional engine feature](formal-verification.md#formal-verification-4), before Symposium adds the engine as a Cargo dependency; and retained reports working on this Book in [Verification Evidence milestone 3](verification-evidence.md#verification-evidence-3).

**Design basis:** [How a project uses Anchor Book](../design/project-integration.md#how-a-project-uses-anchor-book) and [what the crate must not impose](../design/project-integration.md#what-the-crate-must-not-impose).

**Tasks**

- [ ] Add the engine as a development dependency and an integration test calling the library check.
- [ ] Confirm the engine builds on Symposium's stable toolchain and edition on every platform in its CI matrix without Verus, `vstd`, or changes to Symposium's other dependencies.
- [ ] Add the check, render, and `verify` steps to Symposium's CI, keeping its existing jobs unchanged.

### Completion criteria

#### No imposed dependencies {#symposium-integration-5-no-imposed-dependencies}

Symposium's dependency tree with the engine contains no Verus crate, and its existing builds and tests pass unchanged on every platform in its CI matrix.

- **Anchor coverage:** [project-no-imposed-dependencies](../design/project-integration.md#project-no-imposed-dependencies) · `independent-resolution` — Partial: a project without Verus.
- **Evidence:** Pending.

#### Same results everywhere {#symposium-integration-5-same-results}

The library check in `cargo test`, the installed CLI, and the preprocessor report the same findings for the same Book at the same engine version, and a deliberately broken Book fails all three.

- **Anchor coverage:** [project-native-commands](../design/project-integration.md#project-native-commands) · `consumer-fixture` — Partial: a real project's package dependency; [project-one-engine-version](../design/project-integration.md#project-one-engine-version) · `same-version-entry-points` — Partial: one project.
- **Evidence:** Pending.

#### CI results {#symposium-integration-5-ci-results}

Actual CI runs on the Symposium branch check and render the Anchor Book and retain a `verify` report, and a deliberately broken Book fails the check.

- **Anchor coverage:** None — project CI; the report format is covered in Verification Evidence milestone 3.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 6. Agents receive the contract through a Symposium plugin {#symposium-integration-6}

**Outcome:** The engine crate provides a Symposium plugin that gives agents the contract and project bootstrap, used in this repository first and then in Symposium, without changing project files.

**Status:** Proposed

**Identifier:** `symposium-integration-6`

**Dependencies:** [Milestone 3](#symposium-integration-3) and the contract command from [Project Integration milestone 1](project-integration.md#project-integration-1).

**Design basis:** [Agent bootstrap](../design/project-integration.md#agent-bootstrap), which allows other agent integrations that follow the same three steps.

**Tasks**

- [ ] Declare the plugin in the engine's Cargo metadata: a skill that retrieves the contract and points agents to the project guidance and Book, and a session-start hook that only advises.
- [ ] Use it in this repository, then in Symposium, alongside or instead of the `AGENTS.md` bootstrap, as agreed.
- [ ] Confirm the hook changes no project files and that agents without the plugin still start from the `AGENTS.md` bootstrap.

### Completion criteria

#### Plugin start {#symposium-integration-6-plugin-start}

With the plugin enabled, a fresh agent session in this repository and then in Symposium retrieves the contract from the declared engine version and reads the project guidance and Book; project files are unchanged before and after.

- **Anchor coverage:** [project-agent-bootstrap](../design/project-integration.md#project-agent-bootstrap) · `contract-retrieval`, `fresh-session` — Partial: delivery through a plugin.
- **Evidence:** Pending.

### Completion record

Pending.
