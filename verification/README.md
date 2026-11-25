# Private proof workspace

The unpublished `anchor-book-verification` package verifies the production library at `../src/lib.rs`.
It holds vstd and its own lockfile; Cargo excludes this directory from the public
crate. Its build script alone enables `anchorbook_verification`.

From the repository root, use the configured native command:

```sh
cargo run --quiet --locked --bin anchorbook -- proof --config anchorbook.toml --anchor A-records-valid-authority
```

Install the pinned tools described in CONTRIBUTING first. Keep ordinary dependency
declarations aligned with the public manifest; package acceptance checks that
boundary. Do not copy production source into this workspace. The proof runner
requires an unambiguous target with the same library file and module directory,
reports both package identities, and snapshots its manifest,
lockfile and build script as proof inputs. Reverify after changing any of them.
