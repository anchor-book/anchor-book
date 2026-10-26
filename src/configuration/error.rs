use std::{
    error::Error as StdError,
    fmt, io,
    ops::Range,
    path::{Path, PathBuf},
};

/// Category of a configuration-loading failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// A required file could not be read or inspected.
    Read,
    /// The input is not valid TOML.
    Parse,
    /// A field is not part of this schema.
    UnknownField,
    /// A required field is missing or has an invalid value/type.
    InvalidValue,
    /// The configuration schema version is unsupported.
    UnsupportedVersion,
    /// A path is invalid, unresolvable, overlapping, or aliases another role.
    InvalidPath,
}
impl ErrorKind {
    /// Stable diagnostic code, independent of the human-readable explanation.
    pub fn code(self) -> &'static str {
        match self {
            Self::Read => "operation.read-failed",
            Self::Parse => "input.parse-failed",
            Self::InvalidValue => "config.invalid-value",
            Self::UnknownField => "config.unknown-field",
            Self::UnsupportedVersion => "config.unsupported-version",
            Self::InvalidPath => "config.path-invalid",
        }
    }
}

/// A located configuration error with a stable category and retained cause.
#[derive(Debug)]
pub struct Error(Box<Details>);
#[derive(Debug)]
struct Details {
    kind: ErrorKind,
    path: PathBuf,
    field: Option<String>,
    span: Option<Range<usize>>,
    message: String,
    cause: Option<Box<dyn StdError + Send + Sync>>,
}
impl Error {
    /// Failure category; callers need not parse the displayed explanation.
    pub fn kind(&self) -> ErrorKind {
        self.0.kind
    }
    /// The explicit configuration argument supplied to the loader.
    pub fn path(&self) -> &Path {
        &self.0.path
    }
    /// Dot-separated field name when identified by semantic validation.
    pub fn field(&self) -> Option<&str> {
        self.0.field.as_deref()
    }
    /// Half-open UTF-8 byte range in the configuration text, when available.
    pub fn span(&self) -> Option<Range<usize>> {
        self.0.span.clone()
    }
    /// Explanation of the problem and expected input.
    pub fn message(&self) -> &str {
        &self.0.message
    }

    pub(super) fn new(
        kind: ErrorKind,
        path: &Path,
        field: Option<&str>,
        span: Option<Range<usize>>,
        message: impl Into<String>,
        cause: Option<Box<dyn StdError + Send + Sync>>,
    ) -> Self {
        Self(Box::new(Details {
            kind,
            path: path.to_owned(),
            field: field.map(str::to_owned),
            span,
            message: message.into(),
            cause,
        }))
    }
    pub(super) fn io(path: &Path, field: Option<&str>, cause: io::Error) -> Self {
        Self::new(
            ErrorKind::Read,
            path,
            field,
            None,
            format!("cannot read or inspect required input: {cause}"),
            Some(Box::new(cause)),
        )
    }
    pub(super) fn invalid(
        path: &Path,
        field: Option<&str>,
        span: Option<Range<usize>>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::InvalidValue, path, field, span, message, None)
    }
    pub(super) fn invalid_path(
        path: &Path,
        field: &str,
        span: Option<Range<usize>>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            ErrorKind::InvalidPath,
            path,
            Some(field),
            span,
            message,
            None,
        )
    }
    pub(super) fn path_failure(
        path: &Path,
        field: &str,
        span: Option<Range<usize>>,
        cause: io::Error,
    ) -> Self {
        Self::new(
            ErrorKind::InvalidPath,
            path,
            Some(field),
            span,
            format!("expected a resolvable relative path within its declared root: {cause}"),
            Some(Box::new(cause)),
        )
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind().code(), self.path().display())?;
        if let Some(field) = self.field() {
            write!(f, " [{field}]")?;
        }
        write!(f, ": {}", self.message())
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0
            .cause
            .as_deref()
            .map(|e| e as &(dyn StdError + 'static))
    }
}
