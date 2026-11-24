## Develop verification with the code

Write anchor properties with clear behavior, conditions, and outcomes. Follow the
project's schema to declare required verification methods for each criterion.
Every listed method is an obligation; a selection is not evidence of completion.
Anchor satisfaction requires programmatic evidence: executable tests or formal
proofs. Human or agent review, approval, and recorded judgments do not satisfy a
criterion or gate its verification state, including within proof guards. A test
that merely reads an approval or asks an agent to judge success does not meet this
rule. For behavior without an executable check, report the verification gap;
keep subjective judgments in the design and change-review workflow. When
migrating a review criterion, define its executable assertions and leave evidence
outstanding until those checks run; never relabel review evidence as a test pass.
Keep tool choices and execution steps in the verification design and roadmap.

### Declare every verification obligation

An anchor is satisfied only when every criterion is satisfied, and a criterion
only when every required method has current passing evidence. Define criteria
from the whole property before mapping tests. Separate obligations whose evidence
can be missing independently, including compiler checks, failure paths, operating
environments, and limits. Keep known missing or unsupported checks as explicit
criteria; a roadmap caveat must not leave the anchor reporting satisfied.

When linked checks cover only part of a criterion, preserve its full meaning by
splitting out the remaining obligations and reconciling links and roadmap coverage.
Retain existing identifiers for continuing scope and record where extracted scope
moved. Link each check only to the obligation its assertions establish. Do not
reuse a runtime pass for a compiler or platform obligation it does not exercise.
A test can establish several criteria when its assertions cover each of them.
Rerun verification after the split: the covered criteria may pass, but the anchor
must remain partial or unsatisfied until every remaining criterion passes. Passing
links alone cannot reveal an obligation omitted from the Book.

An anchor requires formal verification if at least one of its criteria lists
`"proof"`. Each such criterion requires proof of its stated scope. If all criteria
have valid method lists and none lists proof, formal verification is not required
for that anchor revision. Missing or invalid declarations are unresolved input,
not permission to omit verification. Do not infer requirements from existing proof
files or design-acceptance status.

Agree on methods while designing criteria, considering failure consequences,
input/state complexity, specification precision, the production-code boundary,
and maintenance cost, as the project's verification design describes. Record the
rationale there. Methods
are minimum obligations; propose additional evidence when useful. Report gaps in
the anchor's property even when all listed checks pass. Changes to required
methods or scope require design and roadmap reconciliation with the engineer.

For a criterion requiring tests, derive assertions from the criterion rather than
the implementation's output, cover its failure cases and boundaries, exercise the
interface the criterion concerns, and link the tests to the criterion. Tests named
in a proof's exclusions must exercise the boundary the proof leaves out.

### Develop a required proof

A proof counts only when it establishes exactly what its criterion marks for
proof, over the code the system runs. Verifier success shows that code meets its
specification, not that the specification is the criterion. For each criterion
requiring proof:

1. **Read the criterion.** Read its claim, domain, boundary, assumptions, and
   exclusions, using the engine's brief where the project supports it. Follow the
   project's authoring format, and resolve missing or ambiguous scope with the
   engineer before writing the specification.
2. **Write the specification from the claim.** State the claim through named
   specification functions that follow the design's rules. Prove both directions
   of an "if and only if" claim. Preconditions state only what the code needs to
   run; they never exclude cases the domain includes or assume the claim.
3. **Put it on the production functions.** Keep the specification and proofs with
   the production code, linked to the anchor and criterion IDs. Verify the
   functions the system executes, never a proof-only copy or model. A production
   caller outside verification gets no precondition check, so give the functions
   it calls no preconditions or name that boundary in the exclusions.
4. **Iterate with the verifier.** Develop the proof alongside the implementation
   and rerun the criterion's verification after each change. Never add
   assumptions, trusted bodies, or weaker contracts to obtain a pass; a timeout or
   tool failure is inconclusive.
5. **Check that the proof can fail.** A deliberate behavioral defect in a linked
   function must fail verification. Investigate every change that still verifies:
   strengthen the specification, show that the covering criterion's tests detect
   it, or leave the survivor unresolved. An explanation of equivalence is not
   passing evidence. Show that preconditions can be met.
6. **Cover the exclusions.** Every excluded part of the anchor's property stays
   covered by the criterion its exclusions name, or is reported as a gap. Examine
   input conversion, callers, and failure propagation, and identify which
   boundaries are proved, tested, or unresolved. An algorithm-only proof is never
   reported as proof of its parsing, conversion, or caller integration.
7. **Account for the result.** Give the engineer a readable account of the formal
   claim, assumptions, coverage, survivors, and gaps in its translation from the
   criterion. Review this correspondence through the project's normal change
   workflow; that judgment is separate from programmatic verification.

Do not weaken the anchor or assume its property to obtain a proof; changes to
intended behavior require design reconciliation. Record results against code and
design revisions, and reassess affected evidence when either changes; a changed
specification also needs scrutiny of its correspondence to the claim. Partial
evidence does not satisfy an entire criterion, and tests do not replace a required proof.
Report proof results for the specific anchor and criterion, scope, and revision;
a proof requirement or a successful partial proof does not verify the whole anchor.

