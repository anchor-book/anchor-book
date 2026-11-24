# Shipping Verified Code



A crate whose production code is written inside `verus!` depends on `vstd` to
compile. When that crate is published, every project that depends on it inherits
the dependency and its exact version. This chapter defines how to publish such a
crate without imposing its verifier on dependents. The engine is one such crate:
its [authority validator](rust-engine.md#authority-proof) is verified, and projects
that verify their own code must be able to depend on the engine.

> **`A-package-verified-bodies`** (draft): **Verification and consumer builds use the same executable source**
> Property: The optional verification feature gates specifications, proof blocks, and Verus imports, not executable code. Consumers compile the executable source whose contracts were verified. A package used as verification evidence contains that same executable source; a source change requires fresh correspondence and verification evidence. This correspondence does not remove trust in the compiler, verifier, or dependency contracts.
> Authority: `R-proofs-establish-criteria`; `D-isolate-engine-verification`
> - Criterion `feature-boundary` [test]: An executable correspondence check compares the verification and ordinary-build configurations after erasing permitted ghost material and rejects seeded feature-dependent executable code changes.
> - Criterion `packaged-source` [test]: Verification and runtime acceptance use the packaged implementation; an independent source-identity check rejects an altered executable file in the package rather than carrying forward the earlier proof result.

## Proof packaging for consumers

**The problem.** Verus releases about weekly, and each release requires its
matching `vstd`, pinned exactly. Cargo's
[compatible-version unification](https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility)
treats all `0.0.0-…` prereleases as one version slot, so two exact requirements on
different prereleases cannot both be met. A project pinning its own `vstd` cannot
depend on a crate that pins a different one. No native-library `links` conflict is
involved.

**What does not solve it.** Moving `vstd` into another crate that ordinary builds
still require keeps the constraint. Making `vstd` optional alone gives no runtime
build when the maintained code sits inside `verus!`, because the code needs that
macro to compile. A separately maintained proof-only copy cannot establish that the
verified code is the code that runs.

**The arrangement.** The verified production functions are ordinary Rust whose
contracts and loop invariants are Verus attributes, applied only with an optional
`verus` Cargo feature:

```rust
#[cfg_attr(feature = "verus", verus_spec(yes =>
    ensures yes == kind_matches(kind, target),
))]
fn matches_target_kind(kind: Kind, target: TargetKind) -> bool { /* body */ }
```

Specification functions, which never run, sit in a
`#[cfg(feature = "verus")] verus! { ... }` block, and proof blocks are
`#[cfg(feature = "verus")] proof! { ... }`. The feature enables `vstd`;
`cargo verus verify --features verus` verifies, and every other build, including
every dependent's, compiles the same bodies with no Verus crate. This is the use
the Verus guide gives for its attribute syntax: executable functions with
`#[verus_spec]`, specification and proof functions in `verus!`, and Verus
dependencies controlled by feature flags.

**The rule that keeps the proof about the running code.** The feature may gate only
specifications, proof blocks, and Verus imports, never executable code. Code that
differed under the feature would verify one body and run another; the Verus guide
warns that gating code this way is unsound. The
[executable correspondence criteria](#A-package-verified-bodies) make this boundary
an explicit verification obligation. Design review assesses the intended boundary;
it cannot supply a passing correspondence result.

**Limits.** The Verus guide lists the attribute syntax among contributed extensions
that are not complete, and it changes between weekly releases. Its open problems
concern ghost and tracked parameters through `with` and closures, which the
engine's proof does not use. Reverify on every Verus upgrade. Verus gives no
guidance on publishing verified crates, so this design owns that boundary.

**Acceptance.** The arrangement is accepted only when ordinary dependent builds
work without Verus, the baseline consumer resolves and builds with its own `vstd`,
and the proof and integration tests pass for the same packaged implementation.
The executable correspondence check and production-boundary review are repeated
after any source or packaging change.
This covers the verification-dependency boundary only; other exact dependencies,
public API compatibility, and publication keep their own release obligations.

## Projects shipping verified code

A project that publishes verified code meets the same problem for its own
dependents, and the same arrangement, rule, and acceptance apply. Its proof
evidence is evaluated against the source it verifies, so the correspondence to the
published source is part of its verification boundary: verify the packaged
source, or establish executable-source identity programmatically. Review judges
whether the selected boundary expresses the intended guarantee.
