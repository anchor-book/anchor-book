# Requirements

Requirements state required outcomes and constraints, with their source.

## Initial checker requirements

**Status: draft for review.** These records express the checker goals; their
sources explain the product need. Authority links and backlinks are derived
from the record inventory.

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


## Authority traceability requirement

```anchorbook-requirement
schema-version = 1
id = "requirement-traceable-properties"
title = "Every binding system property has a traceable justification"
status = "draft"
statement = '''
Every binding system property expressed by an anchor must be grounded in at least
one requirement or design decision. Engineers and agents must be able to follow
those references to understand why the property must hold.
'''
source = "The Anchor Book method makes the reasons for system properties explicit so engineers and agents can review and evolve the design together."
```

## Rendering prototype requirement

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
