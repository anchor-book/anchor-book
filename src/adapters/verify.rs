//! Text and structured presentation of a `verify` result, without process I/O.
mod json;
use crate::verification::{ClaimOutcome, Report, State};
pub use json::{json_error, json_report};
use std::fmt::Write;

fn plural(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

fn counts(states: impl Iterator<Item = State>) -> String {
    let states: Vec<State> = states.collect();
    let mut parts = Vec::new();
    for state in [
        State::Satisfied,
        State::Partial,
        State::Failed,
        State::Inconclusive,
        State::NoEvidence,
        State::Unlinked,
        State::NotEvaluated,
    ] {
        let n = states.iter().filter(|s| **s == state).count();
        if n > 0 || matches!(state, State::Satisfied | State::Unlinked) {
            parts.push(format!("{n} {}", state.as_str()));
        }
    }
    parts.join(", ")
}

/// The report as readable text: progress by chapter, link findings, claims, and summary.
pub fn format_report(report: &Report) -> String {
    let mut out = String::new();
    let unsatisfied = report
        .claims
        .iter()
        .filter(|c| matches!(c.outcome, ClaimOutcome::Unsatisfied(_)))
        .count();
    let _ = writeln!(
        out,
        "anchorbook verify: {}",
        if unsatisfied > 0 {
            "claims unsatisfied"
        } else {
            "progress"
        }
    );
    let anchors = if report.scope.anchors.is_empty() {
        "all anchors".to_owned()
    } else {
        format!("anchors {}", report.scope.anchors.join(", "))
    };
    let methods = if report.scope.methods.is_empty() {
        "test, proof, review".to_owned()
    } else {
        report
            .scope
            .methods
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let _ = writeln!(out, "scope: {anchors}; methods {methods}");
    let revision = match (&report.inputs.revision, report.inputs.dirty) {
        (Some(revision), Some(true)) => format!("revision {revision} with uncommitted changes"),
        (Some(revision), _) => format!("revision {revision}"),
        (None, _) => "no Git revision".to_owned(),
    };
    let _ = writeln!(
        out,
        "inputs: criteria {}, configuration {}, code {}, {revision}",
        report.inputs.criteria, report.inputs.configuration, report.inputs.code
    );
    if !report.tools.is_empty() {
        let _ = writeln!(out, "runners: {}", report.tools.join(", "));
    }
    for chapter in &report.chapters {
        let _ = writeln!(out, "\n{} ({})", chapter.title, chapter.path.display());
        for anchor in &chapter.anchors {
            let satisfied = anchor
                .criteria
                .iter()
                .filter(|c| c.state == State::Satisfied)
                .count();
            let _ = writeln!(
                out,
                "  {} ({}) — {}, {} of {} satisfied",
                anchor.id,
                match anchor.status {
                    crate::records::Status::Draft => "draft",
                    crate::records::Status::Accepted => "accepted",
                },
                anchor.state.as_str(),
                satisfied,
                plural(anchor.criteria.len(), "criterion", "criteria")
            );
            for criterion in &anchor.criteria {
                let methods: Vec<&str> = criterion
                    .methods
                    .iter()
                    .map(|m| m.method.as_str())
                    .collect();
                if let [method] = criterion.methods.as_slice() {
                    let _ = writeln!(
                        out,
                        "    {} [{}]: {} — {}",
                        criterion.id,
                        methods.join(", "),
                        method.state.as_str(),
                        method.reason
                    );
                } else {
                    let _ = writeln!(
                        out,
                        "    {} [{}]: {}",
                        criterion.id,
                        methods.join(", "),
                        criterion.state.as_str()
                    );
                    for method in &criterion.methods {
                        let _ = writeln!(
                            out,
                            "      {}: {} — {}",
                            method.method.as_str(),
                            method.state.as_str(),
                            method.reason
                        );
                    }
                }
            }
        }
    }
    if !report.findings.is_empty() {
        let _ = writeln!(out, "\nlink findings:");
        for finding in &report.findings {
            let _ = writeln!(out, "  {finding}");
        }
    }
    if let Some(proof) = &report.proofs {
        let _ = writeln!(
            out,
            "\nproof targets: {}",
            if proof.complete {
                "verified against contracts; criterion guards remain outstanding"
            } else {
                "not established"
            }
        );
        for reason in &proof.reasons {
            let _ = writeln!(out, "  {reason}");
        }
        for target in &proof.targets {
            let _ = writeln!(
                out,
                "  {} ({}): {}",
                target.package,
                target.target,
                if target.complete {
                    "complete"
                } else {
                    "incomplete or failed"
                }
            );
            for reason in &target.reasons {
                let _ = writeln!(out, "    {reason}");
            }
            for function in target.functions.iter().filter(|f| !f.success) {
                let _ = writeln!(out, "    failed: {} ({})", function.function, function.mode);
            }
            for location in &target.locations {
                let _ = writeln!(out, "    at {location}");
            }
        }
    }
    if !report.claims.is_empty() {
        let satisfied = report
            .claims
            .iter()
            .filter(|c| c.outcome == ClaimOutcome::Satisfied)
            .count();
        let unchecked = report.claims.len() - satisfied - unsatisfied;
        let _ = writeln!(
            out,
            "\nclaims: {}: {satisfied} satisfied, {unsatisfied} unsatisfied, {unchecked} unchecked",
            plural(report.claims.len(), "claim", "claims")
        );
        for claim in &report.claims {
            let (label, detail) = match &claim.outcome {
                ClaimOutcome::Satisfied => continue,
                ClaimOutcome::Unsatisfied(state) => ("unsatisfied", state.as_str().to_owned()),
                ClaimOutcome::Unchecked(reason) => ("unchecked", reason.clone()),
            };
            let _ = writeln!(
                out,
                "  {label}: {}#{} ({detail}), claimed by {} at {}:{}",
                claim.claim.anchor,
                claim.claim.criterion,
                claim.claim.completion_criterion,
                claim.claim.path.display(),
                claim.claim.line
            );
        }
    }
    let anchor_count = report.anchors().count();
    let criterion_count: usize = report.anchors().map(|a| a.criteria.len()).sum();
    let _ = writeln!(
        out,
        "\nsummary: {}: {}",
        plural(anchor_count, "anchor", "anchors"),
        counts(report.anchors().map(|a| a.state))
    );
    let _ = writeln!(
        out,
        "         {}: {}",
        plural(criterion_count, "criterion", "criteria"),
        counts(
            report
                .anchors()
                .flat_map(|a| a.criteria.iter().map(|c| c.state))
        )
    );
    let _ = writeln!(
        out,
        "satisfied means linked tests and proofs passed in this run; it does not show they are sufficient"
    );
    let _ = writeln!(out, "files changed: none");
    out
}
