# Working on the Book

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

The engine design is a draft. No `anchorbook` commands are implemented yet.
