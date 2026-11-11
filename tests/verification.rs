//! `verify` against independently specified fixture Books and sources: links,
//! evidence states, claims, configuration, and the actual CLI.
use anchor_book::VerificationMethod;
use anchor_book::verification::{
    ClaimOutcome, FunctionId, Plan, ProofOutcome, ProofResult, ProofRun, Report, RunResults,
    RunnerResult, Scope, State, Target, TestOutcome, evaluate, plan,
};
use std::{collections::BTreeMap, path::PathBuf, process::Command};

#[path = "support/checking.rs"]
mod fixture;

const VERIFICATION: &str = "\n[verification]\nsources = [\"src\", \"tests\", \"other\"]\noutput = \"target/verify\"\nrunners = [\"nextest\"]\n";

/// A line-grammar anchor citing `R-r`; methods are written as in the record, such
/// as `test, review`, and a criterion requiring proof gets its four scope items.
fn anchor(id: &str, criteria: &[(&str, &str)]) -> String {
    let mut out = format!(
        "> **`{id}`** (draft): **Anchor {id}**\n> Property: A property.\n> Authority: `R-r`\n"
    );
    for (criterion, methods) in criteria {
        out.push_str(&format!(
            "> - Criterion `{criterion}` [{methods}]: Criterion {criterion}.\n"
        ));
        if methods.contains("proof") {
            out.push_str(
                ">   - Domain: All inputs.\n>   - Boundary: The function.\n>   - Assumptions: The verifier is trusted.\n>   - Exclusions: None.\n",
            );
        }
    }
    out.push('\n');
    out
}

fn design() -> String {
    format!(
        "# Fixture design\n\n{}{}{}",
        anchor(
            "A-tests",
            &[("one", "test"), ("two", "test"), ("both", "test, review")]
        ),
        anchor(
            "A-proof",
            &[("formal", "proof"), ("checked", "test, proof")]
        ),
        anchor("A-unlinked", &[("none", "test")]),
    )
}

const AUTHORITY: &str = "# Authority\n\n> **`R-r`** (draft): **A requirement**\n> Statement: Something must hold.\n> Source: A fixture.\n";

fn milestone(number: u32, status: &str, coverage: &str) -> String {
    format!(
        "## Milestone {number}. Work {number} {{#work-{number}}}\n\n**Outcome:** Something works.\n\n**Status:** {status}\n\n**Identifier:** `work-{number}`\n\n### Completion criteria\n\n#### It works {{#work-{number}-works}}\n\nThe condition.\n\n- **Anchor coverage:** {coverage}\n- **Evidence:** Pending.\n\n### Completion record\n\nPending.\n\n"
    )
}

const LIB: &str = r#"//! Fixture library.

/// anchorbook: A-proof#formal
#[cfg_attr(feature = "verus", verus_spec(r => ensures r == 1))]
pub fn verified() -> u32 {
    1
}

/// anchorbook: A-tests#one
pub fn not_a_test() {}

pub struct Thing;
impl Thing {
    /// anchorbook: A-tests#one
    #[test]
    fn method() {}
}

#[cfg(test)]
mod tests {
    /// Ordinary documentation may surround a link.
    /// anchorbook: A-tests#one
    #[test]
    fn same_name() {}

    mod inner {
        /// anchorbook: A-tests#two
        #[test]
        fn same_name() {}
    }
}
"#;

const TESTS: &str = r#"// anchorbook: A-tests#one
#[test]
fn ordinary_comment() {}

/// anchorbook: A-tests#one
/// anchorbook: A-tests#both
#[tokio::test]
async fn linked_twice() {}

/// anchorbook: A-tests#one
/// anchorbook: A-tests#one
#[test]
fn duplicate() {}

/// anchorbook: no-such#one
#[test]
fn dangling_anchor() {}

/// anchorbook: A-tests#missing
#[test]
fn dangling_criterion() {}

/// anchorbook: A-proof#formal
#[test]
fn method_mismatch() {}

/// anchorbook: A-tests#one extra
#[test]
fn malformed() {}

/// anchorbook: A-proof#checked
#[test]
fn checked_by_test() {}

some_macro! {
    /// anchorbook: A-tests#one
    fn inside() {}
}

/// anchorbook: A-tests#two
#[test]
fn same_name() {}
"#;

fn book(roadmap: &str) -> fixture::Book {
    let book = fixture::Book::new();
    book.write(
        "anchorbook.toml",
        &format!("{}{VERIFICATION}", fixture::CONFIG),
    );
    book.write("docs/design.md", &design());
    book.write("docs/authority.md", AUTHORITY);
    book.write("docs/roadmap.md", &format!("# Work\n\n{roadmap}"));
    book.write(
        "Cargo.toml",
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n",
    );
    book.write("src/lib.rs", LIB);
    book.write("tests/it.rs", TESTS);
    // A second package with a test of the same name in a target of the same name.
    book.write(
        "other/Cargo.toml",
        "[package]\nname = \"other\"\nversion = \"0.1.0\"\n",
    );
    book.write(
        "other/tests/it.rs",
        "/// anchorbook: A-tests#two\n#[test]\nfn same_name() {}\n",
    );
    book
}

fn function(target: Target, module: &[&str], name: &str) -> FunctionId {
    FunctionId {
        package: "fixture".into(),
        target,
        module: module.iter().map(|s| (*s).to_owned()).collect(),
        name: name.into(),
    }
}

fn it(name: &str) -> FunctionId {
    function(Target::Test("it".into()), &[], name)
}

fn whole(book: &fixture::Book) -> Plan {
    plan(&book.config(), Scope::default()).unwrap_or_else(|e| panic!("{e}"))
}

fn state(report: &Report, anchor: &str, criterion: &str) -> (State, Vec<State>) {
    let criterion = report
        .anchors()
        .find(|a| a.id == anchor)
        .unwrap()
        .criteria
        .iter()
        .find(|c| c.id == criterion)
        .unwrap();
    (
        criterion.state,
        criterion.methods.iter().map(|m| m.state).collect(),
    )
}

