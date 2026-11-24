# Proof Guards

A [proof guard](verification.md#guards) checks what a successful Verus result
alone does not establish. This page shows when those checks happen and which
steps call Verus.

For the engine's implemented workflow, see
[Proof verification today](../working-on-anchor-book.md#proof-verification-today).

## When guards are checked

Guards are evaluated when an agent or CI asks `verify` to evaluate a proof
criterion. They are development and CI checks, not checks performed each time the
application calls a verified function. The checks use Verus results, additional
Verus runs, and executable checks at different points:

| When | Check | Work performed |
| --- | --- | --- |
| During the original Verus run | Trust policy | `--no-cheating` rejects project assumptions and trusted bodies. |
| After the original target verifies completely | Witness coverage | Check that each linked function with preconditions has a witness verified in that same run. The witness's proof was checked by Verus; Anchor Book checks its association and result. |
| After the original target verifies completely | Sensitivity | Make deliberate changes to executable code in fresh copies and run Verus again for each change. Classify survivors using the covering tests. |
| When all required results are available | Proof-method state | Combine the target result, witness coverage, and sensitivity. Only complete passing evidence satisfies the proof method. |

The original code's verification must succeed before the post-verification
stage starts. The [execution diagram](#execution-sequence)
shows the additional Verus calls. A sensitivity run checks a deliberately changed
copy; the project's source files, contracts, and proofs stay unchanged.

For example, the authority validator can pass its original proof. Sensitivity
then changes its accepted-status check to always pass in a disposable copy and
asks Verus to verify that copy. Rejection by the relevant contract shows that the
proof detects this defect; a surviving change needs the
[survivor classification](verification-evidence.md#proof-guards). The first
passing run alone supplies no sensitivity result.

Specification review belongs to development. It does not supply a guard result;
current complete executable evidence is the basis for satisfaction. Code or
specification changes require fresh verification and sensitivity checks.

`anchorbook proof` is the [target gate](verification-evidence.md#ci-proof-gate):
it stops at the original target's result and does not establish these additional
obligations. `verify` reports the complete proof method and any other methods the
criterion requires. See [project guidance](../working-on-anchor-book.md#proof-verification-today)
for the implemented portion of this workflow.

## Execution sequence

The post-verification [guard stage](#when-guards-are-checked)
starts after a complete successful target result. The trust policy is already
enforced during that original Verus invocation, and witness functions in the
target are verified in that run. Anchor Book then checks witness coverage
and sensitivity for the proof criteria in scope.
`anchorbook proof` stops at the target gate; this diagram shows the complete
`verify` workflow.

```mermaid
%%{init: {"sequence": {"useMaxWidth": false, "actorMargin": 30, "width": 130}}}%%
sequenceDiagram
    actor Caller as Agent or CI
    participant Book as Anchor Book
    participant Verus
    Caller->>Book: verify selected<br/>proof criteria
    Book->>Verus: Original whole target<br/>with --no-cheating
    Note over Book,Verus: Witness functions are verified<br/>in this same run
    Verus-->>Book: Target and function results
    alt Complete successful target result
        Book->>Book: Check witness coverage
        loop Each deliberate change to executable code
            Book->>Verus: Verify changed target<br/>in a fresh copy
            Verus-->>Book: Result and failing obligations
            Book->>Book: Classify result and run<br/>covering tests for survivors
        end
        Book->>Book: Combine target and guards
    else Failed or incomplete target
        Book->>Book: Keep failure or<br/>incomplete evidence<br/>without post-run guards
    end
    Book-->>Caller: Proof-method state,<br/>evidence, and gaps
```

The sequence details one proof method; required tests retain their own results.
The surrounding [Book build](verification-evidence.md#book-build-evidence)
consumes the evaluated report before rendering, without starting verification
again inside the renderer.
