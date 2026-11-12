//! Evidence for anchor criteria: links from code to criteria, evidence states,
//! claims of completion, and the progress report that `anchorbook verify` prints.
//!
//! [`plan`] reads the Book, its roadmaps, the `[verification]` configuration,
//! and evidence links in the configured sources. [`evaluate`] combines a plan
//! with runner results into a [`Report`]. [`verify`] runs the configured native
//! runners; [`proof`] exposes their strict target gate. Only runner output is written.
mod config;
mod identity;
mod links;
mod nextest;
mod verus;
mod verus_report;

pub use config::{Settings, VerusSettings, VerusTarget};
pub use identity::Inputs;
pub use links::{FunctionId, Link, LinkFinding, Mode, Target};
pub use nextest::NEXTEST_VERSION;
pub use verus::{FunctionEvidence, ProofReport, TargetEvidence, VERUS_VERSION, proof};

use crate::checking::{self, Outcome};
use crate::configuration::{self, SectionRole};
use crate::records::{
    Chapter, RecordData, RecordInventory, Status, VerificationMethod, read_records,
};
use crate::roadmaps::{CoverageLevel, Milestone, read_roadmaps};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

/// What a run evaluates. Empty lists select every anchor and every method.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scope {
    /// Anchor identifiers to report; empty for the whole Book.
    pub anchors: Vec<String>,
    /// Methods to evaluate; empty for all of them.
    pub methods: Vec<VerificationMethod>,
}
impl Scope {
    fn includes_anchor(&self, id: &str) -> bool {
        self.anchors.is_empty() || self.anchors.iter().any(|a| a == id)
    }
    fn includes_method(&self, method: VerificationMethod) -> bool {
        self.methods.is_empty() || self.methods.contains(&method)
    }
}

/// The state of one required method of one criterion, from a closed set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    /// Passing evidence from this run.
    Satisfied,
    /// Some required methods are satisfied and others are not (criteria and anchors only).
    Partial,
    /// A linked check failed.
    Failed,
    /// The runner could not build, run, or report usably.
    Inconclusive,
    /// Linked functions were absent, ambiguous, or skipped, or a required record is missing.
    NoEvidence,
    /// Nothing links the criterion for this method.
    Unlinked,
    /// Outside the requested scope, or the runner or method is not available.
    NotEvaluated,
}
impl State {
    /// The report's name for the state.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::Inconclusive => "inconclusive",
            Self::NoEvidence => "no-evidence",
            Self::Unlinked => "unlinked",
            Self::NotEvaluated => "not-evaluated",
        }
    }
}

/// Combine states: satisfied when all are, partial when some are, and otherwise
/// the first of failed, inconclusive, no-evidence, unlinked, not-evaluated.
fn combine(states: impl IntoIterator<Item = State>) -> State {
    let states: Vec<State> = states.into_iter().collect();
    let satisfied = states.iter().filter(|s| **s == State::Satisfied).count();
    if !states.is_empty() && satisfied == states.len() {
        State::Satisfied
    } else if satisfied > 0 || states.contains(&State::Partial) {
        State::Partial
    } else {
        states.into_iter().min().unwrap_or(State::NotEvaluated)
    }
}

/// The outcome of one linked test in a runner's results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestOutcome {
    /// Present once, and passed.
    Passed,
    /// Present, and failed or errored.
    Failed,
    /// Present, and skipped or ignored.
    Skipped,
}

/// The outcome of one linked function in a complete proof result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofOutcome {
    /// Verified once in its expected mode.
    Verified,
    /// Failed verification.
    Failed,
}

/// What a runner produced for this run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerResult<T> {
    /// The runner did not run: it is not configured, installed, or implemented.
    NotRun(String),
    /// The runner ran but could not build, run, or report usably.
    Unusable(String),
    /// Results from this run, by linked function. Absent functions have no evidence.
    Ran(BTreeMap<FunctionId, T>),
}

/// A complete or incomplete proof result for the required targets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofRun {
    /// Every required target has a complete result.
    pub complete: bool,
    /// Results for linked functions.
    pub functions: BTreeMap<FunctionId, ProofOutcome>,
    /// Failing obligations outside the criteria's links, with their locations.
    pub failures_elsewhere: Vec<String>,
}

