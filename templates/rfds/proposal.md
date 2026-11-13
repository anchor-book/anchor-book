# {{title}}

```rfd-metadata
format-version = 1
id = "RFD-{{meaningful-name}}"
title = "{{title}}"
status = "draft"
base-revision = "{{immutable-book-revision}}"
anchor-changes = "proposed"
affects = []
planned = []
milestones = []
supersedes = []
sources = []
incorporation = []
```

## Summary

{{Describe the change and the intended outcome.}}

## Motivation

{{Explain the problem and why it matters.}}

## Proposal

{{Describe the resulting system, affected areas, interactions, and compatibility.}}

## Proposed anchors

{{Fill the change below and add further changes as needed. For no anchor changes,
remove the sample, set anchor-changes to none, and add no-anchor-change-reason.}}

```rfd-change
operation = "add"
id = "A-{{capability-property}}"
reason = "{{Why this guarantee belongs in the system definition.}}"
bindings = [{ target = "requirement:R-{{outcome}}", scope = "current" }]
definition = '''
> **`A-{{capability-property}}`** (draft): **{{Title}}**
> Property: {{Behavior, conditions, and guaranteed outcome.}}
> Authority: `R-{{outcome}}`
> - Criterion `{{observable-condition}}` [test]: {{What the test must establish.}}
'''
```

## Alternatives

{{Describe the alternatives and their tradeoffs.}}

## Open questions

{{Record unresolved decisions, or state that none remain and explain the scope.}}

## Implementation

{{Link existing roadmap milestones or identify proposed work; keep task and
evidence tracking in those roadmaps. Explain how the design will be adopted.}}