/// anchorbook: A-verify-links-resolve#link-findings
#[test]
fn links_resolve_to_functions_and_every_misplaced_or_invalid_link_is_located() {
    let book = book("");
    let plan = whole(&book);
    let mut links: Vec<(String, String, FunctionId, usize)> = plan
        .links
        .iter()
        .map(|l| {
            (
                l.anchor.clone(),
                l.criterion.clone(),
                l.function.clone(),
                l.line,
            )
        })
        .collect();
    links.sort();
    let lib = |module: &[&str], name: &str| function(Target::Lib, module, name);
    let mut expected = vec![
        ("A-proof".into(), "formal".into(), lib(&[], "verified"), 3),
        (
            "A-proof".into(),
            "checked".into(),
            it("checked_by_test"),
            31,
        ),
        (
            "A-tests".into(),
            "one".into(),
            lib(&["tests"], "same_name"),
            22,
        ),
        (
            "A-tests".into(),
            "two".into(),
            lib(&["tests", "inner"], "same_name"),
            27,
        ),
        ("A-tests".into(), "one".into(), it("linked_twice"), 5),
        ("A-tests".into(), "both".into(), it("linked_twice"), 6),
        ("A-tests".into(), "one".into(), it("duplicate"), 10),
        ("A-tests".into(), "two".into(), it("same_name"), 40),
        (
            "A-tests".into(),
            "two".into(),
            FunctionId {
                package: "other".into(),
                ..it("same_name")
            },
            1,
        ),
    ];
    expected.sort();
    assert_eq!(links, expected);
    let methods: Vec<_> = plan
        .links
        .iter()
        .map(|l| (l.function.name.as_str(), l.method))
        .collect();
    assert!(methods.contains(&("verified", VerificationMethod::Proof)));
    assert!(methods.contains(&("checked_by_test", VerificationMethod::Test)));

    let findings: Vec<(String, usize, &str)> = plan
        .findings
        .iter()
        .map(|f| (f.path.display().to_string(), f.line, f.code))
        .collect();
    assert_eq!(
        findings,
        [
            ("src/lib.rs".to_owned(), 9, "verify.link-unsupported"),
            ("src/lib.rs".to_owned(), 14, "verify.link-unsupported"),
            ("tests/it.rs".to_owned(), 1, "verify.link-misplaced"),
            ("tests/it.rs".to_owned(), 11, "verify.link-duplicate"),
            ("tests/it.rs".to_owned(), 15, "verify.link-dangling"),
            ("tests/it.rs".to_owned(), 19, "verify.link-dangling"),
            ("tests/it.rs".to_owned(), 23, "verify.link-method-mismatch"),
            ("tests/it.rs".to_owned(), 27, "verify.link-malformed"),
            ("tests/it.rs".to_owned(), 36, "verify.link-unsupported"),
        ]
    );
    let message = |line| {
        &plan
            .findings
            .iter()
            .find(|f| f.line == line)
            .unwrap()
            .message
    };
    assert!(message(9).contains("neither a test"));
    assert!(message(14).contains("methods"));
    assert!(message(23).contains("requires proof"));
    assert!(message(36).contains("macro"));

    // Correcting the ordinary comment turns it into a link and removes its finding.
    book.write(
        "tests/it.rs",
        &TESTS.replacen("// anchorbook", "/// anchorbook", 1),
    );
    let corrected = whole(&book);
    assert!(
        !corrected
            .findings
            .iter()
            .any(|f| f.path.ends_with("it.rs") && f.line == 1)
    );
    assert!(
        corrected
            .links
            .iter()
            .any(|l| l.function == it("ordinary_comment"))
    );
}

/// anchorbook: A-verify-links-resolve#link-findings
#[test]
fn identities_defined_twice_are_ambiguous_and_produce_no_link() {
    let book = book("");
    book.write(
        "tests/twice.rs",
        "/// anchorbook: A-tests#two\n#[cfg(unix)]\n#[test]\nfn twice() {}\n\n/// anchorbook: A-tests#two\n#[cfg(not(unix))]\n#[test]\nfn twice() {}\n",
    );
    let plan = whole(&book);
    let twice = function(Target::Test("twice".into()), &[], "twice");
    assert!(!plan.links.iter().any(|l| l.function == twice));
    let ambiguous: Vec<usize> = plan
        .findings
        .iter()
        .filter(|f| f.code == "verify.link-ambiguous")
        .map(|f| f.line)
        .collect();
    assert_eq!(ambiguous, [1, 6]);
}

/// anchorbook: A-verify-criterion-states#state-per-method
#[test]
fn every_method_state_comes_from_links_and_this_runs_results() {
    let book = book("");
    let plan = whole(&book);
    let lib_test = function(Target::Lib, &["tests"], "same_name");
    let ran = |outcomes: &[(FunctionId, TestOutcome)]| RunResults {
        tests: RunnerResult::Ran(outcomes.iter().cloned().collect::<BTreeMap<_, _>>()),
        proofs: ProofResult::NotRun("no proof runner".into()),
    };
    let passing = [
        (lib_test.clone(), TestOutcome::Passed),
        (it("linked_twice"), TestOutcome::Passed),
        (it("duplicate"), TestOutcome::Passed),
    ];
    use State::*;
    let report = evaluate(&plan, &ran(&passing));
    assert_eq!(
        state(&report, "A-tests", "one"),
        (Satisfied, vec![Satisfied])
    );
    // A linked test absent from the results has no evidence.
    assert_eq!(
        state(&report, "A-tests", "two"),
        (NoEvidence, vec![NoEvidence])
    );
    // Two required methods with one satisfied: the criterion is partial.
    assert_eq!(
        state(&report, "A-tests", "both"),
        (Partial, vec![Satisfied, NotEvaluated])
    );
    assert_eq!(
        state(&report, "A-unlinked", "none"),
        (Unlinked, vec![Unlinked])
    );
    let anchor = |id: &str| report.anchors().find(|a| a.id == id).unwrap().state;
    assert_eq!(anchor("A-tests"), Partial);
    assert_eq!(anchor("A-unlinked"), Unlinked);

    let mut failing = passing.to_vec();
    failing[2].1 = TestOutcome::Failed;
    assert_eq!(
        state(&evaluate(&plan, &ran(&failing)), "A-tests", "one").0,
        Failed
    );
    failing[2].1 = TestOutcome::Skipped;
    assert_eq!(
        state(&evaluate(&plan, &ran(&failing)), "A-tests", "one").0,
        NoEvidence
    );

    let runner = |tests: RunnerResult<TestOutcome>| RunResults {
        tests,
        proofs: ProofResult::NotRun("no proof runner".into()),
    };
    let unusable = evaluate(
        &plan,
        &runner(RunnerResult::Unusable("did not build".into())),
    );
    assert_eq!(state(&unusable, "A-tests", "one").0, Inconclusive);
    let not_run = evaluate(&plan, &runner(RunnerResult::NotRun("not installed".into())));
    assert_eq!(state(&not_run, "A-tests", "one").0, NotEvaluated);
    // A runner that cannot run never hides that a criterion is unlinked.
    assert_eq!(state(&not_run, "A-unlinked", "none").0, Unlinked);

    let proofs = |proofs: ProofResult| RunResults {
        tests: RunnerResult::NotRun("no test runner".into()),
        proofs,
    };
    let verified = function(Target::Lib, &[], "verified");
    let run = |complete, outcome: Option<ProofOutcome>, elsewhere: &[&str]| {
        proofs(ProofResult::Ran(ProofRun {
            complete,
            functions: outcome.map(|o| (verified.clone(), o)).into_iter().collect(),
            failures_elsewhere: elsewhere.iter().map(|s| (*s).to_owned()).collect(),
        }))
    };
    let formal = |results: &RunResults| state(&evaluate(&plan, results), "A-proof", "formal").0;
    assert_eq!(formal(&run(true, Some(ProofOutcome::Failed), &[])), Failed);
    // A linked function's own failure outranks failures elsewhere and incompleteness.
    assert_eq!(
        formal(&run(false, Some(ProofOutcome::Failed), &["lemma"])),
        Failed
    );
    assert_eq!(
        formal(&run(
            true,
            Some(ProofOutcome::Verified),
            &["lemma at x.rs:3"]
        )),
        Inconclusive
    );
    assert_eq!(
        formal(&run(false, Some(ProofOutcome::Verified), &[])),
        Inconclusive
    );
    assert_eq!(formal(&run(true, None, &[])), NoEvidence);
    // Verified functions still lack the specification assessment the proof method needs.
    assert_eq!(
        formal(&run(true, Some(ProofOutcome::Verified), &[])),
        NoEvidence
    );
    assert_eq!(
        formal(&proofs(ProofResult::Unusable("no report".into()))),
        Inconclusive
    );
    assert_eq!(
        formal(&proofs(ProofResult::NotRun("absent".into()))),
        NotEvaluated
    );
}

