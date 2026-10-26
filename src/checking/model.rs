use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

/// The fixed set of checks in structural/v1, in report order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    /// Anchor Book configuration.
    Configuration,
    /// Source directory.
    Source,
    /// SUMMARY.md syntax and interpretation.
    Summary,
    /// mdBook file-based source/output alignment.
    Renderer,
    /// Local chapter target files.
    Chapters,
    /// Configured top-level section roles.
    Roles,
}
impl Group {
    pub(super) const ALL: [Self; 6] = [
        Self::Configuration,
        Self::Source,
        Self::Summary,
        Self::Renderer,
        Self::Chapters,
        Self::Roles,
    ];
    /// Stable display name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::Source => "source",
            Self::Summary => "summary",
            Self::Renderer => "renderer",
            Self::Chapters => "chapters",
            Self::Roles => "roles",
        }
    }
}
/// Result of an individual required check group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// The group was evaluated and its rules hold.
    Passed,
    /// Known structural violations were found.
    Failed,
    /// Reliable evaluation could not finish.
    Error,
    /// A prerequisite prevented evaluating this group.
    Blocked,
}
impl State {
    /// Stable display name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Error => "error",
            Self::Blocked => "blocked",
        }
    }
}
/// Overall result for the observed inputs, not a system-verification claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Every required group passed.
    Passed,
    /// Evaluation found structural violations; dependent groups may be blocked.
    Failed,
    /// Invalid input or an operational error prevented reliable evaluation.
    Incomplete,
}
impl Outcome {
    /// Stable display name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Incomplete => "incomplete",
        }
    }
    /// CLI status corresponding to this outcome.
    pub fn exit_code(self) -> u8 {
        match self {
            Self::Passed => 0,
            Self::Failed => 1,
            Self::Incomplete => 2,
        }
    }
}
/// The origin of a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Observed Book format violation.
    Structure,
    /// Anchor Book configuration input could not be used.
    Configuration,
    /// Another input could not be read or interpreted.
    Operation,
}
impl Kind {
    /// Stable display name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Structure => "structure",
            Self::Configuration => "configuration",
            Self::Operation => "operation",
        }
    }
}
/// A file and, when reliable, a one-based source position or configuration key.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    /// Filesystem path, never reconstructed from display text.
    pub path: PathBuf,
    /// One-based line and Unicode-scalar column; absent for unknown positions.
    pub position: Option<(usize, usize)>,
    /// Configuration key, when applicable.
    pub field: Option<String>,
}
impl Location {
    pub(super) fn file(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            position: None,
            field: None,
        }
    }
    pub(super) fn key(path: &Path, field: &str) -> Self {
        Self {
            path: path.into(),
            position: None,
            field: Some(field.into()),
        }
    }
    pub(super) fn at(path: &Path, text: &str, offset: usize) -> Self {
        let position = text.get(..offset).map(|prefix| {
            let line = prefix.bytes().filter(|b| *b == b'\n').count() + 1;
            let column = prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1;
            (line, column)
        });
        Self {
            path: path.into(),
            position,
            field: None,
        }
    }
}
/// A structural/v1 diagnostic. All findings currently have error severity.
#[derive(Debug)]
pub struct Finding {
    /// Stable diagnostic code.
    pub code: &'static str,
    /// Check group that produced it.
    pub group: Group,
    /// Structural, configuration, or operational origin.
    pub kind: Kind,
    /// Short description of the problem.
    pub summary: String,
    /// Affected location.
    pub location: Location,
    /// Declarations that explain the expectation.
    pub related: Vec<Location>,
    /// The required condition.
    pub expected: String,
    /// What was observed, or why it could not be observed.
    pub found: String,
    /// Repair/resolution guidance; this does not authorize edits.
    pub help: String,
    pub(super) cause: Option<Box<dyn Error + Send + Sync>>,
}
impl Finding {
    /// Underlying I/O or parser cause, where present.
    pub fn cause(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_deref().map(|e| e as &(dyn Error + 'static))
    }
    /// The initial rule scope has error findings only.
    pub fn severity(&self) -> &'static str {
        "error"
    }
    pub(super) fn new(
        group: Group,
        kind: Kind,
        code: &'static str,
        summary: &str,
        location: Location,
    ) -> Self {
        Self {
            group,
            kind,
            code,
            summary: summary.into(),
            location,
            expected: String::new(),
            found: String::new(),
            help: String::new(),
            related: Vec::new(),
            cause: None,
        }
    }
    pub(super) fn details(
        mut self,
        expected: impl Into<String>,
        found: impl Into<String>,
        help: &str,
    ) -> Self {
        self.expected = expected.into();
        self.found = found.into();
        self.help = help.into();
        self
    }
}
/// Evaluation status and explicit blocker for a group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupResult {
    /// Group being reported.
    pub group: Group,
    /// Observed evaluation state.
    pub state: State,
    /// Root-cause diagnostic code preventing evaluation, if blocked.
    pub blocked_by: Option<&'static str>,
}
/// Findings and scope for a completed or incomplete observation of a Book.
#[derive(Debug)]
pub struct CheckReport {
    pub(super) config: PathBuf,
    pub(super) base: Option<PathBuf>,
    pub(super) findings: Vec<Finding>,
    pub(super) groups: [GroupResult; 6],
}
impl CheckReport {
    /// The caller's explicit configuration argument.
    pub fn configuration(&self) -> &Path {
        &self.config
    }
    /// Directory against which report locations are displayed, when resolved.
    pub fn path_base(&self) -> Option<&Path> {
        self.base.as_deref()
    }
    /// Versioned rule scope; unrelated to the configuration schema version.
    pub fn scope(&self) -> &'static str {
        "structural/v1"
    }
    /// Located findings in stable order.
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }
    /// All required groups, including blocked or uncompleted groups.
    pub fn groups(&self) -> &[GroupResult] {
        &self.groups
    }
    /// Compute the result without treating an empty diagnostic list as a pass.
    pub fn outcome(&self) -> Outcome {
        if self.groups.iter().all(|g| g.state == State::Passed) {
            Outcome::Passed
        } else if self.groups.iter().any(|g| g.state == State::Error) {
            Outcome::Incomplete
        } else if self.groups.iter().any(|g| g.state == State::Failed) {
            Outcome::Failed
        } else {
            Outcome::Incomplete
        }
    }
    pub(super) fn new(config: &Path) -> Self {
        Self {
            config: config.into(),
            base: None,
            findings: Vec::new(),
            groups: Group::ALL.map(|group| GroupResult {
                group,
                state: State::Blocked,
                blocked_by: Some("configuration"),
            }),
        }
    }
    pub(super) fn pass(&mut self, group: Group) {
        self.groups[group as usize] = GroupResult {
            group,
            state: State::Passed,
            blocked_by: None,
        };
    }
    pub(super) fn block(&mut self, group: Group, code: &'static str) {
        self.groups[group as usize] = GroupResult {
            group,
            state: State::Blocked,
            blocked_by: Some(code),
        };
    }
    pub(super) fn push(&mut self, finding: Finding) {
        let g = &mut self.groups[finding.group as usize];
        if finding.kind != Kind::Structure {
            g.state = State::Error;
        } else if g.state != State::Error {
            g.state = State::Failed;
        }
        g.blocked_by = None;
        self.findings.push(finding);
    }
    pub(super) fn finish(mut self) -> Result<Self, CheckError> {
        self.findings.sort_by(|a, b| {
            (
                &a.group,
                &a.location.path,
                a.location.position,
                a.code,
                &a.location.field,
                &a.summary,
            )
                .cmp(&(
                    &b.group,
                    &b.location.path,
                    b.location.position,
                    b.code,
                    &b.location.field,
                    &b.summary,
                ))
        });
        if self.outcome() == Outcome::Incomplete {
            Err(CheckError {
                report: Box::new(self),
            })
        } else {
            Ok(self)
        }
    }
}
/// Incomplete evaluation, retaining partial findings and underlying causes.
#[derive(Debug)]
pub struct CheckError {
    report: Box<CheckReport>,
}
impl CheckError {
    /// Partial findings, evaluated scope, and errors from this invocation.
    pub fn report(&self) -> &CheckReport {
        &self.report
    }
    /// Consume the error and retain its complete partial report.
    pub fn into_report(self) -> CheckReport {
        *self.report
    }
}
impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "structural check incomplete ({} diagnostics)",
            self.report.findings.len()
        )
    }
}
impl Error for CheckError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.report.findings.iter().find_map(Finding::cause)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn outcomes_require_complete_evaluation_and_operational_errors_take_precedence() {
        let mut report = CheckReport::new(Path::new("anchorbook.toml"));
        assert_eq!(report.outcome(), Outcome::Incomplete);
        for group in Group::ALL {
            report.pass(group);
        }
        assert_eq!(report.outcome(), Outcome::Passed);
        report.push(
            Finding::new(
                Group::Source,
                Kind::Structure,
                "structure.source-invalid",
                "missing",
                Location::file("book"),
            )
            .details("directory", "missing", "restore it"),
        );
        report.block(Group::Summary, "structure.source-invalid");
        assert_eq!(report.outcome(), Outcome::Failed);
        report.push(
            Finding::new(
                Group::Renderer,
                Kind::Operation,
                "operation.read-failed",
                "cannot read",
                Location::file("book.toml"),
            )
            .details("readable", "denied", "resolve access"),
        );
        report.push(
            Finding::new(
                Group::Renderer,
                Kind::Structure,
                "structure.renderer-missing",
                "missing",
                Location::file("book.toml"),
            )
            .details("file", "missing", "restore"),
        );
        assert_eq!(report.outcome(), Outcome::Incomplete);
        let error = report.finish().unwrap_err();
        assert_eq!(error.report().findings().len(), 3);
        assert!(error.to_string().contains("incomplete (3 diagnostics)"));
        assert_eq!(
            (
                Outcome::Passed.exit_code(),
                Outcome::Failed.exit_code(),
                Outcome::Incomplete.exit_code()
            ),
            (0, 1, 2)
        );
    }
    #[test]
    fn positions_are_one_based_unicode_columns_and_invalid_offsets_are_not_invented() {
        let text = "éx\n  [链接](chapter.md)";
        assert_eq!(
            Location::at(Path::new("SUMMARY.md"), text, 0).position,
            Some((1, 1))
        );
        assert_eq!(
            Location::at(Path::new("SUMMARY.md"), text, 2).position,
            Some((1, 2))
        );
        assert_eq!(
            Location::at(Path::new("SUMMARY.md"), text, 6).position,
            Some((2, 3))
        );
        assert_eq!(
            Location::at(Path::new("SUMMARY.md"), text, 1).position,
            None
        );
        assert_eq!(
            Location::at(Path::new("SUMMARY.md"), text, 999).position,
            None
        );
    }
}
