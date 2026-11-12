# Record Rendering

**Roadmap identifier:** `record-rendering`

## Goal

Read this Book's anchors, requirements, and decisions as formatted, linked
content while retaining the canonical record sources. Use the prototype to refine the
schemas. This work can proceed alongside [Engine Bootstrap](bootstrap.md).

## Design basis

- [Record Rendering](../design/record-rendering.md) defines the adapter, reader
  presentation, validation behavior, and source preservation.
- [Anchor Definitions](../design/anchor-definitions.md) and
  [Authority Definitions](../design/authority-definitions.md) define the current schemas.
- [Rust Engine](../design/rust-engine.md) supplies package and API boundaries. Its
  [valid-authority anchor](../design/rust-engine.md#A-records-valid-authority), grounded
  in [traceable properties](../authority/requirements.md#R-traceable-properties),
  defines the authority checks to exercise explicitly in milestone 3.
- [Readable records requirement](../authority/requirements.md#R-readable-records)
  and [shared rendering decision](../authority/decisions.md#D-shared-record-rendering)
  govern [rendered records](../design/record-rendering.md#A-render-records-in-context)
  and [validated inventory](../design/record-rendering.md#A-render-validated-records).

Records remain drafts; assignment does not establish design acceptance.

## Scope boundaries

Milestone 3 covers the three behavioral criteria of `A-records-valid-authority`.
The [Formal Verification roadmap](formal-verification.md) owns its
`formal-authority` and `production-integration` criteria and the required-method
labels added by [criterion-method preparation](formal-verification.md#formal-verification-3).
Rendering evidence recorded here does not establish those criteria.

## Milestone 1. Engine code can read and validate the Book's records {#record-rendering-1}

**Outcome:** A Rust library constructs a typed record inventory and reports invalid definitions and relationships independently of rendering.

**Status:** Complete — local implementation and verification

**Dependencies:** current schemas and agreement on prototype scope.

**Design basis:** [Rendering design and governing records](#design-basis)

**Identifier:** `record-rendering-1`

**Tasks**

- [x] Establish the single library/CLI crate. Choose and record the edition, supported Rust version, mdBook protocol version, and focused parser dependencies.
- [x] Implement Markdown block recognition with offsets, strict TOML decoding, typed records, global identity indexing, and authority-reference validation.
- [x] Return located diagnostics and derive backlinks from the same inventory.
- [x] Exercise the actual Book's records and independent positive/negative fixtures.

### Completion criteria

#### Valid inventory {#record-rendering-1-valid-inventory}

Read the full current inventory without diagnostics, with every reference resolving to its kind and matching the derived backlinks; chapter order affects neither resolution nor diagnostic ordering.

- **Anchor coverage:** None — inventory acceptance
- **Evidence:** Passed: the separate `current_book` acceptance test reads all chapters listed in SUMMARY and checked 24 records against pinned identities, counts, and edges at the milestone revision. The test now asserts properties of any valid Book instead (references resolve to their kind and match backlinks, source ranges, and chapter-order independence) and passes for the current Book; independent fixtures check every record kind and field. See the milestone evidence for the tested work and commands.

#### Invalid inventory {#record-rendering-1-invalid-inventory}

Reject the canonical-format violations and invalid relationships specified by the anchor; literal examples are not collected.

- **Anchor coverage:** [A-render-validated-records](../design/record-rendering.md#A-render-validated-records) · `invalid-inventory` — Full
- **Evidence:** Passed: `tests/records.rs` rejects noncanonical fences, schema violations, duplicate identities, wrong-kind, missing, invalid, ambiguous, repeated, and incompatible-status references. Literal code examples are excluded. Tests check stable diagnostics, parser causes, Unicode positions, and truncation safety. This covers record validation; it does not verify rendering or the adapter.

#### Independent build {#record-rendering-1-independent-build}

Compile and run fixture tests with an invalid or unrendered Book; formatting, Clippy, and library tests pass.

- **Anchor coverage:** [A-render-validated-records](../design/record-rendering.md#A-render-validated-records) · `protocol-and-independence` — Partial: compilation and fixture-test independence
- **Evidence:** Passed: formatting on Rust 1.90; Clippy, independent tests, and documentation tests on Rust 1.85 and 1.90. `scripts/check-engine-independence.sh` runs a temporary source copy with no Book, then an invalid Book: independent tests pass both times, while explicit Book acceptance fails as expected. No renderer or mdBook build is invoked.

#### Declared scope {#record-rendering-1-declared-scope}

Document available operations without claiming a structural checker or configuration reader.

- **Anchor coverage:** None — delivery scope
- **Evidence:** Passed: Rust Engine and CONTRIBUTING describe the available library API and unsupported operations. Actual-process CLI tests verify help/version, exit status 2 for unsupported commands, and no misleading successful output. Configuration and structural checking are not implemented.

### Evidence {#record-rendering-1-evidence}

*Milestone evidence — local checks passed*

**Tested work:** Record Rendering milestone 1 working tree based on `7f45b14`, verified 2026-10-20.

**Checks:** `cargo fmt --all -- --check`; `cargo clippy --all-targets --locked -- -D warnings`; `cargo test --locked`; `cargo test --locked --test current_book -- --ignored`; and `scripts/check-engine-independence.sh`. Toolchain checks cover Rust 1.85.0 and 1.90.0. Independent tests include 12 record tests, 2 CLI tests, and 1 documentation example; Book acceptance is a separate test.

**Result and supporting records:** Local checks pass. Tests live in `tests/records.rs`, `tests/cli.rs`, and `tests/current_book.rs`; the isolation experiment is reproducible with the script above. The Engine workflow is configured for Rust 1.85 and stable.

**Limitations:** Local results only; remote runs are in milestone 3. This milestone establishes the parsing and validation portion of the rendering anchors, not their complete verification or design acceptance.

### Completion record

Complete locally on 2026-10-20. The library reads this Book and independent fixtures, rejects invalid input with located diagnostics, and builds independently of Book validity.

## Milestone 2. Readers can understand and navigate the rendered records {#record-rendering-2}

**Outcome:** mdBook displays records in their chapters with readable content, status labels, permalinks, linked authorities, and derived backlinks.

**Status:** Complete — local implementation and verification

**Dependencies:** Record Rendering — Milestone 1.

**Design basis:** [Rendering design and governing records](#design-basis)

**Identifier:** `record-rendering-2`

**Tasks**

- [x] Add the thin `anchorbook md` protocol adapter and library rendering operation.
- [x] Style the three kinds, status labels, criteria, and expandable decision details.
- [x] Generate safe relative links and stable fragments, including print navigation.
- [x] Remove the temporary record targets, duplicate titles, and manually maintained relationship lists when their generated replacements are in place.
- [x] Exercise complete-input success, malformed protocol input, invalid records, and renderer capability probes through the actual CLI process.

### Completion criteria

#### Record content {#record-rendering-2-record-content}

Every authored record appears once per chapter with expected fields and links; literal examples remain code.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `record-presentation` — Partial: inventory and field checks
- **Evidence:** Passed at the milestone 2 revision: the former `scripts/check-rendered-book.py` compared all 24 authored records with their rendered fields, counts one record per source chapter, and checks unique combined-print targets. Independent Rust fixtures verify all three kinds and literal examples. The complete input passes the real mdBook 0.5.2 build.

#### Accessible presentation {#record-rendering-2-accessible-presentation}

Review light/dark themes, narrow layout, keyboard access, and printed details.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `record-presentation` — Partial: visual and accessibility review
- **Evidence:** Passed: browser review of Light/Auto and Coal themes, a 390-pixel viewport with navigation collapsed, and keyboard Enter on the decision disclosure. No horizontal overflow was observed. Combined-print review showed all six decision details expanded. A disposable copy disabled only the automatic print-dialog call for inspection; the generated print content and local print helper were unchanged. Physical printer output was not tested.

#### Relationship navigation {#record-rendering-2-relationship-navigation}

Links resolve across chapter depths, the project site prefix, and the combined print view.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `relationship-navigation` — Partial: nested chapters and relative routes through the adapter, which linked tests exercise; the project site prefix and the combined print view were checked in a browser and by rendered-Book acceptance, which reads previously built output and so is not linked as evidence
- **Evidence:** Passed: generated local links and fragments resolve; browser navigation followed authority links under an `/anchor-book/` project prefix and within the combined print page. All generated record links in print resolve to that page's record fragments. Independent nested-route fixtures cover differing source/output paths and percent-encoded Unicode and spaces.

#### Source preservation {#record-rendering-2-source-preservation}

Passing and failing preprocessing preserve sources; field text stays inert and unrelated prose and Mermaid diagrams retain their behavior.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `preservation-and-escaping` — Full
- **Evidence:** Passed: source hashes match before and after successful and deliberately failed mdBook builds. Process and library fixtures preserve unrelated prose, Mermaid input, examples, and navigation metadata. Script/tag-like fields remain escaped text. Temporary authoring targets, duplicate headings, and manual backlinks were deliberately removed in this implementation migration.

#### Adapter protocol {#record-rendering-2-adapter-protocol}

Exercise the actual process with valid and malformed input and renderer probes. Success emits complete Book JSON; failure emits diagnostics and no successful partial Book.

- **Anchor coverage:** [A-render-validated-records](../design/record-rendering.md#A-render-validated-records) · `protocol-and-independence` — Partial: adapter protocol; independent build is addressed in milestone 1
- **Evidence:** Passed: five rendering/protocol tests exercise the actual CLI, complete JSON output, nested/unnumbered chapters, preserved metadata, malformed JSON and chapter shapes, invalid versions/routes/records/references, and supported/unsupported renderer probes. Failure returns nonzero with stderr diagnostics and no success JSON.

### Evidence {#record-rendering-2-evidence}

*Milestone evidence — local checks passed*

**Tested work:** Milestone 2 working tree based on `b405bc1`, verified locally on 2026-10-20 before committing.

**Checks and results:** Formatting; Clippy and the independent suite on Rust 1.85 and 1.90; separate current-Book acceptance; API documentation; absent/invalid-Book independence; successful and deliberately failing mdBook builds with unchanged source hashes; rendered record/field/link checks; and browser presentation/navigation review all passed. The independent suite contains 12 record tests, 5 rendering/protocol tests, 2 CLI tests, and a documentation example.

**Supporting records:** `tests/rendering.rs`, `tests/current_book.rs`, the since-replaced `scripts/check-rendered-book.py`, and `scripts/check-engine-independence.sh`.

**Integration and limits:** Local `book.toml` uses the adapter after link expansion and before Mermaid so the engineer can review the result. Only mdBook 0.5.2 HTML is supported. Fields render as escaped text; other renderer/version support and Markdown inside fields are outside this prototype.

### Completion record

Complete locally on 2026-10-20: records render with preserved identities, fields, relationships, accessible details, and failure behavior.

## Milestone 3. Local preview and CI use the same rendering path {#record-rendering-3}

**Outcome:** Contributors can build and preview the readable Book, and the GitHub workflow builds the same output using the tested crate and locked dependencies.

**Status:** Complete

**Dependencies:** Record Rendering — Milestone 2.

**Design basis:** [Rendering design and governing records](#design-basis)

**Identifier:** `record-rendering-3`

**Tasks**

- [x] Configure `book.toml` and preprocessor ordering; document local setup and preview.
- [x] Review and complete independent test coverage for the `required-authority`, `resolved-authority`, and `compatible-status` criteria of `A-records-valid-authority`; record the exact tests and remaining gaps.
- [x] Make Book rendering and Pages publication depend on passing engine checks and current-Book acceptance for the same revision. [Structural Checking milestone 3](structural-checker.md#structural-checking-3) owns the shared implementation and evidence, now merged with GitHub runs. Preserve `target/anchorbook`.
- [x] Run the Rust `rendered_book` acceptance test in CI after building with the pinned mdBook version, including authority links and combined-print targets.
- [x] Exercise a clean build and a live source-edit/rebuild cycle.
- [x] Record actual GitHub Actions results for the implemented revision when that revision is pushed through the engineer's chosen contribution workflow.

### Completion criteria

#### Authority coverage {#record-rendering-3-authority-coverage}

Independent tests exercise the three behavioral authority criteria through the record API; invalid authority also fails the actual preprocessor without successful partial output. Record the tests and results for the implementation revision.

- **Anchor coverage:** [A-records-valid-authority](../design/rust-engine.md#A-records-valid-authority) · `required-authority`, `resolved-authority`, `compatible-status` — Full
- **Evidence:** Passed locally on Rust 1.88.0 and stable 1.99.1. Review found that earlier tests exercised most authority edges through a decision's requirements rather than an anchor's authorities. `tests/authority_criteria.rs` now covers each criterion through the record API with fixtures built from the criteria: `required-authority` accepts an anchor citing only a requirement or only a decision and rejects a missing or empty list with a located invalid-definition diagnostic; `resolved-authority` rejects missing, wrong-kind, invalid, and ambiguous anchor targets, each at the reference's source text and with no inventory; `compatible-status` checks all eight combinations of anchor status, target status, and target kind. `tests/rendering.rs` `anchor_authority_failures_reach_the_real_adapter` shows a missing anchor authority and an accepted anchor citing a draft decision failing the actual `anchorbook md` process with no Book JSON. Disabling the wrong-kind check, and separately the status check, in `src/records/authority.rs` each failed a new test; the source was restored. Mutation tools do not reach these `verus!` bodies, so those two manual changes are the sensitivity evidence. Existing `tests/records.rs` cases remain for decision edges, duplicates, and repeated references.

#### Local preview {#record-rendering-3-local-preview}

Clean builds and live source-edit/rebuild cycles use the record adapter successfully.

- **Anchor coverage:** None — contributor workflow
- **Evidence:** Passed locally with mdBook 0.5.4 on a disposable copy of this Book. After removing `target/anchorbook`, `mdbook build` rendered every chapter with record and milestone fragments. Under `mdbook watch`, a prose edit rebuilt within 2 seconds; an invalid record status failed the rebuild with located `record.` diagnostics while the previous output remained; restoring it and editing again rebuilt within 2 seconds. Limits: the engine binary was already built, so the clean build measured Book output only; `mdbook serve` and browser live reload were not exercised, since `watch` runs the same preprocessor without a network listener.

#### Independent CI {#record-rendering-3-independent-ci}

Engine compilation and fixture tests remain runnable with an invalid Book; separate current-Book acceptance reports invalid input usefully and must pass before rendering and publishing the same revision.

- **Anchor coverage:** [A-render-validated-records](../design/record-rendering.md#A-render-validated-records) · `protocol-and-independence` — Partial: independence and failure reporting in CI
- **Evidence:** [Structural Checking milestone 3](structural-checker.md#structural-checking-3-evidence) owns the shared gate implementation and evidence. Book now calls the reusable Engine workflow and requires its success before rendering; the Anchor Book preprocessor checks structure before rendering, so ordinary `mdbook build` refuses a failed or incomplete check. Local failure experiments pass. On GitHub, valid main revisions ran Engine checks before Build mdBook, and deliberately broken Books in throwaway PR #9 failed acceptance with useful diagnostics while independent tests passed and rendering was skipped. The evidence is recorded there, and that milestone is accepted.

#### Published build {#record-rendering-3-published-build}

Record passing engine, current-Book, rendered-output, and Pages deployment results for the same implementation revision; review the published cards and authority links under the project URL. A local pass is insufficient.

- **Anchor coverage:** None — CI integration
- **Evidence:** For merged revision `03fb981`, [Engine](https://github.com/anchor-book/anchor-book/actions/runs/39737287093) and [Book build, rendered acceptance, and Pages deployment](https://github.com/anchor-book/anchor-book/actions/runs/39737287245) passed. Published-site review on 2026-11-10 of the site deployed from `main` after #27 (last modified Tue, 10 Nov 2026 05:20:39 GMT): a crawl of `https://anchor-book.github.io/anchor-book/` fetched all 32 pages, including `print.html`, and resolved all 2,170 local links and their fragments under the project prefix; the pages show 54 record cards and 31 milestone cards, and authority links are relative, such as `../authority/requirements.html#requirement-traceable-properties`.

#### Documented limits {#record-rendering-3-documented-limits}

Document setup, supported versions, and schema/rendering limits. Pushing remains a separately directed action.

- **Anchor coverage:** None — contributor documentation
- **Evidence:** Setup, supported versions, and schema/rendering limits are documented in CONTRIBUTING and the rendering design, including the engine-check gate.

### Evidence {#record-rendering-3-evidence}

*Milestone evidence — CI and publication passed; remaining gaps recorded*

The Rust `rendered_book` acceptance test runs in CI after the build and before artifact upload and publication; local negative experiments reject damaged rendered output. For merged revision `03fb981`, the [Engine](https://github.com/anchor-book/anchor-book/actions/runs/39737287093) and [Book](https://github.com/anchor-book/anchor-book/actions/runs/39737287245) workflows passed, including rendered acceptance and Pages deployment. The engine-check gate's runs are recorded under [Structural Checking milestone 3](structural-checker.md#structural-checking-3-evidence). The published site was reviewed on 2026-11-10; see the published-build criterion.

### Completion record

Complete on 2026-11-10: authority coverage, local preview, CI ordering, publication, rendered acceptance, and the published-site review all have evidence.

## Milestone 4. Roadmaps render from plain Markdown {#record-rendering-4}

**Outcome:** The Anchor Book preprocessor recognizes roadmap milestones written in the [roadmap format](index.md#defining-the-work), validates their identifiers and anchor references, and renders each milestone as a card with collapsible tasks, criteria, and evidence, while agents keep reading the compact Markdown source.

**Status:** Complete

**Identifier:** `record-rendering-4`

**Dependencies:** Record Rendering milestone 2 and the converted roadmaps.

**Design basis:** [Roadmap rendering](../design/record-rendering.md#roadmap-rendering), [A-render-roadmap-milestones](../design/record-rendering.md#A-render-roadmap-milestones), and the [roadmap format](index.md#defining-the-work).

**Tasks**

- [x] Define the governing anchor and its criteria, with its authority.
- [x] Recognize milestone and criterion headings with their fragments, the field paragraphs, task lists, and anchor-coverage items in the record library, with located diagnostics for malformed milestones.
- [x] Validate that milestone and criterion identifiers are unique and that anchor-coverage references resolve to existing anchors and criteria.
- [x] Render the collapsible view in the preprocessor, readable without JavaScript and expanded when printing, replacing nothing in the source.
- [x] Exercise this Book's roadmaps and independent fixtures; record results.

### Completion criteria

#### Validated roadmaps {#record-rendering-4-validated-roadmaps}

Independent fixtures with duplicate milestone or criterion identifiers, malformed fields, and anchor-coverage references to missing anchors or criteria fail preprocessing with located diagnostics; this Book's roadmaps pass.

- **Anchor coverage:** [A-render-roadmap-milestones](../design/record-rendering.md#A-render-roadmap-milestones) · `roadmap-validation` — Full
- **Evidence:** Passed locally: `tests/roadmaps.rs` and the module's unit tests cover malformed milestone and criterion headings, missing fields, a mismatched identifier, an unknown status, duplicates across chapters located in the repeating chapter, coverage citing a missing anchor, a requirement, or a missing criterion, section ends, and examples in code blocks. All of this Book's roadmaps pass.

#### Rendered view {#record-rendering-4-rendered-view}

This Book's rendered roadmaps show criterion details that expand and collapse, are readable without JavaScript, and print expanded; every existing fragment still resolves, and the Markdown sources are unchanged.

- **Anchor coverage:** [A-render-roadmap-milestones](../design/record-rendering.md#A-render-roadmap-milestones) · `roadmap-presentation` — Full
- **Evidence:** Tests passed locally: the actual `anchorbook md` process renders the roadmaps role and its nested chapters, leaves other chapters and source files unchanged, and refuses an invalid roadmap with no output; rendering tests assert a complete milestone card, explained statuses, task progress, evidence, and that each card closes before the next opens. This Book renders 28 milestone cards with balanced sections; every local link resolves and the print view has no duplicate identifiers. Source review on 2026-11-10 of this branch's rendered roadmaps: tasks, criteria, and milestone evidence are native `<details>` elements rendered `open`, so they read without JavaScript and collapse through their `<summary>`, which browsers make keyboard-operable; `theme/records.css` gives summaries a visible focus outline. Criterion headings sit inside their summaries with their fragment `id`, so a link still reaches a collapsed criterion; `record-rendering-4-rendered-view` resolves once in its chapter and once in `print.html`. Printing expands closed sections through `theme/records.js` on `beforeprint` and in `print.html`, restoring the reader's choices afterwards, and through a print rule on `::details-content` for browsers that support it. A heading inside `<summary>` may lose its heading role in some screen readers. These styles and the print script have since moved into the engine, in `src/rendering/assets/`, and are delivered inline with each rendered chapter, unchanged except for a guard that runs the script once per page. The published site renders these cards; see [Record Rendering milestone 3](#record-rendering-3-published-build).

### Evidence {#record-rendering-4-evidence}

**Revision and checks:** Working tree based on `0baee60`, checked 2026-11-09. Formatting, Clippy, and 95 tests pass on Rust 1.88.0 and stable; the structural check, current-Book acceptance, `mdbook build`, rendered acceptance, and `scripts/check-engine-independence.sh` pass.

**Mutation testing:** cargo-mutants 27.1.0 with `--jobs 2 --build-timeout 120` on `src/roadmaps.rs`, `src/rendering.rs`, `src/rendering/html.rs`, `src/adapters/mdbook.rs`, and `src/main.rs`. The first run of 204 mutations found 12 missed and 4 timeouts, all in the new code: untested section ends, multiple milestones per chapter, duplicate locations in a differing chapter, and the error source, plus loops whose index arithmetic could hang. Tests were added, the loops rewritten as iterators, and an ineffective match guard removed. The repeat run tested 182 mutations: 163 caught, 19 unviable, none missed or timed out. After milestone cards were added, `src/roadmaps.rs`, `src/rendering.rs`, and `src/rendering/html.rs` were tested again: 166 mutations, 148 caught, 18 unviable, none missed or timed out; 96 tests pass on Rust 1.88.0 and stable.

### Completion record

Complete on 2026-11-10: roadmap validation and rendering have test evidence and a source review of the rendered markup. The in-browser review was dropped from `roadmap-presentation` as a manual check.

## Milestone 5. Readers can collapse anchor records {#record-rendering-5}

**Outcome:** The shared renderer supplies anchors as initially collapsed records. Readers can expand their fields while retaining identity and status, and printed Books include every field.

**Status:** Complete — local implementation and verification

**Identifier:** `record-rendering-5`

**Dependencies:** Record Rendering milestone 2; the engineer requested collapsible anchors as a presentation refinement.

**Design basis:** [Reader presentation](../design/record-rendering.md#reader-presentation) and [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context). Earlier milestones retain their tested scope; this milestone covers the additional disclosure behavior.

**Tasks**

- [x] Render each anchor with an initially closed native disclosure controlled by its heading; retain the title, identifier, and acceptance status when closed.
- [x] Keep property, authority links, criteria, and derived relationships inside that disclosure; deliver its styles from the engine.
- [x] Preserve record fragments, escaped fields, and the existing print expansion and restoration behavior.
- [x] Exercise the actual adapter with an independent fixture.
- [x] Review light, dark, narrow, keyboard, no-JavaScript, permalink, and print behavior; run the supported Rust and Book checks.

### Completion criteria

#### Disclosure behavior {#record-rendering-5-disclosures}

Anchors start collapsed. Clicking the heading or pressing Enter or Space on it toggles all fields while title, identity, and acceptance status remain visible. This works without JavaScript and remains readable in light, dark, and narrow layouts.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `record-presentation` — Partial: collapsible anchor presentation and interaction
- **Evidence:** The independent `anchor_headers_control_complete_collapsed_details` fixture passes through the actual `anchorbook md` process and checks initially closed native details, summary metadata, fields, inert title text, and isolation from other records. Chromium browser review passes in Light and Coal at 1440 pixels and Light at 390 pixels: clicking, Enter, and Space toggle the fields while the title, status, and permalink remain visible. The narrow case also passes with JavaScript disabled; no document overflow was observed.

#### Navigation and printing {#record-rendering-5-navigation-print}

The existing anchor fragment and permalink still identify its visible heading when collapsed. Printing exposes all fields and restores closed records afterward; the combined print view contains the complete records.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `relationship-navigation` — Partial: anchor permalinks and combined-print targets; `record-presentation` — Partial: disclosure printing
- **Evidence:** Browser checks pass: the collapsed permalink navigates to the unchanged record fragment without toggling it; before-print expansion reveals all fields, and after-print restoration returns the closed records to their prior state. Print-view expansion waits for all chapters to load; the combined print page contains open anchor details, including records after the first presentation block, and a unique target per anchor. Rendered-Book acceptance passes for all record fields and local links. Physical printer output and other browser engines were not tested.

#### Shared delivery {#record-rendering-5-shared-delivery}

The library and actual adapter supply the disclosure and its presentation without consumer theme files or source changes. Other record kinds retain their presentation, and supported-toolchain and Book checks pass.

- **Anchor coverage:** [A-render-records-in-context](../design/record-rendering.md#A-render-records-in-context) · `preservation-and-escaping` — Partial: anchor disclosure output and escaped heading
- **Evidence:** The 14-test rendering suite passes, including source preservation, unrelated content, escaping, and the actual adapter boundary. Formatting and Clippy pass. All 167 independent tests, including the 14 rendering tests and doctests, pass on Rust 1.88.0 and stable, with eight optional acceptance tests ignored. Structural checking, the Book build, and all five current/rendered Book acceptance tests pass separately. The presentation remains embedded in the engine; no consumer theme files are required.

### Mutation validation

The [initial PR campaign](https://github.com/anchor-book/anchor-book/actions/runs/43157732566) at `7b721dff4a6a1c8920596b7c5975386df5cd70c9` tested four changed-line mutations: three caught, one missed, no timeouts or unviable changes. The survivor inverted the condition that closes an anchor's disclosure in `record`. Standalone HTML parsing repaired the malformed output, hiding the defect from the fixture assertions.

`record_disclosures_preserve_surrounding_details` now places all three record kinds inside an authored disclosure and checks that the records and following inner prose remain inside it, while subsequent outer prose stays outside. An extra closing tag closes that authored group early. This exercises the observable containment guarantee without relying on a parser to report malformed HTML. No production behavior, assertions, or mutation exclusions were weakened. Revision-specific rerun results are tracked in [PR #39](https://github.com/anchor-book/anchor-book/pull/39).

### Completion record

Complete locally on 2026-11-12 in the working tree based on `15177478934198254815784fb077e169c3ad5508`. The shared renderer supplies the disclosure; independent adapter tests, supported-toolchain checks, Book acceptance, and the browser checks above pass. Commands: `cargo fmt --all -- --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked` (Clippy and tests also with `+1.88.0`), `cargo run --quiet --locked --bin anchorbook -- check --config anchorbook.toml`, `mdbook build`, and `cargo test --locked --test current_book --test rendered_book -- --ignored`. Rendered acceptance used a separate build destination to avoid the live preview rebuilding its output during the check. The work is on `codex/book-guards-and-collapsible-anchors`, based on that revision, and is not integrated into main. No local mutation campaign was run; required GitHub checks, including mutation testing, must pass before merge readiness is established.

## Handoff and later work

Reconcile prototype findings with design and records under the
working contract (`anchorbook contract`). Happy-path rendering alone does
not complete a milestone.

[Structural Checking](structural-checker.md) extends this crate with configuration
and layout/navigation checks, reusing record infrastructure without invoking the
renderer. See [later work](index.md#later-work-retained-from-the-design) for deferred
capabilities. No existing checker milestone is completed
or renumbered by this roadmap.