/// anchorbook: A-verify-criterion-states#scope-and-limits
#[test]
fn a_scoped_run_reports_what_it_excludes_and_plans_only_its_scope() {
    let book = book("");
    let scope = Scope {
        anchors: vec!["A-tests".into()],
        methods: vec![VerificationMethod::Test],
    };
    let plan = plan(&book.config(), scope).unwrap();
    assert_eq!(
        plan.anchors
            .iter()
            .map(|a| a.id.as_str())
            .collect::<Vec<_>>(),
        ["A-tests"]
    );
    let mut tests = plan.tests.clone();
    tests.sort();
    let mut expected = vec![
        function(Target::Lib, &["tests"], "same_name"),
        function(Target::Lib, &["tests", "inner"], "same_name"),
        it("linked_twice"),
        it("duplicate"),
        it("same_name"),
        FunctionId {
            package: "other".into(),
            ..it("same_name")
        },
    ];
    expected.sort();
    assert_eq!(tests, expected);
    assert!(plan.proof_targets.is_empty());
    let report = evaluate(&plan, &RunResults::not_run("none"));
    let (_, methods) = state(&report, "A-tests", "both");
    assert_eq!(methods, [State::NotEvaluated, State::NotEvaluated]);
    let reason = &report.anchors().next().unwrap().criteria[2].methods[1].reason;
    assert!(reason.contains("outside the requested scope"), "{reason}");

    let whole_plan = whole(&book);
    assert_eq!(
        whole_plan.proof_targets,
        [("fixture".to_owned(), Target::Lib)]
    );
}

/// anchorbook: A-verify-claims-evidenced#unchecked-claims
/// anchorbook: A-verify-claims-evidenced#claim-failures
#[test]
fn complete_milestones_claim_their_full_coverage_and_fail_the_run_until_it_holds() {
    let roadmap = [
        milestone(
            1,
            "Complete",
            "`A-tests` · `one` — Full; `A-tests` · `two` — Partial: one case",
        ),
        milestone(2, "In progress", "`A-unlinked` · `none` — Full"),
        milestone(
            3,
            "Complete — merged",
            "`A-unlinked` · `none` — Partial: setup",
        ),
        milestone(4, "Complete", "`A-tests` · `both` — Full"),
    ]
    .concat();
    let book = book(&roadmap);
    let plan = whole(&book);
    let claimed: Vec<_> = plan
        .claims
        .iter()
        .map(|c| {
            (
                c.milestone.as_str(),
                c.completion_criterion.as_str(),
                c.anchor.as_str(),
                c.criterion.as_str(),
            )
        })
        .collect();
    assert_eq!(
        claimed,
        [
            ("work-1", "work-1-works", "A-tests", "one"),
            ("work-4", "work-4-works", "A-tests", "both")
        ]
    );
    let outcome = |report: &Report, criterion: &str| {
        report
            .claims
            .iter()
            .find(|c| c.claim.criterion == criterion)
            .unwrap()
            .outcome
            .clone()
    };

    // Linked but not run: unchecked, never satisfied, and the run does not fail.
    let none = evaluate(&plan, &RunResults::not_run("not installed"));
    assert!(matches!(outcome(&none, "one"), ClaimOutcome::Unchecked(_)));
    assert_eq!(none.exit_code(), 0);

    let results = |outcome: TestOutcome| RunResults {
        tests: RunnerResult::Ran(
            [
                (function(Target::Lib, &["tests"], "same_name"), outcome),
                (it("linked_twice"), outcome),
                (it("duplicate"), outcome),
            ]
            .into_iter()
            .collect(),
        ),
        proofs: ProofResult::NotRun("none".into()),
    };
    let failing = evaluate(&plan, &results(TestOutcome::Failed));
    assert_eq!(
        outcome(&failing, "one"),
        ClaimOutcome::Unsatisfied(State::Failed)
    );
    assert_eq!(failing.exit_code(), 1);
    let passing = evaluate(&plan, &results(TestOutcome::Passed));
    assert_eq!(outcome(&passing, "one"), ClaimOutcome::Satisfied);
    // `both` also requires review, which is not read yet: unchecked, not satisfied.
    assert!(
        matches!(outcome(&passing, "both"), ClaimOutcome::Unchecked(r) if r.contains("review"))
    );
    assert_eq!(passing.exit_code(), 0);

    // An unlinked claimed criterion fails the run even when no runner ran.
    let book = self::book(&milestone(5, "Complete", "`A-unlinked` · `none` — Full"));
    let report = evaluate(&whole(&book), &RunResults::not_run("not installed"));
    assert_eq!(
        outcome(&report, "none"),
        ClaimOutcome::Unsatisfied(State::Unlinked)
    );
    assert_eq!(report.exit_code(), 1);
    // Outside the requested anchors, the claim is unchecked.
    let scoped = plan_scoped(&book, "A-tests");
    let report = evaluate(&scoped, &RunResults::not_run("not installed"));
    assert!(
        matches!(outcome(&report, "none"), ClaimOutcome::Unchecked(r) if r.contains("outside"))
    );
    assert_eq!(report.exit_code(), 0);
}

