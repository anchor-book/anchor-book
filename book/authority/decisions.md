# Decisions

Each decision states its own status. These choices govern the system being built; other parts of the Book must account for them.

> **`D-explicit-configuration`** (accepted): **Configuration defines the Book's layout and roles**
> Choice: Define Book paths and section roles in anchorbook.toml. Resolve Book paths relative to that configuration and section paths relative to the source.
> Requirements: `R-configured-structure`
> Rationale: Checking, initialization, adoption, and materialization need the same intended layout without relying on fixed directory names or display titles.
> Alternatives:
> - Conventional paths reduce configuration, but constrain existing projects and make directory names determine their roles.
> Reconsider when: Books spanning multiple roots require a different path-resolution model.

> **`D-mdbook-structure`** (accepted): **mdBook navigation defines the chapter hierarchy**
> Choice: Use SUMMARY.md as the chapter hierarchy for checking, materialization, adoption, and rendering; do not maintain a separate Anchor Book hierarchy.
> Requirements: `R-configured-structure`; `R-truthful-results`
> Rationale: All operations must agree on the chapters readers see. Reusing mdBook navigation keeps structural checks and managed changes aligned with rendering.
> Alternatives:
> - An independent navigation manifest offers more control but requires synchronization with mdBook.
> Reconsider when: Supporting a different book format requires a hierarchy that mdBook cannot express.

> **`D-separate-check-and-repair`** (accepted): **Checking and repair are separate operations**
> Choice: Checking observes and reports. Repairs and managed writes belong to separate agent actions or engine operations.
> Requirements: `R-observation-only`; `R-repair-feedback`
> Rationale: CI and callers can inspect defects without changing their inputs. Materialization and repair must define their own ownership and conflict behavior.
> Alternatives:
> - Repairing during checking shortens the workflow but makes inspection mutate the state it reports.
> Reconsider when: A combined user workflow is needed; preserve explicit observation and mutation boundaries within it.

> **`D-typed-check-results`** (accepted): **Shared behavior and typed results belong in the Rust library**
> Choice: Keep shared engine behavior and typed results in the Rust library. The CLI and other adapters translate invocation and presentation without defining separate rules.
> Requirements: `R-shared-checker`; `R-truthful-results`; `R-repair-feedback`
> Rationale: CLI commands, mdBook integration, Rust tests, and future adapters must use consistent behavior and inspect results without parsing display text.
> Alternatives:
> - A CLI-only implementation is simpler to expose but requires Rust callers and other adapters to invoke a process and interpret its output.
> Reconsider when: Deployment or language requirements call for a different shared interface; retain one owner of engine behavior.

> **`D-shared-record-rendering`** (draft): **Validation and rendering use one record model**
> Choice: Parse canonical definitions into one shared record inventory. Validation, rendering, and derived links use its identities and relationships; adapters do not define separate interpretations.
> Requirements: `R-readable-records`
> Rationale: Authority resolution, backlinks, future progress views, and additional renderers must agree on what each record means.
> Alternatives:
> - Separate presentation models permit independent evolution but require synchronization with validation.
> Reconsider when: Another output format needs its own view; derive that view from the shared inventory rather than redefine record semantics.

> **`D-native-tool-integration`** (draft): **Anchor Book works through the project's existing tools**
> Choice: Integrate through the extension points of tools a project already uses, such as mdBook preprocessors, Cargo, and CI steps, so their ordinary commands apply Anchor Book checks and rendering. Do not require wrapper scripts or replacement commands.
> Requirements: `R-project-adoption`
> Rationale: Adopting projects keep their existing commands, editors, and CI. A check enforced only by a wrapper is skipped whenever someone runs the underlying tool directly, so the check must run where the tool runs.
> Alternatives:
> - A wrapper command can sequence checks and rendering without changing tool configuration, but it is easy to bypass and every repository must adopt and maintain it.
> Reconsider when: A tool offers no suitable extension point or its protocol cannot express a required check; record the gap and keep any wrapper minimal.

