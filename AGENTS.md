# Start here

This repository develops the Anchor Book engine and its own Book.

Before project work, run this from the repository root and read its output in full:

```sh
cargo run --quiet --locked --bin anchorbook -- contract
```

It prints the AI Working Contract built from this checkout. Use this command, not
an installed `anchorbook`, so you read the contract for the revision you are
working on. Then read [Working on Anchor Book](book/working-on-anchor-book.md) for
project guidance. The Book index is [book/SUMMARY.md](book/SUMMARY.md); read every
chapter it lists before taking up the engineer's task. Paths here are relative
to this repository root, regardless of your current working directory.

If the contract cannot be retrieved, report the problem before proceeding with
work that depends on it. Retrieve it again in a new session, after losing its
context, or when it changes.
