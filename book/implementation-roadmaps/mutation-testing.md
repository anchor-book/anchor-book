# Mutation Testing

**Roadmap identifier:** `mutation-testing`

## Goal

Before merging implementation into `main`, agents can run mutation tests,
investigate findings, improve tests or implementation, and repeat verification.
GitHub independently runs the checks on the pull request; required checks protect
the merge. Tool setup alone does not establish that the engine's tests are adequate.

Merge protection is completed together with the
[Formal Verification roadmap](formal-verification.md#formal-verification-2); until it
is enforced, review existing CI results before merging. This roadmap applies the
testing and correction policy (`anchorbook contract`, “Test before opening a pull request”)
to the existing Rust crate. Project choices and commands are documented in
`CONTRIBUTING.md` at the repository root and the [Rust Engine design](../design/rust-engine.md#mutation-testing).
Mutation outcomes assess tests, not design acceptance or anchor verification.

## Milestone 1. Agents can run mutation tests and act on findings locally {#mutation-testing-1}

**Outcome:** Repeatable local mutation testing and documented correction steps, with a pull-request workflow ready for remote validation.

**Status:** Complete — local setup and correction demonstration

**Identifier:** `mutation-testing-1`

**Design basis:** Working-contract testing policy and Rust Engine testing guidance.

**Dependencies:** The independent test suite from Record Rendering milestones 1 and 2.

**Tasks**

- [x] Pin cargo-mutants, configure temporary source copies and timeouts, and document full and scoped commands.
- [x] Add pull-request and manual CI execution, preserving failed outcomes and evidence.
- [x] Demonstrate a scoped local campaign, investigate survivors, strengthen the affected tests, and rerun.
- [x] Document agent iteration, equivalent-mutation review, inconclusive results, and the separate merge-rule requirement.

### Completion criteria

#### Local run {#mutation-testing-1-local-run}

Ordinary tests pass; the pinned tool reads the shared configuration and tests a stated nonempty scope in temporary copies, preserving source files.

#### Correction loop {#mutation-testing-1-correction-loop}

Record original survivors, their meaning, the corrections, and a repeat run. Tests check specified behavior; exclusions are not used to hide gaps.

#### Workflow definition {#mutation-testing-1-workflow}

PR and manual workflows use the same configuration after ordinary tests, fail on missed mutations or timeouts, and retain evidence. Source review establishes configuration only; milestone 2 owns remote proof and enforcement.

### Anchor coverage

None — testing infrastructure and agent workflow. The demonstration exercises existing behavior but does not verify an entire anchor.

### Evidence {#mutation-testing-1-evidence}

*Local demonstration and limits*

**Revision and scope:** Working tree based on `d66184b`, tested 2026-10-21 with cargo-mutants 27.1.0 and Rust 1.90.0 on macOS. Command: `target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --file src/rendering/links.rs`, using the checkout-local tool installation documented in CONTRIBUTING. This selects 20 of the 226 mutations listed for the crate, not a full campaign.

**Local run and correction loop:** Ordinary tests and the unmutated baseline passed. The first mutation run took 65 seconds: 18 caught, 2 missed, no timeouts or unviable mutations. One survivor weakened route validation; existing invalid URLs also failed another condition, masking the missing assertion. Added isolated scheme and control-character cases. The other changed relative-path arithmetic; a substring assertion accepted an extra parent-directory segment. Changed the assertions to require the complete link destination.

**Result:** After the test changes, ordinary tests passed and the same 20 mutations were all caught in 62 seconds, with no missed, timed-out, or unviable mutations. Reviewed logs confirm the two former survivors now fail the intended protocol and navigation tests. No exclusions were added; engine source and dependencies are unchanged.

**Supporting records:** `tests/rendering.rs`, shared mutation configuration, and documented reproduction commands. Raw local logs remain ignored. Formatting, Clippy, current-Book acceptance, and Book build/rendered-link checks pass; rendering tests and Clippy also pass on Rust 1.85.0. Workflow YAML parses and was reviewed for triggers, failure propagation, and artifact retention.

**Limitations and integration:** Verified locally before the setup commit on main. This demonstration covered only link-rendering mutations; milestone 2 records the subsequent full campaign. No remote mutation workflow result or branch-rule enforcement is claimed by this demonstration.

### Completion record

Complete locally on 2026-10-21. Scoped findings led to stronger tests and a passing repeat run. Full campaign and merge enforcement remain in milestone 2.

## Milestone 2. Mutation testing protects implementation merges {#mutation-testing-2}

**Outcome:** The complete engine campaign has reviewed results, and GitHub requires passing tests before merging into main.

**Status:** In progress — PR execution verified; merge protection and remote correction demonstration pending

**Identifier:** `mutation-testing-2`

**Design basis:** Working-contract testing policy and Rust Engine testing guidance.

**Dependencies:** Mutation Testing milestone 1; publication of the workflow and access to repository merge rules.

**Tasks**

- [x] Run the initial unfiltered engine campaign and record its scope, results, and limits.
- [x] Manually run the workflow on the published prototype checkpoint; verify failure reporting and retained evidence.
- [x] Review survivors and unviable mutations. Correct test gaps or defects, justify any equivalent mutations, and repeat affected checks and the full campaign.
- [x] Exercise the workflow on a PR and record its revision, results, artifacts, and runtime. Tune or split the 45-minute budget if needed without dropping coverage.
- [ ] Mutate only changed lines on PRs, run the full campaign on every merge to `main` with four workers, and record a GitHub run of each.
- [ ] Configure the main branch rule to require Mutation tests and ordinary engine checks, with results current for the revision being merged. Verify that a failed check prevents merging under the intended contributor policy.
- [ ] Demonstrate an agent reading a remote failure, correcting it locally, and obtaining passing checks on an updated PR through authorized pushes.

### Completion criteria

#### Engine campaign {#mutation-testing-2-engine-campaign}

Full results have no unexplained survivors or inconclusive required runs. Any equivalent mutation or exclusion has a specific reviewed rationale. Record scope, counts, versions, revision, and limits.

#### Remote correction loop {#mutation-testing-2-remote-loop}

Actual PR results demonstrate local correction and remote revalidation for the updated revision, with accessible evidence and a workable runtime budget.

#### Merge enforcement {#mutation-testing-2-merge-rule}

Record the applied rule, required check names, stale-result behavior, and observed prevention of merging with failed checks. A workflow YAML file or local success alone is insufficient.

### Anchor coverage

None — test readiness and repository integration; no automatic claim of anchor verification.

### Evidence {#mutation-testing-2-evidence}

*Initial campaign and remote assessment*

**Baseline:** The full campaign at `46f47c8` attempted 226 mutations: 154 caught, 28 missed, 44 unviable, no timeouts. Local Rust 1.90.0 and the [GitHub run](https://github.com/anchor-book/anchor-book/actions/runs/39662175193) on Rust 1.99.1 produced the same survivors with cargo-mutants 27.1.0.

**Local correction evidence:** Tests in `tests/errors.rs` and `tests/records.rs` now check error chains and presentation, diagnostic codes/messages/locations, duplicate-target locations, nonempty inventories, and Markdown boundaries and extensions. The HTML heading assertion is in `7ab9616`. No production engine behavior changed. The full unfiltered rerun on the working tree based on `1fa9b3e` completed on 2026-10-21: 230 attempted, 183 caught, 44 unviable, three equivalent survivors, no timeouts. All 25 non-equivalent survivors from the baseline now fail relevant tests; their failure logs were reviewed. Command: `target/mutation-tools/bin/cargo-mutants mutants --jobs 2 --build-timeout 120 --output target/mutation-findings`, cargo-mutants 27.1.0, Rust 1.90.0, approximately 16 minutes. Exit 2 reflects the three equivalent cases. The subsequent exclusion listing selects 227 mutations; that configured campaign was not rerun. Ordinary tests and Clippy pass on Rust 1.85 and 1.90; formatting, absent/invalid-Book independence, and current/rendered-Book acceptance pass.

**Equivalent mutations:** The three OR-to-XOR changes at `src/records/markdown.rs:24–26` produce the same option mask: the locked parser's four flags occupy distinct bits. Exact exclusions and their review conditions are recorded in `.cargo/mutants.toml`. Listing checks confirm exactly three exclusions and retain all flag-removal mutations. The previous unviable cases were reviewed: generated replacements call unavailable constructors or require missing trait implementations; they are not counted as detected faults.

**Integration and limits:** The [PR #2 mutation run](https://github.com/anchor-book/anchor-book/actions/runs/39735753188) for head `687a4bf` passed with 228 attempted, 184 caught, 44 unviable, no survivors or timeouts, using cargo-mutants 27.1.0 and the reviewed exclusions. That work merged as `03fb981`. The tool does not discover bodies inside `verus!`; those functions have separate proof evidence. Merge protection and a remote failure/correction demonstration remain open. Passing local and GitHub authority proofs now satisfy the prerequisite for that follow-up; passing mutation runs alone do not demonstrate correction or enforcement. No ruleset rules applied to main when checked on 2026-10-24; merge enforcement has not been demonstrated.

**Runtime and scope:** The full campaign grew to 667 mutations and took 35 minutes on two workers for [PR #27](https://github.com/anchor-book/anchor-book/actions/runs/42757352983) (536 caught, 131 unviable, none missed), near the 45-minute budget. PRs now mutate only their changed lines with `--in-diff`, and merges to `main` run the full campaign with four workers and a 60-minute budget; changes to mutation settings, manifests, or the toolchain still run the full campaign on the PR. Locally, PR #28's diff selects 18 mutations, tested in 16 seconds (8 caught, 10 unviable). A changed-line run cannot detect a test gap that a change exposes elsewhere; the full campaign on `main` finds it after merge. GitHub runs of both scopes are pending.

### Completion record

Pending.

Keep the engine's independent tests usable with an invalid Book. Mutation testing
does not invoke mdBook, structural checking, or the ignored current-Book test.
The roadmap retains unresolved findings until addressed; passing a scoped run
does not complete milestone 2 or authorize merging.
