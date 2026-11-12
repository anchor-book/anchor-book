//! Simulated post-verification results exercise the evidence model, not project
//! proofs. Native guard execution and assessment ingestion remain separate work.
use anchor_book::{
    VerificationMethod,
    adapters::verify::format_report,
    verification::{
        ClaimOutcome, CriterionReport, GuardResult, Plan, ProofGuards, ProofOutcome, ProofResult,
        ProofRun, Report, RunResults, RunnerResult, Scope, State, TestOutcome, evaluate, plan,
    },
};
use std::collections::BTreeMap;

#[path = "support/checking.rs"]
mod fixture;

fn book() -> fixture::Book {
    let b = fixture::Book::new();
    b.write(
        "anchorbook.toml",
        &format!(
            "{}\n[verification]\nsources = [\"src\"]\noutput = \"target/verify\"\nrunners = []\n",
            fixture::CONFIG
        ),
    );
    b.write(
        "Cargo.toml",
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n",
    );
    b.write("docs/authority.md", "> **`R-rule`** (draft): **A rule**\n> Statement: Preserve the specified behavior.\n> Source: Independent fixture.\n");
    let criterion = |id, methods| {
        format!(
            "> - Criterion `{id}` [{methods}]: Preserve the specified behavior.\n>   - Domain: All inputs.\n>   - Boundary: Production functions.\n>   - Assumptions: Verifier and compiler.\n>   - Exclusions: None.\n"
        )
    };
    let anchor = |id, criteria| {
        format!(
            "> **`{id}`** (draft): **Preserve behavior**\n> Property: The behavior holds.\n> Authority: `R-rule`\n{criteria}\n"
        )
    };
    b.write(
        "docs/design.md",
        &format!(
            "# Proofs\n\n{}{}",
            anchor(
                "A-first",
                format!(
                    "{}{}",
                    criterion("formal", "proof"),
                    criterion("mixed", "test, proof, review")
                )
            ),
            anchor("A-second", criterion("formal", "proof"))
        ),
    );
    b.write(
        "src/lib.rs",
        r#"
/// anchorbook: A-first#formal
/// anchorbook: A-first#mixed
/// anchorbook: A-second#formal
#[verus_spec(r => ensures r == 1)]
fn value() -> u32 { 1 }

/// anchorbook: A-first#formal
#[verus_spec(r => ensures r == 2)]
fn other() -> u32 { 2 }

/// anchorbook: A-first#mixed
#[test]
fn example() { assert_eq!(value(), 1); }
"#,
    );
    b.write("docs/roadmap.md", "# Work\n\n## Milestone 1. Preserve behavior {#work-1}\n\n**Outcome:** Behavior holds.\n\n**Status:** Complete\n\n**Identifier:** `work-1`\n\n### Completion criteria\n\n#### Behavior {#work-1-behavior}\n\nThe criteria hold.\n\n- **Anchor coverage:** `A-first` · `formal`, `mixed` — Full; `A-second` · `formal` — Full.\n");
    b
}

fn passing_guards() -> ProofGuards {
    ProofGuards {
        witnesses: GuardResult::Passed,
        assessment: GuardResult::Passed,
        sensitivity: GuardResult::Passed,
    }
}

fn run(plan: &Plan) -> ProofRun {
    ProofRun {
        complete: true,
        functions: plan
            .links
            .iter()
            .filter(|l| l.method == VerificationMethod::Proof)
            .map(|l| (l.function.clone(), ProofOutcome::Verified))
            .collect(),
        failures_elsewhere: Vec::new(),
        guards: BTreeMap::new(),
    }
}

fn report(plan: &Plan, run: ProofRun) -> Report {
    evaluate(
        plan,
        &RunResults {
            tests: RunnerResult::Ran(BTreeMap::new()),
            proofs: ProofResult::Ran(run),
        },
    )
}

fn criterion<'a>(report: &'a Report, anchor: &str, criterion: &str) -> &'a CriterionReport {
    report
        .anchors()
        .find(|a| a.id == anchor)
        .unwrap()
        .criteria
        .iter()
        .find(|c| c.id == criterion)
        .unwrap()
}

fn claim(report: &Report, anchor: &str, criterion: &str) -> ClaimOutcome {
    report
        .claims
        .iter()
        .find(|c| c.claim.anchor == anchor && c.claim.criterion == criterion)
        .unwrap()
        .outcome
        .clone()
}

