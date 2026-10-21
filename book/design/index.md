# Design

This section describes the system anchor-book is intended to become. Design
includes the overall system and the details of its capabilities, responsibilities,
interfaces, and behavior.

The initial chapters are **drafts for review**. They establish a proposed goal
state. The record library and mdBook rendering are implemented; other capabilities
remain planned.
See the roadmaps for assigned work and its evidence.

| Chapter | Scope |
| --- | --- |
| [System Overview](system-overview.md) | Components, Book concepts, operations, and how the engine will develop using its own Book |
| [Rust Engine](rust-engine.md) | Library and CLI boundaries, data and errors, and Rust development conventions |
| [Configuration](configuration.md) | Mapping a project's layout into Book roles |
| [Anchor Definitions](anchor-definitions.md) | One structured authoring format, authority references, and verification criteria |
| [Authority Definitions](authority-definitions.md) | Typed requirements and decisions, identity, and authority-link validation |
| [Record Rendering](record-rendering.md) | First prototype: readable records and relationship navigation through mdBook |
| [Structural Checking](structural-checking.md) | Layout and navigation checks, execution flow, findings, and acceptance cases |

[Design Proposals](proposals/index.md) provides a place to discuss changes to
that goal state. Proposed content is distinguished from the accepted design.

The checker chapters now contain initial anchor definitions in the canonical
block format described above. They are drafts for review, grounded in the draft
requirements and decisions under Authority. None has implementation or executed
verification evidence yet.
