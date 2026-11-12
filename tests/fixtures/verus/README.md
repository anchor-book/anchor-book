# Native Verus report fixtures

These reports were captured with Verus 0.2026.10.18.7d82994, Rust 1.99.1,
and Z3 4.16.0 on Linux, built from the pinned source tags. Only verifier identity,
verification results, and timing/function breakdowns are retained; source and
proof-note payloads are omitted.

The library has an executable `linked()` returning 1 with postcondition `r == 1`,
and a proof-mode `lemma()` with postcondition `true`. Both use ordinary Rust
functions and Verus attributes. The failing variants return 2, change the lemma's
postcondition to `false`, call `assume_(false)` in the lemma, or mark the lemma
`#[verifier::external_body]`. Every run uses `--no-cheating --output-json
--time-expanded` and crate name `modes`.

`tests/proof_runner.rs` reconstructs this source and tests the reports through the
actual CLI, including independent malformed and partial variants. Its ignored
native acceptance repeats the passing, failing, supporting-lemma, trust-policy,
and correction cases through Cargo and the shared runner. See CONTRIBUTING for
the pinned-tool invocation. Captured report success establishes a target result;
it does not supply a criterion assessment or proof guards.
