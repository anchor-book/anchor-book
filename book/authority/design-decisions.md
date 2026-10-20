# Design Decisions

Design decisions record durable choices that shape the system and the reasoning
needed to apply them. A decision explains the choice, why it was made, meaningful
alternatives, and any conditions that could cause it to be reconsidered.

Accepted decisions are reflected in the current design. When a decision
establishes a binding property, the corresponding anchor can cite it as authority.

## Initial checker decisions

**Status: draft for review.** These records explain the choices in the current
design draft. None is marked accepted by adding it here. Each records its
requirements, rationale, alternatives, and reconsideration conditions.
Each record uses the single [authority definition format](../design/authority-definitions.md).
The engine does not validate these records yet. Links below the blocks are
reading aids; the anchor definitions own their authority references.

<a id="decision-explicit-configuration"></a>

### Select and resolve a Book through explicit configuration

```anchorbook-decision
schema-version = 1
id = "decision-explicit-configuration"
title = "Select and resolve a Book through explicit configuration"
status = "draft"
choice = '''
Require an explicit `anchorbook.toml` path for the initial check.
Resolve Book paths relative to its directory and section paths relative to the
Book source. Use a versioned schema with required fields and reject unknown
fields, unsupported versions, path escapes, and source/output overlap.
'''
requirements = ["requirement-configured-structure"]
rationale = '''
The caller selects the Book unambiguously. Directory names remain
configurable, while validated paths and roles give checks a reliable basis.
'''
alternatives = [
  '''
Searching parent directories adds implicit selection rules.
Hard-coded directories exclude existing layouts. Permissive fields can hide typos.
''',
]
reconsider-when = '''
Multi-root Books or configuration discovery have a concrete
workflow and explicit selection and containment rules.
'''
```

**Anchors:** [Book selection](../design/configuration.md#check-book-selection)
and [configuration validity](../design/configuration.md#check-configuration-validity).

<a id="decision-mdbook-structure"></a>

### Use mdBook navigation and compare its file-based settings

```anchorbook-decision
schema-version = 1
id = "decision-mdbook-structure"
title = "Use mdBook navigation and compare its file-based settings"
status = "draft"
choice = '''
Use `SUMMARY.md` navigation to locate chapters and required top-level
roles. Compare the source and output resolved from adjacent `book.toml` with
Anchor Book's intended configuration. Document the supported mdBook version and
navigation constructs when selecting the parser.
'''
requirements = ["requirement-configured-structure", "requirement-truthful-results"]
rationale = '''
mdBook already supplies the Book's structure and rendering. Reusing
its navigation model avoids maintaining a second chapter hierarchy.
'''
alternatives = [
  '''
A separate navigation manifest could drift from the rendered
Book. Building the Book to inspect it could execute preprocessors or create files.
The initial check does not reproduce environment or command-line overrides.
''',
]
reconsider-when = '''
A build adapter needs effective renderer settings, or the
supported mdBook grammar changes. Parser implementation remains an open choice.
'''
```

**Anchors:** [Navigation conformance](../design/structural-checking.md#check-navigation-conformance)
and [renderer alignment](../design/structural-checking.md#check-renderer-alignment).

<a id="decision-separate-check-and-repair"></a>

### Keep checking separate from repair

```anchorbook-decision
schema-version = 1
id = "decision-separate-check-and-repair"
title = "Keep checking separate from repair"
status = "draft"
choice = '''
The first checker only observes and reports. It does not write,
launch subprocesses, render, fetch dependencies, or run verifiers. Diagnostics
explain expected and observed conditions and offer repair guidance; the assigned
agent or developer makes changes and reruns the check.
'''
requirements = ["requirement-observation-only", "requirement-repair-feedback"]
rationale = '''
A caller can inspect defects without changing the inputs being
diagnosed. Repair often requires project intent that the checker cannot infer.
'''
alternatives = [
  '''
Automatic repair inside `check` mixes diagnosis with changes
and obscures which operation produced the resulting structure.
''',
]
reconsider-when = '''
Managed writes have an agreed ownership and conflict model.
That adds a separate operation; it does not implicitly make `check` mutating.
'''
```

**Anchors:** [Observation-only checking](../design/structural-checking.md#check-observation-only)
and [actionable diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics).

<a id="decision-typed-check-results"></a>

### Keep checking and typed results in the library

```anchorbook-decision
schema-version = 1
id = "decision-typed-check-results"
title = "Keep checking and typed results in the library"
status = "draft"
choice = '''
A shared synchronous library owns configuration, inspection, rules,
and typed results. The CLI owns formatting and exit status: `0` for a pass,
`1` for structural failure, and `2` for invalid invocation or incomplete
evaluation. Incomplete evaluation takes precedence over collected violations.
Completed reports go to stdout; incomplete reports, including partial findings,
go together to stderr.
'''
requirements = ["requirement-shared-checker", "requirement-truthful-results", "requirement-repair-feedback"]
rationale = '''
Callers can inspect findings and failures without parsing prose.
Stable codes and explicit scope support both agent interpretation and assertions.
Partial findings remain useful without claiming complete evaluation.
'''
alternatives = [
  '''
Separate CLI rules risk inconsistent behavior. Returning only
text or a boolean loses structured locations, causes, and evaluation scope.
''',
]
reconsider-when = '''
Real callers demonstrate a need for additional adapters or a
versioned serialized interface. Exact Rust types remain an implementation choice.
'''
```

**Anchors:** [Shared typed checking](../design/rust-engine.md#check-shared-typed-results),
[truthful results](../design/check-diagnostics.md#check-truthful-results), and
[actionable diagnostics](../design/check-diagnostics.md#check-actionable-diagnostics).

<a id="decision-independent-book-acceptance"></a>

### Test the engine independently, then check its current Book

```anchorbook-decision
schema-version = 1
id = "decision-independent-book-acceptance"
title = "Test the engine independently, then check its current Book"
status = "draft"
choice = '''
Compile and test the engine with independent fixtures. Check the
repository's current Book as a separate acceptance step before rendering.
Exercise deliberate failures in disposable copies with expected outcomes.
'''
requirements = ["requirement-independent-engine"]
rationale = '''
The engine remains available to diagnose a broken Book. Independent
expectations catch mistakes that successful checking of its own Book may miss.
'''
alternatives = [
  '''
Requiring a valid rendered Book to compile the engine creates a
circular dependency. Testing only the current Book leaves failure behavior unproven.
''',
]
reconsider-when = '''
Later evidence integrations change the dependency graph;
they must still avoid depending on results of the test run that contains them.
'''
```

**Anchor:** [Independent engine build](../design/system-overview.md#check-independent-build).
