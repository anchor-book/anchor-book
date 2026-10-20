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

The binary currently supports only `--help` and `--version`. Rendering,
configuration, structural checking, and contract generation remain future work.
The package is not publishable yet; release readiness is a separate milestone.
