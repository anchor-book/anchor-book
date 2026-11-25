# AnchorBook

Anchor Book is a method and toolkit for getting more out of AI-assisted Rust development.
Its Book defines the intended system; the engine checks its structure, renders design
records, and reports executable evidence for anchor criteria.

Read the [Anchor Book](https://anchor-book.github.io/anchor-book/) and
[contributor setup](CONTRIBUTING.md).

The `anchor-book` package contains the `anchor_book` Rust library and the
`anchorbook` command. It requires Rust 1.88 or newer. Ordinary installation and
use do not require Verus. Rendering uses mdBook 0.5.4; verification needs the
project's configured test or proof tools.

The first crates.io release is being prepared. Until it is published, install
from a selected checkout:

```sh
cargo install --path . --locked
anchorbook --version
anchorbook contract
```

Once a release is published, install its exact version with
`cargo install anchor-book --version =VERSION --locked`. A library consumer uses
`anchor-book = "=VERSION"` in Cargo.toml. Upgrade both entry points together;
automatic version alignment and project creation are not implemented.
