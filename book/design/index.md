# Design

This section describes the system anchor-book is intended to become, from the
system overview to component responsibilities, interfaces, and detailed behavior.
It represents the goal as currently understood, rather than only what is implemented.

Chapters remain **drafts for review**. The record library, mdBook rendering,
configuration loading, and structural checking are implemented; other
capabilities remain planned. See the
[roadmaps](../implementation-roadmaps/index.md) for progress and evidence.

| Chapter | Scope |
| --- | --- |
| [System Overview](system-overview.md) | Components, Book concepts, operations, and how the engine will develop using its own Book |
| [Rust Engine](rust-engine.md) | Library and CLI boundaries, data and errors, and Rust development conventions |
| [Formal Verification](formal-verification.md) | Verus proof of production authority-validation logic, integration boundaries, and evidence |
| [Configuration](configuration.md) | Mapping a project's layout into Book roles |
| [Anchor Definitions](anchor-definitions.md) | One structured authoring format, authority references, and verification criteria |
| [Authority Definitions](authority-definitions.md) | Typed requirements and decisions, identity, and authority-link validation |
| [Record Rendering](record-rendering.md) | First prototype: readable records and relationship navigation through mdBook |
| [Structural Checking](structural-checking.md) | Layout and navigation checks, execution flow, findings, and acceptance cases |

[Design Proposals](proposals/index.md) provides a place to discuss changes to
that goal state. Proposed content is distinguished from the accepted design.
