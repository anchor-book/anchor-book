# Research

This chapter holds source observations and exploratory comparisons. They explain
what was observed at a recorded revision, not what the target must do. Governing
design belongs in [Design](design/index.md); unresolved choices belong in
[Open questions](open-questions.md). Roadmaps own scoped trial results, remaining
work, and completion evidence.

## Record syntax observations

Before [milestone 2](implementation-roadmaps/record-line-grammar.md#record-line-grammar-2), the engine and this Book's records
used an earlier TOML form: each record a fenced block labeled `anchorbook-anchor`,
`anchorbook-requirement`, or `anchorbook-decision`, with lowercase identifiers
that carried no kind code and proof scope written as unchecked paragraphs inside
the statement. Searches against this Book's sources showed what that form cost
agents:

| Question | Result with TOML records |
| --- | --- |
| What cites `R-observation-only`? | 6 lines in three shapes: the definition, an anchor citation, a decision citation, and three lines from literal schema examples that look the same without reading the surrounding block |
| Which records are drafts? | 29 lines of `status = "draft"`, none stating the record's kind or identity |
| Which criteria require proof? | `verification = ["proof"]`, without its criterion or anchor identifier |

The renderer needed no change beyond rendering
[inline text](design/record-rendering.md#reader-presentation): it replaces each
record's source range with HTML from the typed inventory.

These observations motivated [Record Line Grammar milestone 1](implementation-roadmaps/record-line-grammar.md#record-line-grammar-1).
Its results, agent versions, and limitations remain in that milestone. The
[pre-reconciliation record](https://github.com/anchor-book/anchor-book/blob/7966c9fddb3e9b9a710444c365cd698550de9b5f/book/implementation-roadmaps/record-line-grammar.md#gap)
preserves the original context; the current line grammar is the goal design.

## Optional Verus packaging

The experiment compared ordinary Rust with optional Verus attributes against a
proof workspace that generates runtime source. Unchanged `verus!` bodies need
Verus macro crates even with the feature disabled. The ordinary-Rust arrangement
was selected, with attributes attached through `cfg_attr` and specifications and
proofs enabled only for verification. A feasibility slice (two helpers, `lookup` with its loop invariant, and the specification functions they use) verified with the feature and built without it, and a deliberate defect failed; the full validator then followed. Research into the Verus guide for the pinned release supports the choice: the guide recommends `#[verus_spec]` for executable functions and `verus!` for specification functions when production code must build without Verus, with dependencies controlled by feature flags. The gate is the Cargo feature rather than Verus's internal `verus_keep_ghost` cfg. Generated runtime source was not needed.

This was the feasibility work for [Formal Verification milestone 4](implementation-roadmaps/formal-verification.md#formal-verification-4),
recorded on 2026-11-10 with Verus 0.2026.10.18.7d82994. Its
[original evidence context](https://github.com/anchor-book/anchor-book/blob/7966c9fddb3e9b9a710444c365cd698550de9b5f/book/implementation-roadmaps/formal-verification.md#formal-verification-4-evidence)
remains available. The chosen target arrangement now lives in
[Shipping Verified Code](design/verified-code-packaging.md#proof-packaging-for-consumers).
Automated drift rejection is still missing; the feasibility result does not
satisfy the packaged-source or feature-boundary criteria.

## Symposium test outcomes

Scope: Symposium `4e2158b89461d17496ff07c5d2a0ebf2619abfe3`, engine
`b7bcd33d0035fdc6af9790e0fd8568394772afb9`, Linux x86_64, Rust 1.99.1,
cargo-nextest 0.9.151. This was a disposable fork experiment, not upstream adoption.

A disposable criterion about consuming the tokio-weather
skill was linked to `agent_reads_tokio_skill_after_cargo_add`, not to an unrelated
configuration criterion. With `SYMPOSIUM_ENABLE_AGENT_TESTING` absent, and again
with it set to `1` and `test-agents.toml` containing `test-agents = []`, nextest
and cargo test both report success. The direct command
`cargo test --locked -p symposium --test hook_agent agent_reads_tokio_skill_after_cargo_add -- --exact --nocapture`
prints `agent-only test ran no agent`; verify reports satisfied, matching the
runner's passing result.
No agent was started. The reports have identical criteria/configuration/code
identities despite the different runtime settings. The temporary records and
link were removed after the experiment. Symposium owns what this test outcome
means and whether it needs correction. The engine trusts the project's current
test result; its code/configuration identity does not claim to capture every
runtime setting. No additional execution-report protocol is required.

[Symposium Integration milestone 4](implementation-roadmaps/symposium-integration.md#symposium-integration-4-evidence)
retains passing/failing/restored verification results, the coverage gaps, and the
platform limits. The governing [project-test trust decision](authority/decisions.md#D-project-test-results-trusted)
separates the engine's result mapping from the meaning of a project's assertions.
