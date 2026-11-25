# Verification



Anchor criteria declare the methods that must establish them: tests and proofs. Verification produces evidence that each declared method holds for the
current code and design, and reports where it does not. Anchor Book provides this
to Rust projects as one capability; the engine's own Book is its first user.

Every method must establish exactly its criterion: tests whose assertions come
from the criterion and exercise the code it concerns, proofs whose specification
expresses the criterion. The most important case
is proofs: **proofs establish exactly the parts of the design marked as needing
proof.** Verus checks that code meets its specification. It cannot check that the specification is the
criterion. Anchor Book helps the project's agents make the specification express
the criterion, prove it of the code the system runs, and show what the proof
covers and what it does not.

| Chapter | Scope |
| --- | --- |
| This chapter | Methods, responsibilities, how tests and proofs correspond to criteria, the guards that check proofs, and their programmatic limits |
| [Running Verification](verification-evidence.md) | The `verify` operation: links from code to criteria, runners, evidence states, and reports |
| [Proof Guards](proof-guards.md) | Timing table and sequence diagram for the checks around Verus |
| [Shipping Verified Code](verified-code-packaging.md) | Publishing a crate containing verified code without imposing its verifier on dependents |

## Anchors

The proof guards below constrain `verify`; the [trust policy](verification-evidence.md#verus-runner)
and the runner's other states are anchored in
[Running Verification](verification-evidence.md#anchors) and
[Project Integration](project-integration.md#A-project-verus-evidence).

> **`A-verify-proof-sensitive`** (draft): **Proofs reject defects in the code they cover**
> Property: For every proof criterion in scope, verify applies deliberate behavioral changes to the executable bodies of the criterion's linked functions, one at a time, and verifies the complete target for each. Every change that still verifies is reported at its source location. The proof method is satisfied only when every surviving change is detected by the linked tests of a criterion that its exclusions name. An unclassified survivor means the specification does not constrain that behavior, and the proof method fails.
> Authority: `R-proofs-establish-criteria`
> - Criterion `surviving-changes` [test]: In a fixture whose specification omits one rule, the change that violates that rule verifies and is reported as an unclassified survivor, failing the proof method; once the specification states the rule, the same change fails verification and the method can be satisfied.
> - Criterion `classified-survivors` [test]: A survivor is classified as excluded only when the linked tests of a criterion named in the exclusions fail against the same change. A judgment of equivalence supplies no evidence; other survivors remain unresolved and fail the method. Specifications and proofs are never changed.
> - Criterion `uninformative-changes` [test]: Changes that do not build are listed as unviable; changes rejected only by overflow, bounds, or termination obligations are reported with the failing obligation; a timed-out run makes the proof method inconclusive. None of these counts as a survivor or as detection by the claim.

> **`A-verify-proof-nonvacuous`** (draft): **Proof contracts can be met by some real input**
> Property: A linked function with preconditions contributes satisfied proof evidence only when a verified witness, linked to the same criterion, establishes those preconditions for at least one input it constructs. A function whose preconditions cannot hold verifies any postcondition, so without a witness its proof method has no evidence, and a witness that fails verification fails the method. Functions without preconditions need no witness.
> Authority: `R-proofs-establish-criteria`
> - Criterion `witness-required` [test]: Fixtures with linked functions whose preconditions are unsatisfiable, satisfiable but unwitnessed, and witnessed by a verified witness function produce no-evidence, no-evidence, and satisfied proof methods; a witness that fails verification fails the method.

> **`A-verify-programmatic-only`** (accepted): **Judgments cannot count as verification**
> Property: Anchor criteria declare only test and proof methods. Verify derives satisfaction from current executable results and programmatic proof guards, never from a human or agent judgment. Unsupported review declarations fail validation; an approval record cannot replace missing, failing, or unevaluated checks.
> Authority: `R-programmatic-verification`
> - Criterion `methods` [test]: The record reader and actual verify command reject review-only and mixed test/review criteria with located diagnostics; the CLI rejects a review method selector. Test, proof, and both method orders remain valid.
> - Criterion `programmatic-states` [test]: With current target and linked function results, passing witness and sensitivity results satisfy a proof without an assessment record. Missing, failing, or skipped guards retain their designed states; a passing test cannot hide a failed proof obligation.


## Methods

The [criterion schema](anchor-definitions.md#verification-methods) defines the
values and their syntax. Every listed method is required.

| Method | Establishes | Evidence |
| --- | --- | --- |
| `test` | The stated cases behave as the criterion requires | Linked tests that ran and passed in the current run, with assertions derived from the criterion |
| `proof` | The claim holds for every input in the stated domain, over the production code | Linked functions verified in a complete run, under the trust policy, with every [guard](#guards) satisfied |

Tests sample behavior; their assertions are visible, so a reader can judge what
they cover. A proof covers every input, but its strength is in the specification,
and nothing in a successful run shows how strong that is. The proof method
therefore includes programmatic guards. Design and specification review examine
whether the checks express the intended behavior, but do not enter verification
states. A passing check does not prove that this translation is sufficient.

## Choosing methods

Choose methods during design review, for each criterion:

- **Consequence of a violation.** Silent acceptance of invalid input or loss of
  data favors proof; a cosmetic defect does not.
- **Input and state complexity.** When the cases that matter are too many to
  enumerate, examples miss combinations a proof covers.
- **Precise statement.** A proof needs a claim stated precisely over the actual
  production functions; behavior defined by a parser or another tool usually
  cannot be.
- **Maintenance.** Proofs constrain how the code can change and need the
  verifier's language subset.

Record the rationale with the criterion's design. Tests and proofs complement
each other: a proof of an algorithm usually leaves its parsing, conversion, and
callers to tests, named in its exclusions. Extending proof across
every boundary is not an automatic obligation.

## Responsibilities

| Owner | Responsibility |
| --- | --- |
| Project | Declare each criterion's claim and scope; write specifications, proofs, and tests beside the production code and link them; choose and install its Verus, `vstd`, verification toolchain, and verified targets; review specifications and record the reviews. |
| Verus | Verify the selected code against its contracts under the recorded assumptions. |
| Anchor Book | Define the criterion schema; resolve links; run the project's tests and Verus; interpret their reports; run the guards; validate current input identities; report evidence states and their limits; give agents a brief of each criterion and the contract's proof workflow (`anchorbook contract`, “Develop a required proof”). |
| Reviewer | Assess whether the specification expresses the criterion, whether production callers meet its preconditions, whether the verified functions are the ones the system runs, and whether exclusions are covered. |

Anchor Book needs no `vstd` and no verifier of its own for this: it runs the
project's installation and reads its report.

## Shared verification adapters

Anchor Book supplies the connection between anchor criteria and the project's
verification tools. An adapter translates a verification plan into tool
invocations and normalizes the results; the engine owns criterion matching,
current-input checks, required guards, and satisfaction across every required
method. The standard Verus adapter invokes the project's installed tools, so
using this capability adds no engine-owned `vstd` dependency to the project.

Anchor Book uses that same project-facing adapter interface to verify its own
code. Its private `verification/` workspace is a consumer of the interface: it
owns the engine's proof dependencies, lockfile, and build configuration, while
the adapter runs the verifier and returns evidence. The workspace's library
points to the maintained production source, under the
[source-correspondence rules](verified-code-packaging.md#proof-packaging-for-consumers).
It is excluded from the published crate; the capability to verify a user's own
criteria remains part of Anchor Book. Engine proofs establish only this Book's
criteria, never those of a consuming project.

A project-specific verification command connects through an adapter with defined
result semantics. Results must identify the checks and inputs evaluated, actual
tools, completeness, and outcomes; a successful process exit alone cannot satisfy
a criterion. Adapter results retain the same evidence and proof-scope rules as
standard runners. A separate model's success does not establish a claim about
production code without the required correspondence.

The adapter boundary does not require another crate. Separate optional adapter
packages are useful when dependencies or verifier compatibility need independent
releases; their packaging preserves the same dependency isolation and evidence
rules. Anchor Book's own proof workspace and independent consumer fixtures
exercise the shared interface, with no engine-only invocation or result parser.
[Running Verification](verification-evidence.md#running-linked-checks) owns runner
behavior; [adapter extension choices](../open-questions.md#verification-adapter-extensions)
identify the interface details to settle before adding custom commands.

## Tests that match the design

Projects own the behavior and outcome semantics of their tests. The engine
[trusts current project test results](verification-evidence.md#trusting-project-test-results)
while validating their identity and association with criteria. The guidance below
is for authoring and reviewing those tests; it is not an additional engine-run
inspection or execution-attestation requirement.

An anchor requires passing evidence for every criterion. Author
[independently verifiable criteria](anchor-definitions.md#independently-verifiable-criteria)
so a known missing compiler check, failure case, or environment cannot be hidden
behind a passing runtime test. A test establishes only the cases it exercises;
its assertions must come from the criterion:

- **Assertions from the criterion.** Derive expected results from the criterion
  and its design, never from the implementation's current output. A test that
  records what the code does cannot detect that it does the wrong thing.
- **The cases the criterion names.** Cover the boundaries, invalid inputs, and
  failures the criterion describes, not only success. A criterion stating both
  acceptance and rejection needs tests of both.
- **Independent fixtures.** Specify fixtures from the design, independently of the
  code under test. The project's own Book or data can be an additional acceptance
  case, not the only one.
- **The entry point the criterion concerns.** When a criterion is about a public
  API, a command, or an adapter, exercise that interface, such as the actual CLI
  process; a test of an internal helper establishes only the helper. This is the
  testing counterpart of proving the code that runs.
- **Proof exclusions.** A test criterion named in a proof's exclusions must
  exercise the boundary the proof leaves out, such as parsing, conversion, or a
  caller outside verification. Its linked tests are also what classify
  [sensitivity](#guards) survivors in that behavior.

A link claims that a test addresses its criterion; whether the tests are
sufficient is visible in their assertions and remains a design-review question,
separate from verification state. Agent-driven tests require executable assertions
over observed behavior; a judge's approval alone is not a test outcome.

**Mutation testing** changes the code and checks that tests fail. It shows where
tests do not constrain behavior, guides strengthening them, and is never evidence
for any method. Timeouts and changes that do not build are not detection. Ordinary
mutation tools find no mutations inside `verus!` bodies; proof sensitivity covers
that code. Engine practice is in [Rust Engine](rust-engine.md#mutation-testing).

## Proofs that match the design

A formal specification is a precise, machine-checkable statement of what code
must do. In Verus, each function's contract is its `requires` and `ensures`:
what callers must provide and what the function guarantees. The specification is
those contracts together with the specification functions they use. The verifier proves that the code meets it; it
cannot tell whether the specification says what the criterion says.

A criterion requiring proof states its claim and four scope items, defined in
[Anchor Definitions](anchor-definitions.md#required-scope-of-a-proof-criterion).
Each becomes part of the Verus code:

| Scope item | In the Verus code |
| --- | --- |
| Claim | The `ensures` of the linked entry functions, written through named specification functions that follow the design's rules. A claim with an "if and only if" is proved in both directions, so a function that rejects everything cannot satisfy it. |
| Domain | The parameter types and `requires`. Preconditions state only what the code needs to run, such as valid indices; they never exclude cases the domain includes or assume the claim. |
| Boundary | Which production functions carry the contracts and links. |
| Assumptions | What the run trusts: the verifier, solver, compiler, and the contracts of `vstd` and other dependencies. Project assumptions and trusted bodies are rejected by the trust policy. |
| Exclusions | Behavior the contracts do not state, each with the criterion that covers it. |

**Prove the code that runs.** The contracts belong on the production functions,
and ordinary builds execute the same bodies Verus verifies; Verus erases only
specifications and proofs. A separately written model or proof-only copy does not
establish the claim about the system, however closely it resembles the code.

**Callers outside verification.** Verus checks preconditions only at verified
call sites. A production caller outside verification gets no such check, so a
function it calls must have no preconditions, or that call boundary is an
exclusion with its covering criterion.

**Exclusions.** Every part of the anchor's property that the proof leaves out
stays covered by another criterion or is reported as a gap. A proof of an
algorithm is never reported as proof of its parsing, conversion, or callers.

The engine's [authority proof](rust-engine.md#authority-proof) is a worked example:
its claim is a biconditional over the complete candidate collection, and its
exclusions name the `production-integration` criterion that tests
parsing, conversion, and diagnostics.

## Guards

A successful Verus run shows the code meets its contracts. A **guard** is a check
`verify` runs on a proof beyond that result, for what it does not show: that the
contracts constrain the exercised changes, can be met, apply to current inputs,
and rest on no unrecorded assumption.

| Concern | Guard | Mechanism |
| --- | --- | --- |
| The specification constrains the code's behavior | [Sensitivity](#A-verify-proof-sensitive): deliberate changes to linked functions must fail verification | Mechanical |
| Preconditions can be met | [Witnesses](#A-verify-proof-nonvacuous) for linked functions with preconditions | Mechanical |
| Current code and specifications | Input identities checked before and after evaluation | Mechanical |
| No project assumptions | Trust policy: assumptions and trusted bodies fail the method | Mechanical |

**Sensitivity** is whether a proof detects defects in the code it covers; it is
to proofs what mutation testing is to tests. A deliberate change to a linked
function that still verifies is a **survivor**: it shows behavior the
specification does not constrain. For example, changing the authority validator's accepted-status check
to always pass must fail verification; if it verifies, the specification missed
the status rule. A survivor in behavior the criterion deliberately excludes, such
as which failure kind is reported, is expected; it must then be caught by the
covering criterion's tests, which cross-checks the exclusion. Changes apply only
to executable code; changing a proof or specification tests nothing about the
system.

A proof is **vacuous** when it holds only because its preconditions can never be
met: Verus proves any guarantee of a function nobody can call. A **witness** is a
verified function that calls a linked function with an input it constructs,
showing that its preconditions can be met; tests cannot show this, because preconditions are not checked when
code runs. Input fingerprints bind evidence to the code, specifications, and
criteria evaluated; changing those inputs requires fresh verification.
[Running Verification](verification-evidence.md#proof-guards) defines how the
guards run.

### When guards are checked

[Proof Guards](proof-guards.md) gives the timing table and execution diagram,
from the original Verus run through witness and sensitivity checks.

## Review and verification

Review examines whether the criterion, specification, production call path, and
exclusions describe the intended system. It can uncover a missing check or require
a design change. Such judgments belong in the project's ordinary review workflow;
`verify` does not consume them, accept assessment files, or gate results on an
approval. Nor may a sensitivity survivor be dismissed by a recorded judgment of
equivalence. An unsupported survivor stays unresolved until executable evidence
accounts for it under the sensitivity policy.

Programmatic checks establish their stated scope. They cannot establish that a
natural-language claim has been translated faithfully merely by running. Reports
retain that limitation even when every required method passes.

## Evidence and reassessment

Evidence holds for the inputs it was produced from. `verify` reports only the
current run, and [run identity](verification-evidence.md#run-identity) records the
inputs, tools, and scope. A change to code, specifications, conversion, or
dependencies requires reverification and scrutiny of affected checks. Earlier success does not establish a new revision, and partial
evidence does not satisfy a whole criterion. Results stay out of authored Book
sources; required methods are obligations, never badges.
