# Requirements

Requirements establish what the system must accomplish and the constraints it
must respect. They provide authority for design choices and for anchors that
express the properties the built system must preserve.

Each requirement should make its intended outcome and source clear. Proposed
requirements remain distinguishable from accepted requirements.

## Initial checker requirements

**Status: draft for review.** All records below are proposed formulations of the
checker goals developed during bootstrap. Recording them does not establish
acceptance or implementation. Their source describes the product need; links to
anchors identify the properties intended to satisfy it.

Each record uses the single [authority definition format](../design/authority-definitions.md).
The engine does not validate these records yet. Links below the blocks are
reading aids; the anchor definitions own their authority references.

<a id="requirement-configured-structure"></a>

### Check the project's configured Book structure

```anchorbook-requirement
schema-version = 1
id = "requirement-configured-structure"
title = "Check the project's configured Book structure"
status = "draft"
statement = '''
The checker must evaluate the Book selected by the caller against its configured
layout and required section roles. Projects must be able to choose directory
names and page titles without changing the meaning of those roles.
'''
source = '''
The product goal of supporting both new Books and existing project
layouts, including a configurable source and generated-output directory.
'''
```

**Anchors:** [Book selection](../design/configuration.md#check-book-selection),
[configuration validity](../design/configuration.md#check-configuration-validity),
[navigation conformance](../design/structural-checking.md#check-navigation-conformance),
and [renderer alignment](../design/structural-checking.md#check-renderer-alignment).

<a id="requirement-repair-feedback"></a>

### Give agents enough information to repair structure

```anchorbook-requirement
schema-version = 1
id = "requirement-repair-feedback"
title = "Give agents enough information to repair structure"
status = "draft"
statement = '''
A developer or agent must be able to use checker output to locate a problem,
understand the expected structure, make an appropriate repair within the assigned
scope, and recheck. A pass obtained by removing obligations or weakening the
intended configuration does not satisfy this requirement.
'''
source = '''
The Structural Checking roadmap's goal of an agent iterating on Book
structure until it conforms.
'''
```

**Anchor:** [Actionable diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics).

<a id="requirement-observation-only"></a>

### Preserve the project during checking

```anchorbook-requirement
schema-version = 1
id = "requirement-observation-only"
title = "Preserve the project during checking"
status = "draft"
statement = '''
Checking must leave project content and directory entries unchanged. It must
work locally without rendering, downloading dependencies, executing configured
tools, or running project tests. The agent's repairs are separate from a check.
'''
source = '''
The separation between checking current structure and operations
that create or manage it.
'''
```

**Anchor:** [Observation-only checking](../design/structural-checking.md#check-observation-only).

<a id="requirement-truthful-results"></a>

### Report only what was established

```anchorbook-requirement
schema-version = 1
id = "requirement-truthful-results"
title = "Report only what was established"
status = "draft"
statement = '''
Results must distinguish conformance, known structural violations, and inability
to evaluate. They must identify the evaluated scope and blocked checks. Missing
or incomplete evaluation cannot count as a pass, and structural conformance must
not imply implementation, verification, or roadmap completion.
'''
source = '''
The Book's separation of intended properties, implementation progress,
verification evidence, and completion claims.
'''
```

**Anchor:** [Truthful results and scope](../design/check-diagnostics.md#check-truthful-results).

<a id="requirement-shared-checker"></a>

### Provide the same checking behavior to tools and Rust callers

```anchorbook-requirement
schema-version = 1
id = "requirement-shared-checker"
title = "Provide the same checking behavior to tools and Rust callers"
status = "draft"
statement = '''
Developers, agents, CI, and Rust integration tests must be able to use the same
checking rules and receive equivalent findings for the same inputs. Rust callers
must be able to inspect results without parsing terminal text.
'''
source = '''
The goal of distributing the engine as a Rust crate with a CLI and a
library usable by consuming projects.
'''
```

**Anchor:** [Shared typed checking](../design/rust-engine.md#check-shared-typed-results).

<a id="requirement-independent-engine"></a>

### Keep the engine usable when its own Book is broken

```anchorbook-requirement
schema-version = 1
id = "requirement-independent-engine"
title = "Keep the engine usable when its own Book is broken"
status = "draft"
statement = '''
The engine must compile and run independent tests when this repository's Book
is invalid or has not been rendered. Its current Book must also be exercised as
a real acceptance case, with independently specified failures supplementing it.
'''
source = '''
The goal of developing Anchor Book using its own Book without a
circular dependency that prevents diagnosing or repairing that Book.
'''
```

**Anchor:** [Independent engine build](../design/system-overview.md#check-independent-build).

## Rendering prototype requirement

<a id="requirement-readable-records"></a>

### Read structured records naturally in the Book

```anchorbook-requirement
schema-version = 1
id = "requirement-readable-records"
title = "Read structured records naturally in the Book"
status = "draft"
statement = '''
Readers must be able to understand anchors, requirements, and design decisions
as formatted content in their Book chapters and navigate their declared authority
relationships. Structured definitions remain the source. Invalid records or
relationships must not be hidden by rendering, and presentation must not claim
that a property has been implemented or verified.
'''
source = "The prototyping goal of reviewing the Book's structured design and authority as readable, connected content while refining the model."
```

**Anchors:** [Rendered records](../design/record-rendering.md#render-records-in-context)
and [validated inventory](../design/record-rendering.md#render-validated-records).
