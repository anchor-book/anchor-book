# Record Rendering

**Roadmap identifier:** `record-rendering`

**Status:** Proposed prototype milestones; implementation not started.

## Goal

Read and review this Book's anchors, requirements, and decisions as formatted,
linked content while keeping their structured TOML as the source. Use the
prototype to exercise and refine the schemas during Prototyping.

This is the next engine implementation increment. It can proceed alongside
[Engine Bootstrap](bootstrap.md), which still prepares the Book for checker
implementation. Prototype work does not require the whole checker design or
bootstrap readiness exercise to be complete. The engineer has selected this
direction; milestone details below are the proposed work agreement.

## Design basis

- [Record Rendering](../design/record-rendering.md) defines the adapter, reader
  presentation, validation behavior, and source preservation.
- [Anchor Definitions](../design/anchor-definitions.md) and
  [Authority Definitions](../design/authority-definitions.md) define the current schemas.
- [Rust Engine](../design/rust-engine.md) supplies package and API boundaries.
- [Readable records requirement](../authority/requirements.md#requirement-readable-records)
  and [shared rendering decision](../authority/design-decisions.md#decision-shared-record-rendering)
  govern [rendered records](../design/record-rendering.md#render-records-in-context)
  and [validated inventory](../design/record-rendering.md#render-validated-records).

All records remain drafts during this prototype. Assignment of an experiment
does not automatically accept its design or prove its properties.

## Preparation already done

The rendering design, component and sequence diagrams, presentation rules,
failure behavior, and acceptance criteria are drafted. One requirement, one
design decision, and two anchors establish the prototype's intended behavior.
The milestones below define the proposed implementation work, and the checker
roadmap now builds on the shared crate this prototype will establish.

The Book builds locally with these drafts. The records still display as TOML;
there is no Rust crate or record-rendering preprocessor yet. Scope review,
toolchain and dependency selection, implementation, and verification remain.

## Milestones

| Milestone | Outcome | Status | Depends on |
| --- | --- | --- | --- |
| [1](#record-rendering-1) | Engine code can read and validate the Book's records | Proposed | Current schema drafts and agreed prototype scope |
| [2](#record-rendering-2) | Readers can understand and navigate the rendered records | Proposed | Milestone 1 |
| [3](#record-rendering-3) | Local preview and CI use the same rendering path | Proposed | Milestone 2 |

<a id="record-rendering-1"></a>

## Milestone 1. Engine code can read and validate the Book's records

**Identifier:** `record-rendering-1`

**Status:** Proposed. **Depends on:** current schemas and agreement on prototype scope.

**Outcome:** A Rust library constructs a typed record inventory and reports
invalid definitions and relationships independently of rendering.

**Relevant anchor:** [Validated inventory](../design/record-rendering.md#render-validated-records).

**Tasks:**

- Establish the single library/CLI crate. Choose and record the edition, supported
  Rust version, mdBook protocol version, and focused parser dependencies.
- Implement Markdown block recognition with offsets, strict TOML decoding, typed
  records, global identity indexing, and authority-reference validation.
- Return located diagnostics and derive backlinks from the same inventory.
- Exercise the actual Book's records and independent positive/negative fixtures.

**Complete when:**

- The full current inventory is read with expected kinds, fields, and edges;
  chapter order does not affect resolution or diagnostic ordering.
- Canonical-format violations and invalid relationships from the design's
  acceptance criteria are rejected; examples are not collected as definitions.
- Engine compilation, formatting, Clippy, and library tests pass independently
  of the current Book's validity or generated output.
- No `anchorbook check` or configuration-reader capability is claimed.

**Completion record:** Pending.

<a id="record-rendering-2"></a>

## Milestone 2. Readers can understand and navigate the rendered records

**Identifier:** `record-rendering-2`

**Status:** Proposed. **Depends on:** Record Rendering — Milestone 1.

**Outcome:** mdBook displays records in their chapters with readable content,
status labels, permalinks, linked authorities, and derived backlinks.

**Relevant anchors:** [Rendered records](../design/record-rendering.md#render-records-in-context)
and [validated inventory](../design/record-rendering.md#render-validated-records).

**Tasks:**

- Add the thin `anchorbook md` protocol adapter and library rendering operation.
- Style the three kinds, status labels, criteria, and expandable decision details.
- Generate safe relative links and stable fragments, including print navigation.
- Remove the temporary record targets, duplicate titles, and manually maintained
  relationship lists when their generated replacements are in place.
- Exercise complete-input success, malformed protocol input, invalid records,
  and renderer capability probes through the actual CLI process.

**Complete when:**

- Every authored record renders with its expected content and links; all literal
  schema examples remain code snippets and no record is duplicated within a chapter.
- Independent output checks and visual review cover light/dark themes, a narrow
  viewport, keyboard access to details, and printed detail visibility.
- Links resolve across chapter depths and under the project site prefix and in
  the combined print view. Field text cannot inject active HTML or scripts.
- Passing and failing preprocessing preserve sources. Invalid input fails with
  diagnostics, without emitting a successful partial Book.
- Existing prose and Mermaid diagrams retain their behavior.

**Completion record:** Pending. Local rendering alone does not establish CI success.

<a id="record-rendering-3"></a>

## Milestone 3. Local preview and CI use the same rendering path

**Identifier:** `record-rendering-3`

**Status:** Proposed. **Depends on:** Record Rendering — Milestone 2.

**Outcome:** Contributors can build and preview the readable Book, and the GitHub
workflow builds the same output using the tested crate and locked dependencies.

**Relevant anchors:** [Rendered records](../design/record-rendering.md#render-records-in-context)
and [validated inventory](../design/record-rendering.md#render-validated-records).

**Tasks:**

- Configure `book.toml` and preprocessor ordering; document local setup and preview.
- Add independent engine checks and record rendering to CI, preserving the
  existing Pages output path. Do not add a placeholder structural check.
- Exercise a clean build and a live source-edit/rebuild cycle.
- Record actual GitHub Actions results for the implemented revision when that
  revision is pushed through the engineer's chosen contribution workflow.

**Complete when:** local builds and preview work, independently compiled engine
tests pass with a deliberately invalid Book, the Book build reports that invalid
input usefully, and the valid revision has successful CI evidence. Schema and
rendering limitations are documented. A local pass does not substitute for the
GitHub run; pushing remains a separately directed action.

**Completion record:** Pending.

## Handoff and later work

Record what the prototype teaches us about authoring, presentation, and schema
changes. Reconcile these changes with the design and records during the work.
Do not mark a milestone complete solely because it renders the happy-path Book.

[Structural Checking](structural-checker.md) will extend the same crate with the
configuration reader and layout/navigation checks. It can reuse record-model
infrastructure where applicable without invoking the renderer. Materialization,
generated contracts, cross-Book authority, evidence pages, and generated progress
indexes remain separate increments. No existing checker milestone is completed
or renumbered by this roadmap.
