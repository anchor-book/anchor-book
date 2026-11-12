//! Versioned projection of current results. No storage, import, or runner I/O.
use crate::verification::{
    ClaimOutcome, FunctionId, Mode, Report, Scope, State, Target, VerifyError,
};
use serde_json::{Value, json};
use std::path::Path;

const LIMITS: &[&str] = &[
    "Satisfied means linked checks passed in this evaluation; it does not establish their sufficiency.",
    "Proof-target success does not satisfy outstanding criterion assessments or guards.",
    "This is a current-result snapshot, not a report importer or a durable evidence archive.",
];

fn scope(scope: &Scope) -> Value {
    let methods: Vec<_> = if scope.methods.is_empty() {
        vec!["test", "proof", "review"]
    } else {
        scope.methods.iter().map(|m| m.as_str()).collect()
    };
    json!({"anchors": scope.anchors, "methods": methods})
}

fn path(path: &Path, base: &Path) -> Option<String> {
    let relative = if path.is_absolute() {
        path.strip_prefix(base).ok()?
    } else {
        path
    };
    Some(relative.to_string_lossy().into_owned())
}

fn location(
    source: &Path,
    relative_to: &str,
    line: Option<usize>,
    column: Option<usize>,
    base: &Path,
) -> Value {
    json!({"relative_to": relative_to, "path": path(source, base), "line": line, "column": column})
}

fn function(id: &FunctionId) -> Value {
    let (kind, name) = match &id.target {
        Target::Lib => ("lib", None),
        Target::Bin(name) => ("bin", Some(name)),
        Target::Test(name) => ("test", Some(name)),
    };
    json!({"package": id.package, "target": {"kind": kind, "name": name},
        "module": id.module, "name": id.name})
}

fn counts(states: impl Iterator<Item = State>) -> Value {
    let states: Vec<_> = states.collect();
    let counts: std::collections::BTreeMap<_, _> = [
        State::Satisfied,
        State::Partial,
        State::Failed,
        State::Inconclusive,
        State::NoEvidence,
        State::Unlinked,
        State::NotEvaluated,
    ]
    .into_iter()
    .map(|state| {
        (
            state.as_str(),
            states.iter().filter(|s| **s == state).count(),
        )
    })
    .collect();
    json!({"total": states.len(), "states": counts})
}

/// Project a completed evaluation into `anchorbook.verify` schema version 1.
///
/// The result is ready for serialization or direct consumption. It writes no
/// files and does not evaluate, import, or refresh evidence. Empty scope anchors
/// mean all anchors. Locations explicitly distinguish Book and configuration bases.
pub fn json_report(report: &Report) -> Value {
    let chapters: Vec<_> = report.chapters.iter().map(|chapter| {
        let anchors: Vec<_> = chapter.anchors.iter().map(|anchor| {
            let criteria: Vec<_> = anchor.criteria.iter().map(|criterion| {
                let methods: Vec<_> = criterion.methods.iter().map(|method| {
                    json!({"method": method.method.as_str(), "state": method.state.as_str(),
                        "reason": method.reason,
                        "links": method.links.iter().map(function).collect::<Vec<_>>()})
                }).collect();
                json!({"id": criterion.id, "statement": criterion.statement,
                    "state": criterion.state.as_str(), "methods": methods})
            }).collect();
            let status = match anchor.status {
                crate::Status::Draft => "draft",
                crate::Status::Accepted => "accepted",
            };
            json!({"id": anchor.id, "status": status, "state": anchor.state.as_str(),
                "location": location(anchor.location.path(), "book", Some(anchor.location.line()),
                    Some(anchor.location.column()), &report.base), "criteria": criteria})
        }).collect();
        json!({"path": path(&chapter.path, &report.base), "title": chapter.title, "anchors": anchors})
    }).collect();
    let links: Vec<_> = report.links.iter().map(|link| {
        let mode = match link.mode { Mode::Test => "test", Mode::Verified => "exec", Mode::Proof => "proof" };
        json!({"anchor": link.anchor, "criterion": link.criterion, "method": link.method.as_str(),
            "function": function(&link.function), "mode": mode, "witness": link.witness,
            "location": location(&link.path, "configuration", Some(link.line), None, &report.base)})
    }).collect();
    let findings: Vec<_> = report.findings.iter().map(|finding| {
        json!({"code": finding.code, "message": finding.message,
            "location": location(&finding.path, "configuration", Some(finding.line), None, &report.base)})
    }).collect();
    let claims: Vec<_> = report.claims.iter().map(|claim| {
        let (outcome, state, reason) = match &claim.outcome {
            ClaimOutcome::Satisfied => ("satisfied", None, None),
            ClaimOutcome::Unsatisfied(state) => ("unsatisfied", Some(state.as_str()), None),
            ClaimOutcome::Unchecked(reason) => ("unchecked", None, Some(reason)),
        };
        json!({"milestone": claim.claim.milestone, "completion_criterion": claim.claim.completion_criterion,
            "anchor": claim.claim.anchor, "criterion": claim.claim.criterion,
            "outcome": outcome, "state": state, "reason": reason,
            "location": location(&claim.claim.path, "book", Some(claim.claim.line), None, &report.base)})
    }).collect();
    json!({"kind": "anchorbook.verify", "schema_version": 1, "complete": true,
        "outcome": if report.exit_code() == 0 { "progress" } else { "claims-unsatisfied" },
        "exit_code": report.exit_code(), "scope": scope(&report.scope), "inputs": report.inputs,
        "book_source": path(&report.book_source, &report.base), "chapters": chapters,
        "evidence_links": links, "findings": findings, "claims": claims, "tools": report.tools,
        "proof_targets": report.proofs,
        "summary": {"anchors": counts(report.anchors().map(|a| a.state)),
            "criteria": counts(report.anchors().flat_map(|a| a.criteria.iter().map(|c| c.state)))},
        "limits": LIMITS})
}