/// What the proof runner produced for this run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofResult {
    /// The runner did not run: it is not configured, installed, or implemented.
    NotRun(String),
    /// The runner ran but could not build, run, or report usably.
    Unusable(String),
    /// A result for the required targets.
    Ran(ProofRun),
}

/// Results from the runners of one run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResults {
    /// The test runner's results.
    pub tests: RunnerResult<TestOutcome>,
    /// The proof runner's results.
    pub proofs: ProofResult,
}
impl RunResults {
    /// Results when no runner has run, with the reason for each.
    pub fn not_run(reason: &str) -> Self {
        Self {
            tests: RunnerResult::NotRun(reason.to_owned()),
            proofs: ProofResult::NotRun(reason.to_owned()),
        }
    }
}

/// One anchor criterion in the evaluated scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedCriterion {
    /// Anchor identifier.
    pub anchor: String,
    /// Criterion identifier.
    pub criterion: String,
    /// Required methods, in authored order.
    pub methods: Vec<VerificationMethod>,
}

/// A milestone's claim that an anchor criterion is established.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    /// The Complete milestone.
    pub milestone: String,
    /// Its completion criterion whose coverage is Full.
    pub completion_criterion: String,
    /// Roadmap chapter and line of the coverage.
    pub path: PathBuf,
    /// One-based line.
    pub line: usize,
    /// Claimed anchor.
    pub anchor: String,
    /// Claimed criterion.
    pub criterion: String,
}

/// An anchor and its chapter, in navigation order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedAnchor {
    /// Anchor identifier.
    pub id: String,
    /// Design acceptance status.
    pub status: Status,
    /// Source-relative chapter path.
    pub chapter: PathBuf,
}

/// What a run evaluates and the evidence links it found, before any runner.
#[derive(Clone, Debug)]
pub struct Plan {
    /// Requested scope.
    pub scope: Scope,
    /// Chapters with anchors, in navigation order, with their titles.
    pub chapters: Vec<(PathBuf, String)>,
    /// Anchors in scope, in navigation order.
    pub anchors: Vec<PlannedAnchor>,
    /// Criteria of the anchors in scope.
    pub criteria: Vec<PlannedCriterion>,
    /// Valid links to criteria in scope.
    pub links: Vec<Link>,
    /// Link and source findings, located; invalid links are not in `links`.
    pub findings: Vec<LinkFinding>,
    /// Linked tests the test runner must run, deduplicated and sorted.
    pub tests: Vec<FunctionId>,
    /// Packages and crate targets the proof runner must verify, sorted.
    pub proof_targets: Vec<(String, Target)>,
    /// Every claim the Book makes, in or out of scope.
    pub claims: Vec<Claim>,
    /// Identities of the inputs the plan was made from.
    pub inputs: Inputs,
    /// The configuration's directory, the base of displayed paths.
    pub base: PathBuf,
    /// The validated `[verification]` settings.
    pub settings: Settings,
    /// The manifest of each package with links in scope.
    pub manifests: BTreeMap<String, PathBuf>,
    proof_inputs: Vec<PathBuf>,
    proof_snapshot: String,
}

/// One required method's state, with what it rests on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodReport {
    /// The required method.
    pub method: VerificationMethod,
    /// Its state.
    pub state: State,
    /// Linked functions.
    pub links: Vec<FunctionId>,
    /// Why the state is what it is.
    pub reason: String,
}

/// One criterion's state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CriterionReport {
    /// Criterion identifier.
    pub id: String,
    /// Combined state of its required methods.
    pub state: State,
    /// Each required method.
    pub methods: Vec<MethodReport>,
}

/// One anchor's state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorReport {
    /// Anchor identifier.
    pub id: String,
    /// Design acceptance status.
    pub status: Status,
    /// Combined state of its criteria.
    pub state: State,
    /// Its criteria.
    pub criteria: Vec<CriterionReport>,
}

/// One design chapter's anchors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChapterReport {
    /// Source-relative path.
    pub path: PathBuf,
    /// Its first heading.
    pub title: String,
    /// Its anchors in scope, in source order.
    pub anchors: Vec<AnchorReport>,
}

