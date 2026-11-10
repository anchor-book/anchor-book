# Structural Checking

**Roadmap identifier:** `structural-checking`

This roadmap builds the checker that [Engine Bootstrap](bootstrap.md) prepared
the design for. It extends the crate from
[Record Rendering — Milestone 1](record-rendering.md#record-rendering-1).
Milestones were assigned as scoped prototypes; review and acceptance of the
checker's governing records continue in
[Engine Bootstrap milestone 1](bootstrap.md#engine-bootstrap-1). The checker runs
independently of rendering and does not require generated Book output.

## Goal

An agent can use checker feedback to bring a Book into conformance with its
configured structure: run the check, understand the findings, make appropriate
changes within its assignment, and recheck until it passes. Diagnostics must
identify the problem, its location, and the expected structure. Repairs must
satisfy the intended rules without weakening them.

The checker provides a read-only CLI and Rust library. It does not render, edit
files, fetch dependencies, or run project tests. Agents make repairs in this
increment; later engine operations may support them.

The design is defined in:

- [Rust Engine](../design/rust-engine.md)
- [Configuration](../design/configuration.md)
- [Structural Checking](../design/structural-checking.md)
- [Checking Diagnostics](../design/check-diagnostics.md)

The roadmap sequences that design without maintaining a second configuration
schema or rule specification.

## Authority and anchors

The [requirements](../authority/requirements.md#initial-checker-requirements),
[decisions](../authority/decisions.md), and referenced anchors were accepted on
2026-11-10 in [Engine Bootstrap milestone 1](bootstrap.md#engine-bootstrap-1).
Milestone criteria link to specific anchor criteria with full or partial planned
coverage, following the [roadmap convention](index.md#anchor-criteria-and-planned-coverage).

Record parsing and authority resolution belong to rendering, including the
[authority-coverage follow-up](record-rendering.md#record-rendering-3-authority-coverage).
They do not expand this layout/navigation check. Evidence aggregation is later work.

Milestone 1 prepares configuration; milestone 2 delivers a usable local checker
whose findings explain format repairs and whose rerun confirms the corrected
structure. Its fixture tests use independently specified corrections. Milestone 4
then demonstrates that an agent can use those findings to choose and make repairs.
Milestones 3 and 4 can proceed independently after milestone 2. All four are
required to complete this roadmap.

Each milestone uses its linked criteria's declared verification methods. Navigation
grammar behavior is tested against the documented supported syntax and version;
it does not require a manual review of each Book. The agent-repair criterion
retains review of the resulting edits and preserved obligations.

## Milestone 1. The engine selects the intended Book and validates its configuration {#structural-checking-1}

**Outcome:** The library resolves the intended Book and its required structure from explicit configuration. Configuration problems identify what needs correcting before structural evaluation; no files are created or changed.

**Status:** Complete — merged with passing CI

**Dependencies:** [Record Rendering — Milestone 1](record-rendering.md#record-rendering-1) and the assigned configuration prototype design. Remaining [Bootstrap](bootstrap.md) review and fresh-session exercises proceed separately for this explicitly assigned milestone.

**Design basis:** [Checker design](../design/structural-checking.md), [Configuration](../design/configuration.md), [Rust Engine](../design/rust-engine.md), and [Diagnostics](../design/check-diagnostics.md)

**Identifier:** `structural-checking-1`

**Tasks**

- [x] Extend the prototype's library/CLI package; retain Rust 1.85 and the existing TOML/Serde dependencies for configuration. Navigation-parser selection belongs to milestone 2.
- [x] Implement typed configuration parsing, validation, and path resolution in a separate `configuration` capability; preserve the record, rendering, and adapter boundaries.
- [x] Validate and expose the configured project phase without changing structural rules or claiming automated design-readiness or completion checks.
- [x] Add the real configuration for this repository once the reader works, with source `book`, output `target/anchorbook`, and phase `prototyping`. Replace the temporary phase declaration in [Working on Anchor Book](../working-on-anchor-book.md#current-phase) with a pointer to configuration as the single source; keep the working contract independent of this project's current phase.
- [x] Exercise this repository and alternative layouts from different invocation directories. Use independently specified invalid configuration and corrected fixtures to verify errors identify the problem and expected form, and that corrected inputs resolve successfully.

### Completion criteria

#### Book selection {#structural-checking-1-book-selection}

Resolve the actual Book and equivalent docs/ and md/ layouts correctly, including different invocation directories and display titles.

- **Anchor coverage:** [check-book-selection](../design/configuration.md#check-book-selection) · `invocation-directory`, `alternative-layout` — Partial: configuration selection, resolved paths, and roles. Milestone 2 verifies equivalent structural findings across invocation directories and layouts.
- **Evidence:** Passed locally: `tests/configuration.rs` loads book/docs/md layouts and every required role from independent fixtures. Separate subprocesses use different working directories to select the same configuration and resolved paths; a symlinked configuration uses the actual file location. Current-project acceptance loads the real repository configuration.

#### Configuration validation {#structural-checking-1-configuration-validation}

Reject invalid fields, versions, phases, mappings, escapes, overlaps, and symlink aliases with located findings and expected conditions. Corrected configuration permits evaluation; allow a resolvable separate nonexistent output and distinguish missing from unresolvable paths.

- **Anchor coverage:** [check-configuration-validity](../design/configuration.md#check-configuration-validity) · `schema-rejection`, `path-boundaries` — Full for configuration loading. `missing-paths` — Partial: missing-source observations and resolution errors; milestone 2 turns those observations into structural findings.
- **Evidence:** Passed locally: schema unit tests cover required and unknown fields, invalid types, duplicate TOML keys, unsupported versions, and all phase values. Filesystem tests cover escapes, overlaps, resolved duplicate roles, symlink aliases/loops/dangling links, non-directory ancestors, and inaccessible descendants. Corrected fixtures resolve successfully; missing and non-directory sources remain explicit observations for milestone 2.

#### Configuration preservation {#structural-checking-1-configuration-preservation}

Configuration operations preserve file contents and directory entries on success and failure, including nonexistent output paths; tests make any attempted write observable.

- **Anchor coverage:** [check-observation-only](../design/structural-checking.md#check-observation-only) · `file-preservation` — Partial: configuration operations only
- **Evidence:** Passed locally: fixture snapshots compare all contents and directory entries before and after successful and failing loads. Missing output/source directories are not created. Source/path review confirms only read/metadata operations; permission and error-classification cases exercise failure handling.

#### Phase handoff {#structural-checking-1-phase-handoff}

The reader exposes the phase from the real repository configuration. Project guidance directs agents to that value and no longer declares a separate bootstrap phase; the working contract describes phase behavior without project-specific values. Valid phase choices do not alter structural rules or imply design acceptance or system completion.

- **Anchor coverage:** None — configuration and working-contract integration; [project phase design](../design/configuration.md#project-phase)
- **Evidence:** Passed locally: `anchorbook.toml` declares the phase; Working on Anchor Book now directs agents there without repeating the value. Unit tests accept all three phases and reject missing/invalid values. The ignored current-project acceptance test confirms the real configured phase and layout; loading makes no structural or completion claim.

#### Independent build {#structural-checking-1-independent-build}

Cargo build, formatting, Clippy, and configuration/API tests pass without rendering or validating the current Book.

- **Anchor coverage:** [check-independent-build](../design/system-overview.md#check-independent-build) · `broken-book` — Partial: engine build and fixture tests; current-Book checking follows
- **Evidence:** Passed locally: tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. The independence experiment passes with no Book and with an invalid Book; explicit Book acceptance rejects the invalid input. Configuration fixtures do not read the repository Book or configuration.

### Evidence {#structural-checking-1-evidence}

*Milestone evidence — local checks passed*

**Revision and scope:** Working tree based on `2359d59`, verified locally on 2026-10-24. The configuration capability, repository configuration, tests, and phase handoff are included together.

**Checks:** Formatting, ordinary tests and Clippy pass on Rust 1.85.0 and local stable 1.90.0. Four configuration unit tests cover decoding and observation-error classification; nine filesystem integration tests cover selection, paths, aliases, errors, preservation, and separate-process working directories. `cargo test --locked --test current_book -- --ignored` loads the real configuration and Book. The absent/invalid-Book independence experiment, mdBook build, and rendered acceptance pass. The existing authority proof still verifies 15 units with zero errors; it does not prove configuration loading.

**Mutation evidence:** cargo-mutants 27.1.0 on `src/configuration.rs` and `src/configuration/*.rs` first found three survivors in missing-path versus I/O-error handling; independent error and permission cases now cover them. Final runs: 65 mutations (34 caught, 31 unviable) and a 14-mutation retest of `paths.rs` (12 caught, 2 unviable), with no survivors, timeouts, or exclusions. Unviable mutations are not counted as detection.

**Limits:** Commands are in CONTRIBUTING. Filesystem tests ran on macOS; symlink and permission fixtures are Unix-specific, privileged runners can bypass permission bits, and native Windows behavior is not claimed.

### Integration

[PR #3](https://github.com/anchor-book/anchor-book/pull/3) merged as `7d795bb`. Its engine, Book, mutation, and authority-proof checks passed. The merged revision also passed [Engine](https://github.com/anchor-book/anchor-book/actions/runs/40262112290), [Book](https://github.com/anchor-book/anchor-book/actions/runs/40262112281), and [authority proof](https://github.com/anchor-book/anchor-book/actions/runs/40262112289) workflows.

### Completion record

Complete and merged: configuration loading, validation, preservation, and phase handoff have local and CI evidence. This establishes configuration loading, not an operational structural checker.

## Milestone 2. The checker identifies format repairs and confirms conformance {#structural-checking-2}

**Outcome:** An engineer or agent can run the library or CLI to locate format defects, understand the expected structure, make separate repairs, and recheck. A pass confirms the evaluated format rules; incomplete evaluation never counts as success.

**Status:** Complete — merged with passing CI

**Dependencies:** Structural Checking — Milestone 1.

**Design basis:** [Checker design](../design/structural-checking.md), [Configuration](../design/configuration.md), [Rust Engine](../design/rust-engine.md), and [Diagnostics](../design/check-diagnostics.md)

**Identifier:** `structural-checking-2`

**Tasks**

- [x] Integrate the selected `mdbook-summary = "=0.5.2"` parser behind a private adapter. Add supported-input validation and source-location mapping using Markdown events, including constructs the parser silently ignores; expose engine-owned results and errors. Read renderer settings, navigation, and chapter metadata using the documented supported syntax and version. Cover source and summary prerequisites, all linked chapter targets, configured section roles, and renderer alignment without executing preprocessors or creating missing files.
- [x] Evaluate the initial structural rules in a separate `checking` capability. Keep rules independent of rendering and protocol handling; share diagnostic primitives only where both implemented capabilities need them.
- [x] Expose the same findings and evaluated scope through the Rust library and `anchorbook check --config path/to/anchorbook.toml`. Include locations, expected and observed structure, and useful next actions; retain the documented result states, exit statuses, and output streams. Review the public API boundary and test actual CLI/library equivalence.
- [x] Explain missing prerequisites and incomplete evaluation so an agent knows whether it needs to repair structure or resolve an input/operational problem.
- [x] Exercise the current Book and independent passing, failing, and incomplete fixtures. Assert repair-relevant findings, apply independently specified corrections outside the checker, and recheck successfully. Verify each invocation preserves files and does not execute project tools; include failures after partial findings have been collected.
- [x] Complete mutation verification and final checks on GitHub, investigate any findings, and record evidence for the final revision before marking this milestone complete.

### Completion criteria

#### Navigation {#structural-checking-2-navigation}

The actual Book passes. Independent fixtures cover source and summary prerequisites, chapter targets, required roles, and supported and unsupported navigation. Assert located findings and expected structure; correcting seeded format defects produces a pass. Grammar-boundary tests use the supported mdBook version (0.5.4) and include ignored prose/code/headings, valid comments and reference links, duplicates, drafts, external destinations, encoded spaces, and source locations. Parser success alone cannot satisfy this criterion.

- **Anchor coverage:** [check-navigation-conformance](../design/structural-checking.md#check-navigation-conformance) · `chapter-inventory`, `required-roles`, `grammar-boundary` — Full
- **Anchor coverage:** [check-configuration-validity](../design/configuration.md#check-configuration-validity) · `missing-paths` — Partial: a resolvable missing source produces the expected structural violation; resolution failures prevent a pass. Configuration-resolution evidence is in milestone 1.
- **Evidence:** Local tests pass in `checking::navigation::tests` and `tests/checking.rs`: hierarchy, reference links, drafts, roles, malformed/ignored input, missing files, and correction/recheck cases. The current Book passes its explicit structural acceptance test. A trial on another project's Book later found that multi-line HTML comments, which mdBook accepts, were reported as unsupported input; [PR #15](https://github.com/anchor-book/anchor-book/pull/15) fixed this with regression tests against mdBook's parser. Other mdBook-parsable constructs remain unhandled; see the [navigation parser](../design/structural-checking.md#navigation-parser) design.

#### Renderer alignment {#structural-checking-2-renderer-alignment}

Report differing renderer settings and expected locations without editing files. Handle defaults, equivalent paths, missing configuration, and malformed or unreadable input; aligning seeded mismatches produces a pass.

- **Anchor coverage:** [check-renderer-alignment](../design/structural-checking.md#check-renderer-alignment) · `resolved-comparison`, `renderer-input` — Full for the criteria as they stood at this milestone. `resolved-comparison` later added a `book.toml` in a subdirectory of the configuration directory; [Project Integration milestone 1](project-integration.md#project-integration-1-mdbook-location) covers that case.
- **Evidence:** Local tests cover mdBook defaults, equivalent relative and absolute paths, missing/invalid renderer input, and corrected mismatches. A CLI test sets an environment override and confirms only file settings are checked. The implementation uses pure mdbook-core deserialization; it never runs the renderer or configured commands. These tests predate the subdirectory case and do not cover it.

#### Typed results {#structural-checking-2-typed-results}

Library and actual CLI results agree against independent expectations, including equivalent configured layouts and invocations from different working directories. Library errors preserve causes without printing, exiting, or panicking.

- **Anchor coverage:** [check-shared-typed-results](../design/rust-engine.md#check-shared-typed-results) · `equivalent-callers`, `library-boundary` — Full for the criteria as they stood at this milestone. `equivalent-callers` later added the `anchorbook md` preprocessor as a caller; [Project Integration milestone 1](project-integration.md#project-integration-1-entry-points) covers that case.
- **Anchor coverage:** [check-book-selection](../design/configuration.md#check-book-selection) · `invocation-directory`, `alternative-layout` — Partial: equivalent structural findings across layouts and caller directories, completing milestone 1 selection evidence.
- **Evidence:** Actual-process tests compare CLI output with library reports for pass, structural failure, and incomplete evaluation, plus independent expected fields and a complete passing report. Alternative layouts and caller directories are exercised. Source review confirms the CLI calls the shared library, findings retain typed locations and causes, and parser types remain private. The final GitHub mutation campaign has no survivors; see milestone evidence.

#### Truthful results {#structural-checking-2-truthful-results}

Verify documented exit statuses, incomplete and mixed failures, blocked checks, evaluated scope, and stdout/stderr behavior.

- **Anchor coverage:** [check-truthful-results](../design/check-diagnostics.md#check-truthful-results) · `outcomes-and-precedence`, `blocked-evaluation`, `report-scope` — Full
- **Evidence:** Local tests verify exit 0/1/2, complete stdout versus incomplete stderr, preserved partial findings, explicit blockers, and operational-error precedence. Model tests reject an empty unevaluated report as a pass. Missing prerequisites do not generate invented missing-role findings.

#### Diagnostics {#structural-checking-2-diagnostics}

For each seeded format defect, assert that findings identify the affected location, expected and observed structure, and useful repair guidance without weakening requirements. Verify documented fields, stable order, and readable output without styling; do not invent unavailable observations.

- **Anchor coverage:** [check-actionable-diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics) · `diagnostic-content`, `stable-presentation` — Full
- **Evidence:** Local tests assert independently specified codes, locations, related declarations, expected/found fields, guidance, and deterministic ordering. Unicode positions and terminal-control escaping are exercised. Guidance preserves intended configuration and required roles; the real agent usability demonstration remains in milestone 4.

#### Observation only {#structural-checking-2-observation-only}

Passing, failing, and incomplete checks preserve contents and directory entries; execution traps establish no subprocess, network, preprocessor, or verifier activity.

- **Anchor coverage:** [check-observation-only](../design/structural-checking.md#check-observation-only) · `file-preservation`, `no-execution` — Full
- **Evidence:** Fixture snapshots preserve all contents and directory entries across pass, failure, and incomplete runs. Missing chapters remain absent even when mdBook create-missing is enabled. Configured preprocessor/renderer command traps remain unexecuted, and external URLs produce no local-target failures. Source review confirms inspection uses file reads/metadata and pure parser calls, with no process or network-fetch path.

#### Acceptance cases {#structural-checking-2-acceptance-cases}

Record results for the design’s [acceptance cases](../design/structural-checking.md#acceptance-against-this-book), including alternate layouts and deliberate failures.

- **Anchor coverage:** None — acceptance suite record
- **Evidence:** The current Book and independent fixtures exercise missing prerequisites, alternate layouts, section placement, renderer drift, configuration errors, inaccessible targets, and corrected reruns. A real agent repair demonstration is deliberately reserved for milestone 4; passing fixture tests do not establish it.

### Evidence {#structural-checking-2-evidence}

*Milestone evidence — local, CI, and mutation checks passed*

**Implementation:** Based on `e02f48b`: structural/v1 in the library and CLI, with separate observation, navigation, rule, report-model, and presentation modules. The parser crates are now pinned to mdbook-summary and mdbook-core 0.5.4 (PR #6); that revision's full mutation campaign is recorded below. Checking invokes no write operation, project test runner, or verifier.

**Local evidence:** Ordinary tests and Clippy pass on Rust 1.88.0 and stable 1.99.1: 16 library tests and 19 checker integration tests covering actual CLI streams and statuses, fixture corrections, source preservation, path and role rules, renderer settings, unreadable input, errors, and source positions. The current Book passes `anchorbook check --config anchorbook.toml`.

**Mutation correction:** The [campaign for `e65cc0c`](https://github.com/anchor-book/anchor-book/actions/runs/40274238323) found four survivors in 464 mutations: standalone inline code was admitted, metadata errors after resolution were treated as missing files, and known top-level roles were blocked in two ways. Regression tests now cover them, without production or exclusion changes. The campaigns for [`fbdc073`](https://github.com/anchor-book/anchor-book/actions/runs/40348199734) and [`322a102`](https://github.com/anchor-book/anchor-book/actions/runs/40425789096) each tested 464 mutations: 354 caught, 110 unviable, none missed or timed out (cargo-mutants 27.1.0).

**Integration and limits:** [PR #5](https://github.com/anchor-book/anchor-book/pull/5) merged as `3d197c0`; its engine, Book, mutation, and authority-proof checks passed, as did the [Engine](https://github.com/anchor-book/anchor-book/actions/runs/40368530912), [Book](https://github.com/anchor-book/anchor-book/actions/runs/40368530949), and [authority proof](https://github.com/anchor-book/anchor-book/actions/runs/40368530948) workflows on main. The checker is test-validated, not formally proved. Unix symlink/permission cases have no native Windows result; privileged runners can bypass permission tests.

### Completion record

Complete on 2026-10-26. Merged in `3d197c0` with passing CI and a full mutation campaign with no survivors. On `a1cf336`, the checker passed this Book and reported a missing chapter and a `book.toml` source mismatch in a disposable broken copy, exiting 1 without changing files. Milestone 3 owns CI gating; milestone 4 owns the real agent repair demonstration.

## Milestone 3. Continuous checks protect the current Book {#structural-checking-3}

The shared rendering and publication gate also addresses Record Rendering
milestone 3; its other criteria remain separate.

**Outcome:** Changes to this repository receive independent engine validation and an acceptance check against its current Book before rendering.

**Status:** Complete — merged with GitHub evidence

**Dependencies:** Structural Checking — Milestone 2.

**Design basis:** [Checker design](../design/structural-checking.md), [Configuration](../design/configuration.md), [Rust Engine](../design/rust-engine.md), and [Diagnostics](../design/check-diagnostics.md)

**Identifier:** `structural-checking-3`

**Tasks**

- [x] Retain configuration and structural-rule tests in the independent engine and supported-toolchain checks introduced by rendering.
- [x] Run the structural check inside the Anchor Book preprocessor so every `mdbook build` refuses a failed or incomplete check, and make rendering depend on successful engine checks and current-Book acceptance for the same revision. Reuse rendered-output acceptance and reconcile shared workflow gating with [Record Rendering milestone 3](record-rendering.md#record-rendering-3).
- [x] Exercise valid, structurally broken, and incomplete-input Book cases locally and on GitHub; failed or incomplete acceptance must prevent rendering while independent engine tests remain runnable.
- [x] Document the preprocessor check and the shared workflow dependency chain.
- [x] Record actual CI results and job ordering for the implemented revision, including failure cases.

### Completion criteria

#### Independent acceptance {#structural-checking-3-independent-acceptance}

With deliberately broken navigation and no rendered output, engine compilation and fixture tests pass while current-Book acceptance reports the known failure.

- **Anchor coverage:** [check-independent-build](../design/system-overview.md#check-independent-build) · `broken-book` — Full
- **Evidence:** Passed locally and on GitHub. `scripts/check-engine-independence.sh` runs independent tests in a disposable checkout without rendered output, with no Book, an invalid record, a missing navigation target, and unsupported summary prose; separate acceptance rejects each with its expected diagnostic. The PR #9 runs below repeat the missing-chapter and unsupported-navigation cases on GitHub.

#### CI order {#structural-checking-3-ci-order}

Record actual CI showing engine checks, structural Book acceptance, then rendering for a valid revision. Broken or unprocessable Book input produces useful findings and prevents rendering; independent engine tests remain runnable.

- **Anchor coverage:** [check-independent-build](../design/system-overview.md#check-independent-build) · `ci-order` — Full. The criterion now requires `review`; the recorded runs below are its subject, and an assessment record for them is pending the [assessment records](verification-evidence.md#verification-evidence-6) support.
- **Evidence:** Passed on GitHub: valid revisions ran both Engine jobs, then Build mdBook, then Publish Book on main; broken and unprocessable Books failed current-Book acceptance with useful findings, independent tests passed, and rendering was skipped. Locally, `mdbook build` also refuses a structural failure (exit 1) and incomplete input (exit 2) through the preprocessor. See milestone evidence.

#### No repair {#structural-checking-3-no-repair}

The acceptance check performs no source repair; a valid Book renders after acceptance.

- **Anchor coverage:** [check-observation-only](../design/structural-checking.md#check-observation-only) · `file-preservation` — Partial: CI acceptance invocation
- **Evidence:** Passed locally: source trees, files, and directory entries are unchanged by passing, failing, and incomplete acceptance invocations, and acceptance creates no rendered output. `build_gate` renders a valid fixture Book and produces no rendered JSON for rejected ones. On GitHub, valid revisions render and publish after acceptance; file preservation was not observed in CI.

#### CI evidence {#structural-checking-3-ci-evidence}

Record the tested revision, actual workflow results, and local commands; local success alone is not CI evidence.

- **Anchor coverage:** None — integration evidence
- **Evidence:** Recorded in the milestone evidence: PR, main, and deliberate-failure runs with revisions, job order, and results.

### Evidence {#structural-checking-3-evidence}

*Milestone evidence — local experiments and GitHub runs passed*

**Revision and integration:** Implementation `7e81db1`, merged as `9ecd950` (PR #8). PR #10 (`613cb4d`, merged as `2aaf496`) runs current-Book acceptance before the failure experiments so a broken Book is reported directly.

**Implementation:** `book.yml` calls `engine.yml` as a local reusable workflow; Build mdBook requires both Rust jobs, which include current-Book acceptance. `[preprocessor.anchorbook] config` names `anchorbook.toml`, and `anchorbook md` stops on a failed or incomplete check before rendering any chapter. Rendered-output acceptance precedes upload and publication. The [native tool integration decision](../authority/decisions.md#decision-native-tool-integration) records why the check runs in the preprocessor. Proof and mutation workflows and merge rules are unchanged.

**Local failure experiments:** `build_gate` runs `anchorbook md` on fixture Books: broken navigation (exit 1) and unsupported navigation, invalid configuration, or missing configuration (exit 2) produce the check report, no rendered JSON, and unchanged snapshots. Real `mdbook build` runs on disposable copies of this Book reject unsupported summary prose (`input.unsupported`) and a renderer path mismatch (`structure.renderer-path-mismatch`) and render nothing; mdBook itself rejects a missing chapter before preprocessing because `create-missing` is false. Under `mdbook watch`, a failing rebuild keeps the previous output and a restored summary rebuilds.

**Local checks:** Formatting, ordinary tests, Clippy, current-Book acceptance, and the independence experiments pass on Rust 1.88.0 and stable 1.99.1. `mdbook build` passes with mdBook 0.5.4 and mdbook-mermaid 0.17.1, leaving sources unchanged; rendered acceptance checks 26 records and 1,066 local links. cargo-mutants 27.1.0 on the changed `src/adapters/mdbook.rs` and `src/main.rs`: 45 mutants, 42 caught, 3 unviable, none missed or timed out.

**GitHub results:** Valid revisions: the [PR #8 Book run](https://github.com/anchor-book/anchor-book/actions/runs/40453654594) for `7e81db1` passed Engine checks on Rust 1.88.0 and stable, then Build mdBook (Publish skipped on a PR); the [PR #10 Book run](https://github.com/anchor-book/anchor-book/actions/runs/40457351906) for `613cb4d` passed. On main, Book runs for [`9ecd950`](https://github.com/anchor-book/anchor-book/actions/runs/40455815967), [`2aaf496`](https://github.com/anchor-book/anchor-book/actions/runs/40458337088), and [`ed2ade0`](https://github.com/anchor-book/anchor-book/actions/runs/40459873774) passed with job order Engine (both toolchains), Build mdBook, Publish Book; mutation and authority-proof workflows also passed for the PR heads. Deliberate failures on throwaway [PR #9](https://github.com/anchor-book/anchor-book/pull/9): a [missing chapter (`e520765`)](https://github.com/anchor-book/anchor-book/actions/runs/40457441522) failed current-Book acceptance on both toolchains with `structure.chapter-missing` (result failed); [unsupported navigation prose (`de73f3e`)](https://github.com/anchor-book/anchor-book/actions/runs/40457892139) failed with `input.unsupported` (result incomplete). In both, the independent suites passed (16 library and 18 integration tests per toolchain), Build mdBook and Publish Book were skipped; PR #9 was closed unmerged.

**Limits:** Commands are in CONTRIBUTING. `build_gate` tests the preprocessor process boundary, not mdBook itself. On GitHub, broken Books are stopped by current-Book acceptance before Build mdBook runs, so the preprocessor's own refusal was demonstrated only in the local `mdbook build` experiments. File preservation was compared locally, not in CI.

### Completion record

Complete on 2026-11-07. Merged in `9ecd950` and `2aaf496`; local and GitHub evidence covers every criterion within the limits above.

## Milestone 4. An agent can repair Book structure using checker feedback {#structural-checking-4}

**Outcome:** An agent can use findings to make scoped structural repairs and reach a passing check while preserving unrelated content.

**Status:** Complete — recorded agent runs and independent review

**Dependencies:** Structural Checking — Milestone 2.

**Design basis:** [Checker design](../design/structural-checking.md), [Configuration](../design/configuration.md), [Rust Engine](../design/rust-engine.md), and [Diagnostics](../design/check-diagnostics.md), and the contract's [structural check guidance](../ai-working-contract.md#check-the-books-structure)

**Identifier:** `structural-checking-4`

**Tasks**

- [x] Prepare disposable copies of this Book with known structural defects and independently specified expected findings and valid repairs.
- [x] Give an agent the Book, the checker, and an explicit repair assignment; have it check, interpret the findings, edit the permitted files, and recheck.
- [x] Include missing chapters, misplaced required sections, and renderer-setting drift. Exercise invalid or unsupported input that must be resolved before evaluation. The agent must distinguish these results from a successful format check and preserve the intended configuration and required roles.
- [x] Improve unclear diagnostics or workflow guidance and repeat affected cases.
- [x] Record the initial defects, findings, edits, final result, and any human help.

### Completion criteria

#### Agent repair {#structural-checking-4-agent-repair}

Recorded agent runs repair seeded defects and reach a passing check. Independent review confirms intended repairs, unrelated content preservation, no weakened requirements, and any human assistance.

- **Anchor coverage:** [check-actionable-diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics) · `agent-repair` — Full. The criterion now requires only `review`; the recorded runs and independent review below are its subject, and an assessment record is pending the [assessment records](verification-evidence.md#verification-evidence-6) support.
- **Evidence:** Passed. Two fresh Codex sessions worked on throwaway branches of this Book, given only the repository bootstrap and an engineer request. Run 1 seeded a link typo, a required role turned into a targetless draft, and a navigation entry for a chapter that never existed. Run 2 seeded unsupported summary prose, renderer output drift, a required role nested under another, and a navigation entry for a chapter that never existed. Both runs repaired every defect whose intent was clear, leaving only the nonexistent chapter for the engineer; in run 2 the engineer removed that entry and the check passed. Independent review diffed each result against its baseline revision: run 1 matched exactly apart from the unresolved entry, and run 2 matched except for keeping the prose as an HTML comment. Neither run changed `anchorbook.toml` or engine code. Human assistance: in run 1 the agent reported the failures and stopped, repairing only after the engineer asked whether it had read the contract. That guidance gap was fixed before run 2, whose only input was the request to run the check.

#### Separate edits {#structural-checking-4-separate-edits}

Each checker invocation remains read-only; agent edits are separately identifiable.

- **Anchor coverage:** [check-observation-only](../design/structural-checking.md#check-observation-only) · `file-preservation` — Partial: invocations during the agent workflow
- **Evidence:** Passed. Every check invocation in both runs reported `files changed by check: none`. Each run's working-tree diff against its baseline contained only the agent's edits; in run 2 the defects were committed beforehand, so the diff isolated the agent's changes.

#### Unresolved cases {#structural-checking-4-unresolved-cases}

Cases outside the assignment or the agent’s ability produce an explicit explanation rather than a false completion claim; record versions and assistance.

- **Anchor coverage:** [check-truthful-results](../design/check-diagnostics.md#check-truthful-results) · `report-scope` — Partial: reported scope and limits during the workflow
- **Evidence:** Passed. In both runs the agent found that the entry's chapter had no file and no Git history, treated its intent as unclear, and neither invented the chapter nor removed the entry. It asked the engineer with concrete options and cited the contract, reporting the check as failed until the entry was resolved. Agent versions were not recorded; see the milestone evidence.

### Evidence {#structural-checking-4-evidence}

*Milestone evidence — recorded runs and review*

**Revisions:** Run 1 used Book and engine revision `ce65360`, with the structural check guidance from `ccf12ef`. Run 2 used `0534f96`, which adds the repair-by-default guidance, with seeded defects committed as `3540981` on a neutrally named branch. Both runs were local and on throwaway branches; the guidance merged in [PR #17](https://github.com/anchor-book/anchor-book/pull/17).

**Checks and observations:** Fresh Codex CLI sessions on Linux ran `anchorbook check --config anchorbook.toml` repeatedly while repairing. Run 2 also ran `mdbook build`, which reported the missing chapter. The engineer supplied only the requests described under each criterion. A preliminary dry run by a subagent on a disposable copy, with four other seeded defects, repaired all of them; it is preparation, not evidence.

**Results:** Every clear-intent defect was repaired without weakening configuration, and the unresolved case was raised rather than guessed. Run 1 exposed that agents stopped at reporting a failed check; the contract now makes repair the default, and run 2 confirmed the change.

**Limitations:** Two runs with one agent, Codex; Claude Code sessions are expected in ordinary work. Agent versions were not recorded. Run 1's branch name revealed that it was a test. The seeded defects and the guidance were written in the same working session, and review was by that session rather than a separate reviewer.

### Completion record

Complete on 2026-11-08. Recorded agent runs and independent review cover every completion criterion within the limits above. With milestones 1–3 complete, this completes the Structural Checking roadmap.

## Updating and completing the work

Use disposable copies for deliberate failures and public-safe reusable fixtures.
Exercise the actual Book as each capability becomes usable; Structural Checking — Milestone 3 is not the first
self-use experiment. Do not add success-returning placeholder commands.

Follow the [working contract](../ai-working-contract.md) for progress, evidence,
and design-to-roadmap updates.

This roadmap is complete when all four milestones are complete: the current
Book and independent fixtures validate the checker, CI exercises it, and recorded
agent runs demonstrate the check, repair, and recheck loop. A successful structural
check does not establish full anchor verification or completion of other roadmaps.
Materialization and contract generation remain separate later work.