fn plan_scoped(book: &fixture::Book, anchor: &str) -> Plan {
    plan(
        &book.config(),
        Scope {
            anchors: vec![anchor.into()],
            methods: Vec::new(),
        },
    )
    .unwrap()
}

fn cli(book: &fixture::Book, args: &[&str]) -> (Option<i32>, String, String) {
    // No cargo at this path, so nextest is never found and nothing is run.
    cli_with(book, args, &[("CARGO", "/nonexistent/cargo".into())])
}

fn cli_with(
    book: &fixture::Book,
    args: &[&str],
    env: &[(&str, std::ffi::OsString)],
) -> (Option<i32>, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .arg("verify")
        .arg("--config")
        .arg(book.config())
        .args(args)
        .envs(env.iter().map(|(k, v)| (*k, v.clone())))
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

/// anchorbook: A-verify-runs-linked-checks#source-preservation
/// anchorbook: A-verify-progress-report#incomplete-runs
/// anchorbook: A-verify-progress-report#progress-output
#[test]
fn the_cli_reports_progress_claims_and_incomplete_runs_and_changes_no_files() {
    let book = book(&milestone(1, "Complete", "`A-unlinked` · `none` — Full"));
    let before = book.snapshot();
    let (code, stdout, stderr) = cli(&book, &[]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(stderr.is_empty());
    for expected in [
        "anchorbook verify: claims unsatisfied",
        "scope: all anchors; methods test, proof, review",
        "Fixture design (design.md)",
        "  A-tests (draft) — not-evaluated, 0 of 3 criteria satisfied",
        "    one [test]: not-evaluated — cargo-nextest is not installed; install it with `cargo install cargo-nextest --version 0.9.151 --locked`",
        "    none [test]: unlinked — no test link names the criterion",
        "      review: not-evaluated — assessment records are not read yet",
        "  verify.link-misplaced: tests/it.rs:1:",
        "claims: 1 claim: 0 satisfied, 1 unsatisfied, 0 unchecked",
        "  unsatisfied: A-unlinked#none (unlinked), claimed by work-1-works at roadmap.md:",
        "no Git revision",
        "files changed: none",
    ] {
        assert!(
            stdout.contains(expected),
            "missing {expected:?} in\n{stdout}"
        );
    }
    // The library and the CLI agree on every anchor's state.
    let report = evaluate(
        &whole(&book),
        &RunResults::not_run("cargo-nextest is not installed"),
    );
    for anchor in report.anchors() {
        assert!(stdout.contains(&format!(
            "  {} (draft) — {},",
            anchor.id,
            anchor.state.as_str()
        )));
    }
    let (code, stdout, _) = cli(&book, &["--anchor", "A-tests", "--method", "test"]);
    assert_eq!(code, Some(0));
    assert!(stdout.contains("scope: anchors A-tests; methods test"));
    assert!(!stdout.contains("A-unlinked (draft)"));

    for (args, message) in [
        (vec!["--anchor", "no-such"], "verify.invalid-scope"),
        (vec!["--anchor", "R-r"], "verify.invalid-scope"),
        (vec!["--method", "guess"], "Usage"),
    ] {
        let (code, stdout, stderr) = cli(&book, &args);
        assert_eq!(code, Some(2), "{args:?}");
        assert!(stdout.is_empty());
        assert!(stderr.contains(message), "{stderr}");
    }
    book.write("docs/SUMMARY.md", "# Summary\n\n- [Design](design.md)\n");
    let (code, _, stderr) = cli(&book, &[]);
    assert_eq!(code, Some(2));
    assert!(stderr.contains("verify.check-failed"), "{stderr}");
    book.write("docs/SUMMARY.md", fixture::SUMMARY);
    let after = book.snapshot();
    let changed: Vec<_> = before
        .keys()
        .filter(|k| before.get(*k) != after.get(*k))
        .collect();
    assert_eq!(changed, Vec::<&PathBuf>::new());
    assert!(
        !book.path("target").exists(),
        "verify created runner output"
    );
}

/// anchorbook: A-verify-progress-report#incomplete-runs
#[test]
fn verification_configuration_is_validated_by_verify_and_ignored_by_check() {
    let book = book("");
    let config = |section: &str| format!("{}\n[verification]\n{section}", fixture::CONFIG);
    for (section, message) in [
        (
            "sources = [\"src\"]\noutput = \"target/v\"\nrunners = []\nextra = 1\n",
            "unknown field",
        ),
        (
            "sources = []\noutput = \"target/v\"\nrunners = []\n",
            "at least one",
        ),
        (
            "sources = [\"missing\"]\noutput = \"target/v\"\nrunners = []\n",
            "not a directory",
        ),
        (
            "sources = [\"src\", \"src\"]\noutput = \"target/v\"\nrunners = []\n",
            "twice",
        ),
        (
            "sources = [\"src\"]\noutput = \"src/out\"\nrunners = []\n",
            "separate from source",
        ),
        (
            "sources = [\"src\"]\noutput = \"docs/out\"\nrunners = []\n",
            "Book source",
        ),
        (
            "sources = [\"src\"]\noutput = \"generated/site\"\nrunners = []\n",
            "Book source and output",
        ),
        (
            "sources = [\"src\"]\noutput = \"../out\"\nrunners = []\n",
            "escapes",
        ),
        (
            "sources = [\"src\"]\noutput = \"target/v\"\nrunners = [\"cargo\"]\n",
            "not supported",
        ),
        (
            "sources = [\"src\"]\noutput = \"target/v\"\nrunners = [\"nextest\", \"nextest\"]\n",
            "twice",
        ),
        ("sources = [\"src\"]\nrunners = []\n", "output"),
    ] {
        book.write("anchorbook.toml", &config(section));
        let error = plan(&book.config(), Scope::default())
            .unwrap_err()
            .to_string();
        assert!(error.starts_with("verify.invalid-configuration"), "{error}");
        assert!(error.contains(message), "{section}: {error}");
        let report = anchor_book::checking::check(&book.config()).unwrap();
        assert_eq!(
            report.outcome(),
            anchor_book::checking::Outcome::Passed,
            "{section}"
        );
    }
    book.write("anchorbook.toml", fixture::CONFIG);
    let error = plan(&book.config(), Scope::default())
        .unwrap_err()
        .to_string();
    assert!(error.contains("needs a [verification] section"), "{error}");
}

/// anchorbook: A-verify-links-resolve#link-findings
#[test]
fn every_supported_target_and_function_form_is_identified() {
    let book = book("");
    let link = |criterion: &str, attr: &str, name: &str| {
        format!("/// anchorbook: {criterion}\n{attr}\nfn {name}() {{}}\n")
    };
    let test = |name: &str| link("A-tests#two", "#[test]", name);
    book.write("src/nested.rs", &test("in_file_module"));
    book.write("src/main.rs", &test("in_main"));
    book.write("src/bin/tool.rs", &test("in_bin"));
    book.write("src/bin/multi/main.rs", "mod part;\nfn main() {}\n");
    book.write("src/bin/multi/part.rs", &test("in_bin_module"));
    book.write("tests/dir/main.rs", "mod helper;\n");
    book.write("tests/dir/helper.rs", &test("in_test_module"));
    book.write("tests/support/helper.rs", &test("in_helper"));
    book.write(
        "src/direct.rs",
        &link(
            "A-proof#formal",
            "#[verus_spec(r => ensures r == 1)]",
            "direct",
        ),
    );
    book.write(
        "src/witness.rs",
        &[
            link(
                "A-proof#formal witness verified",
                "#[cfg_attr(feature = \"verus\", verus_spec)]",
                "witness_fn",
            ),
            link("A-tests#two witness verified", "#[test]", "test_witness"),
        ]
        .concat(),
    );
    book.write(
        "src/block.rs",
        "/**\n * anchorbook: A-tests#two\n */\n#[test]\nfn block_doc() {}\n",
    );
    book.write(
        "src/misplaced.rs",
        "/// anchorbook: A-tests#two\npub struct Thing;\n",
    );
    let plan = whole(&book);
    let find = |name: &str| plan.links.iter().find(|l| l.function.name == name);
    for (name, target, module) in [
        ("in_file_module", Target::Lib, vec!["nested"]),
        ("in_main", Target::Bin("fixture".into()), vec![]),
        ("in_bin", Target::Bin("tool".into()), vec![]),
        ("in_bin_module", Target::Bin("multi".into()), vec!["part"]),
        ("in_test_module", Target::Test("dir".into()), vec!["helper"]),
        ("direct", Target::Lib, vec!["direct"]),
        ("block_doc", Target::Lib, vec!["block"]),
    ] {
        let link = find(name).unwrap_or_else(|| panic!("no link for {name}"));
        assert_eq!(link.function, function(target, &module, name));
    }
    assert_eq!(find("direct").unwrap().method, VerificationMethod::Proof);
    assert_eq!(find("block_doc").unwrap().line, 2);
    let witness = find("witness_fn").unwrap();
    assert_eq!(witness.witness.as_deref(), Some("verified"));
    assert_eq!(witness.method, VerificationMethod::Proof);
    assert!(find("in_helper").is_none());
    assert!(find("test_witness").is_none());
    let finding = |path: &str| {
        plan.findings
            .iter()
            .find(|f| f.path == std::path::Path::new(path))
            .unwrap_or_else(|| panic!("no finding in {path}"))
    };
    assert_eq!(
        finding("tests/support/helper.rs").code,
        "verify.link-unsupported"
    );
    assert!(
        finding("tests/support/helper.rs")
            .message
            .contains("crate target")
    );
    assert_eq!(finding("src/witness.rs").code, "verify.link-malformed");
    assert!(
        finding("src/witness.rs")
            .message
            .contains("verified function")
    );
    assert_eq!(finding("src/misplaced.rs").code, "verify.link-misplaced");
    assert!(
        finding("src/misplaced.rs")
            .message
            .contains("not on a module-level function")
    );
    assert!(!plan.findings.iter().any(|f| f.path.ends_with("block.rs")));
    let ordinary = plan
        .findings
        .iter()
        .find(|f| f.path.ends_with("it.rs") && f.line == 1)
        .unwrap();
    assert!(ordinary.message.contains("ordinary comment"));
    // Only linked tests are planned for the test runner; verified functions are proof targets.
    assert!(
        !plan
            .tests
            .iter()
            .any(|f| f.name == "verified" || f.name == "direct")
    );
    assert!(plan.tests.iter().any(|f| f.name == "in_main"));
}

/// anchorbook: A-verify-claims-evidenced#claim-failures
#[test]
fn roadmaps_are_the_role_chapter_and_its_subtree() {
    let book = book("");
    book.write(
        "docs/SUMMARY.md",
        "# Summary\n\n- [Design](design.md)\n- [Authority](authority.md)\n- [Work](roadmap.md)\n  - [More work](more.md)\n    - [Deeper](deeper.md)\n- [Glossary](glossary.md)\n",
    );
    book.write(
        "docs/more.md",
        &format!(
            "# More\n\n{}",
            milestone(7, "Complete", "`A-unlinked` · `none` — Full")
        ),
    );
    book.write(
        "docs/deeper.md",
        &format!(
            "# Deeper\n\n{}",
            milestone(8, "Complete", "`A-tests` · `one` — Full")
        ),
    );
    // Outside the role, milestone-like text is ordinary prose, not a malformed milestone.
    book.write(
        "docs/glossary.md",
        "# Glossary\n\n## Milestone x. Not a milestone\n",
    );
    let plan = whole(&book);
    let milestones: Vec<&str> = plan.claims.iter().map(|c| c.milestone.as_str()).collect();
    assert_eq!(milestones, ["work-7", "work-8"]);
}

/// anchorbook: A-verify-progress-report#incomplete-runs
#[test]
fn invalid_records_and_roadmaps_make_the_run_incomplete_with_their_causes() {
    use std::error::Error;
    let book = book("");
    book.write("docs/design.md", &design().replace("(draft)", "(sure)"));
    let error = plan(&book.config(), Scope::default()).unwrap_err();
    assert!(
        error.to_string().starts_with("verify.invalid-records"),
        "{error}"
    );
    assert!(error.source().is_some());
    let book = self::book(&milestone(
        1,
        "Complete",
        "`A-tests` · `one` — Full for now",
    ));
    let error = plan(&book.config(), Scope::default()).unwrap_err();
    assert!(
        error.to_string().starts_with("verify.invalid-roadmaps"),
        "{error}"
    );
    assert!(error.source().is_some());
}

/// anchorbook: A-verify-criterion-states#state-per-method
#[test]
fn the_worst_linked_test_decides_and_reasons_name_the_function() {
    let book = book("");
    let plan = whole(&book);
    let lib_test = function(Target::Lib, &["tests"], "same_name");
    let run = |outcomes: Vec<(FunctionId, TestOutcome)>| {
        evaluate(
            &plan,
            &RunResults {
                tests: RunnerResult::Ran(outcomes.into_iter().collect()),
                proofs: ProofResult::NotRun("none".into()),
            },
        )
    };
    let one = |report: &Report| report.anchors().next().unwrap().criteria[0].methods[0].clone();
    // The lib test is absent and comes first; a later failure still decides.
    let report = run(vec![
        (it("linked_twice"), TestOutcome::Failed),
        (it("duplicate"), TestOutcome::Passed),
    ]);
    assert_eq!(one(&report).state, State::Failed);
    let reason = one(&report).reason;
    assert!(
        reason.contains("tests::same_name (fixture lib) is absent from the results"),
        "{reason}"
    );
    assert!(
        reason.contains("linked_twice (fixture test it) failed"),
        "{reason}"
    );
    // A passing test after an absent one does not hide the missing evidence.
    let report = run(vec![
        (it("linked_twice"), TestOutcome::Passed),
        (it("duplicate"), TestOutcome::Passed),
    ]);
    assert_eq!(one(&report).state, State::NoEvidence);
    let report = run(vec![
        (lib_test, TestOutcome::Passed),
        (it("linked_twice"), TestOutcome::Passed),
        (it("duplicate"), TestOutcome::Passed),
    ]);
    assert_eq!(one(&report).reason, "3 linked tests passed");

    let verified = function(Target::Lib, &[], "verified");
    let proof = |functions: Vec<(FunctionId, ProofOutcome)>| {
        let report = evaluate(
            &plan,
            &RunResults {
                tests: RunnerResult::NotRun("none".into()),
                proofs: ProofResult::Ran(ProofRun {
                    complete: true,
                    functions: functions.into_iter().collect(),
                    failures_elsewhere: Vec::new(),
                }),
            },
        );
        report
            .anchors()
            .find(|a| a.id == "A-proof")
            .unwrap()
            .criteria[0]
            .methods[0]
            .clone()
    };
    let absent = proof(vec![]);
    assert!(
        absent
            .reason
            .contains("verified (fixture lib) is absent from the result"),
        "{}",
        absent.reason
    );
    let done = proof(vec![(verified, ProofOutcome::Verified)]);
    assert!(
        done.reason.contains("specification assessment"),
        "{}",
        done.reason
    );
}

/// anchorbook: A-verify-claims-evidenced#unchecked-claims
/// anchorbook: A-verify-progress-report#progress-output
#[test]
fn the_text_report_counts_states_and_claims() {
    let roadmap = [
        milestone(1, "Complete", "`A-tests` · `one` — Full"),
        milestone(2, "Complete", "`A-tests` · `both` — Full"),
    ]
    .concat();
    let book = book(&roadmap);
    let (_, stdout, _) = cli(&book, &[]);
    assert!(
        stdout.contains("\nsummary: 3 anchors: 0 satisfied, 2 unlinked, 1 not-evaluated\n"),
        "{stdout}"
    );
    assert!(
        stdout.contains("\n         6 criteria: 0 satisfied, 2 unlinked, 4 not-evaluated\n"),
        "{stdout}"
    );
    let plan = whole(&book);
    let passing = RunResults {
        tests: RunnerResult::Ran(
            [
                (
                    function(Target::Lib, &["tests"], "same_name"),
                    TestOutcome::Passed,
                ),
                (it("linked_twice"), TestOutcome::Passed),
                (it("duplicate"), TestOutcome::Passed),
            ]
            .into_iter()
            .collect(),
        ),
        proofs: ProofResult::NotRun("none".into()),
    };
    let text = anchor_book::adapters::verify::format_report(&evaluate(&plan, &passing));
    assert!(text.starts_with("anchorbook verify: progress\n"), "{text}");
    assert!(
        text.contains("claims: 2 claims: 1 satisfied, 0 unsatisfied, 1 unchecked"),
        "{text}"
    );
    assert!(
        text.contains("  unchecked: A-tests#both (review not evaluated)"),
        "{text}"
    );
    assert!(
        text.contains("summary: 3 anchors: 0 satisfied, 1 partial, 1 no-evidence, 1 unlinked\n"),
        "{text}"
    );
    assert!(
        text.contains(
            "6 criteria: 1 satisfied, 1 partial, 2 no-evidence, 1 unlinked, 1 not-evaluated\n"
        ),
        "{text}"
    );
}

#[test]
fn flags_are_parsed_strictly() {
    let book = book("");
    let (code, stdout, _) = cli(&book, &["--method", "proof", "--method", "review"]);
    assert_eq!(code, Some(0));
    assert!(
        stdout.contains("scope: all anchors; methods proof, review"),
        "{stdout}"
    );
    let config = book.config();
    let (code, _, stderr) = cli(&book, &["--config", config.to_str().unwrap()]);
    assert_eq!(code, Some(2));
    assert!(stderr.contains("Usage"));
}

/// anchorbook: A-verify-runs-linked-checks#selected-run
#[test]
fn input_identities_follow_the_criteria_and_the_checkout() {
    let book = book("");
    let first = whole(&book).inputs;
    book.write(
        "docs/design.md",
        &design().replace("Criterion none.", "Criterion none, changed."),
    );
    let changed = whole(&book).inputs;
    assert_ne!(first.criteria, changed.criteria);
    assert_eq!(first.configuration, changed.configuration);
    assert_eq!((first.revision, first.dirty), (None, None));

    let git = |args: &[&str]| {
        let status = Command::new("git")
            .arg("-C")
            .arg(&book.root)
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(
            status.status.success(),
            "{}",
            String::from_utf8_lossy(&status.stderr)
        );
        String::from_utf8(status.stdout).unwrap()
    };
    git(&["init", "-q"]);
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "fixture"]);
    let head = git(&["rev-parse", "HEAD"]).trim().to_owned();
    let clean = whole(&book).inputs;
    assert_eq!(
        (clean.revision.as_deref(), clean.dirty),
        (Some(head.as_str()), Some(false))
    );
    book.write("docs/authority.md", &format!("{AUTHORITY}\nEdited.\n"));
    assert_eq!(whole(&book).inputs.dirty, Some(true));
}

