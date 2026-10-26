# Checking Diagnostics

**Status: draft for review.** These examples define proposed output, not output
from an implemented command. Paths and configuration values are illustrative.

Diagnostics support the check, repair, and recheck loop by identifying the
observed problem, expected structure, location, and useful next action.

## Anchors

```anchorbook-anchor
schema-version = 1
id = "check-truthful-results"
title = "Results tell the agent whether format repairs are complete"
status = "draft"
property = '''
An engineer or agent can distinguish a Book that conforms to the checked format,
a Book with known structural defects to repair, and a check that could not finish.
A pass requires every required check in the declared scope to have passed.
Reports identify evaluated and blocked checks; operational or invalid-input
failures remain incomplete even when partial structural findings are available.
Structural success does not claim anchor verification, implementation, or roadmap
completion.
'''
authorities = [
  { kind = "requirement", id = "requirement-truthful-results" },
  { kind = "decision", id = "decision-typed-check-results" },
]

[[criteria]]
id = "outcomes-and-precedence"
verification = ["test"]
statement = "Verify CLI exits 0, 1, and 2 for passing, structural-failure, and incomplete cases; mixed violations and operational failure return 2 and retain explicitly partial findings."

[[criteria]]
id = "blocked-evaluation"
verification = ["test"]
statement = "Missing prerequisites yield root-cause findings and blocked dependent checks without cascaded invented defects; an empty findings list with incomplete evaluation never passes."

[[criteria]]
id = "report-scope"
verification = ["test"]
statement = "Assert evaluated and blocked groups and limits of the passing claim. Completed reports appear on stdout and incomplete reports with partial findings appear together on stderr."
```

```anchorbook-anchor
schema-version = 1
id = "check-actionable-diagnostics"
title = "Findings give agents enough information to repair the format"
status = "draft"
property = '''
An engineer or agent can use each finding to locate a format problem, understand
what was expected and observed, and choose an appropriate repair within the
assignment before rechecking. Findings include stable codes, kind, severity,
relevant locations, and useful next actions. Reports have deterministic order and
remain readable without color or interactive prompts. Guidance makes uncertainty
about project intent explicit; it does not authorize edits or suggest weakening
the intended rules to obtain a pass.
'''
authorities = [
  { kind = "requirement", id = "requirement-repair-feedback" },
  { kind = "decision", id = "decision-separate-check-and-repair" },
  { kind = "decision", id = "decision-typed-check-results" },
]

[[criteria]]
id = "diagnostic-content"
verification = ["test"]
statement = "Assert the documented fields and location accuracy for missing chapters, misplaced roles, renderer drift, and invalid input; unavailable observations or source positions are not invented."

[[criteria]]
id = "stable-presentation"
verification = ["test"]
statement = "Repeat checks with filesystem enumeration order varied and terminal styling disabled; diagnostic ordering and essential information remain consistent."

[[criteria]]
id = "agent-repair"
verification = ["test", "review"]
statement = "Record an agent repairing seeded Book defects using checker feedback and rechecking. Independent review confirms intended repairs, preserved unrelated content, and no weakened requirements; record any human assistance."
```

## Information returned

The Rust library returns typed information; the CLI renders it as readable text.
Agents can use the initial text output through their command tool. A versioned
machine-readable output format is a separate future interface.

| Information | Meaning |
| --- | --- |
| Result | `passed`, `failed`, or `incomplete` for this invocation |
| Configuration | The configuration file used, or the file that could not be loaded |
| Scope | The selected rule set and the checks actually evaluated |
| Diagnostic code | Stable identity for the type of problem |
| Kind | `structure`, `configuration`, or `operation` |
| Severity | `error` for the initial required checks; no warning policy yet |
| Summary | A short description of the problem |
| Location | Affected file or intended path, with a key or source position when available |
| Related locations | The declaration or configuration that explains the expectation |
| Expected and found | The required condition and the observed condition |
| Help | A suggested next action, including alternatives when intent cannot be inferred |
| Blocked checks | Checks that could not run and the prerequisite that blocked them |

Use the configuration directory as the displayed path base and say so in the
report. Identify the configuration argument as supplied by the caller so an
invocation from another directory remains understandable. Paths in the library
remain path values, not strings reconstructed from the display.

Source line and column positions are one-based when present. Do not invent a
line number for a missing file or an unavailable parser location; a file path
and configuration key are sufficient. Default output need not quote file
contents or dump configuration. Preserve underlying error causes in the library,
but present a concise, relevant explanation in the CLI.

If a condition cannot be observed, say so in `found`; a failed read does not
establish that a file is missing. For `input.*` diagnostics, use kind
`configuration` when the Anchor Book configuration is invalid, and `operation`
when another required input cannot be interpreted.

## Structural violation

An independently readable navigation document refers to a missing chapter:

```text
anchorbook check: failed
config: anchorbook.toml
paths: relative to the configuration directory
scope: structural/v1

error[structure.chapter-missing] (structure): Chapter file is missing
  at: book/design/storage.md
  declared by: book/SUMMARY.md -> design/storage.md
  expected: An existing chapter file within the configured source directory
  found: No file at book/design/storage.md
  help: Restore the intended chapter, or correct its navigation target if
        the chapter was moved. Preserve any required section mapping.

checks: source passed; summary passed; renderer passed;
        chapters failed; roles passed
result: failed (1 error)
next: Repair the reported structure, then rerun the same check.
files changed by check: none
```

This invocation exits with status `1`. The checker cannot know whether the
chapter was accidentally deleted or its link is wrong. It presents both
possibilities without inventing chapter content or recommending removal of a
required obligation.

## Required section in the wrong place

This diagnostic comes from a separate example where all referenced files exist:

