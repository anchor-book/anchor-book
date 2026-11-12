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
Milestone 4 supplies the remaining native integration and setup needed for the
[first major milestone](index.md#first-major-milestone-symposium-adopts-anchor-book),
upstream adoption in Symposium.

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

[Verification Evidence milestone 2](verification-evidence.md#verification-evidence-2)
records the engine's proof CI encapsulation and proof-state coverage. The next
work is the Symposium configuration-loading trial. Milestone 4 below supports the
later native-command and CI integration, using the trials to settle the minimum
setup a real adopter needs. Registry publication is a separate outcome; a pinned
Git dependency and installed engine can support the first adoption.

## Milestone 1. This Book uses the integration groundwork {#project-integration-1}

**Outcome:** This repository uses every mechanism a project will rely on: its configuration names its `book.toml`, its agents retrieve the contract with `anchorbook contract`, a pinned engine revision installed the documented way checks and renders this Book, and the declared mdBook versions are tested.

**Status:** Complete

**Dependencies:** The configuration loader, structural checker, and record renderer.

**Design basis:** [mdBook configuration location](../design/configuration.md#relationship-to-mdbook), [contract delivery](../design/system-overview.md#contract-delivery), [running the engine](../design/project-integration.md#running-the-engine), and [agent bootstrap](../design/project-integration.md#agent-bootstrap).

**Identifier:** `project-integration-1`

**Tasks**

- [x] **Configuration:** Implement `[book].mdbook`, resolving mdBook's source and output relative to that file, and migrate this repository's configuration to name its `book.toml`.
- [x] **Contract:** Move the maintained contract source into the engine, add `anchorbook contract`, switch this repository's `AGENTS.md` to retrieve it, and remove the temporary Book copy and its navigation entry, as [contract delivery](../design/system-overview.md#contract-delivery) describes.
- [x] **Running the engine:** Choose and document how a project installs a pinned engine revision before crate release, and exercise that path in this repository's CI against this Book.
- [x] **Entry points:** Test that the library, the CLI, and the `anchorbook md` preprocessor report equivalent results for the same fixtures.
- [x] **mdBook versions:** Decide which mdBook versions the preprocessor supports, including the version Symposium uses or a documented pin, and test each against this Book.

### Completion criteria

#### mdBook location {#project-integration-1-mdbook-location}

The loader accepts a configured `book.toml`, including one in a subdirectory, and compares its resolved source and output; fixtures cover a subdirectory, a mismatch, a missing file, and an escaping path. This repository's configuration names its `book.toml` and its Book still passes and renders.

- **Anchor coverage:** [A-check-renderer-alignment](../design/structural-checking.md#A-check-renderer-alignment) · `resolved-comparison`, `renderer-input` — Partial: a configured `book.toml` location; [A-check-configuration-validity](../design/configuration.md#A-check-configuration-validity) · `path-boundaries` — Partial: the new field.
- **Evidence:** Passed locally on Rust 1.88.0 and stable 1.99.1, branch `project-integration`. `tests/configuration.rs` rejects a missing `book.mdbook` and empty, blank, escaping, and absolute values with located errors, and resolves `anchor-book/book.toml` without creating it. `tests/checking.rs` `renderer_configuration_in_a_subdirectory_resolves_paths_from_its_own_directory` passes with `book.toml` in a subdirectory whose paths are relative to it, while a malformed root `book.toml` is ignored; reports a mismatch at the subdirectory file with expected `docs` and found `anchor-book/docs`; and reports a missing named file as `structure.renderer-missing`, related to `anchorbook.toml` `book.mdbook`. This repository's `anchorbook.toml` names `book.toml`; the structural check, `mdbook build`, and current-Book and rendered-Book acceptance pass. cargo-mutants 27.1.0 on the four changed source files: 81 mutations, 48 caught, 33 unviable, none missed or timed out.

#### Contract {#project-integration-1-contract}

`anchorbook contract` prints the contract packaged with the running engine without reading project files; this repository's agents start from it, and a fresh session in this repository reads it, the project guidance, and the Book. No copy of the contract remains in the Book.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `contract-retrieval` — Full; `fresh-session` — Partial: this repository.
- **Evidence:** `contract-retrieval` passed locally on Rust 1.88.0 and stable 1.99.1, branch `project-integration`. The contract's source is eight section files in `contract/`, joined at compile time by `src/contract.rs`; the joined text is byte-identical to the former Book chapter. `tests/contract.rs` runs the actual CLI in an empty directory and in one with a malformed `anchorbook.toml` and its own `AGENTS.md`: both print the same text, equal to the library's, with no stderr; the project's instructions do not appear. It also rejects extra arguments with exit 2, checks every section file is joined exactly once in order, and resolves every link within the contract. The Book no longer contains the chapter or its navigation entry; its references name the contract's sections and the command. `AGENTS.md` retrieves the contract with `cargo run --quiet --locked --bin anchorbook -- contract`. `fresh-session` (partial: this repository, one agent): on 2026-11-10 a fresh non-interactive Claude Code 2.1.319 session (model Opus 5.5) in a new clone of branch `project-integration`, given only the prompt "Read the Book", first ran `cargo run --quiet --locked --bin anchorbook -- contract` and received the whole contract in one result, read all 29 chapters listed in `SUMMARY.md` (opening each overflowing output's saved file), ran the structural check, and reported the phase (`prototyping`), the check result, what is implemented, and every open milestone with what holds it, starting no work. It also proposed four corrections to stale text, since made. 22 turns, about 105 seconds, no human assistance. Limits: one agent, no Codex session; the session was allowed to read files and run the contract and check commands, but the host's permission settings let it run other read-only shell commands too; the review of its transcript was by the agent that wrote the bootstrap.

#### Installed engine {#project-integration-1-installed-engine}

A CI job installs the pinned engine revision the documented way and checks and renders this Book with it, with the same findings as the in-repository build; each declared mdBook version renders this Book, and an undeclared version is rejected with a diagnostic naming the supported ones.

- **Anchor coverage:** [A-project-no-imposed-dependencies](../design/project-integration.md#A-project-no-imposed-dependencies) · `renderer-versions` — Full; [A-project-one-engine-version](../design/project-integration.md#A-project-one-engine-version) · `same-version-entry-points` — Partial: installed and in-repository builds of one revision.
- **Evidence:** Partial, locally on branch `project-integration`: `cargo install --git file://<checkout> --rev <head> --locked anchor-book` installed the engine; its `anchorbook check` output and exit status and its `anchorbook contract` output were identical to the in-repository build's; `mdbook build` with `MDBOOK_PREPROCESSOR__ANCHORBOOK__COMMAND="anchorbook md"` rendered this Book through it, confirmed by a deliberately wrong command failing the build, and rendered-Book acceptance passed. On GitHub, the Book workflow's [Installed engine](https://github.com/anchor-book/anchor-book/actions/runs/42762907783/job/129653094417) job passed for PR #28 head `ccc7adb`, installing that revision with `cargo install --git ... --rev ... --locked`, matching the in-repository check and contract output, rendering this Book through the installed binary, and passing rendered-Book acceptance. The declared mdBook versions are `SUPPORTED_MDBOOK_VERSIONS` in `src/adapters/mdbook.rs`, currently 0.5.4, the newest release; it is the version both Book jobs use, and Symposium milestone 1 pins Symposium's mdBook to it. `tests/rendering.rs` `undeclared_mdbook_versions_are_rejected_naming_the_supported_ones` accepts each declared version through the actual `anchorbook md` process and rejects 0.5.5, 0.6.0, 0.5.3, and 0.4.52 with no Book JSON and a diagnostic naming the running and supported versions.

#### Equivalent entry points {#project-integration-1-entry-points}

The library, the actual CLI process, and the actual `anchorbook md` preprocessor process report equivalent outcomes, codes, locations, and evaluated scope for the same passing, failing, and incomplete fixtures, against independent expectations.

- **Anchor coverage:** [A-check-shared-typed-results](../design/rust-engine.md#A-check-shared-typed-results) · `equivalent-callers` — Partial: the preprocessor caller; library and CLI equivalence is recorded in [Structural Checking milestone 2](structural-checker.md#structural-checking-2-typed-results).
- **Evidence:** Passed locally on Rust 1.88.0 and stable 1.99.1, branch `project-integration`. `tests/build_gate.rs` `library_cli_and_preprocessor_report_the_same_check` checks one fixture Book passing, failing (a missing chapter), and incomplete (unsupported navigation) through `checking::check`, the actual `anchorbook check` process, and the actual `anchorbook md` process. The CLI and preprocessor exit with the outcome's status, and the CLI's report and, for failed and incomplete checks, the preprocessor's stderr equal the library report's formatting; a passing check lets the preprocessor return Book JSON with no stderr. Making the preprocessor exit 1 for every failed check fails the test on the incomplete case; the source was restored.

### Completion record

Complete on 2026-11-10. This Book names its `book.toml`, its agents retrieve the contract with `anchorbook contract`, an installed engine at the revision under test checks and renders it with the same results as the in-repository build, the library, CLI, and preprocessor agree, and the supported mdBook versions are declared and enforced. `fresh-session` coverage is from one Claude Code session; a Codex session was not run.

## Milestone 4. Projects use consistent engine entry points and reproducible setup {#project-integration-4}

**Outcome:** This Book and independent consumer fixtures exercise the native entry points, version alignment, and contributor setup that Symposium needs, before its own Cargo and CI integration adopts them.

**Status:** Proposed

**Identifier:** `project-integration-4`

**Dependencies:** Milestone 1, complete; the engine and Symposium trial findings in the [agreed sequence](index.md#first-major-milestone-symposium-adopts-anchor-book). Settle the version and authoring-reference interfaces before implementation. Existing milestone identifiers 2 and 3 remain retired.

**Design basis:** [Running the engine](../design/project-integration.md#running-the-engine), [agent bootstrap](../design/project-integration.md#agent-bootstrap), [A-project-native-commands](../design/project-integration.md#A-project-native-commands), and [A-project-one-engine-version](../design/project-integration.md#A-project-one-engine-version).

**Tasks**

- [ ] Expose the library CLI entry point and exercise the documented workspace-binary and installed-binary paths. Implement the exact engine-version declaration and mismatch checks; settle configuration and record-syntax version declarations and migration before changing their schema.
- [ ] Exercise library, CLI, and preprocessor equivalence through independent consumer fixtures, including failing and incomplete Books and mismatched versions. Keep this repository's Book acceptance separate from its ordinary tests so a broken Book cannot prevent building its engine.
- [ ] Provide reproducible bootstrap and authoring references for the engine version the project uses, including the record and roadmap forms, supported tools, and normal commands. Decide how the engine distributes those references and exercise that delivery here before Symposium relies on it.
- [ ] Document a full pinned Git revision for dependency and installation before registry publication, with upgrade steps keeping configuration, locked dependency, and invoked binary aligned. Test the setup outside the development checkout without Verus.

### Completion criteria

#### Native entry points {#project-integration-4-native-entry-points}

Consumer fixtures using a package dependency obtain equivalent findings through
the library, workspace binary, and installed binary. Their ordinary test and
mdBook commands reject the independently specified broken inputs. Exact version
declarations are enforced and unsupported schema or syntax versions produce
incomplete results before evaluation. The engine's own independent tests remain
usable with a broken Book.

- **Anchor coverage:** [A-project-native-commands](../design/project-integration.md#A-project-native-commands) · `consumer-fixture` — Full; [A-project-one-engine-version](../design/project-integration.md#A-project-one-engine-version) · `same-version-entry-points`, `exact-version-declaration`, `unsupported-versions` — Full.
- **Evidence:** Pending.

#### Contributor setup {#project-integration-4-contributor-setup}

A fresh consumer setup retrieves the correct contract and authoring references,
checks and renders its Book, and runs its documented tests using the pinned engine
without this development checkout or Verus. Record tools, commands, versions,
results, and any assistance. General registry release remains separate.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `fresh-session` — Partial: reproducible bootstrap and versioned references in a consumer fixture.
- **Evidence:** Pending.

### Completion record

Pending.
