//! Read-only structural checking, independent of rendering and project tools.
mod inspect;
mod model;
mod navigation;
mod rules;

use crate::configuration::{self, SourceState};
pub use model::{
    CheckError, CheckReport, Finding, Group, GroupResult, Kind, Location, Outcome, State,
};
use std::path::Path;

/// Inspect the configured Book and return a completed report or an incomplete error.
///
/// `Ok` does not necessarily mean conformance: inspect [`CheckReport::outcome`].
/// Structural failures are completed reports. An error retains partial findings
/// and blocked checks. Inputs are observed without executing preprocessors,
/// fetching URLs, running project tests, creating files, or changing the cwd.
///
/// # Errors
/// Invalid configuration, unreadable inputs, or unsupported/malformed navigation
/// prevent complete evaluation and return [`CheckError`]. No Book rendering is
/// required. Observations are a filesystem snapshot, not authorization to write.
///
/// ```no_run
/// use anchor_book::checking::{check, Outcome};
/// let report = check(std::path::Path::new("anchorbook.toml"))?;
/// if report.outcome() == Outcome::Failed {
///     for finding in report.findings() { /* inspect typed findings */ }
/// }
/// # Ok::<(), anchor_book::checking::CheckError>(())
/// ```
pub fn check(config_path: &Path) -> Result<CheckReport, CheckError> {
    let mut report = CheckReport::new(config_path);
    let config = match configuration::load(config_path) {
        Ok(config) => config,
        Err(error) => {
            report.base = std::path::absolute(config_path)
                .ok()
                .and_then(|p| p.parent().map(Path::to_owned));
            let path =
                std::path::absolute(error.path()).unwrap_or_else(|_| error.path().to_owned());
            let mut location = Location::file(path);
            if let Some(span) = error.span()
                && let Ok(text) = inspect::text(error.path())
            {
                location = Location::at(&location.path, &text, span.start);
            }
            location.field = error.field().map(str::to_owned);
            let kind = if error.kind() == configuration::ErrorKind::Read {
                Kind::Operation
            } else {
                Kind::Configuration
            };
            let code = error.kind().code();
            let mut finding = Finding::new(
                Group::Configuration,
                kind,
                code,
                "Book configuration cannot be used",
                location,
            )
            .details(
                "valid schema version 1 configuration with resolvable, separate paths",
                error.to_string(),
                "Resolve the reported configuration or access problem, then rerun the same check.",
            );
            finding.cause = Some(Box::new(error));
            report.push(finding);
            for group in [
                Group::Source,
                Group::Summary,
                Group::Renderer,
                Group::Chapters,
                Group::Roles,
            ] {
                report.block(group, code);
            }
            return report.finish();
        }
    };
    report.base = config.path().parent().map(Path::to_owned);
    report.pass(Group::Configuration);
    rules::renderer(&config, inspect::renderer(&config), &mut report);
    match config.source_state() {
        SourceState::Directory => {
            report.pass(Group::Source);
            rules::navigation(&config, inspect::summary(&config), &mut report);
        }
        other => {
            report.push(
                Finding::new(
                    Group::Source,
                    Kind::Structure,
                    "structure.source-invalid",
                    "Book source is not a directory",
                    Location::file(config.source()),
                )
                .details(
                    "an existing source directory",
                    format!("{other:?}"),
                    "Restore the intended Book source directory, then rerun the check.",
                ),
            );
            for group in [Group::Summary, Group::Chapters, Group::Roles] {
                report.block(group, "structure.source-invalid");
            }
        }
    }
    report.finish()
}