/// anchorbook: A-verify-links-resolve#link-findings
#[test]
fn comment_like_text_inside_string_literals_is_not_a_link() {
    let book = book("");
    book.write(
        "tests/strings.rs",
        concat!(
            "const RAW: &str = r#\"\n",
            "// anchorbook: A-tests#one\n",
            "/// anchorbook: A-tests#one\n",
            "\"#;\n",
            "const PLAIN: &str = \"\n// anchorbook: A-tests#one\n\";\n",
            "const BYTES: &[u8] = b\"\n/// anchorbook: A-tests#one\n\";\n",
            "/// A real link after the strings still links.\n",
            "/// anchorbook: A-tests#two\n",
            "#[test]\n",
            "fn real() {}\n",
            "fn f() { let _s = \"x\"; // anchorbook: A-tests#one\n}\n",
            "fn g() {\n    let _ = \"\n// anchorbook: A-tests#one\n\";\n}\n",
        ),
    );
    let plan = whole(&book);
    let findings: Vec<(usize, &str)> = plan
        .findings
        .iter()
        .filter(|f| f.path.ends_with("strings.rs"))
        .map(|f| (f.line, f.code))
        .collect();
    // Only the ordinary comment after a string on line 15 is a finding.
    assert_eq!(findings, [(15, "verify.link-misplaced")]);
    let real = function(Target::Test("strings".into()), &[], "real");
    assert!(
        plan.links
            .iter()
            .any(|l| l.function == real && l.line == 12)
    );
}

