# Contributing

## Working on the Book

Install the versions used by the Book workflow:

```sh
cargo install mdbook --version 0.5.2 --locked
cargo install mdbook-mermaid --version 0.17.0 --locked
```

From the repository root, prepare diagram assets and build or preview:

```sh
mdbook-mermaid install .
mdbook build
mdbook serve --hostname 127.0.0.1 --port 3000
```

Edit sources under `book/`. Rendered output is in `target/anchorbook/`.
The Mermaid installer supplies `mermaid.min.js` and `mermaid-init.js`; these
reproducible assets are ignored by Git and generated in CI. Rerun the installer
when upgrading the pinned diagram preprocessor.

## Working on the engine

The crate uses Rust 2024 with a minimum supported Rust version of **1.85**.
Keep `Cargo.lock` checked in. The engine workflow checks the minimum version and
stable Rust; local development also uses formatting and Clippy.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo test --locked --test current_book -- --ignored
scripts/check-engine-independence.sh
```

The ordinary test suite uses independent fixtures and does not need the Book.
The explicit `current_book` test reads this checkout's Book and checks its expected
records and relationships. It is ignored in the ordinary suite so a broken Book
cannot prevent engine development. The independence script tests a temporary
copy with the Book absent, then with an invalid definition, and confirms that
only the separate Book acceptance check fails. It leaves the checkout untouched.
Run the ordinary tests first to populate the dependency cache; the script is offline.

`anchor_book::read_records` accepts chapter text and returns a validated inventory
or located diagnostics. See [Rust Engine](book/design/rust-engine.md) for its
boundaries. `cargo doc --no-deps` builds the public API documentation.

The binary supports `anchorbook md`, `anchorbook md supports html`, help, and
version. The adapter targets **mdBook 0.5.2** and its HTML renderer. Configuration,
structural checking, and contract generation remain future work.

The Book's preprocessor entry runs the local binary through Cargo with locked
dependencies, after mdBook's include/link processing and before Mermaid. A first
build may need to download Cargo dependencies. The adapter itself does not fetch
anything or write source files. Source edits rebuild through `mdbook serve`;
after Rust or theme changes, rebuild or restart the preview if needed.

After `mdbook build`, check the rendered record inventory, fields, and links,
including the combined print view:

```sh
cargo test --locked --test rendered_book -- --ignored --nocapture
```

This separate acceptance test reads `book/` and `target/anchorbook/` at runtime;
it does not build the Book. For another build destination, set
`ANCHORBOOK_TEST_OUTPUT`. Both Book acceptance tests support `ANCHORBOOK_TEST_BOOK`
to select another source directory. Build from those same sources before checking
output. Missing output fails with a build reminder; ordinary `cargo test` skips
these Book-dependent tests. No Python installation is required.

The Book workflow runs rendered acceptance after `mdbook build`, before uploading
artifacts. A failure fails **Build mdBook** and prevents Pages publication. Pull
requests run the same acceptance check without deploying Pages.

The rendering check uses the engine's typed inventory and test-only HTML and URL
parsers. It checks relative local links without network access; external and
site-root URLs and visual accessibility still need separate review. Independent
record and renderer fixtures remain necessary because this acceptance test shares
the production record parser.

The generated record HTML is self-contained. The local theme supplies appearance
and a small print helper that opens decision details for printing; it does not
parse records. Schema examples stay literal. Field strings are escaped plain
text with paragraph breaks; inline Markdown is not interpreted.
The package is not publishable yet; release readiness is a separate milestone.

## Configuration loading

`anchor_book::configuration::load(&Path)` reads and resolves an explicit
`anchorbook.toml`. It is a library operation; there is no `anchorbook check`
command yet. See [Configuration](book/design/configuration.md#library-interface)
for the API, error and path behavior, and boundaries.

```sh
cargo test --locked --lib --test configuration
cargo test --locked --test current_book -- --ignored
```

Independent tests use disposable filesystem fixtures and subprocesses with different
working directories. They require neither this repository's configuration nor its
Book. The ignored current-Book suite separately reads the real configuration and
Book. The existing Engine workflow runs both suites on supported Rust toolchains.

## Local authority proof

The production validator in `src/records/authority.rs` has a local Verus proof.
Use [Verus release 0.2026.10.18.7d82994](https://github.com/verus-lang/verus/releases/tag/release/0.2026.10.18.7d82994),
its bundled Z3 4.16.0, and the exactly pinned `vstd` dependency in Cargo.toml.
Download the release archive for your platform, check its published SHA-256,
and add the extracted directory containing `verus` and `cargo-verus` to PATH.
The arm64 macOS archive used for local verification has SHA-256
`3fc56c58277b5f0999368f7b3a2b07ba4eb9ec2965454ce7a9f2769c0464d32b`.

Install the verifier's toolchain alongside the normal development toolchains:

```sh
rustup toolchain install 1.99.1 --profile minimal --component rustc-dev,llvm-tools
cargo verus verify --fwd-verus-args-to roots --locked --lib -- --no-cheating --output-json --time-expanded
```

Verification uses Rust 1.99.1; ordinary engine builds retain the Rust 1.85 minimum.
No Book build or wrapper script is required. `--no-cheating` rejects project
assumptions and trusted function bodies; the verifier, solver, standard-library
specifications, and Rust compiler remain trusted dependencies.

Review the report for zero errors and results for `validate_authorities`, `validate_reference`,
`lookup`, `has_required_authority`, `matches_identity`, `matches_target_kind`, and
`has_unknown_identity` in `records::authority`. Cargo can reuse verified artifacts;
to force a fresh engine run after an initial build, run `cargo clean -p anchor-book`
before verification. This retains dependency artifacts. Reverify after changes to
code, specifications, conversion, or dependencies, and run ordinary tests for the
unproved integration boundaries.

cargo-mutants 27.1.0 does not discover executable bodies inside `verus!` blocks.
Keep this coverage gap explicit: scoped mutation checks exercise the unannotated
integration code, while a deliberate code defect separately tests proof rejection.
Neither result establishes full mutation coverage of the validator.

### GitHub authority proof

`.github/workflows/proof.yml` defines **Formal verification / Authority proof**
for PRs targeting `main`, pushes to `main`, and manual dispatch. It uses Ubuntu
24.04, the pinned Linux Verus archive with a SHA-256 check, and the verifier's
Rust toolchain. Ordinary tests run on Rust 1.85.0 against the same checkout;
the separate Engine workflow also checks stable Rust. No Book build is required.

Verification uses a fresh target directory without a restored build cache.
The command must succeed, and `.github/verus-report.jq` requires exactly one
report from the pinned verifier, full-crate verification, zero errors, a nonzero
verified count, and successful executable results for all seven authority
functions. Missing, partial, malformed, or unsuccessful reports fail the job.
The function list must stay aligned with the agreed proof boundary when it changes.

The job has a 20-minute budget and retains `authority-proof-results` for 14 days,
including failures where evidence is available. The artifact identifies the actual
checkout revision (the merge commit for a PR), run URL, tools, commands, exit codes,
assumptions, and selected proof results. Raw proof-note payloads are omitted;
compiler and verifier diagnostics remain in job logs. Cancelled or incomplete
runs are not passing evidence. Review the anchor interpretation and unproved
boundaries separately; a green job does not establish those review obligations.

The workflow is merged and passes on PRs and main; the
[Formal Verification roadmap](book/implementation-roadmaps/formal-verification.md#formal-verification-2)
records tested revisions and results. The deliberate-failure/correction GitHub
run and merge enforcement remain to be demonstrated.

## Mutation testing

Use [cargo-mutants](https://mutants.rs/) **27.1.0** to check whether the independent
tests detect changes to engine behavior. Install it with the development stable
toolchain; this developer tool does not change the engine's Rust 1.85 minimum.

```sh
cargo +stable install cargo-mutants --version 27.1.0 --locked
cargo +stable test --locked
cargo +stable mutants --jobs 2 --build-timeout 120
```

For an installation confined to this checkout, add `--root target/mutation-tools`
to the install command and prepend `target/mutation-tools/bin` to `PATH` when
running from the repository root.

`.cargo/mutants.toml` supplies locked Cargo dependencies, temporary source copies,
and a test timeout of five times the baseline duration, with a 30-second minimum.
The command limits each build to 120 seconds. It runs an unmodified baseline
before testing mutations. Do not use `--in-place` or skip the baseline. Ordinary
tests, including actual-process CLI tests, run for each mutation; the ignored
Book acceptance tests stay separate. Three exact OR-to-XOR mutations in the
Markdown option mask are excluded because the flags are disjoint and the resulting
mask is identical. The configuration records their locations and rationale;
review them when the flag expression or parser dependency changes. No engine
files are excluded, and mutations that remove flags remain in scope.

For a bounded local run, select the affected file or capability. For example:

```sh
cargo +stable mutants --list --file 'src/rendering/links.rs'
cargo +stable mutants --jobs 2 --build-timeout 120 --file 'src/rendering/links.rs'
```

A filtered run establishes results only for that scope. Include related behavior
when deciding scope; a changed-line-only run can miss effects elsewhere. The
unfiltered command covers the entire crate with the tool's supported mutations,
not every possible defect. Rust or test changes invalidate affected earlier results.

Choose local mutation runs for affected ordinary Rust behavior, including parsing,
conversion, and error propagation. For code inside `verus!`, run proofs and tests
and retain the mutation-coverage gap. Specification changes also need review against
the anchor; a passing weaker claim is not equivalent assurance. Book or workflow-only
changes need their relevant checks, not a new local mutation campaign on unchanged
engine logic. The existing GitHub mutation workflow still runs on every PR; selective
remote execution is a separate workflow change.

Results are in ignored `target/mutants.out/`; the next run rotates the previous
results to `target/mutants.out.old/`. Preserve needed evidence before rerunning.
Review `outcomes.json`, `missed.txt`, `timeout.txt`, and the associated diffs/logs:

- **Caught:** a test failed; inspect the failure to confirm it detects the change.
- **Missed:** tests passed despite a mutation. Add a behavior-based test when this
  exposes a gap. If equivalent, document why it cannot change specified behavior.
- **Unviable:** the mutation did not compile; this is not test detection.
- **Timeout or tool failure:** inconclusive; investigate and rerun. A cancelled
  run or zero selected mutations does not establish coverage.

Do not add blanket exclusions or weaken behavior merely to make the run green.
Any exclusion needs a specific rationale and review. Record the code revision,
toolchain/tool version, filters, counts, survivor analysis, and remaining gaps in
roadmap evidence. Raw local output can contain machine paths and identity metadata;
review it before sharing and do not commit it.

The **Mutation testing** GitHub workflow runs on pull requests targeting `main`
and can also be dispatched manually. It runs ordinary tests first, then the
unfiltered command, with a 45-minute job budget and evidence artifacts retained
for 14 days. Missed mutants and timeouts
fail the job; the result is not suppressed. Budget exhaustion leaves the campaign
incomplete. CI artifacts omit the tool's host/process metadata files.
The workflow must be pushed before it can run. To enforce it before merging,
configure the GitHub rule for `main` to require **Mutation tests**, **Rust 1.85.0**,
**Rust stable**, and **Build mdBook**, after confirming the new mutation check name
in a real PR. Require a pull request and results up to date with the target branch;
do not require **Publish Book**, which only runs after a main-branch push.
A workflow definition alone does not enforce branch protection. The roadmap
records the verified remote state; this workflow does not gate Pages publication.

Agents work locally: run the checks, inspect failures, make scoped corrections,
rerun ordinary tests and the affected mutations, then repeat the full campaign
before declaring readiness. Fix test gaps with assertions derived from the Book;
correct implementation when the investigation reveals a behavior defect. Do not
assume every survivor means production code is wrong. Preserve the original finding
and record how the correction addresses it. After an authorized push, inspect the
PR's GitHub results and repeat the loop for remote failures. New commits invalidate
prior CI success; required checks must pass for the revision being merged.
The [Mutation Testing roadmap](book/implementation-roadmaps/mutation-testing.md)
tracks setup, findings, and validation of that loop and merge enforcement.
