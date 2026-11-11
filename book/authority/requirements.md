# Requirements

Requirements state required outcomes and constraints, with their source.

## Initial checker requirements

**Status: accepted.** These records express the checker goals; their
sources explain the product need. Authority links and backlinks are derived
from the record inventory.

> **`R-configured-structure`** (accepted): **Check the project's configured Book structure**
> Statement: The checker must evaluate the Book selected by the caller against its configured layout and required section roles. Projects must be able to choose directory names and page titles without changing the meaning of those roles.
> Source: The product goal of supporting both new Books and existing project layouts, including a configurable source and generated-output directory.


> **`R-repair-feedback`** (accepted): **Give agents enough information to repair structure**
> Statement: A developer or agent must be able to use checker output to locate a problem, understand the expected structure, make an appropriate repair within the assigned scope, and recheck. A pass obtained by removing obligations or weakening the intended configuration does not satisfy this requirement.
> Source: Agents bring a Book into its configured format by checking, repairing, and rechecking; they can only repair what the checker's output lets them locate and understand.


> **`R-observation-only`** (accepted): **Preserve the project during checking**
> Statement: Checking must leave project content and directory entries unchanged. It must work locally without rendering, downloading dependencies, executing configured tools, or running project tests. The agent's repairs are separate from a check.
> Source: The separation between checking current structure and operations that create or manage it.


> **`R-truthful-results`** (accepted): **Report only what was established**
> Statement: Results must distinguish conformance, known structural violations, and inability to evaluate. They must identify the evaluated scope and blocked checks. Missing or incomplete evaluation cannot count as a pass, and structural conformance must not imply implementation, verification, or roadmap completion.
> Source: The Book's separation of intended properties, implementation progress, verification evidence, and completion claims.


> **`R-shared-checker`** (accepted): **Provide the same checking behavior to tools and Rust callers**
> Statement: Developers, agents, CI, and Rust integration tests must be able to use the same checking rules and receive equivalent findings for the same inputs. Rust callers must be able to inspect results without parsing terminal text.
> Source: The goal of distributing the engine as a Rust crate with a CLI and a library usable by consuming projects.


> **`R-independent-engine`** (accepted): **Keep the engine usable when its own Book is broken**
> Statement: The engine must compile and run independent tests when this repository's Book is invalid or has not been rendered. Its current Book must also be exercised as a real acceptance case, with independently specified failures supplementing it.
> Source: The goal of developing Anchor Book using its own Book without a circular dependency that prevents diagnosing or repairing that Book.


## Authority traceability requirement

> **`R-traceable-properties`** (draft): **Every binding system property has a traceable justification**
> Statement: Every binding system property expressed by an anchor must be grounded in at least one requirement or decision. Engineers and agents must be able to follow those references to understand why the property must hold.
> Source: The Anchor Book method makes the reasons for system properties explicit so engineers and agents can review and evolve the design together.

## Rendering prototype requirement

> **`R-readable-records`** (draft): **Read structured records naturally in the Book**
> Statement: Readers must be able to understand anchors, requirements, and decisions as formatted content in their Book chapters and navigate their declared authority relationships. Structured definitions remain the source. Invalid records or relationships must not be hidden by rendering, and presentation must not claim that a property has been implemented or verified.
> Source: The prototyping goal of reviewing the Book's structured design and authority as readable, connected content while refining the model.

## Record syntax requirement

> **`R-searchable-records`** (draft): **Agents can find records and references with one text search**
> Statement: An agent working from Book sources must be able to locate every record of a kind, with its identity, status, and title, and every reference to a given record, with one line-oriented text search and without running a parser.
> Source: Agents read and search Book sources rather than the rendered Book; record structure that needs block-level reading multiplies their reads and wrong matches.

## Roadmap requirement

> **`R-checkable-roadmaps`** (draft): **Agreed work is compact for agents and checked for references**
> Statement: Implementation roadmaps must be readable by agents as compact source and by people as rendered pages, and their milestone and criterion identifiers and anchor-coverage references must be checked, so a roadmap cannot silently reuse an identifier or cite an anchor or criterion that does not exist.
> Source: Roadmaps written as HTML tables spent about a fifth of the Book's source on presentation that agents read in every session, and nothing checked the anchor references they record.

## Verification requirements

> **`R-criterion-evidence`** (draft): **See whether current evidence supports each criterion**
> Statement: Engineers and agents must be able to see the project's progress toward its goal design: for every anchor criterion and each verification method it requires, whether current results support it, which tests or proofs they come from, and where evidence is missing or failed. Agents in the build loop must be able to get this for the anchors they are changing with one command. Missing or outdated evidence must never appear as support.
> Source: Anchors declare required verification methods, but completion and verification are otherwise recorded by hand in roadmaps, so nothing shows which anchor criteria are actually supported.

> **`R-proofs-establish-criteria`** (draft): **A proof counts only when it establishes its criterion**
> Statement: A proof must count as evidence for a criterion only when it establishes that criterion's stated claim over the production code the system executes, within the criterion's declared domain, boundary, assumptions, and exclusions. A successful verification whose specification is weaker than the claim, holds vacuously, rests on unrecorded assumptions, or differs from the specification that was reviewed must not appear as support for the criterion.
> Source: Verus establishes that code meets its specification, not that the specification expresses the design. Agents that write both the code and its specification can obtain a passing proof that establishes less than the criterion requires.

## Project integration requirements

> **`R-project-adoption`** (draft): **Rust projects adopt Anchor Book through the crate**
> Statement: A Rust project must be able to adopt Anchor Book by depending on the published crate and adding configuration, while keeping its existing commands, Book layout, toolchains, and dependency choices. Adoption must not require the engine's development checkout, wrapper commands, or changes to the versions of unrelated dependencies.
> Source: The goal of distributing the engine as a crate for other Rust projects, including projects with existing mdBook Books, multi-crate workspaces, and pinned toolchains.

> **`R-project-verification`** (draft): **Projects formally verify their own code against their Book**
> Statement: A project must be able to formally verify its own production code with Verus against the proof criteria in its Book, and see which of those criteria its proofs establish, using its own Verus version and verification toolchain.
> Source: The goal of integrating formal verification into the development of projects that use Anchor Book, not only of the engine itself.