/// anchorbook: A-verify-criterion-states#state-per-method
/// anchorbook: A-verify-claims-evidenced#claim-failures
#[test]
fn each_proof_guard_controls_the_method_and_its_completion_claim() {
    use GuardResult::*;
    let b = book();
    let before = b.snapshot();
    let plan = plan(&b.config(), Scope::default()).unwrap();
    assert!(plan.findings.is_empty(), "{:?}", plan.findings);
    // Expectations come from the proof-method state table, including missing
    // and stale assessments and a sensitivity run explicitly skipped.
    let cases = [
        (
            Passed,
            Passed,
            Passed,
            State::Satisfied,
            "witnesses, specification assessment, and sensitivity passed",
        ),
        (
            NoEvidence("unsatisfiable precondition has no witness".into()),
            Passed,
            Passed,
            State::NoEvidence,
            "witnesses: unsatisfiable precondition",
        ),
        (
            NoEvidence("value has no witness".into()),
            Passed,
            Passed,
            State::NoEvidence,
            "witnesses: value has no witness",
        ),
        (
            Failed("witness at src/lib.rs:9 failed".into()),
            Passed,
            Passed,
            State::Failed,
            "witnesses: witness at src/lib.rs:9 failed",
        ),
        (
            Passed,
            NoEvidence("missing record".into()),
            Passed,
            State::NoEvidence,
            "specification assessment: missing record",
        ),
        (
            Passed,
            NoEvidence("stale fingerprint".into()),
            Passed,
            State::NoEvidence,
            "specification assessment: stale fingerprint",
        ),
        (
            Passed,
            Failed("reviewer rejected the claim".into()),
            Passed,
            State::Failed,
            "specification assessment: reviewer rejected the claim",
        ),
        (
            Passed,
            Passed,
            Failed("unclassified survivor at src/lib.rs:5".into()),
            State::Failed,
            "sensitivity: unclassified survivor at src/lib.rs:5",
        ),
        (
            Passed,
            Passed,
            Inconclusive("change at src/lib.rs:5 timed out".into()),
            State::Inconclusive,
            "sensitivity: change at src/lib.rs:5 timed out",
        ),
        (
            Passed,
            Passed,
            NotEvaluated("skipped by request".into()),
            State::NotEvaluated,
            "sensitivity: skipped by request",
        ),
    ];
    for (witnesses, assessment, sensitivity, expected, reason) in cases {
        let mut run = run(&plan);
        run.guards.insert(
            ("A-first".into(), "formal".into()),
            ProofGuards {
                witnesses,
                assessment,
                sensitivity,
            },
        );
        let report = report(&plan, run);
        let actual = criterion(&report, "A-first", "formal");
        assert_eq!(actual.state, expected, "{reason}");
        assert_eq!(actual.methods[0].state, expected, "{reason}");
        assert!(
            actual.methods[0].reason.contains(reason),
            "{:?}",
            actual.methods[0]
        );
        assert!(format_report(&report).contains(reason));
        match expected {
            State::Satisfied => {
                assert_eq!(claim(&report, "A-first", "formal"), ClaimOutcome::Satisfied)
            }
            State::NotEvaluated => assert_eq!(
                claim(&report, "A-first", "formal"),
                ClaimOutcome::Unchecked("proof not evaluated".into())
            ),
            other => assert_eq!(
                claim(&report, "A-first", "formal"),
                ClaimOutcome::Unsatisfied(other)
            ),
        }
    }
    assert_eq!(
        b.snapshot(),
        before,
        "evaluating fixture outcomes writes no sources or assessments"
    );
}

