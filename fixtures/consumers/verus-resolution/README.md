# Verus dependency-resolution fixture

This standalone consumer preserves the failing baseline for Formal Verification
milestone 4's `consumer-resolution` criterion. It depends on the engine and pins
its own `vstd` to `=0.0.0-2026-10-11-0230`. Keep that pin unchanged when testing
engine isolation.

From the repository root:

```sh
cargo metadata --format-version 1 \
  --manifest-path fixtures/consumers/verus-resolution/Cargo.toml
```

Add `--offline` when the required registry index entries are cached. This is an
expected failure before isolation, not a passing test of the current engine.
At engine revision `fab05d6`, Cargo 1.99.1 exits 101: the engine's exact
`vstd = "=0.0.0-2026-10-18-0147"` requirement conflicts with the consumer pin.
The resolver reports ``failed to select a version for `vstd` which could resolve
this conflict``. Cargo tries to unify these compatible prerelease versions;
their exact requirements cannot select one version. No native-library `links`
conflict is involved.

After isolation, resolution and an ordinary build must both succeed without
Verus installed. Repeat using an extracted packaged engine candidate by changing
only the `anchor-book` path in a disposable copy of this fixture. Inspect the
engine's dependency graph separately: the consumer still intentionally depends
on its own `vstd`.

The fixture exercises dependency resolution, not native proof execution or
correspondence between verified and packaged authority code. Those acceptance
obligations remain in milestone 4.