/// Project an incomplete evaluation into the same versioned result envelope.
///
/// `base` is the configuration directory, used to relativize known paths. No
/// successful input identity or evidence is invented when planning failed.
pub fn json_error(error: &VerifyError, requested: &Scope, base: &Path) -> Value {
    let mut structural_check = Value::Null;
    let simple = |code, text: &str| vec![json!({"code": code, "message": text, "location": null})];
    let findings = match error {
        VerifyError::Check(report) => {
            let base = report.path_base().unwrap_or(base);
            let groups: Vec<_> = report.groups().iter().map(|group| json!({
                "group": group.group.as_str(), "state": group.state.as_str(), "blocked_by": group.blocked_by
            })).collect();
            structural_check = json!({"scope": report.scope(), "outcome": report.outcome().as_str(), "groups": groups});
            report.findings().iter().map(|finding| {
                let at = |loc: &crate::checking::Location| {
                    let mut at = location(&loc.path, "configuration", loc.position.map(|p| p.0), loc.position.map(|p| p.1), base);
                    at["field"] = json!(loc.field);
                    at
                };
                json!({"code": finding.code, "message": finding.summary,
                    "location": at(&finding.location), "related": finding.related.iter().map(at).collect::<Vec<_>>(),
                    "expected": finding.expected, "found": finding.found,
                    "help": finding.help})
            }).collect()
        }
        VerifyError::Records(errors) => errors.diagnostics().iter().map(|finding| {
            let at = |loc: &crate::SourceLocation| location(loc.path(), "book", Some(loc.line()), Some(loc.column()), base);
            json!({"code": finding.code().as_str(), "message": finding.message(),
                "location": at(finding.location()), "related": finding.related_locations().iter().map(at).collect::<Vec<_>>()})
        }).collect(),
        VerifyError::Roadmaps(errors) => errors.diagnostics().iter().map(|finding| json!({
            "code": finding.code(), "message": finding.message(),
            "location": location(finding.path(), "book", Some(finding.line()), None, base)
        })).collect(),
        VerifyError::Configuration(text) => simple("verify.invalid-configuration", text),
        VerifyError::Scope(text) => simple("verify.invalid-scope", text),
        VerifyError::Read(text) => simple("verify.read-failed", text),
    };
    json!({"kind": "anchorbook.verify", "schema_version": 1, "complete": false,
        "outcome": "incomplete", "exit_code": 2, "scope": scope(requested), "inputs": null,
        "book_source": null, "chapters": [], "evidence_links": [], "claims": [], "tools": [],
        "proof_targets": null, "summary": null, "findings": findings,
        "structural_check": structural_check, "limits": LIMITS})
}