```text
error[structure.role-not-top-level] (structure): Authority is not a top-level chapter
  at: book/SUMMARY.md -> authority/index.md
  configured by: anchorbook.toml [book.sections].authority
  expected: A top-level linked chapter for the authority role
  found: The configured chapter appears beneath Design
  help: Move the existing Authority navigation entry to the top level,
        preserving its chapter and children.
```

The help describes a structural repair. It does not authorize changing the
project's authority content. Names in messages come from the configured role
and observed navigation, not an assumption about a particular folder name.

## Renderer configuration drift

The intended Book source is `book`, but mdBook still points at `md`:

```text
error[structure.renderer-path-mismatch] (structure): mdBook source differs from the configured Book source
  at: book.toml [book].src
  configured by: anchorbook.toml [book].source
  expected: book
  found: md
  help: Set book.toml [book].src to "book" to match the intended layout.
        If the intended layout has changed, reconcile that design and
        configuration change before retrying.
```

Compare resolved path identities, not just the literal strings shown above.
The checker suggests alignment with the intended configuration; it does not
silently rewrite either file or tell the agent to weaken rules to obtain a pass.

## Invalid input or incomplete evaluation

An invalid configuration prevents structural evaluation:

```text
anchorbook check: incomplete
config: anchorbook.toml
paths: relative to the configuration directory
scope: structural/v1 requested; no structural checks evaluated

error[config.invalid-value] (configuration): Schema version must be an integer
  at: anchorbook.toml schema-version
  expected: An integer identifying a supported configuration schema
  found: String value "one"
  help: Use schema-version = 1 if this file is intended to use schema 1.

checks: source, summary, renderer, chapters, roles blocked by config.invalid-value
result: incomplete (1 error; structure not evaluated)
next: Resolve the configuration error, then rerun the same check.
files changed by check: none
```

This invocation exits with status `2`, not a structural pass or a structural
violation result. An unreadable input similarly reports `operation.read-failed`,
identifies the affected path, and explains the failure. It must not recommend
blind retries when the agent needs access or an engineer's decision.

If an operational error happens after some findings have been collected, retain
them in the same report, mark the result `incomplete`, and state exactly which
checks ran. Do not imply that those findings are an exhaustive repair list.

## Missing prerequisites

A missing `book/SUMMARY.md` is a known structural failure:

```text
error[structure.summary-missing] (structure): Book navigation is missing
  at: book/SUMMARY.md
  expected: SUMMARY.md in the configured source directory
  found: No file
  help: Restore the intended Book navigation, then rerun the check.

checks: source passed; summary failed; renderer passed;
        chapters and roles blocked by structure.summary-missing
result: failed (1 error)
```

This exits with status `1`. The dependent checks are explicitly blocked; they
are not passes. Do not generate one missing-role or missing-chapter error for
each condition that cannot be evaluated without the summary.

Thus `failed` can include checks blocked by a diagnosed structural prerequisite.
`incomplete` means invalid input or an operational problem prevents reliable
evaluation. An operational error takes precedence if both conditions occur.

## Passing result

After valid repairs, a subsequent invocation can report:

```text
anchorbook check: passed
config: anchorbook.toml
paths: relative to the configuration directory
scope: structural/v1

checks: source passed; summary passed; renderer passed;
        chapters passed; roles passed
result: passed (0 errors)
not evaluated: anchor semantics, implementation, verification evidence,
               roadmap completion, or links within chapter prose
files changed by check: none
```

This exits with status `0`. The message establishes structural conformance for
the observed inputs, not correctness of the system or permanence of the result.
The agent records its edits and rerun results separately.

## Codes and reporting behavior

`structural/v1` names the initial rule scope; it is distinct from the configuration
schema version. The five check groups above correspond to the initial structural
rules. Each records `passed`, `failed`, `error`, or `blocked`, with a reason when
not passed. Known missing structure is `failed`; read/parse/unsupported-input
failures are `error`; prerequisite-dependent checks are `blocked`.
Only a fully evaluated scope with every required group passed can produce `passed`.

Proposed initial diagnostic families include:

| Code | Meaning |
| --- | --- |
| `structure.source-invalid` | Configured source is absent or is not a directory |
| `structure.summary-missing` | Navigation file is absent |
| `structure.renderer-missing` | mdBook configuration is absent |
| `structure.renderer-path-mismatch` | Resolved source or output paths disagree |
| `structure.chapter-missing` | A referenced chapter file is absent |
| `structure.chapter-invalid` | A chapter target is not a file or escapes the source |
| `structure.role-missing` | A configured required role is absent from navigation |
| `structure.role-not-top-level` | A configured required role appears only below the top level |
| `config.invalid-value` | An input field has an invalid value or type |
| `config.unknown-field` | A configuration key is not recognized |
| `config.unsupported-version` | Configuration schema version is not supported |
| `config.path-invalid` | A configured path is unusable, overlaps, or escapes its allowed root |
| `input.parse-failed` | Required input cannot be parsed |
| `input.unsupported` | Required input contains unsupported constructs |
| `operation.read-failed` | Required input cannot be read, including an unavailable anchorbook.toml |

This is the initial vocabulary to refine during implementation. Codes identify
conditions; repeated instances include distinct locations. They are not anchor
identifiers. Keep a code's meaning stable once exposed.

Return independent findings in one run where possible. Order them by check
group (configuration, source, summary, renderer, chapters, roles), then path,
source position, code, and role when needed to break ties. Include blocked
prerequisites explicitly rather than relying on output order.

Completed reports go to stdout. Incomplete reports, including any partial
findings and the operational error, go to stderr as one coherent report.
Do not split a single report across streams. All essential information must be
readable without color, terminal control sequences, or an interactive prompt.
Help is guidance, not an automatic repair or additional authorization.
