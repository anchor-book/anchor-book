# Formal Verification

**Status: proof of concept implemented; see the [roadmap](../implementation-roadmaps/formal-verification.md) for evidence.** Verus verifies selected executable engine logic,
starting with [valid authority](rust-engine.md#records-valid-authority). Its
authority remains [traceable properties](../authority/requirements.md#requirement-traceable-properties).
The anchor and its verification criteria live in Rust Engine; this chapter defines
the proof boundary and integration.

This experiment also develops a reusable way for projects to connect anchor
criteria to proofs of their production code. The engine is the first case. The
[criterion schema](anchor-definitions.md#verification-methods) declares required
evidence methods; Verus is this project's chosen proof tool. Formal specifications
and supporting proofs live with the code and identify the anchor and criterion
they address. The consumer association and report adapter proposed in
[Verification Evidence](verification-evidence.md) build on this first proof;
supported native report identities still need compatibility experiments.

## First proof: authority validation

The first specification expresses the existing [authority rules](authority-definitions.md#validating-authority-relationships):
an anchor has at least one authority; every reference resolves to exactly one valid
record of the declared kind; references are distinct; and accepted records cite
only accepted authorities. Include the requirements cited by decisions, including
unreferenced decisions. Draft records may cite draft or accepted authority.

Prove that the executable validator succeeds exactly when these conditions hold
over its declared input domain. Proving rejection as well as acceptance avoids a
validator that satisfies a one-way safety statement by rejecting every input.
The domain must include empty lists, duplicate identities, missing or invalid
targets, wrong kinds, repeated references, and incompatible statuses. Preconditions
must not assume away these cases or assume the property being proved.

The first proof of concept is local and limited to this authority validator. Its
claim is `validator accepts ⇔ authority rules hold` for the complete typed input.
This does not imply that parsing succeeded or that the whole Book is valid;
existing parse errors must still prevent a successful inventory.

Use a small input matrix to review the specification and integration tests:

| Input | Authority result |
| --- | --- |
| Draft anchor with a nonempty list of distinct, valid requirement or decision targets | Accept; targets may be draft or accepted. |
| Accepted anchor with a nonempty list of distinct, valid, accepted authorities | Accept. |
| Anchor with no authorities | Reject, including in Prototyping. |
| Missing, invalid, ambiguous, wrong-kind, or repeated authority target | Reject. |
| Accepted anchor or decision citing a draft authority | Reject. |
| Decision with no requirements | Accept; requirements are optional for decisions. |
| Decision citing requirements, even if no anchor cites the decision | Check every reference under the same resolution and status rules. |

These cases guide human review and tests; the proof must quantify over the input
collections rather than establish only these examples. Parser correctness,
diagnostic wording and locations, remote verification, and a general proof-evidence
framework are outside this first task.

## Prove the production implementation

`src/records/authority.rs` contains the executable checks, their contracts, and
proof annotations. `schema.rs` calls its verified presence check when decoding
anchor authorities; `inventory.rs` passes the complete candidate collection to
its verified `validate_authorities` function. `read_records` retains existing errors and prevents
inventory construction whenever diagnostics exist. Ordinary builds execute these
same function bodies; there is no proof-only implementation or alternate validator.

The module owns lookup, iteration, and authority acceptance. Its internal
candidate contains an optional identity, kind, acceptance status, references,
and a `decoded` flag indicating whether decoding produced a record. That flag is
not an authority-validation result or a precondition that relationships are valid.
Invalid candidates retain recoverable identities and participate in target lookup;
their existing decoding errors still prevent a successful inventory.

The specification names `records-valid-authority` and `formal-authority` in the
module. `reference_satisfies_authority_rules` combines three named predicates:
`reference_is_not_repeated` checks duplicates within one record,
`resolves_uniquely_to` checks identity and uniqueness across all candidates,
and `target_is_suitable_authority` checks decoding, kind, and acceptance status.
Undecodable candidates still count toward identity ambiguity.
`record_satisfies_authority_rules` adds required authority;
`records_satisfy_authority_rules` quantifies over the collection. `validate_authorities` ensures its violation list
is empty if and only if those rules hold. Loop invariants connect each processed
prefix to that complete specification. Supporting lemmas can move to separate
modules when useful; file layout does not determine the proof boundary.

This prototype uses string identities directly, borrowed slices, linear target
lookup, and a scan of earlier references for duplicates. The simplicity makes the
proof inspectable, at the cost of repeated scans. It has not been optimized or
benchmarked for large inventories. Global duplicate-identity diagnostics,
candidate conversion, backlinks, and diagnostic locations remain outside the
proof and are covered by integration tests.

```mermaid
flowchart TB
    input["Chapter text"] --> parse["Markdown and TOML parsing"]
    parse --> bridge["Typed candidates and references"]
    bridge --> rules["Executable authority validation"]
    rules --> result["Inventory or located diagnostics"]
    spec["Formal specification from anchor criteria"] --> verus["Verus verification"]
    rules -.-> verus
    verus --> evidence["Proof result with scope and assumptions"]
```

The solid runtime path uses the same function bodies that Verus verifies. The
verification path runs separately; reading or rendering a Book never invokes it.
The proof must not rely on a rendered or valid repository Book.

## What the proof establishes

| Boundary | Initial treatment |
| --- | --- |
| Authority lookup and validation | Verify the executable functions against the formal conditions, including collection traversal and all authority edges. |
| Markdown/TOML decoding and candidate conversion | Review and test that identity, kind, status, references, invalid candidates, and existing errors are faithfully represented. Parser correctness is outside the first proof. |
| Inventory construction and diagnostics | Review the call path and test that validation failures cannot become success; preserve located diagnostics and the all-or-nothing inventory. Diagnostic wording and locations are not initially proved. |
| Public library and mdBook adapter | Exercise independent valid and invalid fixtures through the real APIs/process. No successful partial inventory or Book JSON on invalid authority. |

A mathematical model may describe the specification, but proving that model alone
does not prove the engine. Identify every unverified conversion, external function
contract, and trusted assumption. Do not assume a validation function's result or
mark its body trusted to obtain a successful proof. Review preconditions for
consistency and demonstrate valid inputs satisfying them. The verifier, solver,
and Rust compilation remain part of the trusted toolchain.

Human review establishes whether the formal statement matches the anchor. A proof
does not establish that an authority logically justifies a property or that a
human approved an acceptance status. The first result is a proof of authority
logic under stated assumptions, supported by integration tests—not a proof of
the entire parser, renderer, or engine.

## Cargo and continuous integration

[Verus](https://verus-lang.github.io/verus/guide/) supplies specifications and
proofs for executable Rust, with limits on supported language features and libraries.
Its [Cargo integration](https://verus-lang.github.io/verus/guide/cargo_verus.html)
supports verification through `cargo verus verify` and ordinary Cargo builds of
annotated code. The pinned setup and native verification command are documented
in CONTRIBUTING.

Pin a compatible Verus release, its required Rust toolchain, and `vstd` during the
prototype. Check the engine's Rust 1.88 minimum and current stable builds before adopting
the dependency. Start within the current package; split out a crate only if the
experiment establishes a dependency/toolchain need, keeping that crate on the
production call path. Resolve any minimum-version change explicitly.

The prototype adds `vstd` as a normal dependency. Before release, the
[isolation decision](../authority/decisions.md#decision-isolate-engine-verification)
removes it, and the Verus toolchain, from what consuming projects build, so that
projects verifying their own code choose their own versions; see
[Project Integration](project-integration.md#what-the-crate-must-not-impose).

Treat compatibility as the first implementation checkpoint: confirm that a small
representative validator using the required types and collection operations both
verifies and builds normally before undertaking the full extraction. If that
requires a different package boundary or minimum Rust version, bring that concrete
constraint back for design review before expanding the refactor.

Run full verification for the selected proof scope locally and in a separate
GitHub job, alongside ordinary tests and mutation testing. Check that the intended
functions were verified: an empty or skipped verification run is not success.
The GitHub job uses a fresh build directory and checks the verifier's JSON report
for full-crate verification, successful results for the seven production authority
functions, and zero errors. It also runs independent engine and integration tests
on the same checkout. Selected results and revision metadata are retained on
success and failure; no generated Book is needed. CONTRIBUTING documents the job
and its limits.

Demonstrate that a deliberate behavioral defect fails verification, then restore
the implementation and rerun. Retain failure evidence; a timeout or tool failure
is inconclusive. Do not weaken specifications or add assumptions to hide defects.

cargo-mutants 27.1.0 discovers no mutations inside this module's `verus!` block.
Integration code is mutation-tested separately; a deliberate status-check defect
was rejected by the proof. This is a recorded coverage gap, not full mutation
coverage of annotated code. Tests remain required, and mutation findings stay in
their [own roadmap](../implementation-roadmaps/mutation-testing.md).

## Proof packaging for consumers

The published engine must meet the
[dependency-isolation property](project-integration.md#project-no-imposed-dependencies):
consuming builds do not compile the engine's verification libraries or inherit
its `vstd` choice. The current normal, exactly pinned dependency does not meet
that goal. On 2026-11-08, Cargo 1.99.1 rejected a consumer depending on the engine
and `vstd = "=0.0.0-2026-10-11-0230"`: the engine requires
`=0.0.0-2026-10-18-0147`, and resolution ended with
``failed to select a version for `vstd` which could resolve this conflict``.
Cargo's [compatible-version unification](https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility)
applies to these prereleases of the same `0.0.0` version, but their exact
requirements do not overlap. Both explicitly select a prerelease; this is not
implicit prerelease exclusion or a native-library `links` conflict.
The [consumer-resolution criterion](../implementation-roadmaps/formal-verification.md#formal-verification-4-consumer-resolution)
retains the failing baseline and requires the same consumer pin to pass after
isolation. Moving `vstd` into a crate still required by ordinary engine builds
would retain the constraint.

Keep one maintained authority implementation and demonstrate how the code
verified corresponds to the functions ordinary builds execute. A development
verification workspace with reproducible erasure into runtime source is a
candidate for the [isolation milestone](../implementation-roadmaps/formal-verification.md#formal-verification-4).
The source arrangement and any generation mechanism remain to be established by
a small feasibility experiment before changing the package boundary. Record
generation or conversion assumptions and detect drift between verified and
compiled code. A separately maintained proof-only validator cannot establish
production-code correspondence. Making `vstd` optional alone does not provide a
runtime build: the maintained validator is inside `verus!` and still needs that
macro to compile. The source arrangement must establish how ordinary builds
compile the same implementation without the verification dependency.

Accept the arrangement only after ordinary consumer builds work without Verus,
a consumer can choose a different `vstd`, and the authority proof and integration
tests pass for the same packaged implementation. Repeat the production-boundary
review after any source or packaging change. Isolation evidence addresses this
dependency boundary; other exact dependencies, public API compatibility, and
crate publication retain their separate release obligations.

## Evidence and later integration

Record the anchor and criterion IDs, specification and executable function names,
code and Book revisions, verifier/toolchain versions, command and selected scope,
result, assumptions, unverified boundaries, and supporting run. Tie integration
test results to the same implementation revision. Reassess affected proofs when
code, specifications, conversion, or dependencies change; earlier success does not
prove the new revision.

Initially, maintain this evidence in the [Formal Verification roadmap](../implementation-roadmaps/formal-verification.md).
Keep generated proof output out of authored Book sources and review artifacts
for public-safe content. Required verification methods do not create an automatic
verified badge; proof results remain separate from the authored anchor.
[Verification Evidence](verification-evidence.md#verus-runner) owns the reusable
runner, supported report adapters, function-mode associations, run identity, and
criterion states for the engine and consumers. The engine's workflow and
hard-coded seven-function report check are the first pilot. The
[consumer acceptance work](../implementation-roadmaps/verification-evidence.md#verification-evidence-5)
then exercises proofs of another project's production code. Until that capability
is implemented, this roadmap records engine proof evidence manually; proof
development does not depend on it.
