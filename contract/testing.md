## Test before opening a pull request

Apply these requirements to implementation pull requests in Building. During
Prototyping, test assigned experiments and record results and gaps while refining
the testing workflow. A written policy does not establish that the project's
tooling or CI implements it; project guidance records that readiness.

Before opening an implementation pull request:

- Add or update **unit tests** for changed behavior, failure cases, and boundaries.
  Bug fixes need a regression test exposing the defect. Derive expectations from
  design and anchor criteria, not implementation output.
- Run relevant **integration and acceptance tests**, exercising the actual CLI or
  adapter when its behavior is affected. Use independently specified fixtures as
  well as representative project acceptance cases; a successful build alone is
  insufficient.
- Run **mutation testing** on affected implementation logic after ordinary tests pass,
  to assess whether tests detect deliberately introduced faults. Review surviving
  mutations and strengthen tests where they expose gaps. Document equivalent
  mutations and exclusions with reasons. Timeouts, untested mutations, and tool
  failures do not demonstrate fault detection.
- Run relevant formatting, Clippy, and supported-toolchain checks. Build the Book
  and check affected records, links, and diagrams for content or rendering changes.
  Use documented project commands.
- Record revision, commands, results, mutation scope and findings, and remaining
  gaps in the pull request and applicable roadmap evidence. Report CI as pending
  until it runs; unresolved required checks prevent a ready-to-merge claim.

Prose or presentation changes need relevant Book checks, not invented unit tests
or mutation runs on unchanged implementation logic.

Before Building adopts this policy, agree on mutation tooling, scope, runtime
budget, survivor and exclusion handling, and local/CI commands. Record the setup
in roadmaps and contributor instructions. Mutation results assess test sensitivity;
they do not prove an anchor's property.

### Iterate on test findings before merge

For assigned implementation, investigate failing tests and surviving mutations.
Use the goal design and anchor criteria to distinguish an implementation defect,
an inadequate test, an equivalent mutation, and an inconclusive tool result.
Correct code or strengthen tests as appropriate, then rerun ordinary tests and
the affected mutations. Repeat the required verification scope before declaring
the change ready. Preserve the finding and record the correction and new results.
Do not weaken assertions, alter intended behavior, or exclude unexplained mutants
to obtain a passing run; resolve changes outside the assignment with the engineer.

Run local checks before an authorized push, then inspect the pull request's CI
results for that revision. Agents can make corrections locally while CI executes
independently. Repeat the correction and verification loop for CI failures.
Pending, cancelled, stale, or failed required checks do not establish merge
readiness. Following this loop does not itself authorize pushing or merging.