/// anchorbook: A-verify-criterion-states#state-per-method
#[test]
fn guard_failures_outrank_gaps_and_all_reasons_survive() {
    use GuardResult::*;
    let b = book();
    let plan = plan(&b.config(), Scope::default()).unwrap();
    // Literal expected precedence: failure, inconclusive, no evidence, skipped.
    // Rotate obligations so no field's position decides the method's state.
    let pairs = [
        (
            Failed("rejected".into()),
            Inconclusive("timeout".into()),
            State::Failed,
        ),
        (
            Failed("rejected".into()),
            NoEvidence("missing".into()),
            State::Failed,
        ),
        (
            Failed("rejected".into()),
            NotEvaluated("skipped".into()),
            State::Failed,
        ),
        (
            Inconclusive("timeout".into()),
            NoEvidence("missing".into()),
            State::Inconclusive,
        ),
        (
            Inconclusive("timeout".into()),
            NotEvaluated("skipped".into()),
            State::Inconclusive,
        ),
        (
            NoEvidence("missing".into()),
            NotEvaluated("skipped".into()),
            State::NoEvidence,
        ),
    ];
    for (a, c, expected) in pairs {
        for [witnesses, assessment, sensitivity] in [
            [a.clone(), c.clone(), Passed],
            [c.clone(), Passed, a.clone()],
            [Passed, a, c],
        ] {
            let mut run = run(&plan);
            run.guards.insert(
                ("A-first".into(), "formal".into()),
                ProofGuards {
                    witnesses,
                    assessment,
                    sensitivity,
                },
            );
            let report = report(&plan, run);
            let method = &criterion(&report, "A-first", "formal").methods[0];
            assert_eq!(method.state, expected, "{method:?}");
            assert_eq!(method.reason.split("; ").count(), 2, "{method:?}");
        }
    }
    let mut run = run(&plan);
    run.guards.insert(
        ("A-first".into(), "formal".into()),
        ProofGuards {
            witnesses: Failed("witness failed".into()),
            assessment: NoEvidence("record stale".into()),
            sensitivity: Inconclusive("run timed out".into()),
        },
    );
    let report = report(&plan, run);
    let method = &criterion(&report, "A-first", "formal").methods[0];
    assert_eq!(method.state, State::Failed);
    assert_eq!(
        method.reason,
        "witnesses: witness failed; specification assessment: record stale; sensitivity: run timed out"
    );
}

/// anchorbook: A-verify-criterion-states#state-per-method
#[test]
fn guards_cannot_replace_target_or_link_evidence() {
    let b = book();
    let plan = plan(&b.config(), Scope::default()).unwrap();
    // Post-verification results are ignored until the original target and all
    // linked functions pass, even if contradictory results were supplied.
    for guards in [
        passing_guards(),
        ProofGuards {
            witnesses: GuardResult::Failed("unreachable post-run failure".into()),
            ..passing_guards()
        },
    ] {
        for (complete, other_failed, missing, elsewhere, expected) in [
            (false, false, false, false, State::Inconclusive),
            (true, false, false, true, State::Inconclusive),
            (false, true, true, true, State::Failed),
            (true, true, false, false, State::Failed),
            (true, false, true, false, State::NoEvidence),
        ] {
            let mut run = run(&plan);
            run.complete = complete;
            if other_failed {
                for (id, outcome) in &mut run.functions {
                    if id.name == "other" {
                        *outcome = ProofOutcome::Failed;
                    }
                }
            }
            if missing {
                run.functions.retain(|id, _| id.name != "value");
            }
            if elsewhere {
                run.failures_elsewhere.push("lemma at src/lib.rs:12".into());
            }
            run.guards
                .insert(("A-first".into(), "formal".into()), guards.clone());
            let report = report(&plan, run);
            let method = &criterion(&report, "A-first", "formal").methods[0];
            assert_eq!(method.state, expected, "{method:?}");
            assert!(!method.reason.contains("unreachable"));
        }
    }
    for (proofs, expected) in [
        (
            ProofResult::NotRun("tools unavailable".into()),
            State::NotEvaluated,
        ),
        (
            ProofResult::Unusable("build failed".into()),
            State::Inconclusive,
        ),
    ] {
        let report = evaluate(
            &plan,
            &RunResults {
                tests: RunnerResult::NotRun("none".into()),
                proofs,
            },
        );
        assert_eq!(criterion(&report, "A-first", "formal").state, expected);
    }
}

/// anchorbook: A-verify-criterion-states#state-per-method
/// anchorbook: A-verify-criterion-states#scope-and-limits
#[test]
fn guards_belong_to_one_criterion_and_cannot_supply_links_or_scope() {
    let b = book();
    let mut plan = plan(&b.config(), Scope::default()).unwrap();
    let mut run = run(&plan);
    run.guards
        .insert(("A-first".into(), "formal".into()), passing_guards());
    let first = report(&plan, run.clone());
    assert_eq!(
        criterion(&first, "A-first", "formal").state,
        State::Satisfied
    );
    // These criteria share the verified function, but neither the anchor nor
    // the criterion name alone identifies a guard result.
    assert_eq!(
        criterion(&first, "A-first", "mixed").methods[1].state,
        State::NoEvidence
    );
    assert_eq!(
        criterion(&first, "A-second", "formal").state,
        State::NoEvidence
    );
    assert!(
        criterion(&first, "A-second", "formal").methods[0]
            .reason
            .contains("proof guards have no results")
    );
    run.guards.clear();
    assert_eq!(
        criterion(&report(&plan, run.clone()), "A-first", "formal").state,
        State::NoEvidence
    );
    run.guards
        .insert(("A-first".into(), "formal".into()), passing_guards());
    plan.links
        .retain(|l| !(l.anchor == "A-first" && l.criterion == "formal"));
    let unlinked = report(&plan, run.clone());
    assert_eq!(
        criterion(&unlinked, "A-first", "formal").state,
        State::Unlinked
    );
    assert_eq!(
        claim(&unlinked, "A-first", "formal"),
        ClaimOutcome::Unsatisfied(State::Unlinked)
    );
    plan.scope.methods = vec![VerificationMethod::Test];
    let excluded = report(&plan, run);
    assert_eq!(
        criterion(&excluded, "A-first", "formal").state,
        State::NotEvaluated
    );
    assert_eq!(
        claim(&excluded, "A-first", "formal"),
        ClaimOutcome::Unchecked("proof not evaluated".into())
    );
}

