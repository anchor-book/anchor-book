# Symposium Integration

**Roadmap identifier:** `symposium-integration`

## Goal

The first major milestone is upstream Symposium adopting Anchor Book for real
development. Its agents start from the contract and its Book, then use current
build reports to implement and correct assigned work. Ordinary commands and CI
check the Book and present the same evidence in its rendered view, maintaining the relationship
between design and code. The engine's own project and Symposium are the test
subjects for developing that workflow. Symposium does not use Verus.

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
retained reports in [milestone 3](verification-evidence.md#verification-evidence-3),
and the current build results for agents and readers in
[milestone 9](verification-evidence.md#verification-evidence-9).
A gap found in Symposium is fixed and exercised here before Symposium relies on
the fix, or recorded as engine work in the relevant roadmap.

**The engine is the subject.** Symposium is the first real project to exercise
the engine's checking, rendering, and `verify` at scale and with agents building
in it. Its Book's content is ported in Anchor Book's format with light review and
stays draft for its maintainers to review when it is integrated; the trial's
findings concern the engine, not Symposium's design.

Follow the [agreed sequence](index.md#first-major-milestone-symposium-adopts-anchor-book).
[Verification Evidence milestone 2](verification-evidence.md#verification-evidence-2)
first encapsulates Verus and fixes the engine's proof CI. That work belongs to
the engine; it adds no Verus requirement to Symposium.

Milestone 4 has assessed the configuration-loading loop, using the Book and
bootstrap established by milestones 1 and 3. Correct engine gaps here and repeat
their Symposium cases before relying on the results. The no-agent correction is
proposed in [Verification Evidence milestone 10](verification-evidence.md#verification-evidence-10);
review support is in [milestone 8](verification-evidence.md#verification-evidence-8),
independent of the broader proof guards. Verification Evidence milestones 3 and 9
then supply the shared current build report and rendered evidence. Milestone 2 ports the remaining design
with light review, while milestone 4 extends test links to each area. Milestone 7
exercises real agent assignments through the Book, checks, and build reports.

Milestone 5 adopts the shared version alignment, native commands, and retained
test reports in Symposium's CI. Milestone 8 completes upstream integration and
contributor setup. The contract plugin in milestone 6 is deferred beyond this
first major milestone unless an adoption finding establishes that it is needed.

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

- **Anchor coverage:** [A-project-native-commands](../design/project-integration.md#A-project-native-commands) · `consumer-fixture` — Partial: one real project using an installed engine rather than a package dependency; [A-project-no-imposed-dependencies](../design/project-integration.md#A-project-no-imposed-dependencies) · `renderer-versions` — Partial: the mdBook version Symposium uses.
- **Evidence:** Passed locally on 2026-11-10, Symposium branch `anchor-book` (commit `98219876`, based on upstream `cb858e53`), on the fork. The engine was installed with `cargo install --git https://github.com/anchor-book/anchor-book --rev 40b0098fcbcc9948115680f7884ca709b9df18fc --locked anchor-book` (into a separate `--root`). With mdBook 0.5.4 and mdbook-mermaid 0.17.1, `anchorbook check --config anchorbook.toml` passed and `mdbook build anchor-book` rendered the three anchors, seven authority records, and one milestone card into `target/anchorbook`. The root `mdbook build` of the user Book still succeeded, and the commit adds files only, changing no tracked file. In disposable copies, a broken navigation target failed in mdBook before preprocessing, an unresolved authority reference failed with `record.invalid-reference`, and a renderer output mismatch failed with `structure.renderer-path-mismatch` (check exit 1); none produced output. Limits: one design area; installed engine only, not a package dependency; Symposium's CI does not build the Anchor Book yet. Setup note: mdBook copies `additional-js` paths with `../` outside the output directory, so a Book in a subdirectory carries its own copies of Symposium's Mermaid files.

#### Gaps recorded {#symposium-integration-1-gaps}

Every gap found has a fix with evidence in this Book or an entry in the relevant roadmap.

- **Anchor coverage:** None — trial record.
- **Evidence:** Recorded. (1) Cargo does not resolve a shortened `--rev`; fixed in [running the engine](../design/project-integration.md#running-the-engine), which now asks for the full commit hash. (2) `anchorbook --version` printed only the package version, so installations of different revisions were indistinguishable; fixed: it now prints the commit the binary was built from, as [running the engine](../design/project-integration.md#running-the-engine) describes. `tests/cli.rs` compares it with Git's `HEAD` for this checkout; by hand, a packaged crate reported its `.cargo_vcs_info.json` commit, and the same source without Git, or nested inside another repository, reported the revision as unknown. (3) The record and milestone styles and print script lived only in this repository's theme, so Symposium's cards rendered unstyled and its details did not expand when printing; fixed: the engine compiles them in and `anchorbook md` appends them to each chapter it renders records or milestones in, as [mdBook integration](../design/record-rendering.md#mdbook-integration) describes. This Book no longer lists them in its `book.toml` and renders with the delivered copies; `tests/rendering.rs` and `tests/roadmaps.rs` check the block appears once, last, in rendered chapters and not in others. Both fixes merged in PR #32, and Symposium's Anchor Book then pinned the merged engine `2af5c89`. (4) A project's authoring references are this Book's chapters at a pinned revision, with no versioned reference delivered with the engine; this is an entry in [remaining project integration](index.md#later-work-retained-from-the-design). (5) Symposium's `AGENTS.md` asks agents to add a `Co-authored-by` line, which conflicts with the engineer's practice of no commit attribution; it is left for [milestone 3](#symposium-integration-3)'s reconciliation.

### Completion record

Complete on 2026-11-10, on fork branch `anchor-book`, pushed to the fork and not proposed upstream. Symposium's Anchor Book checks and renders through its own commands with a pinned installed engine, the user Book is unchanged, broken copies are refused, and every gap found is fixed here or recorded. Its records serve both as a test case for the engine and as the start of Symposium's own Book: the two configuration decisions are reconstructed from Symposium's design and code, with alternatives written in the trial, and Symposium's maintainers review the content when it is integrated.

## Milestone 2. Symposium's design is ported into Anchor Book's format {#symposium-integration-2}

**Outcome:** Symposium's Anchor Book holds its whole design in Anchor Book's format, so the engine's checks and `verify` have a full-size project to work on: each design chapter with its anchors and criteria, and requirements and decisions adapted from its accepted and completed RFDs. Content is ported with light review and stays draft.

**Status:** Proposed

**Identifier:** `symposium-integration-2`

**Dependencies:** [Milestone 1](#symposium-integration-1), the [configuration-loop assessment](#symposium-integration-4-configuration-loop), and corrections needed before expanding that trial. Completion of all of milestone 4 is not required; it extends test links as this milestone supplies areas.

**Design basis:** [Anchor Definitions](../design/anchor-definitions.md), [Authority Definitions](../design/authority-definitions.md), and [choosing methods](../design/verification.md#choosing-methods).

**Tasks**

- [ ] **Design areas:** Port each design chapter in Symposium's contribution guide, such as sync, init, state, hooks, agents, subcommands, telemetry, and the report layer, keeping its text largely as written. Put anchors at the top of each chapter, stating the properties the chapter already describes.
- [ ] **Tenets:** Carry Symposium's tenets into the design overview as prose. A tenet becomes a requirement's source or a decision's rationale where a record needs it; tenets are not records themselves.
- [ ] **Authority:** Adapt accepted and completed RFDs into the requirements and decisions they establish, each naming its RFD; draft RFDs stay out until accepted.
- [ ] **Methods:** Give each criterion its required methods, `test` or `review`; no criterion requires `proof`.
- [ ] **Gaps:** Record record-format, authority, checking, or rendering gaps found at this size as in milestone 1.

### Completion criteria

#### Design coverage {#symposium-integration-2-design-coverage}

Every design area in Symposium's contribution guide is either represented by a design chapter with anchors or listed with a reason for leaving it out, and the Book passes the structural check and renders.

- **Anchor coverage:** None — project content; the engine's anchors are exercised by the check.
- **Evidence:** Pending.

#### Authority traced {#symposium-integration-2-authority-traced}

Each anchor cites at least one requirement or decision that resolves, and each adapted requirement and decision names its source. Light review checks that the port preserves the existing design and flags uncertain interpretations; maintainers review the content at upstream integration. Tenets remain prose except where a particular record uses one as its source or rationale.

- **Anchor coverage:** None — project content.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 3. Symposium's agents start from the contract and its Anchor Book {#symposium-integration-3}

**Outcome:** A fresh agent session in Symposium reads the contract, the project guidance, and the Anchor Book through the `AGENTS.md` bootstrap, and Symposium's existing agent instructions live in its project guidance without conflicting with the contract.

**Status:** Complete

**Identifier:** `symposium-integration-3`

**Dependencies:** [Milestone 1](#symposium-integration-1) and the contract command from [Project Integration milestone 1](project-integration.md#project-integration-1).

**Design basis:** [Agent bootstrap](../design/project-integration.md#agent-bootstrap) and [contract delivery](../design/system-overview.md#contract-delivery).

**Tasks**

- [x] Add the bootstrap section to Symposium's `AGENTS.md`.
- [x] Move Symposium's own instructions, such as checking common issues, keeping design chapters current, and its commit conventions, into the project guidance chapter, and record any that conflict with the contract for the engineer and maintainers to resolve.
- [x] Run fresh agent sessions and record what they read and report.

### Completion criteria

#### Agent start {#symposium-integration-3-agent-start}

A fresh agent session in the Symposium branch reads the contract, project guidance, and Book through the bootstrap, and identifies the phase, commands, and governing records of an adapted design area. Record agent versions and any assistance.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `fresh-session` — Partial: a second project, after this Book.
- **Evidence:** Passed on 2026-11-10, Symposium branch `anchor-book` at `e63d4e38`, on the fork, with the engine installed at `2af5c89`. Two fresh non-interactive Claude Code 2.1.319 sessions (model Opus 5.5), each in a new clone of the branch. Given only "Read the Book", the first ran `anchorbook contract` before anything else, read the guidance and all nine chapters in `SUMMARY.md`, ran the structural check, and reported the phase (`prototyping`), the passing check, the three draft anchors with the records' counts, the open `SYMPOSIUM_HOME` question, and the one Proposed milestone with what holds it, starting no work; it did not list the project's commands. The second was also asked which commands check, build, and test the project and which records govern each anchor; it followed the same start and named all six commands and each anchor's requirements and decisions correctly, and noticed that `decision-raw-and-normalized-configuration` is cited by no anchor. 4 and 6 turns, 22 and 40 seconds; neither changed a file. Limits: one agent, no Codex session; the sessions were allowed to run `anchorbook contract` and `anchorbook check`, and the host's permission settings let them run read-only shell commands too; the second session's question is assistance; the transcripts were reviewed by the agent that wrote the bootstrap.

#### Instructions reconciled {#symposium-integration-3-instructions}

Every instruction in Symposium's previous `AGENTS.md` is in its project guidance or listed as a conflict with its resolution.

- **Anchor coverage:** None — project guidance.
- **Evidence:** Passed. Every part of Symposium's previous `AGENTS.md` now has a place: the user Book imports (`@md/introduction.md`, `@md/SUMMARY.md`, `@md/design/module-structure.md`, `@md/design/important-flows.md`) became pointers in Working on Symposium, under design areas not yet in the Anchor Book, so they no longer load into every session; "check common issues first" and "build and test" are working rules there, with the build rule's TypeScript and Swift dropped because the repository no longer has that code, and its commands taken from the completed config-normalization RFD; the `Co-authored-by` commit convention is kept as Symposium's rule. One instruction conflicts with the contract: "update the design documentation in `md/design`" would keep a second copy of each design area the Anchor Book now owns. Interim resolution in the guidance: update the Anchor Book chapter and keep `md/design` consistent until the maintainers decide which is kept. The `Co-authored-by` rule does not conflict with the contract; the fork's first commits do not carry it, and later commits follow it.

### Completion record

Complete on 2026-11-10, on fork branch `anchor-book`, pushed to the fork and not proposed upstream. Fresh sessions start from the contract and Symposium's Anchor Book through the `AGENTS.md` bootstrap, and its earlier instructions live in the project guidance, with the design-documentation conflict resolved for now pending the maintainers' decision. Coverage is from Claude Code sessions only.

## Milestone 4. Symposium's tests are evidence for its criteria {#symposium-integration-4}

**Outcome:** Symposium's tests carry evidence links to its anchors' criteria, and `anchorbook verify --method test` reports each criterion's state across its workspace in the build loop.

**Status:** In progress

**Identifier:** `symposium-integration-4`

**Dependencies:** The existing test runner in [Verification Evidence milestone 1](verification-evidence.md#verification-evidence-1), and the early engine CI work in [milestone 2](verification-evidence.md#verification-evidence-2). Start with configuration loading, already anchored in [Symposium milestone 1](#symposium-integration-1); the first-loop criterion can be assessed before porting more design. Complete this milestone after milestone 2's design coverage, extending links as each area arrives and resolving the engine findings needed for honest results.

**Design basis:** [Running Verification](../design/verification-evidence.md) and [evidence links](../design/verification-evidence.md#evidence-links).

**Tasks**

- [x] Add the `[verification]` section to Symposium's `anchorbook.toml`.
- [x] Use the current pinned engine and its line grammar in the existing configuration-loading area; preserve any migration already completed on the fork. Record a passing, deliberately failing, and restored `verify --anchor` run before expanding the Book.
- [ ] Link existing tests to the criteria they address, only where a test exercises its criterion, and record the criteria that have none.
- [x] Run `verify` over Symposium's workspace, which has several member crates and excludes its evaluation fixtures. The current configuration links select tests in the root package only.
- [ ] Agent-mode tests report success without running anything when no agent is configured. Make sure `verify` reports such a run as not evaluated, never as satisfied; fix this in the engine and this Book first if it does not.
- [x] Symposium runs `cargo test`, and the runner uses nextest. Confirm nextest runs Symposium's tests with the same results on its supported platforms, or record the gap. Linux results agree; Windows and macOS remain unassessed, including the Windows-ignored auto-update link.
- [ ] Review the assertions behind each link against the whole criterion. Record partial coverage without weakening the criterion, and use [criterion review records](verification-evidence.md#verification-evidence-8) where review is required. Passing linked tests alone never establish sufficiency.
- [ ] For every engine gap, record the expected behavior, reproducer, and affected design and milestone in this Book. Implement and exercise the correction here before Symposium relies on it, then repeat the original trial. Findings outside the current assignment receive bounded work with completion criteria.

### Completion criteria

#### Configuration loop assessed {#symposium-integration-4-configuration-loop}

The existing configuration-loading area checks and renders with the current
pinned engine and syntax. A linked test produces passing, deliberately failing,
and restored evidence for the intended criterion. Recorded cases assess partial
coverage and a test that runs no agent, identifying what the engine reports and
every correction needed before depending on it. The trial records revisions,
commands, results, and limits; recording a gap is not evidence of its correction.

- **Anchor coverage:** None — first project trial and input to engine corrections.
- **Evidence:** Passed on 2026-11-12 at Symposium `4e2158b8`, using engine `b7bcd33`. The configuration Book checks and renders, eight links resolve, and removing/restoring the save guard produces failed/satisfied states. Partial coverage and both no-agent cases are assessed below, with the remaining corrections assigned proposed scope. This completes the first-loop assessment, not this milestone's broader outcome.

#### Linked tests {#symposium-integration-4-linked-tests}

Every link in Symposium's sources resolves to a criterion requiring `test`, misplaced or dangling links are reported at their location, and unlinked criteria are listed. Every ported area's test criteria have reviewed links or an explicit missing-coverage entry; review identifies what the assertions cover and the remaining gaps.

- **Anchor coverage:** [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve) · `link-findings` — Partial: a multi-crate project workspace.
- **Evidence:** Configuration area assessed: eight tests link to six of seven criteria, with no link findings; `hooks-continue` is unlinked. The [fork's coverage map](https://github.com/kwdev/symposium/blob/4e2158b89461d17496ff07c5d2a0ebf2619abfe3/anchor-book/src/implementation-roadmaps/configuration-loading.md#configuration-loading-1-trial) records each assertion boundary and missing case. Full design coverage and links across other member packages remain pending.

#### Honest test states {#symposium-integration-4-honest-states}

A whole-Book run reports every anchor by chapter with its criteria's states; a passing, a deliberately failing, and a restored test change the expected criterion; an agent-mode test that ran no agent is not satisfied evidence. The engine fixes needed for these results have independent cases and evidence in this Book, and the original Symposium cases pass on the corrected revision. Required reviews expose partial coverage and cannot turn missing test evidence into satisfied evidence.

- **Anchor coverage:** [A-verify-criterion-states](../design/verification-evidence.md#A-verify-criterion-states) · `state-per-method` — Partial: tests that skip themselves at run time; [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) · `progress-output` — Partial: a second project.
- **Evidence:** Not met. Configuration failures and corrections report correctly, but an agent-only test with no agent reports satisfied. [Verification Evidence milestone 10](verification-evidence.md#verification-evidence-10) owns the proposed execution-evidence correction; [milestone 8](verification-evidence.md#verification-evidence-8) owns current reviews of test sufficiency.

### Evidence {#symposium-integration-4-evidence}

**Revision and scope:** Symposium `4e2158b89461d17496ff07c5d2a0ebf2619abfe3`,
branch `codex/configuration-trial`, based on the migrated Book at `99d12af2`;
engine `b7bcd33d0035fdc6af9790e0fd8568394772afb9`, installed from that exact Git
revision. Runs used Linux x86_64, Rust 1.99.1, cargo-nextest 0.9.151, mdBook 0.5.4,
and mdbook-mermaid 0.17.1. This is a fork trial, not upstream adoption.

**Configuration loop:** `anchorbook verify --config anchorbook.toml --anchor
A-config-unreadable-file-unchanged --method test` reports passing linked tests.
Removing the `config_error` guard from `Symposium::save_config` makes
`save-refused` and `commands-refuse` failed; restoring the code restores satisfied
results. A second deliberate defect, an error omitting the filename, is also
caught by the strengthened save-refusal tests. All temporary changes were
restored, and each run preserved its tracked inputs. Warm selected runs took
about 0.6 seconds, or 7 seconds with recompilation. All exit 0: the Book has no
Complete milestone claiming these criteria, so the report's states carry the
failure, as designed.

**Coverage:** A whole-Book test run reports six satisfied criteria and one
unlinked, with eight tests in the root package. Passing tests cover only part of
the directory matrix, saving-command set, missing/empty-file behavior, and
default/warning behavior. In particular, `init` alone cannot establish the five
commands in `commands-refuse`. The preliminary auto-update link was attached to
`auto_update_re_execs_on_sync`, which expects an update; the trial replaces it
with `unparseable_config_never_auto_updates`, which actually exercises the
criterion's boundary. The fork's coverage map preserves each remaining gap;
the criteria and required methods were not weakened.

**No-agent defect:** A disposable criterion about consuming the tokio-weather
skill was linked to `agent_reads_tokio_skill_after_cargo_add`, not to an unrelated
configuration criterion. With `SYMPOSIUM_ENABLE_AGENT_TESTING` absent, and again
with it set to `1` and `test-agents.toml` containing `test-agents = []`, nextest
and cargo test both report success. The direct command
`cargo test --locked -p symposium --test hook_agent agent_reads_tokio_skill_after_cargo_add -- --exact --nocapture`
prints `agent-only test ran no agent`; verify nevertheless reports satisfied.
No agent was started. The reports have identical criteria/configuration/code
identities despite the different runtime settings. The temporary records and
link were removed after the experiment. A current, defined execution signal and
its relevant runtime inputs must be accounted for before relying on these tests;
milestone 10 defines the bounded proposed correction.

**Rendering acceptance correction:** The rendered Book was correct, but
`tests/rendered_book.rs` discarded literal asterisks while normalizing expected
fields, rejecting `XDG_*`. Its helper now reads Markdown text/code events;
`field_text_preserves_literal_punctuation` detects the original fault. Rendered
acceptance then passed against Symposium. This changes the acceptance test, not
the installed engine's rendering behavior.

**Validation and limits:** The structural check and Book build pass. Symposium's
full `cargo test --workspace --locked` reports 1,713 passed, no failures, and 15
ignored; agent-only `ok` results still supply no agent evidence. Formatting
passes, and Clippy exits 0 with eight pre-existing SDK macro warnings and two
type-complexity warnings in unchanged agent tests. The engine's tests, Clippy,
and formatting pass on stable and Rust 1.88.0. Actual agent execution, Windows,
macOS, linked tests in other workspace packages, criterion review records, and
the no-agent correction remain outside this assessment's evidence. The Windows
auto-update test is ignored in ordinary Cargo runs, while verify requests ignored
tests too; that link needs platform treatment before it is used there.

The engine acceptance-test correction and this assessment were tested in
`71d9c41`, based on `b7bcd33`, on branch `codex/symposium-configuration-trial`.
[PR #41](https://github.com/anchor-book/anchor-book/pull/41) tracks the required
revision-specific checks and integration of that work. Symposium's trial commit
is pushed to the engineer's fork; no upstream adoption is claimed.

### Completion record

Pending. The configuration-loop assessment is complete; design expansion,
review records, and the no-agent correction remain before this milestone can
claim honest evidence across Symposium's Book.

## Milestone 5. Symposium's tests and CI check its Anchor Book {#symposium-integration-5}

**Outcome:** Symposium's `cargo test` checks its Anchor Book through the library, its CI checks and renders the Book on every pull request, and agents and the rendered Book receive the same current evidence from a retained report.

**Status:** Proposed

**Identifier:** `symposium-integration-5`

**Dependencies:** [Milestones 2](#symposium-integration-2), [4](#symposium-integration-4), and [7](#symposium-integration-7); version alignment and setup in [Project Integration milestone 4](project-integration.md#project-integration-4); and retained reports and the evidence build working on this Book in [Verification Evidence milestone 3](verification-evidence.md#verification-evidence-3) and [milestone 9](verification-evidence.md#verification-evidence-9). [Verus as an optional engine feature](formal-verification.md#formal-verification-4) is already complete. Broader proof guards and consumer Verus support are not prerequisites.

**Design basis:** [How a project uses Anchor Book](../design/project-integration.md#how-a-project-uses-anchor-book) and [what the crate must not impose](../design/project-integration.md#what-the-crate-must-not-impose).

**Tasks**

- [ ] Add the engine as a development dependency and an integration test calling the library check.
- [ ] Align the dependency, CLI, preprocessor, and configuration with the declared engine version using the shared setup. Pin a full Git revision before registry publication; do not copy the engine's Verus installation or proof workflow into Symposium.
- [ ] Confirm the engine builds on Symposium's stable toolchain and edition on every platform in its CI matrix without Verus, `vstd`, or changes to Symposium's other dependencies.
- [ ] Add the check, render, and `verify` steps to Symposium's CI, keeping its existing jobs unchanged.
- [ ] Connect the shared build report to agents and rendering. Run verification before the evidence-consuming build, compare current inputs, and retain diagnostics for missing or stale reports; include configured review results and explicit unevaluated methods.
- [ ] Use the supported pinned mdBook version, run tests once for the retained nextest evidence, and document local equivalents and missing-tool diagnostics. Exercise a broken Book, failed claimed evidence, and corrected runs.

### Completion criteria

#### No imposed dependencies {#symposium-integration-5-no-imposed-dependencies}

Symposium's dependency tree with the engine contains no Verus crate, and its existing builds and tests pass unchanged on every platform in its CI matrix.

- **Anchor coverage:** [A-project-no-imposed-dependencies](../design/project-integration.md#A-project-no-imposed-dependencies) · `independent-resolution` — Partial: a project without Verus.
- **Evidence:** Pending.

#### Same results everywhere {#symposium-integration-5-same-results}

The library check in `cargo test`, the installed CLI, and the preprocessor report the same findings for the same Book at the same engine version, and a deliberately broken Book fails all three.

- **Anchor coverage:** [A-project-native-commands](../design/project-integration.md#A-project-native-commands) · `consumer-fixture` — Partial: a real project's package dependency; [A-project-one-engine-version](../design/project-integration.md#A-project-one-engine-version) · `same-version-entry-points` — Partial: one project.
- **Evidence:** Pending.

#### CI results {#symposium-integration-5-ci-results}

Actual CI runs on the Symposium branch check and render the Anchor Book and retain a revision-specific evidence report from one test run. The agent report and rendered criteria agree on inputs, scope, and states. A deliberately broken Book fails the check, failed claimed evidence is reported with its failure status, stale evidence cannot produce a current-evidence build, and corrected revisions pass. These jobs run without Verus or the engine's verification toolchain.

- **Anchor coverage:** None — project CI; the report format is covered in Verification Evidence milestone 3.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 6. Agents receive the contract through a Symposium plugin {#symposium-integration-6}

**Outcome:** The engine crate provides a Symposium plugin that gives agents the contract and project bootstrap, used in this repository first and then in Symposium, without changing project files.

**Status:** Proposed

**Identifier:** `symposium-integration-6`

**Dependencies:** Deferred beyond the first major milestone unless a recorded adoption finding requires it. [Milestone 3](#symposium-integration-3) and the contract command from [Project Integration milestone 1](project-integration.md#project-integration-1) supply the existing bootstrap.

**Design basis:** [Agent bootstrap](../design/project-integration.md#agent-bootstrap), which allows other agent integrations that follow the same three steps.

**Tasks**

- [ ] Declare the plugin in the engine's Cargo metadata: a skill that retrieves the contract and points agents to the project guidance and Book, and a session-start hook that only advises.
- [ ] Use it in this repository, then in Symposium, alongside or instead of the `AGENTS.md` bootstrap, as agreed.
- [ ] Confirm the hook changes no project files and that agents without the plugin still start from the `AGENTS.md` bootstrap.

### Completion criteria

#### Plugin start {#symposium-integration-6-plugin-start}

With the plugin enabled, a fresh agent session in this repository and then in Symposium retrieves the contract from the declared engine version and reads the project guidance and Book; project files are unchanged before and after.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `contract-retrieval`, `fresh-session` — Partial: delivery through a plugin.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 7. Agents build in Symposium through its Anchor Book {#symposium-integration-7}

**Outcome:** Recorded sessions show whether agents assigned real Symposium changes keep its design, records, code, and tests consistent by working from the contract, the Book, and current build results, using `anchorbook check` and the `verify --anchor` build loop. The engine gaps they expose become work in this Book.

**Status:** Proposed

**Identifier:** `symposium-integration-7`

**Dependencies:** [Milestone 3](#symposium-integration-3), milestone 4's assessed configuration loop and required engine corrections, and enough ported design from [milestone 2](#symposium-integration-2) to state the three assignments. Full Book coverage is not required to begin a bounded assignment. Review records from [Verification Evidence milestone 8](verification-evidence.md#verification-evidence-8) are required where an assignment relies on the review method.

The current build report and rendering from [Verification Evidence milestone 9](verification-evidence.md#verification-evidence-9) must work in the engine's Book first and be available in the trial's local workflow; Symposium's CI integration follows in milestone 5.

**Design basis:** The contract's build-and-verify guidance (`anchorbook contract`, "Work with the engineer" and "Develop verification with the code"), [using verify](../design/verification-evidence.md#using-verify), and [agent bootstrap](../design/project-integration.md#agent-bootstrap).

**Tasks**

- [ ] Choose at least three assignments with independently stated expected results: adding tests for an unlinked criterion, a behavior change within an anchored area, and a design change that must update the design chapter, its records, and the roadmap together.
- [ ] Run each in a fresh agent session on a disposable branch, given only the bootstrap and the assignment. Record agent versions, tool calls, the checks it ran, and any human assistance.
- [ ] Have agents read Markdown sources and consume current CLI or machine-readable build results, then act on located findings and evidence gaps. Record correction and reassessment after changes, and confirm that rendered evidence agrees without requiring agents to inspect HTML.
- [ ] Review each result against its expected result: the structural check and `verify` pass, links exercise their criteria, design and records describe the changed code, and nothing was weakened to obtain a pass.
- [ ] Record where the checks caught drift, where they missed it, and where their output misled the agent; fix each engine gap in this Book first, or add it to the relevant roadmap.

### Completion criteria

#### Assigned changes {#symposium-integration-7-assigned-changes}

Each assignment ends with the structural check passing and current build evidence reporting the affected criteria consistently to agents and the rendered Book. Agents use the results to identify and correct gaps. Independent review confirms the Book, code, and tests are consistent, unrelated content is preserved, and no criterion or check was weakened; assistance is recorded.

- **Anchor coverage:** None — trial of the engine in a project; informs [A-verify-progress-report](../design/verification-evidence.md#A-verify-progress-report) and [A-verify-links-resolve](../design/verification-evidence.md#A-verify-links-resolve).
- **Evidence:** Pending.

#### Findings recorded {#symposium-integration-7-findings}

Every inconsistency the checks missed or misreported has a fix with evidence in this Book or an entry in the relevant roadmap.

- **Anchor coverage:** None — trial record.
- **Evidence:** Pending.

### Completion record

Pending.

## Milestone 8. Symposium adopts Anchor Book upstream {#symposium-integration-8}

**Outcome:** Symposium's upstream repository uses Anchor Book for ordinary development, with reviewed project content, agent bootstrap, current build results for agents and the rendered Book, native checks, retained evidence, and reproducible contributor setup. This completes the first major milestone.

**Status:** Proposed

**Identifier:** `symposium-integration-8`

**Dependencies:** Milestones 2, 3, 4, 5, and 7, resolution of findings that would make the adopted workflow misleading or unusable, and the maintainers' integration workflow. The contract plugin, consumer proofs, and registry publication are not prerequisites.

**Design basis:** [Project Integration](../design/project-integration.md), the [agreed sequence](index.md#first-major-milestone-symposium-adopts-anchor-book), and the preceding trial evidence.

**Tasks**

- [ ] Prepare the upstream change with the Book, bootstrap, test links, native integration, and contributor instructions, supported by the trial and CI results.
- [ ] Resolve maintainers' content review and which design documentation is authoritative. Preserve the user Book's purpose and replace the interim requirement to maintain duplicate design text with the agreed ownership rule.
- [ ] Record remaining limitations and their owned follow-up work. Resolve misleading evidence and workflow blockers before adoption; do not silently narrow criteria to clear the checks.
- [ ] Follow the maintainers' contribution workflow to integrate the change. Record the upstream revision and its checks; a fork branch or an open pull request alone does not complete this milestone.
- [ ] From a fresh checkout of that revision, reproduce installation, agent startup, checking, testing, rendering, and a scoped evidence run using only the contributor instructions.

### Completion criteria

#### Upstream workflow {#symposium-integration-8-upstream-workflow}

The integrated upstream revision contains the agreed Book and development workflow,
its required checks pass, and design-document ownership is explicit. Recorded agent
assignments demonstrate Book, implementation, and evidence remaining consistent.
Remaining limitations have explicit follow-up work and do not invalidate the
adopted workflow's claims.

- **Anchor coverage:** None — first major adoption outcome; engine guarantees retain their own evidence.
- **Evidence:** Pending.

#### Reproducible adoption {#symposium-integration-8-reproducible-adoption}

A fresh upstream checkout reproduces the documented commands and agent bootstrap
without the engine's development checkout, Verus, or undocumented local setup.
The record identifies versions, revision, commands, results, and any assistance.
The agent report and rendered Book expose the same current scope and evidence;
changing inputs invalidates earlier results until the documented build loop runs
again.

- **Anchor coverage:** None — upstream contributor acceptance.
- **Evidence:** Pending.

### Completion record

Pending.