/// A stand-in for `cargo` that records each invocation and, for `nextest run`,
/// writes a prepared JUnit report where the tool configuration puts the store.
const RECORDING_CARGO: &str = r#"#!/bin/sh
printf '%s\n' "$*" >> "$RECORD"
if [ "$1 $2" = "nextest --version" ]; then
    echo "cargo-nextest 9.9.9 (recording)"
    exit 0
fi
for argument; do
    case "$argument" in anchorbook:*) tool="${argument#anchorbook:}" ;; esac
done
store=$(sed -n 's/^dir = "\(.*\)"$/\1/p' "$tool")
mkdir -p "$store/anchorbook-verify"
if [ -n "$TOUCH" ]; then echo "// changed" >> "$TOUCH"; fi
if [ "$MODE" = unbuildable ]; then exit 101; fi
cp "$JUNIT" "$store/anchorbook-verify/junit.xml"
exit 100
"#;

fn junit(cases: &[(&str, &str, &str)]) -> String {
    let mut xml = String::from("<testsuites>\n");
    for (class, name, outcome) in cases {
        match *outcome {
            "pass" => xml.push_str(&format!(
                "<testcase name=\"{name}\" classname=\"{class}\"/>\n"
            )),
            "fail" => xml.push_str(&format!(
                "<testcase name=\"{name}\" classname=\"{class}\"><failure>no</failure></testcase>\n"
            )),
            _ => xml.push_str(&format!(
                "<testcase name=\"{name}\" classname=\"{class}\"><skipped/></testcase>\n"
            )),
        }
    }
    xml.push_str("</testsuites>\n");
    xml
}