> **`D-verify-runs-linked-checks`** (draft): **Verify runs the linked checks through the project's tools**
> Choice: The verify operation selects linked test functions and the complete proof targets required by criteria in scope, runs them through supported runners of the project's own tools, and reads that run's reports. Test selection is exact; proof selection covers whole declared crate targets and the report states the executed scope. The operation does not replace the project's test commands or CI jobs.
> Requirements: `R-criterion-evidence`
> Rationale: Agents in the build loop need one command that gives current results for the anchor they are changing. Anchor Book knows which functions address which criteria, so it adds selection, tool execution, and association of results with current inputs. Complete proof-target runs preserve the verifier's declared scope, while input identities prevent cached or changed inputs from being reported as fresh evidence.
> Alternatives:
> - Verify reads reports produced by separate runs, which avoids invoking tools but needs several steps in the build loop and recorded revisions to tell current results from old ones.
> - Agents run the whole suite and read its output, which loses the link from results to criteria.
> Reconsider when: A project needs evidence from runs verify cannot invoke, such as tests on other platforms; add reading of supplied reports as a separate input rather than removing selected runs.

> **`D-completion-claims-checked`** (draft): **Complete milestones claim their fully covered criteria, and verify checks the claim**
> Choice: A roadmap milestone with status Complete claims every anchor criterion its completion criteria cover in full. verify fails the run when a claimed criterion in scope is not satisfied, and lists claims it could not check. Partial or no coverage, and milestones in other states, claim nothing.
> Requirements: `R-criterion-evidence`
> Rationale: Whether a criterion has linked tests depends on someone adding them, so absence must be caught where it matters: when the Book records work as done. Tying the check to Complete milestones uses a claim the Book already makes, keeps work in progress as a report rather than a failure, and makes a later regression in that evidence visible.
> Alternatives:
> - Every test must carry a link or an explicit exemption, which catches unlinked tests directly but is noisy for projects with many existing tests; kept as a possible opt-in.
> - A separate completion marker on anchors or design areas, which is more precise but adds a field the Book does not have.
> - Planned coverage stays informational, which avoids failures but lets completion be recorded with no evidence.
> Reconsider when: Completion is recorded somewhere other than roadmap milestones, or claims need finer scope than a milestone's criteria.

> **`D-evidence-links-in-code`** (draft): **Evidence links live beside the code they describe**
> Choice: A test or proof links to the anchor criteria it addresses with a doc-comment line on the function, /// anchorbook: <anchor-id>#<criterion-id>; the engine derives each criterion's linked functions rather than listing them in the Book.
> Requirements: `R-criterion-evidence`
> Rationale: Links move with renamed or relocated tests, the Book holds only criteria and stays small, and the existing proof module already names its anchor and criterion beside the code. A doc comment is an attribute of the item it precedes, so a Rust parser attaches the link to its function directly, without inferring which item follows a line, and the project needs no dependency.
> Alternatives:
> - Criteria list their tests in the Book, which is visible to readers without the engine but grows the Book and drifts whenever tests are renamed.
> - An ordinary // comment, which Rust parsers discard, so the engine must infer from line positions which function a link belongs to.
> - An attribute such as #[anchorbook::criterion(...)], which reads well but needs an engine proc-macro crate as a project dependency, since stable Rust rejects unknown tool attributes.
> Reconsider when: A supported language or tool cannot carry comments next to its tests or proofs; define its link form without moving links into the Book.

> **`D-identifier-kind-codes`** (draft): **Identifiers carry a kind code and a descriptive name**
> Choice: Every anchor, requirement, and decision identifier is a kind code (A, R, or D), a hyphen, and a name that starts with a letter and states the record's meaning. The identifier is the only place the kind is stated: headings and references consist of the code-spanned identifier, with no kind word. Criteria have names without a code, unique within their anchor. The engine enforces the form and the codes each reference field accepts; review judges whether a name describes its record.
> Requirements: `R-searchable-records`
> Rationale: Identifiers appear without their records in references, evidence links, fragments, diagnostics, and search results, so an agent must recognize a record and its kind from the identifier alone. A two-character code keeps the kind visible everywhere at little length and makes every mention of a kind findable with one search, such as a code span starting with D-. Requiring the name to start with a letter rules out sequence numbers, which say nothing about their records. Stating the kind once, in the identifier, gives every reference one spelling.
> Alternatives:
> - Full-word prefixes such as decision-, which are self-describing but add nine to twelve characters and, with a kind word in references, state the kind twice.
> - Names with no kind marker, which are shortest but leave the kind invisible in fragments, evidence links, and prose.
> - Sequential identifiers such as D-12, which are short and easy to allocate but carry no meaning, so every reference must be looked up.
> Reconsider when: A new record kind needs a code that would be ambiguous with an existing one, or descriptive names collide often enough to need a different scheme; keep names descriptive whatever the form.

