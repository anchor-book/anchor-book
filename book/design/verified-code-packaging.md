# Shipping Verified Code



A crate whose production code is written inside `verus!` depends on `vstd` to
compile. When that crate is published, every project that depends on it inherits
the dependency and its exact version. This chapter defines how to publish such a
crate without imposing its verifier on dependents. The engine is one such crate:
its [authority validator](rust-engine.md#authority-proof) is verified, and projects
that verify their own code must be able to depend on the engine.

> **`A-package-verified-bodies`** (draft): **Verification and consumer builds use the same executable source**
> Property: The repository-only verification configuration gates specifications, proof blocks, and Verus imports, not executable code. Consumers compile the executable source whose contracts were verified. A package used as verification evidence contains that same executable source; a source change requires fresh correspondence and verification evidence. This correspondence does not remove trust in the compiler, verifier, or dependency contracts.
> Authority: `R-proofs-establish-criteria`; `D-isolate-engine-verification`
> - Criterion `feature-boundary` [test]: An executable source audit establishes that private verification configuration gates only permitted ghost material, leaving the same executable declarations and bodies for consumers. It rejects seeded conditional executable changes, including uses of implicit Verus configuration flags.
> - Criterion `packaged-source` [test]: The packaged executable source is byte-identical to the production source selected by the private proof manifest, and runtime acceptance uses the extracted crate. An independent source-identity check rejects altered or missing executable files; source identity alone does not establish a passing proof.

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

**The arrangement.** The published manifest declares no Verus dependency or
verification feature. Default and all-feature consumer builds use ordinary Rust
and never select the engine's verifier libraries. An unpublished package in an
isolated workspace under `verification/` owns `vstd`, its lockfile, and its build
configuration. It is excluded from the distributed crate.
The workspace uses the same [verification adapter](verification.md#shared-verification-adapters)
as consuming projects. Keeping this proof build private does not remove the
published engine's support for verifying a user's own anchor criteria.

The private package has a distinct name, `anchor-book-verification`, so standard
Cargo tooling can distinguish it from the public package. Its library target points directly
to the production `src/lib.rs`; Rust resolves the production modules and embedded
assets from those same files. It maintains no second implementation. Its ordinary
dependency declarations match the public manifest, with that correspondence checked
by tests. Only the private build script sets `anchorbook_verification`, enabling
Verus attributes, ghost functions, proof blocks, and imports in the shared source.
The public package has no switch that enables this configuration.

A configured proof target can use an unpublished private manifest only when its
library root resolves to the identical production file and module directory.
The selection must be unique; multiple matching targets are rejected. The proof
report names both the linked production package and the verification package.
An alternate source file, even an initially identical copy,
is rejected. The private manifest, lockfile, and build script participate in input
identity, so changing them invalidates an in-flight proof result. A manifest
redirect does not make a proof-only implementation evidence for production.

**The correspondence boundary.** Verification configuration may gate only ghost
material, never executable code or module selection. A conservative token audit rejects configuration-dependent executable declarations,
statements, and expressions, including implicit Verus flags. It accepts only the
explicitly supported ghost syntax; it is not a general Rust macro expander.
Package acceptance compares shipped production source with the source the private
manifest verifies and rejects altered source. The compiler, verifier, procedural
macros, and dependency contracts remain trusted; source identity is not a proof of
compiler correctness. A native proof and defect/correction trial accompany changes
to this boundary.

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
dependents, and the same isolation and correspondence rules apply; a private proof manifest can
keep their own verification dependencies out of their public package. Its proof
evidence is evaluated against the source it verifies, so the correspondence to the
published source is part of its verification boundary: verify the packaged
source, or establish executable-source identity programmatically. Review judges
whether the selected boundary expresses the intended guarantee.
