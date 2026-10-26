//! Plain-text presentation of typed structural reports; no process I/O.
use crate::checking::{CheckReport, Location, Outcome};
use std::{fmt::Write, path::Path};

fn safe(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if c.is_control() {
                c.escape_default().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}
fn path(path: &Path, base: Option<&Path>) -> String {
    safe(
        &base
            .and_then(|b| path.strip_prefix(b).ok())
            .unwrap_or(path)
            .to_string_lossy(),
    )
}
fn location(value: &Location, report: &CheckReport) -> String {
    let mut text = path(&value.path, report.path_base());
    if let Some((line, column)) = value.position {
        let _ = write!(text, ":{line}:{column}");
    }
    if let Some(field) = &value.field {
        let _ = write!(text, " [{}]", safe(field));
    }
    text
}

/// Format a report for engineers and agents without color or interactive prompts.
/// The caller selects stdout/stderr and the exit status from the report outcome.
pub fn format_report(report: &CheckReport) -> String {
    let outcome = report.outcome();
    let mut text = format!(
        "anchorbook check: {}\nconfig: {}\npaths: relative to the configuration directory\nscope: {}\n",
        outcome.as_str(),
        safe(&report.configuration().to_string_lossy()),
        report.scope()
    );
    for finding in report.findings() {
        let _ = write!(
            text,
            "\n{}[{}] ({}): {}\n  at: {}\n",
            finding.severity(),
            finding.code,
            finding.kind.as_str(),
            safe(&finding.summary),
            location(&finding.location, report)
        );
        for related in &finding.related {
            let _ = writeln!(text, "  declared by: {}", location(related, report));
        }
        let _ = write!(
            text,
            "  expected: {}\n  found: {}\n  help: {}\n",
            safe(&finding.expected),
            safe(&finding.found),
            safe(&finding.help)
        );
    }
    text.push_str("\nchecks:");
    for group in report.groups() {
        let _ = write!(text, " {} {}", group.group.as_str(), group.state.as_str());
        if let Some(reason) = group.blocked_by {
            let _ = write!(text, " by {reason}");
        }
        text.push(';');
    }
    let _ = writeln!(
        text,
        "\nresult: {} ({} errors)",
        outcome.as_str(),
        report.findings().len()
    );
    match outcome {
        Outcome::Passed=>text.push_str("not evaluated: anchor semantics, implementation, verification evidence, roadmap completion, or links within chapter prose\n"),
        Outcome::Failed=>text.push_str("next: Repair the reported structure, then rerun the same check.\n"),
        Outcome::Incomplete=>text.push_str("findings are partial; resolve input or access problems, then rerun the same check.\n"),
    }
    text.push_str("files changed by check: none\n");
    text
}
