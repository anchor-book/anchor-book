# Run with jq -s --arg version "$VERUS_VERSION" -f .github/verus-report.jq.
# Keep public evidence to result fields; omit source/proof-note payloads.
. as $reports |
[
  "has_required_authority", "matches_identity", "matches_target_kind",
  "lookup", "has_unknown_identity", "validate_reference", "validate_authorities"
] | map("anchor_book::records::authority::" + .) as $expected |
($reports[0] // {}) as $report |
($report["verification-results"] // {}) as $result |
([$report["times-ms"].smt["smt-run-module-times"][]?
  | .["function-breakdown"][]?
  | {function, mode: .["mode:"], success}]) as $functions |
{
  complete: (
    ($reports | length) == 1
    and $report.verus.version == $version
    and $result.success == true
    and $result["encountered-error"] == false
    and $result["encountered-vir-error"] == false
    and $result["is-verifying-entire-crate"] == true
    and ($result.verified | type) == "number"
    and $result.verified > 0
    and $result.errors == 0
    and all($functions[]; .success == true)
    and all($expected[]; . as $name |
      any($functions[]; .function == $name and .mode == "exec" and .success == true))
  ),
  report_count: ($reports | length),
  verifier: ($report.verus | {version, commit, platform, toolchain}),
  result: $result,
  expected_functions: $expected,
  functions: $functions
}
