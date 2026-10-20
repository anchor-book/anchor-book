# Introduction

anchor-book is an open-source engine for creating, maintaining, and checking
Anchor Books. An Anchor Book holds a project's living system definition and
working context: its design, the requirements and decisions that govern it,
the properties that must hold, and the path toward implementation.

This is the Book for the anchor-book engine itself. It uses mdBook for structure
and rendering. As the engine develops, it will manage this Book through the same
interfaces it provides to other projects.

## Reading this Book

- [Design](design/index.md) describes the intended system, starting with a
  system overview and developing component responsibilities and behavior.
- [Authority](authority/index.md) records the requirements and decisions that
  establish why properties bind the system.
- [Implementation Roadmaps](implementation-roadmaps/index.md) turn sufficiently
  clear design into AI-proposed implementation work that we review and agree on
  before AI agents carry it out.

## Current state

The project is prototyping. The Book contains an initial system design, structured
anchor and authority definitions, and designs for record rendering, configuration,
and structural checking, marked as drafts for review. The first Rust library
reads and validates structured records and their authority relationships.

Rendering and operational commands are still pending. The
[Record Rendering](implementation-roadmaps/record-rendering.md),
[Engine Bootstrap](implementation-roadmaps/bootstrap.md), and
[Structural Checking](implementation-roadmaps/structural-checker.md) roadmaps are now
in the Book, with completion evidence and proposed work kept distinct.
