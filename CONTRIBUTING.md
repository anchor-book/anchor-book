# Contributing

## Working on the Book

Install the versions used by the Book workflow:

```sh
cargo install mdbook --version 0.5.4 --locked
cargo install mdbook-mermaid --version 0.17.1 --locked
```

From the repository root, prepare diagram assets and build or preview:

```sh
mdbook-mermaid install .
mdbook build
mdbook serve --hostname 127.0.0.1 --port 3000
```

The Anchor Book preprocessor runs the structural checker on the configuration
named by `[preprocessor.anchorbook] config` before rendering any chapter. A failed
or incomplete check prints the `anchorbook check` report and stops `mdbook build`.
`mdbook serve` applies the same check on each rebuild and keeps serving the last
good output until the Book passes again. Record validation also runs during
preprocessing. Current-Book acceptance is a separate test; see below.

Edit sources under `book/`. Rendered output is in `target/anchorbook/`.
The Mermaid installer supplies `mermaid.min.js` and `mermaid-init.js`; these
reproducible assets are ignored by Git and generated in CI. Rerun the installer
when upgrading the pinned diagram preprocessor.

## Working on the engine

The crate uses Rust 2024 with a minimum supported Rust version of **1.88**.
Develop with current stable Rust, selected by `rust-toolchain.toml`; update it
with `rustup update stable`. The stable channel can advance independently of the
minimum version. Keep `Cargo.lock` checked in. CI tests Rust 1.88.0 and current
stable, with formatting on stable and Clippy on both. The pinned Verus toolchain
remains separate from this development policy.

```sh
rustup update stable
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
scripts/check-engine-independence.sh
cargo test --locked --test current_book -- --ignored
```

The ordinary test suite uses independent fixtures and does not need the Book.
The explicit `current_book` test reads this checkout's Book and checks properties
that hold for any valid Book: records read without diagnostics, every reference
resolves to its kind and matches the derived backlinks, chapter order changes
nothing, the configuration resolves, and the structural check passes. It does not
pin the Book's contents, so design changes need no test edits. It is ignored in the ordinary suite so a broken Book
cannot prevent engine development. The independence script tests a temporary
copy with the Book absent, an invalid definition, a missing chapter, and unsupported
navigation. Independent tests pass in each case; the appropriate acceptance check
rejects the seeded defects. It leaves the checkout untouched. The ordinary
`build_gate` test runs `anchorbook md` against fixture Books: a valid Book is
rendered, while structural failures (exit 1) and invalid or incomplete input
(exit 2) produce the check report, no rendered JSON, and no file changes.
Run the ordinary tests first to populate the dependency cache; the script is offline.

`anchor_book::read_records` accepts chapter text and returns a validated inventory
or located diagnostics. See [Rust Engine](book/design/rust-engine.md) for its
boundaries. `cargo doc --no-deps` builds the public API documentation.

The binary supports `anchorbook check --config PATH`, `anchorbook md`,
`anchorbook md supports html`, `anchorbook contract`,
`anchorbook verify --config PATH [--anchor ID]... [--method test|proof|review]...`,
`anchorbook proof --config PATH [--anchor ID]...`,
help, and version. The
adapter targets **mdBook 0.5.4** and its HTML renderer. `anchorbook contract`
prints the AI Working Contract, whose source is the section files in `contract/`;
see `contract/README.md`. Materialization remains future work.

The Book's preprocessor entry runs the local binary through Cargo with locked
dependencies, after mdBook's include/link processing and before Mermaid. A first
build may need to download Cargo dependencies. The preprocessor reads the Book
for its structural check but does not fetch anything or write source files. Source edits rebuild through `mdbook serve`;
after Rust or theme changes, rebuild or restart the preview if needed.

