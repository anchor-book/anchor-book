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
That feasibility result did not establish automated drift rejection. The owning
roadmap records the later private-workspace and source-audit work.

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

## Registry release automation

The 2026-11-25 assessment inspected Sparkle MCP at
[`fd0840b45af23e5ad3900bb5b251c7188bd0ff30`](https://github.com/sparkle-ai-space/sparkle-mcp/tree/fd0840b45af23e5ad3900bb5b251c7188bd0ff30).
Its latest release, `v0.7.0`, has a matching crates.io version and five binary
archives plus `extension.json` on GitHub.

- [`release-plz.yml`](https://github.com/sparkle-ai-space/sparkle-mcp/blob/fd0840b45af23e5ad3900bb5b251c7188bd0ff30/.github/workflows/release-plz.yml)
  runs separate `release-pr` and `release` jobs on pushes to `main`. The former
  updates version and changelog in a release PR. The latter publishes eligible
  package versions, tags, and GitHub releases. Both restrict the repository owner.
- Both jobs use `RELEASE_PLZ_TOKEN` for GitHub operations, including checkout.
  The publishing job grants `id-token: write` and supplies no registry token.
  This matches release-plz's automatic crates.io trusted-publishing flow. The
  actual crates.io publisher registration and the token's scope were not inspected.
- No release-plz configuration file was present. Its default `release_always`
  behavior attempts publication whenever the release command runs; it is not
  restricted to release-PR merges.
- [`release-binaries.yml`](https://github.com/sparkle-ai-space/sparkle-mcp/blob/fd0840b45af23e5ad3900bb5b251c7188bd0ff30/.github/workflows/release-binaries.yml)
  runs on `release: published`. It builds macOS ARM64/x86-64, Linux musl
  ARM64/x86-64, and Windows x86-64 archives, then uploads an ACP extension manifest.
  The manifest serves Sparkle's ACP distribution and is not an Anchor Book need.

Release-plz's [quickstart](https://github.com/release-plz/release-plz/blob/c9e4ab586016cb5fa0ba9760387f3dcde26c543b/website/docs/github/quickstart.md)
explains that OIDC needs `id-token: write` and no separately supplied registry
token, but a new crate needs its first publication before trusted publishing can
be configured. Its [GitHub token guidance](https://github.com/release-plz/release-plz/blob/c9e4ab586016cb5fa0ba9760387f3dcde26c543b/website/docs/github/token.md)
explains why an app token or PAT is needed when generated release PRs and release
events must trigger other workflows. Its [configuration reference](https://github.com/release-plz/release-plz/blob/c9e4ab586016cb5fa0ba9760387f3dcde26c543b/website/docs/config.md#the-release_always-field)
defines `release_always = false` for release-PR-controlled publication.

Anchor Book at `70355d4d55f81667fd2b9a851987cc6a2b84befd` still sets
`publish = false`, has no release workflow or listed repository release secret,
and uses read-only default Actions permissions. Its existing main ruleset
requires the Rust, Book, installed-engine, proof, and mutation checks. Public
crates.io lookups for `anchor-book` and `anchorbook` returned 404; this is not a
reservation. `cargo package --locked` successfully packaged 159 files (1.6 MiB,
445.1 KiB compressed) and compiled the extracted crate. The archive contains
contract and template sources, licenses, `build.rs`, and Cargo's revision metadata.
This did not publish, establish crate ownership, test a registry installation,
or verify independent consumer dependency resolution.

The proposed target is [registry distribution](design/project-integration.md#registry-distribution),
with work and acceptance in [Project Integration milestone 7](implementation-roadmaps/project-integration.md#project-integration-7).


Two release-tool details affect the preparation. Release-plz's
[commit selection](https://github.com/release-plz/release-plz/blob/c9e4ab586016cb5fa0ba9760387f3dcde26c543b/website/docs/usage/release.md#what-commit-is-released)
can publish the release PR's head instead of its merge commit. The release workflow therefore selects the same candidate before passing it
to its validation jobs. Cargo-semver-checks 0.51.0's default feature heuristic selects
`verus`. An Anchor Book all-features build on ordinary stable Rust failed at its
Verus-only unstable attributes. An explicit `--default-features` scan avoided that build in the initial
preparation. Isolating the verifier in an unpublished workspace removes the
public feature entirely, allowing the standard release-plz scan again.
