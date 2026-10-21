# Design Decisions

**Status: draft for review.** These choices constrain other parts of the design.

```anchorbook-decision
schema-version = 1
id = "decision-explicit-configuration"
title = "Configuration defines the Book's layout and roles"
status = "draft"
choice = "Define Book paths and section roles in anchorbook.toml. Resolve Book paths relative to that configuration and section paths relative to the source."
requirements = ["requirement-configured-structure"]
rationale = "Checking, initialization, adoption, and materialization need the same intended layout without relying on fixed directory names or display titles."
alternatives = ["Conventional paths reduce configuration, but constrain existing projects and make directory names determine their roles."]
reconsider-when = "Books spanning multiple roots require a different path-resolution model."
```

```anchorbook-decision
schema-version = 1
id = "decision-mdbook-structure"
title = "mdBook navigation defines the chapter hierarchy"
status = "draft"
choice = "Use SUMMARY.md as the chapter hierarchy for checking, materialization, adoption, and rendering; do not maintain a separate Anchor Book hierarchy."
requirements = ["requirement-configured-structure", "requirement-truthful-results"]
rationale = "All operations must agree on the chapters readers see. Reusing mdBook navigation keeps structural checks and managed changes aligned with rendering."
alternatives = ["An independent navigation manifest offers more control but requires synchronization with mdBook."]
reconsider-when = "Supporting a different book format requires a hierarchy that mdBook cannot express."
```

```anchorbook-decision
schema-version = 1
id = "decision-separate-check-and-repair"
title = "Checking and mutation are separate operations"
status = "draft"
choice = "Checking observes and reports. Repairs and managed writes belong to separate agent actions or engine operations."
requirements = ["requirement-observation-only", "requirement-repair-feedback"]
rationale = "CI and callers can inspect defects without changing their inputs. Materialization and repair must define their own ownership and conflict behavior."
alternatives = ["Repairing during checking shortens the workflow but makes inspection mutate the state it reports."]
reconsider-when = "A combined user workflow is needed; preserve explicit observation and mutation boundaries within it."
```

```anchorbook-decision
schema-version = 1
id = "decision-typed-check-results"
title = "Shared behavior and typed results belong in the Rust library"
status = "draft"
choice = "Keep shared engine behavior and typed results in the Rust library. The CLI and other adapters translate invocation and presentation without defining separate rules."
requirements = ["requirement-shared-checker", "requirement-truthful-results", "requirement-repair-feedback"]
rationale = "CLI commands, mdBook integration, Rust tests, and future adapters must use consistent behavior and inspect results without parsing display text."
alternatives = ["A CLI-only implementation is simpler to expose but requires Rust callers and other adapters to invoke a process and interpret its output."]
reconsider-when = "Deployment or language requirements call for a different shared interface; retain one owner of engine behavior."
```

```anchorbook-decision
schema-version = 1
id = "decision-shared-record-rendering"
title = "Validation and rendering use one record model"
status = "draft"
choice = "Parse canonical definitions into one shared record inventory. Validation, rendering, and derived links use its identities and relationships; adapters do not define separate interpretations."
requirements = ["requirement-readable-records"]
rationale = "Authority resolution, backlinks, future progress views, and additional renderers must agree on what each record means."
alternatives = ["Separate presentation models permit independent evolution but require synchronization with validation."]
reconsider-when = "Another output format needs its own view; derive that view from the shared inventory rather than redefine record semantics."
```
