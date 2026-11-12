# Design

This section describes the system anchor-book is intended to become, from the
system overview to component responsibilities, interfaces, and detailed behavior.
It defines the goal, not progress toward it.

Chapters remain **drafts for review**. The
[roadmaps](../implementation-roadmaps/index.md) record progress and evidence.

Read from the system overview through record definitions and configuration,
then the engine and its operations. Verification defines how tests, proofs, and
reviews establish criteria and how `verify` reports them; project integration
brings those capabilities into other projects.

| Chapter | Scope |
| --- | --- |
| [System Overview](system-overview.md) | Components, Book concepts, operations, and how the engine will develop using its own Book |
| [Anchor Definitions](anchor-definitions.md) | One structured authoring format, authority references, and verification criteria |
| [Authority Definitions](authority-definitions.md) | Typed requirements and decisions, identity, and authority-link validation |
| [Configuration](configuration.md) | Mapping a project's layout into Book roles |
| [Rust Engine](rust-engine.md) | Library and CLI boundaries, data and errors, and Rust development conventions |
| [Record Rendering](record-rendering.md) | Readable records and relationship navigation through mdBook |
| [Structural Checking](structural-checking.md) | Layout and navigation checks, execution flow, findings, and acceptance cases |
| [Checking Diagnostics](check-diagnostics.md) | Typed findings, result states, repair guidance, and CLI reports |
| [Verification](verification.md) | Methods, responsibilities, proofs that match their criteria, proof guards, and assessment records |
| [Proof Guards](proof-guards.md) | When guards run, the original Verus result, and subsequent witness, assessment, and sensitivity checks |
| [Running Verification](verification-evidence.md) | The verify operation: links from code to criteria, runners, guards, evidence states, and reports |
| [Shipping Verified Code](verified-code-packaging.md) | Publishing verified code without imposing its verifier on dependents |
| [Project Integration](project-integration.md) | How other Rust projects adopt the crate, run the engine, bootstrap agents, and verify their own code with Verus |

Changes to the goal are made directly in these chapters through the project's
review workflow. Git history keeps superseded designs.
