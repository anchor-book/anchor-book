# AnchorBook

Anchor Book is a method and toolkit for getting more out of AI-assisted Rust development. Its Book defines the intended system; the engine checks its structure, renders design records, and reports executable evidence for anchor criteria.

Read the [Anchor Book](https://anchor-book.github.io/anchor-book/) and [contributor setup](CONTRIBUTING.md).

The `anchor-book` package contains the `anchor_book` Rust library and the `anchorbook` command. It requires Rust 1.88 or newer. Ordinary installation and use, including all published features, do not depend on Verus or `vstd`. The engine's own verifier is confined to an unpublished workspace in this repository. Rendering uses mdBook 0.5.4; verification needs the project's configured test or proof tools.

Install the `0.1.0` release from crates.io:

```sh
cargo install anchor-book --version "=0.1.0" --locked
anchorbook --version
anchorbook contract
```

For library-based project checks, add the matching version to Cargo.toml:

```toml
[dev-dependencies]
anchor-book = "=0.1.0"
```

Upgrade the CLI and library together; automatic version alignment and project creation are not implemented. Version `0.1.0` is a prototype release. Native proof-target success alone does not satisfy proof criteria; the remaining proof guards and other limitations are listed in the [changelog](CHANGELOG.md).

For development from a selected checkout, install with `cargo install --path . --locked`.
