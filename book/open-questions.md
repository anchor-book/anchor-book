# Open questions

These are unresolved choices for the linked design work and experiments. They do
not define the target system or authorize implementation. Existing agreements
remain valid; settle only the missing choice with the engineer, then update its
owning design, authority, criteria, and work. Source observations belong in
[Research](research.md), not in this list.

## RFD support

Should Anchor Book implement the proposed optional RFD capability? The engineer
is reconsidering whether to proceed. Its [design](design/rfds.md), governing
requirement, and anchors remain draft, and both [RFD milestones](implementation-roadmaps/rfds.md)
remain Proposed. Retain the preparation for evaluation; implementation needs a
new decision and assignment. Ordinary design iteration, roadmap work, and
verification do not depend on selecting this capability. Existing external RFDs
can still supply source context without being imported into a new engine role.

## Anchor walkthroughs

Should an agent proactively offer an optional anchor walkthrough when proposing
implementation, or provide it only when requested? Both preserve design acceptance
and programmatic verification. [Project Integration milestone 6](implementation-roadmaps/project-integration.md#project-integration-6)
owns this interaction choice and the conversation trials. The [guide](design/user-guidance.md#anchor-walkthroughs)
already supports requested walkthroughs; do not introduce another approval gate.

## Packaged source comparison

Which source-comparison boundary should enforce permitted ghost erasure and the
relationship between verified and packaged executable bodies? The guarantee in
[Shipping Verified Code](design/verified-code-packaging.md#A-package-verified-bodies)
is already explicit. [Formal Verification milestone 4](implementation-roadmaps/formal-verification.md#formal-verification-4)
needs a concrete comparison design and seeded mismatch checks; manual comparison
of one package does not settle that automated policy.

## Managed content

How are owned regions declared, how are conflicts presented, and when is a plan
applied? [Materialization](design/system-overview.md#A-materialize-owned-content)
already requires preservation, conflict reporting, and a conforming no-op.
[Deferred materialization work](implementation-roadmaps/index.md#later-work-retained-from-the-design)
needs these choices before implementation; atomic writes are not implied by the
current design.

## Evidence integration

1. **Build configuration.** The native configuration and report-handoff interface
   for [Book build evidence](design/verification-evidence.md#book-build-evidence), settled with the artifact schema
   before implementation.
2. **Further claims.** [Claims of completion](design/verification-evidence.md#claims-of-completion) come from
   Complete milestones. Whether the `complete` [project phase](design/configuration.md#project-phase)
   also claims every criterion in the Book, and whether a test with no link should
   fail when a project opts in, remain open.
3. **Evidence handoff.** The transport and input-validation interface for consuming structured results in the Book build; output alone does not make saved results current.
4. **Other languages and runners.** Link forms and runners beyond Rust, nextest,
   and Verus.
5. **Sensitivity cost.** Each change verifies the complete target once, so large
   targets may need a sampling or
   per-function selection policy that reports which changes were not tried.

[Verification Evidence milestone 9](implementation-roadmaps/verification-evidence.md#verification-evidence-9)
owns the build/handoff design. Milestone 6 owns sensitivity operators and budgets;
other runners and broader completion policies remain later work requiring their
own scoped design. Build configuration and evidence handoff are one interface
choice, not two independent implementations.

## Project adoption

1. **Implementation associations.** Whether evidence links extend from tests and
   proofs to implementation sites, with explicit not-yet-implemented and
   not-applicable states.
2. **Supported Verus versions.** How wide a report-version range the engine can
   maintain, and how a project learns of the supported range.
3. **Bootstrap ownership.** Whether initialization writes and checks the
   `AGENTS.md` section as managed content, or the project maintains it from the
   template.

[Project Integration milestone 4](implementation-roadmaps/project-integration.md#project-integration-4)
owns version alignment and bootstrap references. Consumer proof experiments in
[Verification Evidence milestone 5](implementation-roadmaps/verification-evidence.md#verification-evidence-5)
inform supported Verus versions. Implementation-site associations remain deferred;
no association or availability state is selected by this question.