/// Whether a claim holds in this run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimOutcome {
    /// The claimed criterion is satisfied.
    Satisfied,
    /// A method evaluated in this run is not satisfied; the run fails.
    Unsatisfied(State),
    /// The run could not check it, with the reason; never counted as satisfied.
    Unchecked(String),
}

/// A claim and its outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimReport {
    /// The claim.
    pub claim: Claim,
    /// Its outcome.
    pub outcome: ClaimOutcome,
}

/// A completed evaluation: progress by chapter, link findings, and claims.
#[derive(Clone, Debug)]
pub struct Report {
    /// Requested scope.
    pub scope: Scope,
    /// Chapters in navigation order.
    pub chapters: Vec<ChapterReport>,
    /// Link and source findings.
    pub findings: Vec<LinkFinding>,
    /// Every claim and its outcome.
    pub claims: Vec<ClaimReport>,
    /// Input identities.
    pub inputs: Inputs,
    /// Base of displayed paths.
    pub base: PathBuf,
    /// Versions of the runners that ran.
    pub tools: Vec<String>,
    /// Native proof-target evidence, separate from criterion assessments and guards.
    pub proofs: Option<ProofReport>,
}
impl Report {
    /// `1` when a claim evaluated in this run is unsatisfied, otherwise `0`.
    /// Unsatisfied and unlinked criteria that no milestone claims are progress.
    pub fn exit_code(&self) -> u8 {
        u8::from(
            self.claims
                .iter()
                .any(|c| matches!(c.outcome, ClaimOutcome::Unsatisfied(_))),
        )
    }
    /// Every anchor in the report, in chapter order.
    pub fn anchors(&self) -> impl Iterator<Item = &AnchorReport> {
        self.chapters.iter().flat_map(|c| c.anchors.iter())
    }
}

/// A run that could not evaluate its scope.
#[derive(Debug)]
pub enum VerifyError {
    /// The structural check did not pass; its report explains why.
    Check(Box<checking::CheckReport>),
    /// The configuration, including `[verification]`, is invalid or incomplete.
    Configuration(String),
    /// Book records are invalid.
    Records(crate::records::RecordErrors),
    /// Roadmaps are invalid.
    Roadmaps(crate::roadmaps::RoadmapErrors),
    /// The requested scope names something the Book does not define.
    Scope(String),
    /// A required input could not be read.
    Read(String),
}
impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Check(report) => write!(
                f,
                "verify.check-failed: the structural check did not pass\n{}",
                crate::adapters::check::format_report(report)
            ),
            Self::Configuration(message) => write!(f, "verify.invalid-configuration: {message}"),
            Self::Records(errors) => {
                write!(f, "verify.invalid-records: Book records are invalid")?;
                for diagnostic in errors.diagnostics() {
                    write!(f, "\n{diagnostic}")?;
                }
                Ok(())
            }
            Self::Roadmaps(errors) => write!(f, "verify.invalid-roadmaps: {errors}"),
            Self::Scope(message) => write!(f, "verify.invalid-scope: {message}"),
            Self::Read(message) => write!(f, "verify.read-failed: {message}"),
        }
    }
}
impl Error for VerifyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Records(errors) => Some(errors),
            Self::Roadmaps(errors) => Some(errors),
            _ => None,
        }
    }
}

