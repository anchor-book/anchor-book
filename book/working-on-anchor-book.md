# Working on Anchor Book

This chapter provides the project-specific context for developing the Anchor Book
engine with its own Book. Read it after the [AI Working Contract](ai-working-contract.md).
The contract defines the reusable working method; this chapter identifies the
current state and resources for applying it in this repository.

## Book sources and authoring references

Read `book/SUMMARY.md` and every chapter it lists from the current checkout.
Edit sources under `book/`; generated output belongs in `target/anchorbook/`.

Use these definitions and conventions while authoring this Book:

- [Anchor Definitions](design/anchor-definitions.md) defines the canonical anchor schema.
- [Authority Definitions](design/authority-definitions.md) defines requirements,
  decisions, and authority-reference rules.
- [Implementation Roadmaps](implementation-roadmaps/index.md) defines milestone
  structure, identifiers, planned coverage, statuses, and evidence records.

The record schemas are unpublished drafts that we are refining through use.
Follow their documented evolution policy and update schemas, affected records,
and validation expectations together. Per-capability phase configuration is not
currently defined.

## Current phase

Read `[project].phase` in the repository-root `anchorbook.toml` for the current
phase. That file is the single authoritative setting; this chapter does not
repeat its value. The configuration reader validates the field without inferring
design acceptance, implementation progress, or completion.

## Available tools and checks

The record library, configuration loader, and `anchorbook md` preprocessor are
implemented. The binary also supports `anchorbook md supports html`, help, and
version. Configuration is currently a Rust library API; the `check` and `contract`
commands remain planned. Do not run those proposed commands or install similarly
named tools in their place. Loading configuration alone does not check Book format.

Read `CONTRIBUTING.md` at the repository root for the supported Rust and mdBook
versions and exact commands for tests, formatting, Clippy, builds, and previews.
The [Rust Engine design](design/rust-engine.md) describes package and API boundaries.

Engine verification uses independent fixtures and a separate acceptance test
against this Book. For engine changes, exercise the supported Rust toolchains and
checks relevant to the work. For Book changes, build it and check rendered records,
links, and affected diagrams. Engine compilation and independent tests must remain
usable when this Book is broken.

The contract's pre-PR testing policy is being developed during prototyping.
Mutation testing uses pinned cargo-mutants with local commands and a pull-request
workflow; read `CONTRIBUTING.md` for scope, runtime budgets, results, and the
agent correction loop. The [Mutation Testing roadmap](implementation-roadmaps/mutation-testing.md)
records local evidence separately from the full campaign, GitHub execution,
and required merge rules. These must be demonstrated before claiming merge
protection; adding a workflow alone does not enable that protection.

[Formal Verification](design/formal-verification.md) implements a Verus proof of the
production authority-validation functions. The proof and GitHub workflow are merged
in `03fb981`; PR and main-branch verification and integration tests pass. Its
[roadmap](implementation-roadmaps/formal-verification.md) records the evidence and
remaining scope/integration review, remote failure/correction demonstration, and
merge protection. These milestones are not yet complete.
CONTRIBUTING documents the pinned tools, native command, and mutation-coverage gap.
Criterion-level required methods are supported by
the record parser and renderer; they declare obligations without executing any
verification tool or consuming results.

## Public content

This repository is public. Keep private reference material, credentials, personal
contact information, and local machine paths out of its content, generated
artifacts, commit messages, and published logs. Context access is not publication
permission. Review the exact files and changes, relevant generated output, and
Git author metadata before committing or publishing. Leave material with uncertain
publication permission outside the repository.

## Contract during bootstrap

The contract is temporarily maintained in `book/ai-working-contract.md` for review.
Even in this temporary location, it contains only reusable guidance. Keep project
state and instructions in normal Book chapters such as this one.

The [contract delivery design](design/system-overview.md#contract-delivery) describes
moving the maintained source into the engine and retrieving it through a command.
That command is not implemented yet; agents read the current file directly.
