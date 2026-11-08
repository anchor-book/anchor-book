# Authority

Authority establishes why an anchor's property must hold. An anchor can be based
on a requirement, a decision, or both:

- [Requirements](requirements.md) state required outcomes and constraints.
- [Decisions](decisions.md) record choices that govern the system being built,
  such as its scope, design, interfaces, verification, or supported environments,
  including their rationale, alternatives, and conditions for reconsideration.

Design explains the intended system; authority explains the obligations and choices
that govern it. Anchors cite those records so readers can trace each property to
its justification.

These records are **drafts for review**. Each uses the
[structured authority format](../design/authority-definitions.md); anchors cite
its typed identity. The engine validates those references, while review establishes
whether the authority justifies the property. Acceptance, implementation, and
verification remain separate claims.
