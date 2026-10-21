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

After building, check the rendered record inventory, fields, and links (including
combined-print links) with Python 3.11 or later:

```sh
python3 scripts/check-rendered-book.py target/anchorbook
```

The generated record HTML is self-contained. The local theme supplies appearance
and a small print helper that opens decision details for printing; it does not
parse records. Schema examples stay literal. Field strings are escaped plain
text with paragraph breaks; inline Markdown is not interpreted.
The package is not publishable yet; release readiness is a separate milestone.

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
current-Book acceptance test stays separate. There are no project exclusions.

For a bounded local run, select the affected file or capability. For example:

```sh
cargo +stable mutants --list --file 'src/rendering/links.rs'
cargo +stable mutants --jobs 2 --build-timeout 120 --file 'src/rendering/links.rs'
```

A filtered run establishes results only for that scope. Include related behavior
when deciding scope; a changed-line-only run can miss effects elsewhere. The
unfiltered command covers the entire crate with the tool's supported mutations,
not every possible defect. Rust or test changes invalidate affected earlier results.

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
