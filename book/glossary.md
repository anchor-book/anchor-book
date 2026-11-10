# Glossary

**Status: draft for review.** Each entry gives a short meaning and links to the
section that defines the term; that section is authoritative. Use these terms in
Book text and identifiers. To introduce a new term, propose it to the engineer
and add it here when agreed. Prefer plain words to abbreviations and tool jargon.

## Book concepts

| Term | Meaning |
| --- | --- |
| Goal system | The system the engineer and agents are building, as the Book describes it; see the AI Working Contract (`anchorbook contract`). |
| Design | The Book's description of the goal system as a whole; see [Design](design/index.md). |
| Anchor | A named property that must hold in the built system, grounded in authority and with verification criteria; see [Anchor Definitions](design/anchor-definitions.md). |
| Property | The statement of what an anchor guarantees, with its scope and limits; see [anchor fields](design/anchor-definitions.md#anchor-fields). |
| Criterion | A statement of what verification must establish for an anchor, with its required methods; see [anchor fields](design/anchor-definitions.md#anchor-fields). |
| Verification method | `test`, `proof`, or `review`; every method a criterion lists is required. See [verification methods](design/anchor-definitions.md#verification-methods) and [Methods](design/verification.md#methods). |
| Proof scope | A proof criterion's claim, domain, boundary, assumptions, and exclusions; see [required scope](design/anchor-definitions.md#required-scope-of-a-proof-criterion). |
| Authority | The requirements and decisions that explain why a property must hold; see [Authority](authority/index.md). |
| Requirement | A required outcome or constraint, with its source; see [requirement fields](design/authority-definitions.md#requirement-fields). |
| Decision | A choice that governs the system, with its rationale and alternatives; see [decision fields](design/authority-definitions.md#decision-fields). |
| Identifier | A record's stable name: a kind code and a name stating its meaning, never a number; see [Identifiers](design/anchor-definitions.md#identifiers). |
| Status | A record's design acceptance, `draft` or `accepted`; it does not mean implemented or verified. See [Heading](design/anchor-definitions.md#heading). |
| Phase | The project's working expectations: prototyping, building, or complete; see [Project phase](design/configuration.md#project-phase). |
| Implementation roadmap | Agreed work toward the goal system, divided into milestones; see [Defining the work](implementation-roadmaps/index.md#defining-the-work). |
| Milestone | A coherent outcome within a roadmap, with tasks and completion criteria; see [Defining the work](implementation-roadmaps/index.md#defining-the-work). |
| Completion criterion | An observable condition showing a milestone's outcome is reached, distinct from an anchor criterion; see [planned coverage](implementation-roadmaps/index.md#anchor-criteria-and-planned-coverage). |
| Structural check | The read-only check that a Book conforms to its configured layout and navigation; see [Structural Checking](design/structural-checking.md). |

## Verification

| Term | Meaning |
| --- | --- |
| Evidence | Results from the current code and design showing a criterion's methods hold; see [Evidence and reassessment](design/verification.md#evidence-and-reassessment). |
| Evidence link | A source comment associating a test or proof function with a criterion; see [Evidence links](design/verification-evidence.md#evidence-links). |
| Evidence state | The single state `verify` reports for each required method of a criterion; see [Evidence states](design/verification-evidence.md#evidence-states). |
| Formal specification | A precise, machine-checkable statement of what code must do: in Verus, the contracts of its functions and the specification functions they use. See [Proofs that match the design](design/verification.md#proofs-that-match-the-design). |
| Contract | A function's `requires` and `ensures`: what its callers must provide and what it guarantees; see [Proofs that match the design](design/verification.md#proofs-that-match-the-design). |
| Precondition | A condition in `requires` that callers must establish; Verus checks it only at verified call sites. |
| Production code | The functions the system actually executes, as opposed to a model or proof-only copy; see [Prove the code that runs](design/verification.md#proofs-that-match-the-design). |
| Trust policy | The verifier settings that reject project assumptions and trusted function bodies; see [Verus runner](design/verification-evidence.md#verus-runner). |
| Review | The `review` method: a recorded judgment a criterion requires because no test or proof can settle it; see [Assessment records](design/verification.md#assessment-records). Not to be confused with design or pull-request review. |
| Specification assessment | Part of the `proof` method: a recorded judgment that a formal specification expresses its criterion, that production callers meet its preconditions, and that its exclusions are covered; see [Assessment records](design/verification.md#assessment-records). |
| Assessment record | A file recording a review or specification assessment, what was examined, and the conclusion; see [Assessment records](design/verification.md#assessment-records). |
| Mutation testing | Changing code to check that tests detect the change; it assesses tests and is never evidence. See [Tests that match the design](design/verification.md#tests-that-match-the-design). |
| Guard | A check `verify` runs on a proof beyond the verifier's own result: that the specification constrains the code, its preconditions can be met, it is the one assessed, and no assumptions were added; see [Guards](design/verification.md#guards). |
| Sensitivity | Whether a proof detects defects in the code it covers, measured by deliberate changes to linked functions that must fail verification; the proof counterpart of mutation testing. See [Guards](design/verification.md#guards). |
| Survivor | A deliberate change to a linked function that still verifies. It is behavior the criterion leaves to tests, which must catch it; an assessed equivalent change; or a gap in the specification. See [Guards](design/verification.md#guards). |
| Vacuous | Holding only because its preconditions can never be met; Verus proves any guarantee of a function nobody can call. See [Guards](design/verification.md#guards). |
| Witness | A verified function that calls a linked function with an input it constructs, showing that its preconditions can be met, so its proof is not vacuous; see [Guards](design/verification.md#guards). |
| Fingerprint | A hash of what a specification assessment or review examined; it ignores comments and formatting, and a change makes the record stale. See [Guards](design/verification.md#guards). |
| Brief | The output of `verify --brief`: a criterion's scope, linked functions, assessment status, and exclusions with their covering criteria, produced without running tools; see [Using verify](design/verification-evidence.md#using-verify). |