/// Read the Book, roadmaps, configuration, and evidence links for `scope`.
///
/// Runs the structural check first; a Book that does not pass cannot be
/// evaluated. Reads files only; runs no tool other than `git`, for the revision.
///
/// # Errors
/// Returns [`VerifyError`] when the check fails, configuration or records are
/// invalid, or the scope names an anchor the Book does not define.
pub fn plan(config_path: &Path, scope: Scope) -> Result<Plan, VerifyError> {
    let initial_config = fs::read(config_path).ok();
    let report = match checking::check(config_path) {
        Ok(report) => report,
        Err(error) => error.into_report(),
    };
    if report.outcome() != Outcome::Passed {
        return Err(VerifyError::Check(Box::new(report)));
    }
    let configuration =
        configuration::load(config_path).map_err(|e| VerifyError::Configuration(e.to_string()))?;
    let base = configuration
        .path()
        .parent()
        .map_or_else(PathBuf::new, Path::to_owned);
    let settings = config::load(&configuration)?;
    let proof_inputs = vec![
        configuration.path().to_owned(),
        configuration.source().to_owned(),
    ];
    let proof_snapshot = identity::snapshot(&proof_inputs);

    // Chapters in navigation order; roadmaps are the role chapter and its subtree.
    let listed = checking::chapters(&configuration).map_err(VerifyError::Read)?;
    let roadmap_role = configuration
        .section(SectionRole::ImplementationRoadmaps)
        .strip_prefix(configuration.source())
        .map(Path::to_owned)
        .ok();
    let mut sources = Vec::with_capacity(listed.len());
    let mut roadmap_depth = None;
    for (path, depth) in &listed {
        let roadmap = match roadmap_depth {
            Some(role) if *depth > role => true,
            _ => {
                roadmap_depth = (Some(path) == roadmap_role.as_ref()).then_some(*depth);
                roadmap_depth.is_some()
            }
        };
        let full = configuration.source().join(path);
        let text = fs::read_to_string(&full)
            .map_err(|e| VerifyError::Read(format!("{}: {e}", full.display())))?;
        sources.push((path.clone(), text, roadmap));
    }
    let chapters: Vec<Chapter<'_>> = sources
        .iter()
        .map(|(path, text, _)| Chapter::new(path, text))
        .collect();
    let inventory = read_records(&chapters).map_err(VerifyError::Records)?;
    let roadmap_chapters: Vec<Chapter<'_>> = sources
        .iter()
        .filter(|(_, _, roadmap)| *roadmap)
        .map(|(path, text, _)| Chapter::new(path, text))
        .collect();
    let milestones = read_roadmaps(&roadmap_chapters, &inventory).map_err(VerifyError::Roadmaps)?;

    for anchor in &scope.anchors {
        match inventory.get(anchor).map(|r| r.data()) {
            Some(RecordData::Anchor(_)) => {}
            _ => {
                return Err(VerifyError::Scope(format!(
                    "`{anchor}` is not an anchor in this Book"
                )));
            }
        }
    }

    let (anchors, criteria, chapter_titles) = planned_records(&sources, &inventory, &scope);
    let (all_links, mut findings) = links::read(&settings.sources, &base, &inventory);
    let in_scope: BTreeSet<(&str, &str)> = criteria
        .iter()
        .map(|c| (c.anchor.as_str(), c.criterion.as_str()))
        .collect();
    let links: Vec<Link> = all_links
        .into_iter()
        .filter(|l| in_scope.contains(&(l.anchor.as_str(), l.criterion.as_str())))
        .collect();
    findings.sort();

    let tests: BTreeSet<FunctionId> = links
        .iter()
        .filter(|l| l.method == VerificationMethod::Test && scope.includes_method(l.method))
        .map(|l| l.function.clone())
        .collect();
    let proof_targets: BTreeSet<(String, Target)> = links
        .iter()
        .filter(|l| l.method == VerificationMethod::Proof && scope.includes_method(l.method))
        .map(|l| (l.function.package.clone(), l.function.target.clone()))
        .collect();
    let claims = claims(&milestones);
    let manifests: BTreeMap<String, PathBuf> = links
        .iter()
        .map(|l| (l.function.package.clone(), l.manifest.clone()))
        .collect();
    let mut plan = Plan {
        scope,
        chapters: chapter_titles,
        anchors,
        criteria,
        links,
        findings,
        tests: tests.into_iter().collect(),
        proof_targets: proof_targets.into_iter().collect(),
        claims,
        inputs: Inputs {
            criteria: String::new(),
            configuration: String::new(),
            code: String::new(),
            revision: None,
            dirty: None,
        },
        base,
        settings,
        manifests,
        proof_inputs,
        proof_snapshot,
    };
    check_planning_inputs(
        configuration.path(),
        &initial_config,
        &plan.proof_inputs,
        &plan.proof_snapshot,
    )?;
    plan.inputs = identity::inputs(
        &configuration,
        &plan.criteria,
        &inventory,
        &nextest::input_paths(&plan),
    );
    Ok(plan)
}

