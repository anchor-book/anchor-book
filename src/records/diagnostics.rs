use std::{
    error::Error,
    fmt,
    ops::Range,
    path::{Path, PathBuf},
};

/// A location in a supplied source, retaining byte offsets for later adapters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceLocation {
    pub(super) path: PathBuf,
    pub(super) range: Range<usize>,
    pub(super) line: usize,
    pub(super) column: usize,
}

impl SourceLocation {
    pub(super) fn new(path: &Path, content: &str, range: Range<usize>) -> Self {
        let prefix = &content[..range.start];
        let line_start = prefix.rfind('\n').map_or(0, |n| n + 1);
        Self {
            path: path.into(),
            range: range.clone(),
            line: prefix.bytes().filter(|&b| b == b'\n').count() + 1,
            column: content[line_start..range.start].chars().count() + 1,
        }
    }
    /// Source path exactly as supplied; it has not been canonicalized or opened.
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// Half-open UTF-8 byte range in the complete source. May be empty at EOF.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
    /// One-based line number.
    pub fn line(&self) -> usize {
        self.line
    }
    /// One-based Unicode scalar column, not a terminal display width.
    pub fn column(&self) -> usize {
        self.column
    }
}

/// Stable diagnostic families for record reading, separate from structural checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticCode {
    /// The supplied chapter inventory cannot be interpreted unambiguously.
    InvalidInput,
    /// A definition violates its canonical format or schema.
    InvalidDefinition,
    /// More than one definition declares an identity.
    DuplicateId,
    /// An authority reference is invalid or cannot resolve.
    InvalidReference,
    /// An accepted record cites a draft authority.
    IncompatibleStatus,
}

impl DiagnosticCode {
    /// Stable string suitable for assertions and display.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "record.invalid-input",
            Self::InvalidDefinition => "record.invalid-definition",
            Self::DuplicateId => "record.duplicate-id",
            Self::InvalidReference => "record.invalid-reference",
            Self::IncompatibleStatus => "record.incompatible-status",
        }
    }
}

/// Machine-readable reason within a diagnostic family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Problem {
    /// Missing or repeated chapter source identity.
    InvalidInput,
    /// Invalid Markdown boundary, TOML, field, or value.
    InvalidDefinition,
    /// Duplicate record identity, with no selected winner.
    DuplicateId,
    /// No target definition exists in an otherwise interpretable inventory.
    MissingTarget,
    /// A target definition is invalid, or invalid definitions prevent resolution.
    InvalidTarget,
    /// Multiple definitions claim the target identity.
    AmbiguousTarget,
    /// The target has the wrong record kind.
    WrongTargetKind,
    /// An authority identity occurs more than once in the same reference field.
    RepeatedReference,
    /// The target is draft but the source is accepted.
    IncompatibleStatus,
}

impl Problem {
    fn code(self) -> DiagnosticCode {
        match self {
            Self::InvalidInput => DiagnosticCode::InvalidInput,
            Self::InvalidDefinition => DiagnosticCode::InvalidDefinition,
            Self::DuplicateId => DiagnosticCode::DuplicateId,
            Self::IncompatibleStatus => DiagnosticCode::IncompatibleStatus,
            _ => DiagnosticCode::InvalidReference,
        }
    }
}

/// A single located error. Library code does not print diagnostics.
#[derive(Debug)]
pub struct Diagnostic {
    pub(super) problem: Problem,
    pub(super) message: String,
    pub(super) location: SourceLocation,
    pub(super) related: Vec<SourceLocation>,
    pub(super) cause: Option<Box<dyn Error + Send + Sync>>,
}

impl Diagnostic {
    pub(super) fn new(
        problem: Problem,
        location: SourceLocation,
        message: impl Into<String>,
    ) -> Self {
        Self {
            problem,
            message: message.into(),
            location,
            related: Vec::new(),
            cause: None,
        }
    }
    /// Stable family code.
    pub fn code(&self) -> DiagnosticCode {
        self.problem.code()
    }
    /// Reason, without requiring consumers to parse human wording.
    pub fn problem(&self) -> Problem {
        self.problem
    }
    /// Human-readable explanation; not a stable serialized interface.
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Primary source location.
    pub fn location(&self) -> &SourceLocation {
        &self.location
    }
    /// Related declaration locations when available.
    pub fn related_locations(&self) -> &[SourceLocation] {
        &self.related
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}: {}",
            self.location.path.display(),
            self.location.line,
            self.location.column,
            self.code().as_str(),
            self.message
        )
    }
}
impl Error for Diagnostic {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

/// Errors from the whole inventory. No partially validated inventory is exposed.
#[derive(Debug)]
pub struct RecordErrors(pub(super) Vec<Diagnostic>);
impl RecordErrors {
    /// All findings, sorted by path, source offset, code, and message.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.0
    }
}
impl fmt::Display for RecordErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "record validation failed with {} diagnostic(s)",
            self.0.len()
        )
    }
}
impl Error for RecordErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.first().map(|d| d as &dyn Error)
    }
}
