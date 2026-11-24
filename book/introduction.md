# Introduction

anchor-book is an open-source engine for creating, maintaining, and checking
Anchor Books. An Anchor Book is a project's living system definition and working
context. It describes the design, the requirements and decisions that govern it,
and anchors: properties that must hold in the built system. Its implementation
roadmaps organize the agreed work needed to reach that goal.

This is the engine's own Book. It uses mdBook for structure and rendering and
will use the same engine interfaces provided to other projects.

## Using Anchor Book

You can start by asking an agent: “I'm new to Anchor Book. Explain how we should
use it in this project and recommend the next step.” The
[user guide](design/user-guidance.md) explains the working loop and gives prompts
for starting, designing, porting, implementing, checking progress, and resuming.
Your project's agent receives the conversational guidance through the engine's
working contract and applies it to the current Book.

## Reading this Book

[Working on Anchor Book](working-on-anchor-book.md) identifies this repository's
phase, source locations, authoring references, and development instructions.

- [Design](design/index.md) defines the intended system and its behavior.
- [Authority](authority/index.md) records governing requirements and decisions.
- [Implementation Roadmaps](implementation-roadmaps/index.md) organize the work
  agreed between engineers and AI agents, with completion criteria and evidence.

## Current state

The [roadmaps](implementation-roadmaps/index.md#current-roadmaps) record what is
implemented, remaining work, and evidence.
