# Terminology

Each entry gives a short meaning and links to the section that defines the term;
that section is authoritative. Use these terms in
Book text and identifiers. Prefer plain words to abbreviations and tool jargon.

The two tables follow [Project Guidance and Terminology](design/project-guidance.md).
Anchor Book owns the standard vocabulary; consumers own their project vocabulary.
Here the consuming project is the Anchor Book engine itself, so its implementation
and verification vocabulary belongs in the project table.

## Anchor Book terminology

This table is maintained by Anchor Book and versioned with the engine. Changes
here change the shared vocabulary; reconcile the contract and defining design.
Consumers preserve these meanings when adding their own terms.
This table belongs to this engine revision. Consumer copies identify their
selected revision and adapt reference links to that version's authoring material;
the relative links here refer to Anchor Book's own Book. Updating an installed
engine does not synchronize a consumer's copy.

| Term | Meaning |
| --- | --- |
| Anchor Book | A project's system definition, governing authority, correctness anchors, and implementation roadmaps, read alongside project guidance; see [System Overview](design/system-overview.md). |
| AI Working Contract | Reusable agent guidance owned and versioned by Anchor Book, retrieved from the engine; see [contract delivery](design/system-overview.md#contract-delivery). |
| Project guidance | An engine-managed Book role for developer workflow, review guidance, and project logistics, with project-owned instructions alongside the contract; see [Project Guidance](design/project-guidance.md#project-guidance). |
| Goal system | The system the engineer and agents are building, as the Book describes it; see the AI Working Contract (`anchorbook contract`). |
| Design | The Book's description of the goal system as a whole; see [Design](design/index.md). |
| RFD (Request for Discussion) | A coherent proposal for a substantial design change, with review disposition and links across affected capabilities and work; see [Requests for Discussion](design/rfds.md). An optional capability; listing the term does not enable it. |
| Anchor | A named property that must hold in the built system, grounded in authority and with verification criteria; see [Anchor Definitions](design/anchor-definitions.md). |
| Anchor satisfaction | The computed verification result when every declared criterion and every required method has current passing evidence. It does not establish design acceptance or reveal omitted obligations; see [Evidence states](design/verification-evidence.md#evidence-states). |
| Property | The statement of what an anchor guarantees, with its scope and limits; see [anchor fields](design/anchor-definitions.md#anchor-fields). |
| Criterion | A statement of what verification must establish for an anchor, with its required methods; see [anchor fields](design/anchor-definitions.md#anchor-fields). |
| Verification method | `test` or `proof`; every method a criterion lists is required. See [verification methods](design/anchor-definitions.md#verification-methods) and [Methods](design/verification.md#methods). |
| Proof scope | A proof criterion's claim, domain, boundary, assumptions, and exclusions; see [required scope](design/anchor-definitions.md#required-scope-of-a-proof-criterion). |
| Authority | The requirements and decisions that explain why a property must hold; see [Authority](authority/index.md). |
| Requirement | A required outcome or constraint, with its source; see [requirement fields](design/authority-definitions.md#requirement-fields). |
| Decision | A choice that governs the system, with its rationale and alternatives; see [decision fields](design/authority-definitions.md#decision-fields). |
| Record identifier | A record's stable name: a kind code and a name stating its meaning, never a number. Criteria have local identifiers within their anchors. See [Identifiers](design/anchor-definitions.md#identifiers). |
| Record status | A record's design acceptance, `draft` or `accepted`; it does not mean implemented or verified. See [Heading](design/anchor-definitions.md#heading). |
| Phase | The project's working expectations: port, prototyping, building, or complete; see [Project phase](design/configuration.md#project-phase). |
| Implementation roadmap | Planned work toward the goal system, divided into milestones that distinguish proposals from agreed and completed work; see [Defining the work](implementation-roadmaps/index.md#defining-the-work). |
| Milestone | A coherent outcome within a roadmap, with tasks and completion criteria; see [Defining the work](implementation-roadmaps/index.md#defining-the-work). |
| Milestone status | Work progress: Proposed, Ready, In progress, Blocked, or Complete. It is separate from record acceptance and anchor verification; see [Recording progress](implementation-roadmaps/index.md#recording-progress). |
| Completion criterion | An observable condition showing a milestone's outcome is reached, distinct from an anchor criterion; see [planned coverage](implementation-roadmaps/index.md#anchor-criteria-and-planned-coverage). |
| Structural check | The read-only check that a Book conforms to its configured layout and navigation; see [Structural Checking](design/structural-checking.md). |
| Evidence | Recorded verification results and supporting artifacts for a stated scope and revision. Results can show failure or incomplete coverage; only applicable current passing results support satisfaction. See [Evidence and reassessment](design/verification.md#evidence-and-reassessment). |
| Evidence link | An association between a test or proof function and a criterion, recorded in a source comment by the current engine. A link alone establishes no passing result; see [Evidence links](design/verification-evidence.md#evidence-links). |
| Evidence state | A computed result for a required method, criterion, or anchor, including missing, partial, failed, and satisfied results; see [Evidence states](design/verification-evidence.md#evidence-states). |
| Review | A judgment about design, code, or evidence in the project's change workflow. It does not satisfy an anchor or gate verification; see [Review and verification](design/verification.md#review-and-verification). |

## Project terminology

This table is maintained by this project's agents and engineer. Add terms from
agreed design; raise new meanings or conflicts for the engineer's judgment.
Updates to the standard table preserve this table and project-authored prose.
“Function contract” distinguishes that technical meaning from the AI Working
Contract above.
This Book's [RFD capability remains an unselected draft](open-questions.md#rfd-support).

| Term | Meaning |
| --- | --- |
| Formal specification | A precise, machine-checkable statement of what code must do: in Verus, the contracts of its functions and the specification functions they use. See [Proofs that match the design](design/verification.md#proofs-that-match-the-design). |
| Function contract | A function's `requires` and `ensures`: what its callers must provide and what it guarantees; see [Proofs that match the design](design/verification.md#proofs-that-match-the-design). |
| Precondition | A condition in `requires` that callers must establish; Verus checks it only at verified call sites. |
| Production code | The functions the system actually executes, as opposed to a model or proof-only copy; see [Prove the code that runs](design/verification.md#proofs-that-match-the-design). |
| Trust policy | The verifier settings that reject project assumptions and trusted function bodies; see [Verus runner](design/verification-evidence.md#verus-runner). |
| Mutation testing | Deliberately changing code to check that tests detect faults. It assesses test sensitivity and does not satisfy an anchor's verification methods; see [Tests that match the design](design/verification.md#tests-that-match-the-design). |
| Guard | A check `verify` runs on a proof beyond the verifier's own result: that the specification constrains the code, its preconditions can be met, it is the one assessed, and no assumptions were added; see [Guards](design/verification.md#guards). |
| Sensitivity | Whether a proof detects defects in the code it covers, measured by deliberate changes to linked functions that must fail verification; the proof counterpart of mutation testing. See [Guards](design/verification.md#guards). |
| Survivor | A deliberate change to a linked function that still verifies. It is behavior the criterion leaves to tests, which must catch it; an assessed equivalent change; or a gap in the specification. See [Guards](design/verification.md#guards). |
| Vacuous | Holding only because its preconditions can never be met; Verus proves any guarantee of a function nobody can call. See [Guards](design/verification.md#guards). |
| Witness | A verified function that calls a linked function with an input it constructs, showing that its preconditions can be met, so its proof is not vacuous; see [Guards](design/verification.md#guards). |
| Fingerprint | A hash identifying inputs to verification, used to detect changes that invalidate current evidence; see [run identity](design/verification-evidence.md#run-identity). |
| Brief | A criterion's scope, linked functions, and exclusions with their covering criteria, presented without running verification tools. The designed `verify --brief` command is not implemented in this engine revision; see [Using verify](design/verification-evidence.md#using-verify) and [available tools](working-on-anchor-book.md#available-tools-and-checks). |
