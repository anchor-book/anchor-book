# Decisions

Each decision states its own status. These choices govern the system being built; other parts of the Book must account for them.

```anchorbook-decision
schema-version = 1
id = "decision-explicit-configuration"
title = "Configuration defines the Book's layout and roles"
status = "accepted"
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
status = "accepted"
choice = "Use SUMMARY.md as the chapter hierarchy for checking, materialization, adoption, and rendering; do not maintain a separate Anchor Book hierarchy."
requirements = ["requirement-configured-structure", "requirement-truthful-results"]
rationale = "All operations must agree on the chapters readers see. Reusing mdBook navigation keeps structural checks and managed changes aligned with rendering."
alternatives = ["An independent navigation manifest offers more control but requires synchronization with mdBook."]
reconsider-when = "Supporting a different book format requires a hierarchy that mdBook cannot express."
```

```anchorbook-decision
schema-version = 1
id = "decision-separate-check-and-repair"
title = "Checking and repair are separate operations"
status = "accepted"
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
status = "accepted"
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

```anchorbook-decision
schema-version = 1
id = "decision-native-tool-integration"
title = "Anchor Book works through the project's existing tools"
status = "draft"
choice = "Integrate through the extension points of tools a project already uses, such as mdBook preprocessors, Cargo, and CI steps, so their ordinary commands apply Anchor Book checks and rendering. Do not require wrapper scripts or replacement commands."
requirements = ["requirement-project-adoption"]
rationale = "Adopting projects keep their existing commands, editors, and CI. A check enforced only by a wrapper is skipped whenever someone runs the underlying tool directly, so the check must run where the tool runs."
alternatives = ["A wrapper command can sequence checks and rendering without changing tool configuration, but it is easy to bypass and every repository must adopt and maintain it."]
reconsider-when = "A tool offers no suitable extension point or its protocol cannot express a required check; record the gap and keep any wrapper minimal."
```

```anchorbook-decision
schema-version = 1
id = "decision-verify-runs-linked-checks"
title = "Verify runs the linked checks through the project's tools"
status = "draft"
choice = "The verify operation selects linked test functions and the complete proof targets required by criteria in scope, runs them through supported runners of the project's own tools, and reads that run's reports. Test selection is exact; proof selection covers whole declared crate targets and the report states the executed scope. The operation does not replace the project's test commands or CI jobs."
requirements = ["requirement-criterion-evidence"]
rationale = "Agents in the build loop need one command that gives current results for the anchor they are changing. Anchor Book knows which functions address which criteria, so it adds selection, tool execution, and association of results with current inputs. Complete proof-target runs preserve the verifier's declared scope, while input identities prevent cached or changed inputs from being reported as fresh evidence."
alternatives = ["Verify reads reports produced by separate runs, which avoids invoking tools but needs several steps in the build loop and recorded revisions to tell current results from old ones.", "Agents run the whole suite and read its output, which loses the link from results to criteria."]
reconsider-when = "A project needs evidence from runs verify cannot invoke, such as tests on other platforms; add reading of supplied reports as a separate input rather than removing selected runs."
```

```anchorbook-decision
schema-version = 1
id = "decision-completion-claims-checked"
title = "Complete milestones claim their fully covered criteria, and verify checks the claim"
status = "draft"
choice = "A roadmap milestone with status Complete claims every anchor criterion its completion criteria cover in full. verify fails the run when a claimed criterion in scope is not satisfied, and lists claims it could not check. Partial or no coverage, and milestones in other states, claim nothing."
requirements = ["requirement-criterion-evidence"]
rationale = "Whether a criterion has linked tests depends on someone adding them, so absence must be caught where it matters: when the Book records work as done. Tying the check to Complete milestones uses a claim the Book already makes, keeps work in progress as a report rather than a failure, and makes a later regression in that evidence visible."
alternatives = ["Every test must carry a link or an explicit exemption, which catches unlinked tests directly but is noisy for projects with many existing tests; kept as a possible opt-in.", "A separate completion marker on anchors or design areas, which is more precise but adds a field the Book does not have.", "Planned coverage stays informational, which avoids failures but lets completion be recorded with no evidence."]
reconsider-when = "Completion is recorded somewhere other than roadmap milestones, or claims need finer scope than a milestone's criteria."
```

```anchorbook-decision
schema-version = 1
id = "decision-evidence-links-in-code"
title = "Evidence links live beside the code they describe"
status = "draft"
choice = "A test or proof links to the anchor criteria it addresses with a doc-comment line on the function, /// anchorbook: <anchor-id>#<criterion-id>; the engine derives each criterion's linked functions rather than listing them in the Book."
requirements = ["requirement-criterion-evidence"]
rationale = "Links move with renamed or relocated tests, the Book holds only criteria and stays small, and the existing proof module already names its anchor and criterion beside the code. A doc comment is an attribute of the item it precedes, so a Rust parser attaches the link to its function directly, without inferring which item follows a line, and the project needs no dependency."
alternatives = ["Criteria list their tests in the Book, which is visible to readers without the engine but grows the Book and drifts whenever tests are renamed.", "An ordinary // comment, which Rust parsers discard, so the engine must infer from line positions which function a link belongs to.", "An attribute such as #[anchorbook::criterion(...)], which reads well but needs an engine proc-macro crate as a project dependency, since stable Rust rejects unknown tool attributes."]
reconsider-when = "A supported language or tool cannot carry comments next to its tests or proofs; define its link form without moving links into the Book."
```

```anchorbook-decision
schema-version = 1
id = "decision-identifier-kind-codes"
title = "Identifiers carry a kind code and a descriptive name"
status = "draft"
choice = "Every anchor, requirement, and decision identifier is a kind code (A, R, or D), a hyphen, and a name that starts with a letter and states the record's meaning. The identifier is the only place the kind is stated: headings and references consist of the code-spanned identifier, with no kind word. Criteria have names without a code, unique within their anchor. The engine enforces the form and the codes each reference field accepts; review judges whether a name describes its record."
requirements = ["requirement-searchable-records"]
rationale = "Identifiers appear without their records in references, evidence links, fragments, diagnostics, and search results, so an agent must recognize a record and its kind from the identifier alone. A two-character code keeps the kind visible everywhere at little length and makes every mention of a kind findable with one search, such as a code span starting with D-. Requiring the name to start with a letter rules out sequence numbers, which say nothing about their records. Stating the kind once, in the identifier, gives every reference one spelling."
alternatives = ["Full-word prefixes such as decision-, which are self-describing but add nine to twelve characters and, with a kind word in references, state the kind twice.", "Names with no kind marker, which are shortest but leave the kind invisible in fragments, evidence links, and prose.", "Sequential identifiers such as D-12, which are short and easy to allocate but carry no meaning, so every reference must be looked up."]
reconsider-when = "A new record kind needs a code that would be ambiguous with an existing one, or descriptive names collide often enough to need a different scheme; keep names descriptive whatever the form."
```

```anchorbook-decision
schema-version = 1
id = "decision-library-cli-entry"
title = "The library provides the CLI entry point and projects declare the engine version"
status = "draft"
choice = "The library exposes the command-line entry point, and the published anchorbook binary is a thin wrapper around it. A project runs the engine either through a small binary in its own workspace or through an installed binary. It declares one exact engine version in anchorbook.toml using an exact Cargo version requirement, and every entry point stops before evaluating when its version differs. Missing, malformed, or non-exact declarations make evaluation incomplete."
requirements = ["requirement-project-adoption"]
rationale = "A dependency provides only the library, while mdBook, CI, and agents invoke a binary, and Cargo does not run binaries belonging to dependencies. A workspace binary makes the project's Cargo.lock the single version source for tests and commands; an installed binary suits projects that prefer no extra workspace member. An exact declaration keeps either from silently running different rules than the project's tests."
alternatives = ["Installed binary only, which keeps the project unchanged but leaves the installed version and the locked library version free to diverge.", "Workspace binary only, which gives one version source but requires every project to add and maintain a workspace member.", "A compatible version range allows entry points to run different engine versions and therefore cannot guarantee identical rules."]
reconsider-when = "Cargo can run binaries provided by dependencies, which would make a project-local wrapper unnecessary."
```

```anchorbook-decision
schema-version = 1
id = "decision-isolate-engine-verification"
title = "Engine proofs stay inside the engine; projects verify with their own Verus"
status = "draft"
choice = "The engine's own proofs, vstd, and Verus toolchain are excluded from what consuming projects build. Verifying a project's code runs the Verus installation and verification toolchain that the project configures."
requirements = ["requirement-project-adoption", "requirement-project-verification"]
rationale = "Projects verifying their own code choose their own Verus and vstd versions, which are pinned exactly and change frequently. An exact vstd requirement in the engine's dependency graph would conflict with them or force the engine's choice, and a verification toolchain must not replace a project's build toolchain."
alternatives = ["Projects use the engine's pinned Verus and vstd, which avoids version handling in the engine but forces every project onto the engine's release cadence.", "The engine bundles a Verus installation, which simplifies setup but still dictates the version and adds a large download to every installation."]
reconsider-when = "Verus and vstd provide compatible version ranges across releases, so a shared requirement no longer forces a single version."
```

```anchorbook-decision
schema-version = 1
id = "decision-assessment-records-as-files"
title = "Assessment records are files beside the code, outside the Book"
status = "draft"
choice = "A specification assessment or a review is recorded as a file in an assessment directory configured in anchorbook.toml, naming the criterion, the subject, the fingerprints of what was examined, the conclusion, the reviewer, and the revision. verify reads these records and treats one whose fingerprints no longer match as stale; it never writes them."
requirements = ["requirement-criterion-evidence", "requirement-proofs-establish-criteria"]
rationale = "Assessments and reviews are evidence, so they stay out of authored Book sources, which hold the goal rather than results. Files go through the project's ordinary review workflow and history, can record a judgment that spans several source files, and fingerprints make a record of changed sources visibly stale instead of silently carried forward."
alternatives = ["Records in Book chapters, which keeps the records visible to readers but mixes results into the goal design and churns the Book on every specification change.", "Comments beside the reviewed code, which move with that code but cannot hold a judgment spanning several files and clutter the code with conclusions.", "An external tracker, which keeps the repository small but separates the evidence from the revision it describes."]
reconsider-when = "Assessments or reviews need signatures or approval workflows that files in the repository cannot represent; keep fingerprinted staleness whatever the store."
```