struct Recording {
    record: PathBuf,
    env: Vec<(&'static str, std::ffi::OsString)>,
}
fn recording(book: &fixture::Book, cases: &[(&str, &str, &str)]) -> Recording {
    use std::os::unix::fs::PermissionsExt;
    book.write("runner/cargo", RECORDING_CARGO);
    let cargo = book.path("runner/cargo");
    std::fs::set_permissions(&cargo, std::fs::Permissions::from_mode(0o755)).unwrap();
    book.write("runner/junit.xml", &junit(cases));
    let record = book.path("runner/record.txt");
    Recording {
        env: vec![
            ("CARGO", cargo.into()),
            ("RECORD", record.clone().into()),
            ("JUNIT", book.path("runner/junit.xml").into()),
        ],
        record,
    }
}

/// anchorbook: A-verify-runs-linked-checks#source-preservation
/// anchorbook: A-verify-runs-linked-checks#selected-run
#[test]
fn the_runner_selects_exactly_the_linked_tests_and_reads_only_this_runs_report() {
    let book = book(&milestone(1, "Complete", "`A-tests` · `one` — Full"));
    let runner = recording(
        &book,
        &[
            ("fixture", "tests::same_name", "pass"),
            ("fixture::it", "linked_twice", "pass"),
            ("fixture::it", "duplicate", "pass"),
            ("fixture::it", "same_name", "fail"),
            ("fixture::it", "checked_by_test", "skip"),
            ("other::it", "same_name", "pass"),
        ],
    );
    let before: Vec<_> = book
        .snapshot()
        .into_iter()
        .filter(|(path, _)| !path.starts_with("runner"))
        .collect();
    let (code, stdout, stderr) = cli_with(&book, &[], &runner.env);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    let record = std::fs::read_to_string(&runner.record).unwrap();
    let runs: Vec<&str> = record
        .lines()
        .filter(|l| l.starts_with("nextest run"))
        .collect();
    assert_eq!(runs.len(), 2, "one run per package:\n{record}");
    let tool = book.path("target/verify/nextest/tool.toml");
    let expected = |manifest: &str, filter: &str| {
        format!(
            "nextest run --manifest-path {} --locked --no-fail-fast --no-tests=pass --run-ignored all --profile anchorbook-verify --tool-config-file anchorbook:{} -E {filter}",
            book.path(manifest).display(),
            tool.display()
        )
    };
    assert_eq!(
        runs[0],
        expected(
            "Cargo.toml",
            "(binary_id(=fixture) & test(=tests::same_name)) | (binary_id(=fixture) & test(=tests::inner::same_name)) | (binary_id(=fixture::it) & test(=checked_by_test)) | (binary_id(=fixture::it) & test(=duplicate)) | (binary_id(=fixture::it) & test(=linked_twice)) | (binary_id(=fixture::it) & test(=same_name))"
        )
    );
    assert_eq!(
        runs[1],
        expected(
            "other/Cargo.toml",
            "(binary_id(=other::it) & test(=same_name))"
        )
    );
    for expected in [
        "runners: cargo-nextest 9.9.9 (recording)",
        "    one [test]: satisfied — 3 linked tests passed",
        "    two [test]: failed — tests::inner::same_name (fixture lib) is absent from the results; same_name (fixture test it) failed",
        "      test: no-evidence — checked_by_test (fixture test it) was skipped",
        "claims: 1 claim: 1 satisfied, 0 unsatisfied, 0 unchecked",
    ] {
        assert!(
            stdout.contains(expected),
            "missing {expected:?} in\n{stdout}"
        );
    }
    // Runner output lands only under the configured output directory.
    let after: Vec<_> = book
        .snapshot()
        .into_iter()
        .filter(|(path, _)| {
            // Only the configured output directory may be new: target/ and target/verify/.
            !path.starts_with("runner")
                && !path.starts_with("target/verify")
                && *path != std::path::Path::new("target")
        })
        .collect();
    assert_eq!(before, after);
    assert!(book.path("target/verify/nextest/fixture.log").is_file());
}

/// anchorbook: A-verify-runs-linked-checks#selected-run
/// anchorbook: A-verify-progress-report#incomplete-runs
#[test]
fn stale_reports_changed_inputs_and_failed_builds_are_never_evidence() {
    let book = book("");
    let cases = [("fixture::it", "linked_twice", "pass")];
    let state_of_one = |env: &[(&str, std::ffi::OsString)]| {
        let (code, stdout, _) = cli_with(&book, &["--anchor", "A-tests"], env);
        assert_eq!(code, Some(0), "{stdout}");
        stdout
            .lines()
            .find(|l| l.starts_with("    one [test]:"))
            .unwrap()
            .to_owned()
    };
    // A report left by an earlier run is removed before the run, so a runner
    // that writes none produces no evidence.
    let runner = recording(&book, &cases);
    book.write(
        "target/verify/nextest/store/anchorbook-verify/junit.xml",
        &junit(&[
            ("fixture", "tests::same_name", "pass"),
            ("fixture::it", "linked_twice", "pass"),
            ("fixture::it", "duplicate", "pass"),
        ]),
    );
    let mut env = runner.env.clone();
    env.push(("MODE", "unbuildable".into()));
    let line = state_of_one(&env);
    assert!(
        line.ends_with(
            "inconclusive — cargo nextest for package `fixture` produced no report (exit 101); see target/verify/nextest/fixture.log"
        ),
        "{line}"
    );

    let mut env = runner.env.clone();
    env.push(("TOUCH", book.path("src/lib.rs").into()));
    let line = state_of_one(&env);
    assert!(
        line.contains("inconclusive — sources, manifests, or lockfiles changed during the run"),
        "{line}"
    );
}

/// anchorbook: A-verify-criterion-states#scope-and-limits
#[test]
fn the_runner_runs_only_when_configured_and_needed() {
    let book = book("");
    let runner = recording(&book, &[]);
    let (_, stdout, _) = cli_with(&book, &["--method", "proof"], &runner.env);
    assert!(
        !runner.record.exists(),
        "a proof-only run must not start nextest"
    );
    assert!(!stdout.contains("runners:"));
    book.write(
        "anchorbook.toml",
        &format!(
            "{}{}",
            fixture::CONFIG,
            VERIFICATION.replace("[\"nextest\"]", "[]")
        ),
    );
    let (_, stdout, _) = cli_with(&book, &[], &runner.env);
    assert!(!runner.record.exists());
    assert!(
        stdout.contains(
            "    one [test]: not-evaluated — nextest is not listed in [verification] runners"
        ),
        "{stdout}"
    );
}

/// anchorbook: A-verify-runs-linked-checks#selected-run
/// The build loop against cargo-nextest itself: a passing, a deliberately
/// failing, and a restored test change the linked criterion's state.
#[test]
#[ignore = "runs the installed cargo-nextest; run with --ignored when it is on PATH"]
fn native_nextest_follows_a_change_through_the_build_loop() {
    let book = fixture::Book::new();
    book.write(
        "anchorbook.toml",
        &format!("{}\n[verification]\nsources = [\"src\", \"tests\"]\noutput = \"target/verify\"\nrunners = [\"nextest\"]\n", fixture::CONFIG),
    );
    book.write(
        "docs/design.md",
        &format!("# Design\n\n{}", anchor("A-loop", &[("works", "test")])),
    );
    book.write("docs/authority.md", AUTHORITY);
    book.write("Cargo.toml", "[package]\nname = \"loop-fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n");
    book.write("src/lib.rs", "pub fn value() -> u32 { 2 }\n");
    // The runner passes --locked, so the project must have its lockfile.
    let lock = Command::new(env!("CARGO"))
        .args(["generate-lockfile", "--offline", "--manifest-path"])
        .arg(book.path("Cargo.toml"))
        .status()
        .unwrap();
    assert!(lock.success());
    let test = |expected: u32| {
        format!(
            "/// anchorbook: A-loop#works\n#[test]\nfn value_is_right() {{ assert_eq!(loop_fixture::value(), {expected}); }}\n\n#[test]\n#[ignore]\nfn ignored_but_run() {{}}\n"
        )
    };
    let env = [("CARGO", std::ffi::OsString::from(env!("CARGO")))];
    let state = |book: &fixture::Book| {
        let (code, stdout, stderr) = cli_with(book, &[], &env);
        assert_eq!(code, Some(0), "{stdout}{stderr}");
        assert!(stdout.contains("runners: cargo-nextest"), "{stdout}");
        stdout
            .lines()
            .find(|l| l.starts_with("    works [test]:"))
            .unwrap()
            .to_owned()
    };
    book.write("tests/loop.rs", &test(2));
    assert_eq!(
        state(&book),
        "    works [test]: satisfied — 1 linked test passed"
    );
    book.write("tests/loop.rs", &test(3));
    assert!(
        state(&book).starts_with(
            "    works [test]: failed — value_is_right (loop-fixture test loop) failed"
        )
    );
    book.write("tests/loop.rs", &test(2));
    assert_eq!(
        state(&book),
        "    works [test]: satisfied — 1 linked test passed"
    );
}

#[test]
fn the_code_identity_covers_a_member_packages_lockfile_at_the_root() {
    let book = book("");
    // Only the second package has links; its lockfile is at the configuration root.
    book.write("src/lib.rs", "pub fn f() {}\n");
    book.write("tests/it.rs", "#[test]\nfn t() {}\n");
    book.write("Cargo.lock", "version = 4\n");
    let first = whole(&book);
    assert_eq!(first.manifests.keys().collect::<Vec<_>>(), ["other"]);
    book.write("Cargo.lock", "version = 4\n# changed\n");
    assert_ne!(whole(&book).inputs.code, first.inputs.code);
}