/// anchorbook: A-verify-criterion-states#state-per-method
/// anchorbook: A-verify-claims-evidenced#unchecked-claims
#[test]
fn satisfied_proofs_do_not_satisfy_required_tests_or_reviews() {
    let b = book();
    let plan = plan(&b.config(), Scope::default()).unwrap();
    let mut run = run(&plan);
    for c in &plan.criteria {
        run.guards
            .insert((c.anchor.clone(), c.criterion.clone()), passing_guards());
    }
    for (test, expected_test, expected_claim, exit) in [
        (
            Some(TestOutcome::Passed),
            State::Satisfied,
            ClaimOutcome::Unchecked("review not evaluated".into()),
            0,
        ),
        (
            Some(TestOutcome::Failed),
            State::Failed,
            ClaimOutcome::Unsatisfied(State::Failed),
            1,
        ),
        (
            Some(TestOutcome::Skipped),
            State::NoEvidence,
            ClaimOutcome::Unsatisfied(State::NoEvidence),
            1,
        ),
        (
            None,
            State::NoEvidence,
            ClaimOutcome::Unsatisfied(State::NoEvidence),
            1,
        ),
    ] {
        let results = RunResults {
            tests: RunnerResult::Ran(
                test.map(|o| (plan.tests[0].clone(), o))
                    .into_iter()
                    .collect(),
            ),
            proofs: ProofResult::Ran(run.clone()),
        };
        let report = evaluate(&plan, &results);
        let mixed = criterion(&report, "A-first", "mixed");
        assert_eq!(mixed.state, State::Partial);
        assert_eq!(
            mixed.methods.iter().map(|m| m.state).collect::<Vec<_>>(),
            [expected_test, State::Satisfied, State::NotEvaluated]
        );
        assert_eq!(
            report.anchors().find(|a| a.id == "A-first").unwrap().state,
            State::Partial
        );
        assert_eq!(
            report.anchors().find(|a| a.id == "A-second").unwrap().state,
            State::Satisfied
        );
        assert_eq!(claim(&report, "A-first", "mixed"), expected_claim);
        assert_eq!(report.exit_code(), exit);
        assert!(format_report(&report).contains("satisfied means linked tests and proofs passed in this run; it does not show they are sufficient"));
    }
}

/// anchorbook: A-verify-criterion-states#state-per-method
/// anchorbook: A-verify-claims-evidenced#claim-failures
#[test]
fn passing_tests_do_not_hide_failed_proof_guards() {
    let b = book();
    let plan = plan(&b.config(), Scope::default()).unwrap();
    let mut run = run(&plan);
    for c in &plan.criteria {
        run.guards
            .insert((c.anchor.clone(), c.criterion.clone()), passing_guards());
    }
    run.guards
        .get_mut(&("A-first".into(), "mixed".into()))
        .unwrap()
        .sensitivity = GuardResult::Failed("unclassified survivor".into());
    let report = evaluate(
        &plan,
        &RunResults {
            tests: RunnerResult::Ran(
                [(plan.tests[0].clone(), TestOutcome::Passed)]
                    .into_iter()
                    .collect(),
            ),
            proofs: ProofResult::Ran(run),
        },
    );
    let mixed = criterion(&report, "A-first", "mixed");
    assert_eq!(mixed.state, State::Partial);
    assert_eq!(
        mixed.methods.iter().map(|m| m.state).collect::<Vec<_>>(),
        [State::Satisfied, State::Failed, State::NotEvaluated]
    );
    assert_eq!(
        claim(&report, "A-first", "mixed"),
        ClaimOutcome::Unsatisfied(State::Failed)
    );
    assert_eq!(report.exit_code(), 1);
}