The Book workflow's **Installed engine** job checks the path other projects use. It
installs the revision under test with
`cargo install --git <repository> --rev <commit> --locked anchor-book`, requires its
`anchorbook check` output, exit status, and `anchorbook contract` output to match
the in-repository build, and renders the Book with
`MDBOOK_PREPROCESSOR__ANCHORBOOK__COMMAND="anchorbook md"` before rendered-Book
acceptance. Reproduce it locally by installing from `file://` and this checkout's
`HEAD`.

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

The Book workflow owns push, pull-request, and manual build triggers. Its
**Engine checks** job calls the local reusable Engine workflow, which checks
Rust 1.88.0 and stable, independent tests, current-Book acceptance, and then the failure
experiments, which assume a valid Book. **Build mdBook** depends on that entire job succeeding, runs
`mdbook build` (which applies the structural check), and checks rendered output
before uploading artifacts.
**Publish Book** depends on Build mdBook and only deploys from main outside a
pull request. Validation and build jobs check out the triggering revision; no
separate workflow's latest success is used as evidence. Engine also retains
manual standalone execution.

Failed, skipped, or cancelled prerequisites cannot authorize rendering or
publication. Proof and mutation workflows remain separate; this change does not
add merge protection or make their results prerequisites for Pages. Structural
Checking milestone 3 records the GitHub runs that demonstrate this dependency
chain for valid and deliberately broken Books.

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
`anchorbook.toml`. It is also the first step of `anchorbook check`; loading alone
does not establish structural conformance. See [Configuration](book/design/configuration.md#library-interface)
for the API, error and path behavior, and boundaries.

```sh
cargo test --locked --lib --test configuration
cargo test --locked --test current_book -- --ignored
```

Independent tests use disposable filesystem fixtures and subprocesses with different
working directories. They require neither this repository's configuration nor its
Book. The ignored current-Book suite separately reads the real configuration and
Book. The existing Engine workflow runs both suites on supported Rust toolchains.

## Structural checking

```sh
cargo run --locked --bin anchorbook -- check --config anchorbook.toml
cargo test --locked --lib --test checking
cargo test --locked --test current_book -- --ignored
```

The CLI uses `anchor_book::checking::check` and reports structural findings without
changing files. Exit 0 means the declared scope passed, 1 means structural defects,
and 2 means invalid invocation or incomplete evaluation. Completed reports go to
stdout; incomplete reports and partial findings go together to stderr.
The current-Book acceptance suite includes a separate structural check. Independent
fixtures cover the actual process, broken Books, and correction/recheck cases.

Full mutation campaigns run on GitHub by default. Local development uses focused
ordinary tests; a local mutation run should have an explicitly chosen scope and
resource budget. Use one worker for an agreed local run. Do not launch parallel
local mutation campaigns or repeat a full campaign while remote validation is available.

## Evidence for criteria

```sh
cargo run --quiet --locked --bin anchorbook -- verify --config anchorbook.toml
```

`verify` reads the `[verification]` section of `anchorbook.toml`, the evidence
links (`/// anchorbook: <anchor-id>#<criterion-id>` on a test or verified
function) in its `sources`, and the Book, and reports every criterion's state by
chapter. It exits 0 when every claim it checked holds, 1 when a Complete
milestone's Full coverage names a criterion that is not satisfied, and 2 when it
cannot evaluate. It runs the linked tests in scope with cargo-nextest, pinned at
the version the engine supports:

```sh
cargo install cargo-nextest --version 0.9.151 --locked
```

For structured feedback while coding, use the same command with a scoped anchor:

```sh
cargo run --quiet --locked --bin anchorbook -- verify --config anchorbook.toml --anchor A-verify-progress-report --method test --format json
```

This emits one `anchorbook.verify` result, schema version 1, on standard output.
The library exposes the same projection as `adapters::verify::json_report` and
`json_error`. It includes criterion statements, definition and evidence-link
locations, current states, input identities, claims, counts, and limitations.
`complete` means evaluation completed, not that criteria passed or the system is
complete. JSON mode preserves the normal exit codes, including structured
incomplete results with exit 2; invalid CLI arguments still print usage to stderr.
Text is the default and can be selected with `--format text`.

Neither format saves result files or report history. Consume output directly;
redirect it only when the workflow needs a temporary handoff or selected evidence.
The version names the format, not a history of results. The current command does
not import reports or attach evidence to rendered pages. Those consumers must
validate saved results against current inputs when implemented. CI retention
requires an explicit purpose and expiration policy. Durable milestone evidence
stays summarized in the Book; review authored content and diagnostics before
publishing any selected full result.

`--anchor <id>` and `--method test` narrow the run for the build loop. Without
nextest, linked tests show as not evaluated; ordinary `cargo test` and CI do not
need it. Configured Verus library targets run through the native proof runner;
criterion assessments and guards remain outstanding after target success. Runner
output, including each package's log, is under `target/anchorbook-verify/`.
`tests/verification.rs` holds the fixture tests, which use a recording stand-in
for `cargo`; `native_nextest_follows_a_change_through_the_build_loop` is ignored
and runs the installed nextest.

## Local authority proof

The production validator in `src/records/authority.rs` has a local Verus proof.
Use [Verus release 0.2026.10.18.7d82994](https://github.com/verus-lang/verus/releases/tag/release/0.2026.10.18.7d82994),
its bundled Z3 4.16.0, and the exactly pinned `vstd` dependency in Cargo.toml.
`vstd` is optional: the `verus` feature enables it and attaches the proof's
specifications, so ordinary builds and projects that depend on the engine need no
Verus crate.
Download the release archive for your platform, check its published SHA-256,
and add the extracted directory containing `verus` and `cargo-verus` to PATH.
The arm64 macOS archive used for local verification has SHA-256
`3fc56c58277b5f0999368f7b3a2b07ba4eb9ec2965454ce7a9f2769c0464d32b`.

The x86-64 Linux release archive and Z3 4.16.0's Linux release require glibc
2.39 or later. On an older Linux system, build both from source at the pinned
tags; Z3 4.16 needs a C++20 compiler with `<format>`, such as GCC 13 or later:

```sh
git clone --depth 1 --branch z3-4.16.0 https://github.com/Z3Prover/z3.git
(cd z3 && CXX=g++-14 CC=gcc-14 python3 scripts/mk_make.py && cd build && make -j"$(nproc)")
git clone --depth 1 --branch release/0.2026.10.18.7d82994 https://github.com/verus-lang/verus.git
cp z3/build/z3 verus/source/z3
cd verus/source && source ../tools/activate && vargo build --release
```

Substitute your distribution's names for the GCC 14 compilers. Add
`verus/source/target-verus/release` to PATH; it contains `verus`, `cargo-verus`,
`vstd`, and Z3. A source build is the same pinned version as the release but not
the same binary, so record local results as built from source; the GitHub proof
job uses the release archive.

Install the verifier's toolchain alongside the normal development toolchains:

```sh
rustup toolchain install 1.99.1 --profile minimal --component rustc-dev,llvm-tools
cargo run --quiet --locked --bin anchorbook -- proof --config anchorbook.toml --anchor A-records-valid-authority
```

The production functions are ordinary Rust carrying Verus attributes such as
`#[cfg_attr(feature = "verus", verus_spec(...))]`; specification functions sit in a
`#[cfg(feature = "verus")] verus! { ... }` block. The feature may gate only
specifications and proofs, never executable code: code that differs under the
feature would make the proof describe something other than what runs.

Verification uses Rust 1.99.1; ordinary engine builds require Rust 1.88 or later.
No Book build or wrapper script is required. `--no-cheating` rejects project
assumptions and trusted function bodies; the verifier, solver, standard-library
specifications, and Rust compiler remain trusted dependencies.

The `[verification.verus]` configuration pins the verifier and Rust toolchain,
manifest, package, library target, and features. Set its optional `installation`
directory or put the pinned tools on PATH. The engine checks actual Verus, Rust,
and Z3 identities, then verifies each required target once in a fresh directory
under `target/anchorbook-verify/`. It selects expected functions from their
`/// anchorbook:` links and checks modes, unique results, full-target success,
and current inputs. The JSON report's `complete` field and exit 0 mean target
contracts verified; missing tools and incomplete evidence fail the gate. Raw
logs stay in that directory, with their paths in the report. Reverify after changes to
code, specifications, conversion, or dependencies, and run ordinary tests for the
unproved integration boundaries.

Because the validator's bodies are ordinary Rust, cargo-mutants reaches them.
Five loop-counter mutations are excluded in `.cargo/mutants.toml`: they make a
loop run forever, so tests can only time out, while each loop's `decreases` clause
makes verification reject them. Mutation results assess the tests; a deliberate
code defect separately tests that the proof rejects it.

After changing the validator or its specification, demonstrate that the proof
still rejects a defect: in a disposable copy, make a deliberate behavioral change,
such as making the accepted-status check always pass, confirm that verification
fails, then verify the unchanged source again. A timeout or tool failure is
inconclusive. Never weaken a specification or add an assumption to make a proof
pass. The planned [sensitivity guard](book/design/verification.md#guards)
automates this check.

### GitHub authority proof

`.github/workflows/proof.yml` defines **Formal verification / Authority proof**
for PRs targeting `main`, pushes to `main`, and manual dispatch. It uses Ubuntu
24.04, the pinned Linux Verus archive with a SHA-256 check, and the verifier's
Rust toolchain. Ordinary tests run on Rust 1.88.0 against the same checkout;
the separate Engine workflow also checks stable Rust. No Book build is required.

The job calls `anchorbook proof` through the checkout's engine. The shared runner
requires exactly one supported report from each fresh target run, full-crate
verification, zero errors, nonzero coverage, and each linked function exactly
once in the expected mode. Missing, partial, malformed, stale, or unsuccessful
results fail the job. There is no separate workflow parser or function list.
The job also runs the native fixture acceptance for linked and supporting
failures, trust violations, and correction. Reproduce it locally with
`ANCHORBOOK_TEST_VERUS=<tool-directory> cargo test --locked --test proof_runner native_proof_failure_support_failure_trust_and_correction -- --ignored --nocapture`.

The job has a 20-minute budget and retains `authority-proof-results` for 14 days,
including failures where evidence is available. The artifact identifies the actual
checkout revision (the merge commit for a PR), run URL, tools, commands, exit codes,
assumptions, and selected proof results. Raw proof-note payloads are omitted;
native diagnostic locations and local log paths are in the report. Raw logs are
not uploaded. Cancelled or incomplete
runs are not passing evidence. Review the anchor interpretation and unproved
boundaries separately; a green job does not establish those review obligations.

The [Verification Evidence roadmap](book/implementation-roadmaps/verification-evidence.md#verification-evidence-2)
records validation of the shared runner, the CI failure/correction demonstration,
and proof-state aggregation. The broader proof guards remain in milestone 6.
Earlier proof-workflow evidence remains in the Formal Verification roadmap.

## Mutation testing

Use [cargo-mutants](https://mutants.rs/) **27.1.0** to check whether the independent
tests detect changes to engine behavior. Install it with the development stable
toolchain; this developer tool does not change the engine's Rust 1.88 minimum.

```sh
cargo +stable install cargo-mutants --version 27.1.0 --locked
cargo +stable test --locked
# Optional local campaign, only with an agreed scope and resource budget:
cargo +stable mutants --jobs 1 --build-timeout 120 --file 'src/checking/rules.rs'
```

For an installation confined to this checkout, add `--root target/mutation-tools`
to the install command and prepend `target/mutation-tools/bin` to `PATH` when
running from the repository root.

`.cargo/mutants.toml` supplies locked Cargo dependencies, temporary source copies,
and a test timeout of five times the baseline duration, with a 30-second minimum.
The command limits each build to 120 seconds. It runs an unmodified baseline
before testing mutations. Do not use `--in-place` or skip the baseline. Ordinary
tests, including actual-process CLI tests, run for each mutation; the ignored
Book acceptance tests stay separate. One reviewed equivalent mutation in the
record reader's code-block line start is excluded; the configuration records why.
Five loop-counter mutations in `src/records/authority.rs` are also excluded, because
verification rejects them and tests can only time out; see the local authority
proof section. No engine files are excluded, and mutations that remove flags
remain in scope.

For a bounded local run, select the affected file or capability. For example:

```sh
cargo +stable mutants --list --file 'src/rendering/links.rs'
cargo +stable mutants --jobs 1 --build-timeout 120 --file 'src/rendering/links.rs'
```

A filtered run establishes results only for that scope. Include related behavior
when deciding scope; a changed-line-only run can miss effects elsewhere. The
unfiltered command covers the entire crate with the tool's supported mutations,
not every possible defect. Rust or test changes invalidate affected earlier results.

Choose local mutation runs for affected ordinary Rust behavior, including parsing,
conversion, and error propagation. For the verified validator, run the proof as
well as tests and mutations. Specification changes also need review against
the anchor; a passing weaker claim is not equivalent assurance. Book or workflow-only
changes need their relevant checks, not a new local mutation campaign on unchanged
engine logic. The GitHub mutation workflow reports on every PR. A PR that changes
`src/`, `contract/`, or `tests/` mutates only the lines it changes, with
`--in-diff`; one that changes Cargo manifests, the toolchain file, `.cargo/`, or the
workflow itself runs the full campaign, because those settings affect every
mutation. Other PRs pass with a notice. Every merge to `main` and every manual
dispatch runs the full campaign, which covers effects of a change outside its own
lines. A survivor found there is corrected in a follow-up change.

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

The **Mutation testing** GitHub workflow runs on pull requests targeting `main`, on
pushes to `main`, and on manual dispatch. It runs ordinary tests first, then the
changed-line or full command described above, split into four shards
(`--shard 0/4` to `3/4`) on separate runners that together test each mutation
once, each with four workers, a 60-minute budget, and its own evidence artifact
retained for 14 days. The **Mutation tests** job, the required check, passes only
when every shard passes. A newer push cancels a PR's
superseded run; full campaigns on `main` always complete. Missed mutants and timeouts
fail the job; the result is not suppressed. Budget exhaustion leaves the campaign
incomplete. CI artifacts omit the tool's host/process metadata files.
The **Protect main** repository ruleset enforces these checks. Changes reach
`main` only through a pull request whose required checks pass for a revision
current with `main`: **Engine checks / Rust 1.88.0**, **Engine checks / Rust
stable**, **Build mdBook**, **Installed engine**, **Authority proof**, and
**Mutation tests**. No approving review is required, force-pushes to and deletion
of `main` are blocked, and the ruleset has no bypass list. **Publish Book** is not
required; it runs only after a push to `main`. When adding or renaming a required
job, add its check name to the ruleset after it has run on a pull request, so
existing pull requests are not blocked waiting for a check they never run.

Agents make corrections locally and run relevant ordinary tests. GitHub runs
the full mutation campaign on the PR; inspect that result before declaring
readiness. Use local mutation reruns only for an explicitly agreed bounded scope. Fix test gaps with assertions derived from the Book;
correct implementation when the investigation reveals a behavior defect. Do not
assume every survivor means production code is wrong. Preserve the original finding
and record how the correction addresses it. After an authorized push, inspect the
PR's GitHub results and repeat the loop for remote failures. New commits invalidate
prior CI success; required checks must pass for the revision being merged.
The [Mutation Testing roadmap](book/implementation-roadmaps/mutation-testing.md)
tracks setup, findings, and validation of that loop and merge enforcement.