// Configuration is first read before structural checking; the Book snapshot is
// captured later, before reading records. Both intervals must stay unchanged.
fn check_planning_inputs(
    config: &Path,
    initial_config: &Option<Vec<u8>>,
    paths: &[PathBuf],
    snapshot: &str,
) -> Result<(), VerifyError> {
    if identity::snapshot(paths) != snapshot || &fs::read(config).ok() != initial_config {
        return Err(VerifyError::Read(
            "Book or configuration changed while planning; rerun on stable inputs".into(),
        ));
    }
    Ok(())
}

type Planned = (
    Vec<PlannedAnchor>,
    Vec<PlannedCriterion>,
    Vec<(PathBuf, String)>,
);

/// Anchors and criteria in scope in navigation and source order, and the titles
/// of the chapters that hold them.
fn planned_records(
    sources: &[(PathBuf, String, bool)],
    inventory: &RecordInventory,
    scope: &Scope,
) -> Planned {
    let mut anchors = Vec::new();
    let mut criteria = Vec::new();
    let mut titles = Vec::new();
    for (path, text, _) in sources {
        let mut records: Vec<_> = inventory
            .records()
            .filter(|r| r.location().path() == path.as_path() && scope.includes_anchor(r.id()))
            .filter_map(|r| match r.data() {
                RecordData::Anchor(anchor) => Some((r, anchor)),
                _ => None,
            })
            .collect();
        if records.is_empty() {
            continue;
        }
        records.sort_by_key(|(r, _)| r.location().range().start);
        let title = text
            .lines()
            .find_map(|l| l.strip_prefix("# "))
            .unwrap_or_default()
            .trim()
            .to_owned();
        titles.push((path.clone(), title));
        for (record, anchor) in records {
            anchors.push(PlannedAnchor {
                id: record.id().to_owned(),
                status: record.status(),
                chapter: path.clone(),
            });
            for criterion in &anchor.criteria {
                criteria.push(PlannedCriterion {
                    anchor: record.id().to_owned(),
                    criterion: criterion.id.clone(),
                    methods: criterion.verification.clone(),
                });
            }
        }
    }
    (anchors, criteria, titles)
}

/// Every Full coverage entry of a Complete milestone, one claim per criterion.
fn claims(milestones: &[Milestone]) -> Vec<Claim> {
    let mut claims = Vec::new();
    for milestone in milestones.iter().filter(|m| m.status == "Complete") {
        for coverage in milestone
            .coverage
            .iter()
            .filter(|c| c.level == CoverageLevel::Full)
        {
            for criterion in &coverage.criteria {
                claims.push(Claim {
                    milestone: milestone.id.clone(),
                    completion_criterion: coverage.completion_criterion.clone(),
                    path: milestone.path.clone(),
                    line: coverage.line,
                    anchor: coverage.anchor.clone(),
                    criterion: criterion.clone(),
                });
            }
        }
    }
    claims
}

