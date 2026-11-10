# Shipping Verified Code

**Status: draft for review.** Not implemented; the engine's own case is planned in
[Formal Verification milestone 4](../implementation-roadmaps/formal-verification.md#formal-verification-4).

A crate whose production code is written inside `verus!` depends on `vstd` to
compile. When that crate is published, every project that depends on it inherits
the dependency and its exact version. This chapter defines how to publish such a
crate without imposing its verifier on dependents. The engine is the first case:
its [authority validator](rust-engine.md#authority-proof) is verified, and projects
that verify their own code must be able to depend on the engine.

## Proof packaging for consumers

**The problem.** Verus releases about weekly, and each release requires its
matching `vstd`, pinned exactly. Cargo's
[compatible-version unification](https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility)
treats all `0.0.0-…` prereleases as one version slot, so two exact requirements on
different prereleases cannot both be met. A project pinning its own `vstd` cannot
depend on a crate that pins a different one. No native-library `links` conflict is
involved.

The engine has this problem today. `vstd = "=0.0.0-2026-10-18-0147"` is a normal
dependency, so it fails the
[dependency-isolation property](project-integration.md#project-no-imposed-dependencies)
required by the [isolation decision](../authority/decisions.md#decision-isolate-engine-verification).
On 2026-11-08, Cargo 1.99.1 could not resolve a consumer depending on the engine
and pinning `vstd = "=0.0.0-2026-10-11-0230"`:
``failed to select a version for `vstd` which could resolve this conflict``.
The fixture `fixtures/consumers/verus-resolution` reproduces it, and the
[consumer-resolution criterion](../implementation-roadmaps/formal-verification.md#formal-verification-4-consumer-resolution)
keeps it as the baseline.

**What does not solve it.** Moving `vstd` into another crate that ordinary builds
still require keeps the constraint. Making `vstd` optional alone gives no runtime
build, because the maintained code sits inside `verus!` and needs that macro to
compile. A separately maintained proof-only copy cannot establish that the
verified code is the code that runs.

**The requirement.** Keep one maintained implementation, and show how the code
Verus verifies corresponds to the functions ordinary builds execute. Two
candidates: a development verification workspace that reproducibly erases
specifications and proofs into the published runtime source; or an optional Cargo
feature over ordinary Rust, where the verified functions carry their contracts
through Verus's attribute syntax applied only with the feature, and specification
functions and proofs compile only with it, so the default build needs no Verus
crate. The arrangement, and any generation
mechanism, is established by a small feasibility experiment before the package
boundary changes. Generation and conversion assumptions are recorded, and drift
between verified and compiled code is detected.

**Acceptance.** The arrangement is accepted only when ordinary dependent builds
work without Verus, the baseline consumer resolves and builds with its own `vstd`,
and the proof and integration tests pass for the same packaged implementation.
The production-boundary review is repeated after any source or packaging change.
This covers the verification-dependency boundary only; other exact dependencies,
public API compatibility, and publication keep their own release obligations.

## Projects shipping verified code

A project that publishes verified code meets the same problem for its own
dependents, and the same requirement and acceptance apply. Its proof evidence is
evaluated against the source it verifies, so the correspondence to the published
source is part of the boundary its review covers. Once the engine's arrangement is
accepted, it is documented as the supported pattern; until then, a project's own
arrangement needs its own correspondence evidence.