> **`D-library-cli-entry`** (draft): **The library provides the CLI entry point and projects declare the engine version**
> Choice: The library exposes the command-line entry point, and the published anchorbook binary is a thin wrapper around it. A project runs the engine either through a small binary in its own workspace or through an installed binary. It declares one exact engine version in anchorbook.toml using an exact Cargo version requirement, and every entry point stops before evaluating when its version differs. Missing, malformed, or non-exact declarations make evaluation incomplete.
> Requirements: `R-project-adoption`
> Rationale: A dependency provides only the library, while mdBook, CI, and agents invoke a binary, and Cargo does not run binaries belonging to dependencies. A workspace binary makes the project's Cargo.lock the single version source for tests and commands; an installed binary suits projects that prefer no extra workspace member. An exact declaration keeps either from silently running different rules than the project's tests.
> Alternatives:
> - Installed binary only, which keeps the project unchanged but leaves the installed version and the locked library version free to diverge.
> - Workspace binary only, which gives one version source but requires every project to add and maintain a workspace member.
> - A compatible version range allows entry points to run different engine versions and therefore cannot guarantee identical rules.
> Reconsider when: Cargo can run binaries provided by dependencies, which would make a project-local wrapper unnecessary.

> **`D-isolate-engine-verification`** (draft): **Engine proofs stay inside the engine; projects verify with their own Verus**
> Choice: The engine's own proofs, vstd, and Verus toolchain are excluded from what consuming projects build. Verifying a project's code runs the Verus installation and verification toolchain that the project configures.
> Requirements: `R-project-adoption`; `R-project-verification`
> Rationale: Projects verifying their own code choose their own Verus and vstd versions, which are pinned exactly and change frequently. An exact vstd requirement in the engine's dependency graph would conflict with them or force the engine's choice, and a verification toolchain must not replace a project's build toolchain.
> Alternatives:
> - Projects use the engine's pinned Verus and vstd, which avoids version handling in the engine but forces every project onto the engine's release cadence.
> - The engine bundles a Verus installation, which simplifies setup but still dictates the version and adds a large download to every installation.
> Reconsider when: Verus and vstd provide compatible version ranges across releases, so a shared requirement no longer forces a single version.

> **`D-assessment-records-as-files`** (draft): **Assessment records are files beside the code, outside the Book**
> Choice: A specification assessment or a review is recorded as a file in an assessment directory configured in anchorbook.toml, naming the criterion, the subject, the fingerprints of what was examined, the conclusion, the reviewer, and the revision. verify reads these records and treats one whose fingerprints no longer match as stale; it never writes them.
> Requirements: `R-criterion-evidence`; `R-proofs-establish-criteria`
> Rationale: Assessments and reviews are evidence, so they stay out of authored Book sources, which hold the goal rather than results. Files go through the project's ordinary review workflow and history, can record a judgment that spans several source files, and fingerprints make a record of changed sources visibly stale instead of silently carried forward.
> Alternatives:
> - Records in Book chapters, which keeps the records visible to readers but mixes results into the goal design and churns the Book on every specification change.
> - Comments beside the reviewed code, which move with that code but cannot hold a judgment spanning several files and clutter the code with conclusions.
> - An external tracker, which keeps the repository small but separates the evidence from the revision it describes.
> Reconsider when: Assessments or reviews need signatures or approval workflows that files in the repository cannot represent; keep fingerprinted staleness whatever the store.