/// The state of one required method from the plan's links and the run's results.
fn method_state(
    plan: &Plan,
    results: &RunResults,
    criterion: &PlannedCriterion,
    method: VerificationMethod,
) -> MethodReport {
    let links: Vec<FunctionId> = plan
        .links
        .iter()
        .filter(|l| {
            l.anchor == criterion.anchor && l.criterion == criterion.criterion && l.method == method
        })
        .map(|l| l.function.clone())
        .collect();
    let report = |state: State, reason: String| MethodReport {
        method,
        state,
        links: links.clone(),
        reason,
    };
    if !plan.scope.includes_method(method) {
        return report(
            State::NotEvaluated,
            "method outside the requested scope".into(),
        );
    }
    if method == VerificationMethod::Review {
        return report(
            State::NotEvaluated,
            "assessment records are not read yet".into(),
        );
    }
    if links.is_empty() {
        return report(
            State::Unlinked,
            format!("no {} link names the criterion", method.as_str()),
        );
    }
    match method {
        VerificationMethod::Test => match &results.tests {
            RunnerResult::NotRun(reason) => report(State::NotEvaluated, reason.clone()),
            RunnerResult::Unusable(reason) => report(State::Inconclusive, reason.clone()),
            RunnerResult::Ran(outcomes) => {
                let mut problems = Vec::new();
                let mut reasons = Vec::new();
                for function in &links {
                    let (found, why) = match outcomes.get(function) {
                        Some(TestOutcome::Passed) => continue,
                        Some(TestOutcome::Failed) => (State::Failed, "failed"),
                        Some(TestOutcome::Skipped) => (State::NoEvidence, "was skipped"),
                        None => (State::NoEvidence, "is absent from the results"),
                    };
                    reasons.push(format!("{function} {why}"));
                    problems.push(found);
                }
                // Failed outranks no-evidence; with neither, every linked test passed.
                let state = problems.into_iter().min().unwrap_or(State::Satisfied);
                let reason = if reasons.is_empty() {
                    match links.len() {
                        1 => "1 linked test passed".to_owned(),
                        n => format!("{n} linked tests passed"),
                    }
                } else {
                    reasons.join("; ")
                };
                report(state, reason)
            }
        },
        VerificationMethod::Proof => match &results.proofs {
            ProofResult::NotRun(reason) => report(State::NotEvaluated, reason.clone()),
            ProofResult::Unusable(reason) => report(State::Inconclusive, reason.clone()),
            ProofResult::Ran(run) => {
                let failed: Vec<String> = links
                    .iter()
                    .filter(|f| run.functions.get(f) == Some(&ProofOutcome::Failed))
                    .map(|f| format!("{f} failed verification"))
                    .collect();
                if !failed.is_empty() {
                    return report(State::Failed, failed.join("; "));
                }
                if !run.complete {
                    return report(
                        State::Inconclusive,
                        format!(
                            "a required target has no complete result; failing obligations: {}",
                            run.failures_elsewhere.join("; ")
                        ),
                    );
                }
                if !run.failures_elsewhere.is_empty() {
                    return report(
                        State::Inconclusive,
                        format!(
                            "verification failed outside the links: {}",
                            run.failures_elsewhere.join("; ")
                        ),
                    );
                }
                let missing: Vec<String> = links
                    .iter()
                    .filter(|f| !run.functions.contains_key(*f))
                    .map(|f| format!("{f} is absent from the result"))
                    .collect();
                if !missing.is_empty() {
                    return report(State::NoEvidence, missing.join("; "));
                }
                report(
                    State::NoEvidence,
                    "linked functions verified; the specification assessment the proof method also needs is not read yet".into(),
                )
            }
        },
        VerificationMethod::Review => unreachable!("handled above"),
    }
}

/// Combine a plan with one run's results into a report.
pub fn evaluate(plan: &Plan, results: &RunResults) -> Report {
    let mut criteria: BTreeMap<(&str, &str), CriterionReport> = BTreeMap::new();
    for criterion in &plan.criteria {
        let methods: Vec<MethodReport> = criterion
            .methods
            .iter()
            .map(|m| method_state(plan, results, criterion, *m))
            .collect();
        criteria.insert(
            (criterion.anchor.as_str(), criterion.criterion.as_str()),
            CriterionReport {
                id: criterion.criterion.clone(),
                state: combine(methods.iter().map(|m| m.state)),
                methods,
            },
        );
    }
    let mut chapters: Vec<ChapterReport> = plan
        .chapters
        .iter()
        .map(|(path, title)| ChapterReport {
            path: path.clone(),
            title: title.clone(),
            anchors: Vec::new(),
        })
        .collect();
    for anchor in &plan.anchors {
        let reports: Vec<CriterionReport> = plan
            .criteria
            .iter()
            .filter(|c| c.anchor == anchor.id)
            .map(|c| criteria[&(c.anchor.as_str(), c.criterion.as_str())].clone())
            .collect();
        let report = AnchorReport {
            id: anchor.id.clone(),
            status: anchor.status,
            state: combine(reports.iter().map(|c| c.state)),
            criteria: reports,
        };
        if let Some(chapter) = chapters.iter_mut().find(|c| c.path == anchor.chapter) {
            chapter.anchors.push(report);
        }
    }
    let claims = plan
        .claims
        .iter()
        .map(|claim| ClaimReport {
            claim: claim.clone(),
            outcome: claim_outcome(
                criteria.get(&(claim.anchor.as_str(), claim.criterion.as_str())),
                &plan.scope,
            ),
        })
        .collect();
    Report {
        scope: plan.scope.clone(),
        chapters,
        findings: plan.findings.clone(),
        claims,
        inputs: plan.inputs.clone(),
        base: plan.base.clone(),
        tools: Vec::new(),
        proofs: None,
    }
}

