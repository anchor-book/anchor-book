# Introduction

anchor-book is an open-source engine for creating, maintaining, and checking
Anchor Books. An Anchor Book is a project's living system definition and working
context. It describes the design, the requirements and decisions that govern it,
and anchors: properties that must hold in the built system. Its implementation
roadmaps organize the agreed work needed to reach that goal.

This is the engine's own Book. It uses mdBook for structure and rendering and
will use the same engine interfaces provided to other projects.

## Reading this Book

- [Design](design/index.md) defines the intended system and its behavior.
- [Authority](authority/index.md) records governing requirements and decisions.
- [Implementation Roadmaps](implementation-roadmaps/index.md) organize the work
  agreed between engineers and AI agents, with completion criteria and evidence.

## Current state

The project is prototyping. Record parsing, authority validation, and mdBook
record rendering are implemented; other operations remain planned. See the
[roadmaps](implementation-roadmaps/index.md#current-roadmaps) for progress and evidence.
