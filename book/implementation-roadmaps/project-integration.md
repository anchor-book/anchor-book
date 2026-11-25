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
records the engine's proof CI encapsulation and proof-state coverage. The
[Symposium configuration-loading trial](symposium-integration.md#symposium-integration-4-evidence)
has assessed the verification commands; later coding trials exercise
implementation from the Book's definition. The next engine increment supplies
direct structured verification results using engine fixtures and this Book, without waiting
for new project tests. Symposium owns test semantics, and
Anchor Book trusts current runner outcomes. Milestone 4 below uses those trials
to settle the minimum setup for reproducible native-command and CI integration.
Registry publication is a separate outcome; a pinned Git dependency and installed
engine can support the first adoption.

[Milestone 5](#project-integration-5) adds the Port phase and configuration-selected
contract guidance while preserving common retrieval for diagnosis.

## Milestone 1. This Book uses the integration groundwork {#project-integration-1}

**Outcome:** This repository uses every mechanism a project will rely on: its configuration names its `book.toml`, its agents retrieve the contract with `anchorbook contract`, a pinned engine revision installed the documented way checks and renders this Book, and the declared mdBook versions are tested.

**Status:** Complete

**Identifier:** `project-integration-1`

**Dependencies:** The configuration loader, structural checker, and record renderer.

**Design basis:** [mdBook configuration location](../design/configuration.md#relationship-to-mdbook), [contract delivery](../design/system-overview.md#contract-delivery), [running the engine](../design/project-integration.md#running-the-engine), and [agent bootstrap](../design/project-integration.md#agent-bootstrap).

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

The fresh-session evidence above predates the contract's required readiness-report
template. It establishes the reported bootstrap behavior, not consistent report
format and criterion detail across agents. Repeating that assessment is part of
[milestone 4's contributor setup](#project-integration-4-contributor-setup).

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

**Status:** In progress

**Identifier:** `project-integration-4`

**Dependencies:** Milestone 1, complete; findings from the [development sequence](index.md#first-major-milestone-symposium-adopts-anchor-book). The existing pinned installed-engine workflow supports early Symposium coding trials. This milestone supplies the version alignment and native contributor setup needed for ordinary-command and upstream adoption, not a prerequisite for that baseline. Settle the version and authoring-reference interfaces before implementation. Existing milestone identifiers 2 and 3 remain retired.

**Design basis:** [Running the engine](../design/project-integration.md#running-the-engine), [agent bootstrap](../design/project-integration.md#agent-bootstrap), [project guidance and terminology](../design/project-guidance.md), [A-project-native-commands](../design/project-integration.md#A-project-native-commands), and [A-project-one-engine-version](../design/project-integration.md#A-project-one-engine-version).

**Tasks**

- [ ] Expose the library CLI entry point and exercise the documented workspace-binary and installed-binary paths. Implement the exact engine-version declaration and mismatch checks; settle configuration and record-syntax version declarations and migration before changing their schema.
- [ ] Exercise library, CLI, and preprocessor equivalence through independent consumer fixtures, including failing and incomplete Books and mismatched versions. Keep this repository's Book acceptance separate from its ordinary tests so a broken Book cannot prevent building its engine.
- [ ] Provide reproducible bootstrap and authoring references for the engine version the project uses, including the record and roadmap forms, standard terminology, supported tools, and normal commands. Supply a separate project terminology table; decide how the engine distributes portable versioned references and exercise that delivery here before consumers rely on it. Settle terminology adoption and any additional configuration migration before enforcing new requirements.
- [x] Define standard and project terminology ownership and manual maintenance; apply the two-table structure in this Book, preserving existing concepts and distinguishing acceptance, progress, and verification.
- [x] Deliver the project-guidance role, read-only starter command, contract instructions, and own-Book adoption. Check legacy compatibility, top-level placement, distinct paths including aliases, rendering gates, and preservation of authored instructions.
- [ ] Exercise the contract's required readiness-report template in fresh Codex and Claude sessions. Check all sections, every open milestone and completion criterion, evidence limitations, and assignment prerequisites; record versions, assistance, omissions, and differences.
- [ ] Trial the common design-before-roadmaps guidance in fresh Prototyping and Port sessions using disposable fixtures; assess design quality, ordering, missing-design readiness findings, and concise milestone updates.
- [ ] Document a full pinned Git revision for dependency and installation before registry publication, with upgrade steps keeping configuration, locked dependency, and invoked binary aligned. Test the setup outside the development checkout without Verus.
- [ ] Exercise local checking, rendering, and configured verification outside Git and with Git unavailable, preserving honest revision metadata and runner results.

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
including standard terminology for its engine version, checks and renders its
Book, and runs its documented tests using the pinned engine without this
development checkout or Verus.
The project guidance identifies its local context, sources, workflow, commands,
and publication rules; its terminology page distinguishes standard and project
terms. Exercise different project layouts and assert that delivered references
resolve without assumptions about a consumer's design paths. Record tools,
commands, versions, results, and any assistance. General registry release remains
separate; automated table updates belong to deferred materialization work.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `fresh-session` — Partial: reproducible bootstrap and versioned references in a consumer fixture.
- **Evidence:** Pending.

**Local preparation.** On `project-guidance-terminology`, based on merged cleanup
`02945a9990de3481144edc8cd47808cdf231c135`, this Book adopts the guidance section
responsibilities and separates standard and project terminology. The common
contract supplies the corresponding agent instructions. The assigned guidance integration adds an optional schema-1 mapping and a
packaged starter; the checker enforces mapped placement and separation. Terminology
remains manually authored. Portable reference delivery, consumer setup, fresh-session
trials, and automated updates remain outstanding. The existing
[ownership anchor](../design/system-overview.md#A-materialize-owned-content)
now explicitly includes local guidance and terminology, including two tables in
one file; its expanded `ownership-preservation` criterion remains unlinked.
No new anchors are needed for headings or individual terms.
The terminology page now has 27 shared terms and 13 project terms. Record status
and milestone status are qualified separately; anchor satisfaction distinguishes
computed evidence from design acceptance and omitted obligations. The planned
brief command is explicitly unavailable in this revision. Copied tables identify
their selected engine version and use matching references, while project terms
remain owned by the consumer. No record identities changed.

Local validation of this preparation passes formatting, all six contract tests,
structural checking, four current-Book checks, and rendered record/link acceptance.
Scoped verification reports the ownership anchor's three criteria unlinked;
these authoring checks do not verify materialization or consumer delivery.

#### Project guidance integration {#project-integration-4-project-guidance}

Projects retrieve a portable starter, adapt existing working instructions, and
map guidance separately from design, authority, and roadmaps. Library, CLI, and
mdBook reject invalid mapped placement and paths while preserving project content.
Existing schema-1 configurations without the mapping keep their prior behavior.

- **Anchor coverage:** [A-check-configuration-validity](../design/configuration.md#A-check-configuration-validity) · `schema-rejection`, `path-boundaries` — Partial: the additional guidance mapping; [A-check-navigation-conformance](../design/structural-checking.md#A-check-navigation-conformance) · `required-roles` — Partial: configured guidance; [A-check-observation-only](../design/structural-checking.md#A-check-observation-only) · `file-preservation` — Partial: guidance integration.
- **Evidence:** On 2026-11-25, the guidance changes on `project-guidance-terminology`, based on `02945a9990de3481144edc8cd47808cdf231c135`, passed 189 ordinary and documentation tests. The 41 configuration, structural, contract, and guidance integration tests also passed on Rust 1.88. Formatting and all-target Clippy passed on stable and Clippy passed on Rust 1.88. The five independent guidance tests exercise compatibility, invalid types and paths, role collisions including symlinks, missing and nested navigation, read-only starter retrieval, and the actual mdBook preprocessor's failure and repair behavior. Four current-Book checks and rendered acceptance passed, with 68 records and 3,038 local links. Scoped verification using cargo-nextest 0.9.151 satisfied `required-roles` through its four linked tests with no link findings; the navigation anchor remains Partial because its independent `mdbook-compatibility` criterion is still unlinked.

A disposable copy of Symposium's Book at `2c0e00e7` mapped `review.md` as project
guidance and added a separate roadmap index. Structural checking and rendering
passed. Assigning both guidance and roadmaps to `review.md` produced a located
configuration error and exit 2. This trial changed no Symposium checkout or design
and did not run its linked tests. Automatic content synchronization, standard
terminology delivery, fresh-agent trials, mutation CI, and publication remain
outside this local result.

The CI independence experiment at `22f2195` exposed an omitted compile-time
asset in its temporary engine copy: `templates/project-guidance.md`. The fixture
now includes `templates/` alongside source, contract, and tests. The complete
`scripts/check-engine-independence.sh` passed locally on stable and Rust 1.88:
ordinary engine tests run with an absent, invalid, or incomplete Book, while
Book acceptance rejects each seeded defect. This correction preserves the
independence boundary and the checks it exercises.

#### Design before roadmaps {#project-integration-4-design-before-roadmaps}

In disposable Prototyping and Port fixtures, fresh sessions establish a capability's
coherent target design before proposing its implementation milestones. The design
explains behavior, interfaces, interactions, constraints, and relevant failure
cases; linked milestones follow from it. Scoped research or design milestones may
establish missing knowledge with an agreed question, scope, and assessment.
Readiness reporting identifies an implementation milestone's missing design basis
even when its recorded status is Ready. Record inputs, engine and agent versions,
observed ordering, content assessment, assistance, and limitations. Give the
session a subsequent result or correction: it updates the existing milestone,
keeps evidence and unresolved gaps retrievable, and avoids assignment narration,
duplicated reports, and another milestone for the same outcome. Include an
unanchored key correctness guarantee and one outside the scope of an existing anchor: the
session surfaces their constraints, consequences, and missing coverage for engineer
judgment, preserves existing approvals, and proposes executable criteria without
inventing evidence or silently accepting a new constraint. An overview that only
links owning definitions needs no duplicate anchor. Include routine testable
behavior as a counterexample: the session leaves it in ordinary design and tests,
explains the consequence that warrants each selected anchor, and groups related
obligations without making an anchor per test or edge case.

- **Anchor coverage:** None: behavioral assessment of working guidance, beyond the engine's contract-delivery guarantees.
- **Evidence:** Fresh-session trials remain pending. The local assessment below
  applies the rule to this Book; packaging and link tests cannot establish that
  fresh agents follow the procedure.

**Self-application.** On 2026-11-24, the `book-self-reconciliation` branch based
on `7966c9fddb3e9b9a710444c365cd698550de9b5f` assessed all 18 design chapters and
the linked roadmap records. Selection follows the
[importance threshold](../design/anchor-definitions.md#coverage-of-design-constraints):
correctness consequences justify anchors; testability alone does not. This is
local Book reconciliation, not a fresh-agent conversation trial.

| Assessed design | Key guarantee and coverage disposition |
| --- | --- |
| Design index and System Overview | The overview links owning definitions. Independent builds and current build evidence already have anchors. Preserving user-authored content during materialization is owned by [A-materialize-owned-content](../design/system-overview.md#A-materialize-owned-content) and its draft authority; violating this boundary could destroy the Book's intended design. |
| Anchor Definitions, Authority Definitions, and Rust Engine | Existing record grammar, valid-authority, and shared-result anchors cover identifiable definitions, justified properties, and consistent callers. No duplicate anchor for each record kind or API. |
| Configuration, Structural Checking, and Checking Diagnostics | Existing selection, conformance, preservation, and truthful/actionable-result anchors own these boundaries. Routine diagnostic examples remain test cases under those properties. |
| Verification, Running Verification, and Proof Guards | Existing criterion-state, linkage, execution, completion-claim, and proof-guard anchors own evidence validity. Proof Guards now links its owning anchors directly. Missing native guards remain explicit work; no new anchor is needed for that chapter. |
| Shipping Verified Code | [A-package-verified-bodies](../design/verified-code-packaging.md#A-package-verified-bodies) makes source correspondence explicit: verifying a different executable body would invalidate the proof's application to shipped code. Prior consumer-build evidence does not establish this property. |
| Project Integration and Record Rendering | Existing version, dependency, bootstrap, and rendering anchors own reproducibility and faithful presentation. Git-independent local setup remains a design/testing obligation within this milestone; it does not need a separate correctness anchor. |
| Requests for Discussion, RFD Source Format, and RFD Worked Examples | Draft RFD anchors describe the proposed isolation, relationships, and distinct acceptance/implementation/evidence states; the decision to implement the capability remains open. Format details and literal example records do not add live goal anchors. |
| Using Anchor Book with an Agent | Existing contract-retrieval and phase-guidance criteria cover delivery. Conversational usefulness is assessed in milestone 6; neither prompts nor each workflow step needs an anchor. The optional walkthrough choice is recorded in Open questions. |

The [earlier assessment and validation](https://github.com/anchor-book/anchor-book/blob/7966c9fddb3e9b9a710444c365cd698550de9b5f/book/implementation-roadmaps/project-integration.md#project-integration-4-design-before-roadmaps)
added two anchors with five unlinked criteria. This pass preserves those anchors
and all other record identities, authority, and property statements. It adds no
anchors. Three independent obligations now have their own unlinked criteria:
`mdbook-compatibility`, `site-prefix-navigation`, and `print-navigation`. The
existing adapter-navigation criterion retains the tested nested-route behavior;
its site-prefix and print obligations moved into the two explicit criteria.
The guarantee is unchanged. Supported grammar tests do not establish every
mdBook-parseable input form.

[Research](../research.md) now owns source-format observations, packaging
alternatives, and the Symposium no-agent experiment; roadmap results keep their
revisions, limitations, and supporting references. [Open questions](../open-questions.md)
owns unresolved guide, packaging, materialization, and integration choices.
Milestones use the documented field order and current verification methods;
completed delivery records identify merged PRs without relabeling old trials as
fresh evidence. Core executable acceptance is independent of the unselected RFD
proposal; its incorporation and agent-workflow checks retain their owners in the
RFD roadmap. Diagram classes use the shared theme.

Two existing completion claims were unsupported by their own evidence:
[Formal Verification 4](formal-verification.md#formal-verification-4) requires
automated source-drift rejection, and [Structural Checking 4](structural-checker.md#structural-checking-4)
requires independent review and recorded agent versions. Both return to In
progress with the missing work explicit; delivered implementation and original
results remain valid within their recorded scope. Formal Verification 1 retains
its completed target-proof experiment, with Partial coverage of today's
`formal-authority` criterion until milestone 6 supplies native guards.

**Validation:** The structural check, mdBook 0.5.4 build with mdbook-mermaid
0.17.1, and all five current/rendered-Book acceptance checks pass; rendered
acceptance covers 68 records and 2,966 local links. Browser inspection covers ten
diagrams on seven pages in light, navy, narrow, and print layouts, with no script
errors; wide screen diagrams remain horizontally scrollable and print diagrams
fit the page width. Configured `verify --method test --format json` with nextest
0.9.151 reports 48 satisfied criteria, 45 unlinked, and one proof criterion not
evaluated across 37 anchors: 13 satisfied, ten partial, and 14 unlinked. There are
no link findings or failed evaluated completion claims. Navigation conformance
moves from satisfied to partial because its missing compatibility obligation is
now explicit. Record identities, property statements, authority references, and
milestone fragments are preserved. Proofs and fresh-agent trials were not run;
these checks do not establish complete semantic coverage or conversational usefulness.

### Completion record

Pending.

## Milestone 5. Porting projects receive the applicable working contract {#project-integration-5}

**Outcome:** A project selects `port` and its agents retrieve the common contract plus port guidance from its configured engine. Source research becomes explicit target design before implementation planning.

**Status:** Complete

**Identifier:** `project-integration-5`

**Dependencies:** Milestone 1's contract delivery and configuration loader; the reverse-engineering guidance in `64f0c3a`.

**Design basis:** [Project phase](../design/configuration.md#project-phase), [contract delivery](../design/system-overview.md#contract-delivery), and [agent bootstrap](../design/project-integration.md#agent-bootstrap).

**Tasks**

- [x] Add `port` to configuration and define exploration, implementation, and transition expectations in the contract.
- [x] Select additional port guidance through `contract --config PATH`, preserving common retrieval without project files.
- [x] Cover engineer questions, source and target identification, compatibility, coherent target design, verification, provenance, and completion boundaries.
- [x] Update this repository's startup and contributor guidance and document compatibility with earlier engines.
- [x] Verify phase selection, failure behavior, supported toolchains, and Book structure and rendering independently of unrelated local work.

### Completion criteria

#### Configured contract {#project-integration-5-configured-contract}

Port loads as an explicit phase and preserves the same structural checks as other phases. The actual CLI and library select the port section exactly once, preserve common retrieval and project files, and report configuration failures without partial output. Retrieval still works with missing Book sources when configuration otherwise loads.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `phase-guidance`, `contract-retrieval` — Full; [A-check-configuration-validity](../design/configuration.md#A-check-configuration-validity) · `schema-rejection` — Partial: phase values and configuration errors.
- **Evidence:** Passed locally on 2026-11-24 in an isolated checkout based on `64f0c3a` plus this change. `cargo test --offline --locked` passed 182 tests on stable 1.99.1 and Rust 1.88.0; eight tool-dependent acceptance tests were ignored by the ordinary suite. `cargo clippy --offline --all-targets --locked -- -D warnings` passed on both toolchains. The six contract tests exercise actual CLI selection across all four phases and two invocation directories, malformed and missing configuration, absent Book sources, unchanged files, common fallback, source packaging, and internal links. The explicit structural check, `mdbook build` with mdBook 0.5.4, and `cargo test --offline --locked --test current_book --test rendered_book -- --ignored` passed all five Book acceptance tests. CI and fresh-agent port trials have not run for this change.

#### Port workflow {#project-integration-5-port-workflow}

The delivered port section directs agents to recover missing engineer intent, reconstruct the source, incorporate agreed target design, and derive implementation work. It distinguishes experiments from implementation and phase transitions from milestone completion. Bootstrap retrieves configured guidance and documents the common fallback for diagnosis.

- **Anchor coverage:** None: manual assessment of the reusable working guidance, beyond contract selection and delivery.
- **Evidence:** Reviewed the delivered common and Port sections together. The Port section asks for missing intent and constraints while reusing prior answers, records source and target context, requires incorporation into owning design, preserves accepted-design requirements for implementation, and separates phase transitions from completion of individual milestones and source retirement. The repository bootstrap selects the configured contract and documents diagnosis through common retrieval. This assesses the written procedure; automated delivery tests cannot establish that agents consistently follow it in real ports.

### Completion record

Complete locally on 2026-11-24 for phase parsing, configured contract delivery,
working guidance, and the checks above. Changes are based on `64f0c3a`; the implementation subsequently merged in
[PR #50](https://github.com/anchor-book/anchor-book/pull/50) as
`b809f76f107e1ed0d7edc9e66f18f516fb94e34d`. Observation of the workflow in a real
port remains outside this milestone's evidence.

## Milestone 6. Engineers receive useful workflow guidance in conversation {#project-integration-6}

**Outcome:** An engineer can ask how to use Anchor Book or what to do next and
receive an understandable explanation, a grounded recommendation, and a usable
prompt through the project's agent.

**Status:** In progress

**Identifier:** `project-integration-6`

**Dependencies:** Existing contract delivery and configured phase selection.
Conversation trials need agents with access to the selected engine and fixture
Books; they do not require a new command or completion of milestone 4's broader
integration work.

**Design basis:** [User flow and guide](../design/user-guidance.md),
[R-guided-workflow](../authority/requirements.md#R-guided-workflow), and
[agent bootstrap](../design/project-integration.md#A-project-agent-bootstrap).

**Tasks**

- [x] Define orientation from current state, intent clarification, recommended paths, and iteration between design and building with an explicit design prerequisite.
- [x] Deliver concise guidance through the common contract in every phase, with project-specific recommendations grounded in the current Book.
- [x] Distinguish a conversational guidance request from an explicit request for the full readiness report.
- [x] Check compiled delivery, navigation, and rendering; record their scope separately from conversational usefulness.
- [ ] Trial orientation and next-step requests in this Book and independent fresh sessions; record misleading advice, missing choices, unnecessary approvals, and revisions to the guidance.

### Completion criteria

#### Guide delivery {#project-integration-6-delivery}

The common and configured contracts deliver the conversational guidance and
prompt examples exactly once, with valid internal links and unchanged retrieval
semantics. The Book's introduction and design navigation lead to the fuller guide.

- **Anchor coverage:** [A-project-agent-bootstrap](../design/project-integration.md#A-project-agent-bootstrap) · `contract-retrieval`, `phase-guidance` — Partial: inclusion of the user guidance in the existing common and phase-selected contract delivery.
- **Evidence:** Passed locally on 2026-11-24 on `conversational-user-guide`, based
  on `4a24dca491b731e69a779fd69328068ef9ff6a21` plus this change. All six contract
  tests pass, covering source packaging, actual CLI retrieval, every configured
  phase, invalid configuration, preservation, and internal links. Formatting,
  structural checking, mdBook build, and all five current/rendered-Book checks
  pass; rendered acceptance covers 68 records and 2,842 local links, using a build directory separate from the live
  preview. Browser
  inspection confirms both guide diagrams render without script errors and use
  the intended colors in light and navy themes. The diagrams retain their flow
  and labels. These checks do not establish conversational usefulness.

#### Contextual guidance {#project-integration-6-conversation}

In observed sessions, an engineer can understand the relevant next step, why it
fits, and what decision is needed without knowing the Book's syntax. Recommendations
cite the current project context, preserve existing assignments and approvals,
and do not present missing evidence or planned tools as available capability.

- **Anchor coverage:** None — conversational usefulness and design judgment are assessed separately from executable contract-delivery guarantees.
- **Evidence:** Pending. The following cases define the trial scope; packaging and structural tests alone cannot establish this criterion.

| Situation | Expected conversation |
| --- | --- |
| General “How does this work?” with no Book | Explain the loop and offer supported setup help without inventing project state or requiring a readiness inventory. |
| New or existing project asking for a first step | Summarize current state, ask the intended outcome if unknown, and offer relevant paths with a recommendation. Reuse an already stated goal. Distinguish establishing intended design from observing implementation. |
| Port with unresolved compatibility choices | Reuse known source/target facts, surface consequential choices, and recommend target design before implementation milestones. |
| Implementation roadmap lacking design or key anchors and authority | Identify the missing basis and a concrete design prompt before implementation planning; preserve existing work and evidence. |
| Accepted design and an assigned milestone in progress | Give brief guidance and continue the assignment when requested; allow design iteration when needed without restarting the workflow or requesting the same approval. |
| Passing tests with unlinked criteria | Explain what passed and what remains uncovered, with a useful verification next step. |
| Explicit request to read the Book or give readiness | Return the full readiness template, preserving all milestone and criterion details. |

Record the engine revision, agent/version, fixture state, prompt and observable
response, any assistance, and remaining limitations. Trials assess bounded behavior;
they do not prove that every agent will follow the guide.

### Completion record

Guide design and contract delivery merged in
[PR #52](https://github.com/anchor-book/anchor-book/pull/52) as
`7966c9fddb3e9b9a710444c365cd698550de9b5f`. The checks above retain their original
tested revision. Conversational trials remain outstanding; merge and downstream
pinning do not establish conversational usefulness.
