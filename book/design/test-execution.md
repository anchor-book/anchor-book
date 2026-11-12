# Test Execution

A test can return success without doing the work its criterion requires. For
tests that can skip required work at runtime, `verify` combines nextest's result
with an explicit report from the test or its harness. Unqualified links retain
their ordinary test-result handling.

This interface serves [honest states](verification-evidence.md#A-verify-criterion-states),
[resolved links](verification-evidence.md#A-verify-links-resolve),
[current runs](verification-evidence.md#A-verify-runs-linked-checks), and
[completion claims](verification-evidence.md#A-verify-claims-evidenced), under
[D-test-execution-reports](../authority/decisions.md#D-test-execution-reports).

## Declaring required execution

```rust
/// anchorbook: A-agent-skill-discovery#first-session execution symposium/agent
#[tokio::test]
async fn agent_reads_a_new_skill() { /* execute and assess the agent interaction */ }
```

The optional suffix is exactly `execution <producer>/<requirement>`. Both names
begin with a lowercase ASCII letter and contain only lowercase letters, digits,
and hyphens. They must be declared in configuration. The suffix applies only to
its own link and only to `test`; proof witness syntax cannot be combined with it.
Unknown, malformed, or misplaced qualifiers are located link findings, never
ordinary links with the extra text ignored.

When the criterion can be resolved, an invalid execution qualifier leaves a
`no-evidence` obligation. The engine cannot discard that link and satisfy the
criterion using its other links. An unresolvable criterion retains the normal
located link finding.

One function can link different criteria with different requirements. Repeated
links to the same criterion with distinct requirements require **all** of them.
Identical duplicates, or a qualified and unqualified link from the same function
to the same criterion, are findings. The function runs once; requirements are
evaluated per link before combining criterion states.

```toml
[[verification.execution]]
producer = "symposium"
protocol = "json-v1"
requirements = ["simulation", "agent"]
environment = ["SYMPOSIUM_ENABLE_AGENT_TESTING", "SYMPOSIUM_TEST_AGENT"]
files = ["test-agents.toml"]
```

All five fields are required. Input lists may be empty; requirements must be
nonempty. Producer names are unique, and requirements and input names cannot
repeat. Environment names must also be valid, nonempty OS environment names.
Unknown fields and protocols are configuration errors. Files use
configuration-relative paths within the project, outside generated output. They
may be absent, but directories and unreadable files are input errors. Environment
names and values must be UTF-8.

`json-v1` supports cooperating Rust tests and harnesses under cargo-nextest
0.9.151. A producer names reviewed project code, not an executable or plugin.
Its code belongs in the scanned sources and its declaration covers the runtime
controls used to select required work. No Anchor Book dependency, custom test
harness, or new verification method is required.

## Current-run exchange

For each nextest invocation, the engine creates a fresh, exclusively owned
directory under verification output and a new invocation identifier. Concurrent
invocations never clear or reuse each other's reports. It supplies two variables:

- `ANCHORBOOK_EXECUTION_CONTEXT`: an absolute path to a context JSON file.
- `ANCHORBOOK_EXECUTION_OUTPUT`: the directory for requested result files.

The context contains schema version `1`, the invocation identifier, project root,
and one request per selected `(binary ID, test name, producer)`. A request lists
required names, declared input descriptors and initial observations, and a unique
relative result filename assigned by the engine. Only requirements in scope are
requested. The context is read-only; changing it makes affected results inconclusive.

The producer matches its name, `NEXTEST_BINARY_ID`, and `NEXTEST_TEST_NAME` to
exactly one request. It writes one final UTF-8 JSON object to the assigned filename
with exclusive creation, closing it before the test exits. A second publication
is an error. Without both Anchor Book variables, it publishes nothing and leaves
ordinary test behavior unchanged; a partially supplied context is an error.

| Result field | Required content |
| --- | --- |
| `schema` | Integer `1` |
| `invocation` | The engine's identifier from the context |
| `nextest-run` | `NEXTEST_RUN_ID`, matching the current JUnit root's `uuid` |
| `binary`, `test` | Exact identities matching the request and JUnit case |
| `attempt` | Integer `1`, from `NEXTEST_ATTEMPT` |
| `total-attempts` | Integer `1`, from `NEXTEST_TOTAL_ATTEMPTS` |
| `producer` | The configured producer name |
| `inputs-before`, `inputs-after` | Observations of all declared inputs before and after required work |
| `requirements` | Exactly one result per requested name, with no extras |

The engine passes `--retries 0`; `NEXTEST_TOTAL_ATTEMPTS` must also be `1`.
Version 1 supports one attempt, preventing an earlier execution report from
combining with a later passing test. Unsupported runner versions need native
compatibility cases before their qualified results can be evaluated.

After nextest finishes, the engine reads only assigned regular result files,
without following symlinks or searching earlier runs. Partial files, duplicate
JSON members, unknown fields, and duplicate requirements are malformed. Assigned
filenames locate affected tests even when payloads cannot be parsed. Unexpected
files supply no evidence.

## Reporting what happened

Each requirement result has `name`, `status`, `planned`, and `completed`.
Counts are nonnegative integers. `planned` counts cases required by the test's
selection policy; `completed` counts cases that performed the required work and
passed its assertions, and cannot exceed `planned`.

| Status | Meaning |
| --- | --- |
| `passed` | All required cases completed; `planned` is positive and equals `completed`. |
| `failed` | Required work or its assertions failed. |
| `unavailable` | Required work could not be performed, including zero selected cases or a partially unavailable set. Positive equal counts are invalid. |

Failed and unavailable results also require a short producer-defined `reason`
code, containing lowercase letters, digits, and hyphens. Passed results have no
reason. Counts describe required cases, not process starts or prompts. Producers
cannot discard an unavailable required case and pass the remainder. For example:

```json
[
  {"name":"simulation","status":"passed","planned":1,"completed":1},
  {"name":"agent","status":"unavailable","planned":0,"completed":0,
   "reason":"agent-testing-disabled"}
]
```

Completion is reported only after the relevant operations and assertions finish.
Selecting an agent, creating a session, entering a callback, printing a log line,
or returning `Ok(())` does not establish that work occurred. Early-return paths
explicitly report unavailable; destructors and successful harness returns must
not automatically report passed. A missing report remains missing evidence.

In Symposium, an agent case needs an explicit completion point after actual agent
interaction and assertions. The harness combines all configured required cases,
separately from simulation. Disabled testing, an enabled but empty agent list,
and an unsupported-agent early return report unavailable with distinct reasons.
Agent-only evidence links are added only after this reporting is exercised; the
engine does not infer `TestMode` or parse human log messages.

These are trusted observations from reviewed test code, like its assertions.
They do not independently prove execution or sufficiency. Review examines the
completion point, selected-case policy, and inputs. An unqualified test can still
return early: the engine cannot discover an undeclared execution requirement.

## Runtime inputs

The engine snapshots declared inputs before and after the runner invocation.
The producer observes them before and after its required work. All observations
must match the engine's initial snapshot. The producer reads actual environment
values and files; copying expected observations from the context is invalid.
An override from Cargo, nextest setup, or test code that changes a declared input
therefore makes affected evidence inconclusive.

Each observation contains `kind` (`environment` or `file`), `name`, and `state`
(`absent` or `present`). Present observations also contain `hash`: the lowercase
16-digit 64-bit FNV-1a identity of the environment value's UTF-8 bytes or the
file's raw bytes. Empty and absent differ. File names are declared relative paths.
Arrays contain every declared input once and compare by kind and name, not order.
Reading or encoding errors never become absence.

The report retains these observations and a separate runtime identity covering
producer declarations and observations, alongside existing code, configuration,
criteria, and tool identities. Declaration changes and absent, empty, or changed
values change the identity, including in later report handoffs. Even matching
input identities cannot authorize reading an earlier execution report.

Declarations cover only nonsecret controls needed to interpret results. Reports
omit environment values, file contents, credentials, prompts, responses, and raw
errors. Hashing is not secret redaction; credentials must not be declared inputs.
This is not a fingerprint of every process input. Undeclared settings, external
services, and changes restored between observations remain explicit limitations.

## Evidence and completion claims

Changed inputs make affected evidence inconclusive regardless of success.
Runner failures and unsupported runners retain their
[existing states](verification-evidence.md#evidence-states). With unchanged inputs
and usable current JUnit results, qualified links follow these rules:

| Observation | Link evidence state |
| --- | --- |
| JUnit failed | `failed`, even if the execution report is absent |
| JUnit test absent, ambiguous, or skipped | `no-evidence`; a producer result cannot replace it |
| JUnit passed; every execution result valid and passed | `satisfied` |
| JUnit passed; required execution failed | `failed` |
| JUnit passed; required execution explicitly unavailable | `not-evaluated`, with reason and counts |
| JUnit passed; assigned report absent, stale, or mismatched to invocation, test, or producer | `no-evidence`, with the specific reason |
| JUnit passed; report malformed, incomplete, or counts inconsistent | `inconclusive` |

Multiple requirements and links use the existing priority `failed`,
`inconclusive`, `no-evidence`, `not-evaluated`. The test method is satisfied only
when all its links are satisfied; some passing requirements do not introduce a
partial method state. Individual results remain visible.

For one function linking simulation and agent criteria, passing JUnit and
simulation results can satisfy the simulation link while unavailable agent work
leaves the agent link not evaluated. A JUnit failure fails every link from that
function; independent assertion outcomes need separate test functions.

Unclaimed progress still exits `0`. Explicit unavailable execution from a
selected test is an **unsatisfied checked claim** when a Complete milestone
claims that criterion, causing exit `1`. It is not unchecked merely because its
state is `not-evaluated`. Excluded methods and unavailable runners remain
unchecked. Claim evaluation must retain the reason, rather than deciding from
the state name alone. Invalid configuration still exits `2` before tests run.
