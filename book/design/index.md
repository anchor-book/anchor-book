# Design

This section describes the system anchor-book is intended to become, from the
system overview to component responsibilities, interfaces, and detailed behavior.
It defines the goal, not progress toward it.

Chapters remain **drafts for review**. The
[roadmaps](../implementation-roadmaps/index.md) record progress and evidence.

Read from the system overview through the core records and configuration, then
RFDs as the optional process for evolving that definition. The operations follow:
check its structure, produce verification evidence, and present the resulting
Book. Project integration applies those capabilities in another repository;
Rust Engine closes with implementation boundaries and APIs. Within Verification,
read the general execution workflow before the proof-specific details.

| Chapter | Scope |
| --- | --- |
| [System Overview](system-overview.md) | Components, Book concepts, operations, and how the engine will develop using its own Book |
| [Anchor Definitions](anchor-definitions.md) | One structured authoring format, authority references, and verification criteria |
| [Authority Definitions](authority-definitions.md) | Typed requirements and decisions, identity, and authority-link validation |
| [Configuration](configuration.md) | Mapping a project's layout into Book roles |
| [Requests for Discussion](rfds.md) | Optional proposals, review lifecycle, and relationships spanning design and implementation work |
| [RFD Source Format](rfd-format.md) | Configuration compatibility, proposal syntax, typed relationships, and template delivery |
| [RFD Worked Examples](rfd-examples.md) | A cross-capability proposal through acceptance and incorporation into the system definition |
| [Structural Checking](structural-checking.md) | Layout and navigation checks, execution flow, findings, and acceptance cases |
| [Checking Diagnostics](check-diagnostics.md) | Typed findings, result states, repair guidance, and CLI reports |
| [Verification](verification.md) | Methods, responsibilities, proofs that match their criteria, proof guards, and assessment records |
| [Running Verification](verification-evidence.md) | The verify operation: links from code to criteria, runners, guards, evidence states, and reports |
| [Proof Guards](proof-guards.md) | When guards run, the original Verus result, and subsequent witness, assessment, and sensitivity checks |
| [Shipping Verified Code](verified-code-packaging.md) | Publishing verified code without imposing its verifier on dependents |
| [Record Rendering](record-rendering.md) | Readable records and relationship navigation through mdBook |
| [Project Integration](project-integration.md) | How other Rust projects adopt the crate, run the engine, bootstrap agents, and verify their own code with Verus |
| [Rust Engine](rust-engine.md) | Library and CLI boundaries, data and errors, and Rust development conventions |

Changes to the goal are made directly in these chapters through the project's
review workflow. When the Book enables RFDs, they coordinate changes across all
affected chapters and work. Git history keeps superseded designs; RFDs retain the
coherent proposal and its review context.