/// A claim fails when any method this run evaluated is not satisfied; methods
/// the run did not evaluate leave it unchecked, never satisfied.
fn claim_outcome(report: Option<&CriterionReport>, scope: &Scope) -> ClaimOutcome {
    let Some(report) = report else {
        return ClaimOutcome::Unchecked(if scope.anchors.is_empty() {
            "the criterion is not in this run".into()
        } else {
            "outside the requested anchors".into()
        });
    };
    if let Some(method) = report
        .methods
        .iter()
        .find(|m| !matches!(m.state, State::Satisfied | State::NotEvaluated))
    {
        return ClaimOutcome::Unsatisfied(method.state);
    }
    let unchecked: Vec<&str> = report
        .methods
        .iter()
        .filter(|m| m.state == State::NotEvaluated)
        .map(|m| m.method.as_str())
        .collect();
    if unchecked.is_empty() {
        ClaimOutcome::Satisfied
    } else {
        ClaimOutcome::Unchecked(format!("{} not evaluated", unchecked.join(", ")))
    }
}

/// Plan `scope`, run configured linked tests and proofs, and evaluate. Runner
/// output is written only under `[verification] output`. Proof assessments and
/// guards remain explicit outstanding obligations even after target success.
///
/// # Errors
/// As [`plan`].
pub fn verify(config_path: &Path, scope: Scope) -> Result<Report, VerifyError> {
    let plan = plan(config_path, scope)?;
    let (tests, version) = nextest::run(&plan);
    let (proofs, proof_report) = verus::run(&plan);
    let results = RunResults { tests, proofs };
    let mut report = evaluate(&plan, &results);
    report.tools.extend(version);
    report.tools.extend(proof_report.tools.iter().cloned());
    if !plan.proof_targets.is_empty() {
        report.proofs = Some(proof_report);
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{State, combine};

    #[test]
    fn planning_rejects_book_edits_and_configuration_edits_before_the_book_snapshot() {
        use super::{check_planning_inputs, identity};
        use std::fs;
        let directory =
            std::env::temp_dir().join(format!("anchorbook-plan-inputs-{}", std::process::id()));
        fs::create_dir(&directory).unwrap();
        struct Cleanup(std::path::PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
        let _cleanup = Cleanup(directory.clone());
        let config = directory.join("anchorbook.toml");
        let book = directory.join("chapter.md");
        fs::write(&config, "configuration before structural checking").unwrap();
        fs::write(&book, "original criterion").unwrap();
        let initial_config = fs::read(&config).ok();
        let paths = vec![config.clone(), book.clone()];
        let snapshot = identity::snapshot(&paths);
        assert!(check_planning_inputs(&config, &initial_config, &paths, &snapshot).is_ok());
        fs::write(&book, "changed criterion").unwrap();
        assert!(check_planning_inputs(&config, &initial_config, &paths, &snapshot).is_err());
        fs::write(&config, "configuration changed before records are read").unwrap();
        let later_snapshot = identity::snapshot(&paths);
        assert!(check_planning_inputs(&config, &initial_config, &paths, &later_snapshot).is_err());
        assert!(check_planning_inputs(&config, &initial_config, &paths, &snapshot).is_err());
        fs::remove_file(&config).unwrap();
        assert!(check_planning_inputs(&config, &initial_config, &paths, &snapshot).is_err());
    }

    #[test]
    fn states_combine_as_the_design_orders_them() {
        use State::*;
        assert_eq!(combine([Satisfied, Satisfied]), Satisfied);
        assert_eq!(combine([Satisfied, Unlinked]), Partial);
        assert_eq!(combine([Partial, Unlinked]), Partial);
        assert_eq!(combine([Unlinked, Failed, NotEvaluated]), Failed);
        assert_eq!(combine([NotEvaluated, NoEvidence, Unlinked]), NoEvidence);
        assert_eq!(combine([Unlinked, Inconclusive]), Inconclusive);
        assert_eq!(combine([NotEvaluated]), NotEvaluated);
        assert_eq!(combine([]), NotEvaluated);
    }
}
