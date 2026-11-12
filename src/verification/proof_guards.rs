//! Aggregation of already-evaluated proof obligations. This module neither runs
//! guards nor reads assessments; the native runner leaves these results absent.
use super::State;

/// A normalized result for one post-verification obligation in the current run.
/// Producers must establish the result for the criterion and inputs being
/// evaluated. Supplying an outcome does not execute or validate a guard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuardResult {
    /// The entire obligation has current passing evidence.
    Passed,
    /// A witness failed, a specification assessment rejected the claim, or a
    /// sensitivity survivor was unclassified. The reason identifies the finding.
    Failed(String),
    /// Evaluation was inconclusive, for example a sensitivity timeout.
    Inconclusive(String),
    /// Evidence is missing, for example an absent witness or missing/stale assessment.
    NoEvidence(String),
    /// Evaluation was skipped or is unavailable, with its reason.
    NotEvaluated(String),
}

/// The three required post-verification results for one proof criterion.
/// Trust-policy enforcement belongs to the original target run. These results
/// are consulted only after that run is complete and every linked function passed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofGuards {
    /// All required witnesses verified in the original run, or no linked
    /// function has preconditions. An unwitnessed precondition has no evidence.
    pub witnesses: GuardResult,
    /// An accepting assessment matches the current specification fingerprint.
    /// Missing or stale assessments have no evidence; rejection fails the method.
    pub assessment: GuardResult,
    /// Every sensitivity result is accounted for under the criterion's policy.
    /// Unclassified survivors fail; timeouts are inconclusive; skipping leaves
    /// this obligation not evaluated. Unviable and safety-only rejections do
    /// not establish detection by the claim.
    pub sensitivity: GuardResult,
}

impl ProofGuards {
    pub(super) fn state(&self) -> (State, String) {
        let mut problems = Vec::new();
        let mut reasons = Vec::new();
        for (name, result) in [
            ("witnesses", &self.witnesses),
            ("specification assessment", &self.assessment),
            ("sensitivity", &self.sensitivity),
        ] {
            let (state, reason) = match result {
                GuardResult::Passed => continue,
                GuardResult::Failed(reason) => (State::Failed, reason),
                GuardResult::Inconclusive(reason) => (State::Inconclusive, reason),
                GuardResult::NoEvidence(reason) => (State::NoEvidence, reason),
                GuardResult::NotEvaluated(reason) => (State::NotEvaluated, reason),
            };
            problems.push(state);
            reasons.push(format!("{name}: {reason}"));
        }
        // These are obligations of one method, not separate criterion methods:
        // a passing guard never turns an unsatisfied proof method into Partial.
        let state = problems.into_iter().min().unwrap_or(State::Satisfied);
        let reason = if reasons.is_empty() {
            "linked functions verified; witnesses, specification assessment, and sensitivity passed"
                .into()
        } else {
            reasons.join("; ")
        };
        (state, reason)
    }
}
