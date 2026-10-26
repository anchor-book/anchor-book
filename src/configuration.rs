//! Read and resolve an explicitly selected `anchorbook.toml` without writing files.
//!
//! Configuration validity does not establish Book conformance. In particular,
//! [`SourceState`] preserves a missing or non-directory source for the checker
//! to report as a structural defect. Navigation is not read by this capability.
mod error;
mod paths;
mod schema;

pub use error::{Error, ErrorKind};
pub(crate) use paths::resolve as resolve_within;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The engineer-selected working phase, not evidence of readiness or completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Explore and refine the system through scoped experiments.
    Prototyping,
    /// Implement the agreed goal design.
    Building,
    /// The engineer declares the scoped goal complete.
    Complete,
}

/// A required Book section, independent of its directory name or display title.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionRole {
    /// Goal-system design.
    Design,
    /// Requirements and design decisions.
    Authority,
    /// Agreed implementation work.
    ImplementationRoadmaps,
}
impl SectionRole {
    /// All roles required by configuration schema version 1, in stable order.
    pub const ALL: [Self; 3] = [Self::Design, Self::Authority, Self::ImplementationRoadmaps];

    /// The configuration field name for this role.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Design => "design",
            Self::Authority => "authority",
            Self::ImplementationRoadmaps => "implementation-roadmaps",
        }
    }
}

/// Source-directory observation made while loading configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceState {
    /// The source is a directory. Its navigation has not been checked.
    Directory,
    /// Its intended location resolves, but the directory does not exist.
    Missing,
    /// The selected source exists but is not a directory.
    NotDirectory,
}

/// Validated layout and phase, constructed only by [`load`].
///
/// Paths are absolute and resolve existing symbolic links. Missing suffixes are
/// resolved without creating them. Observations are a snapshot, not permission
/// for future writes; writing operations must revalidate paths and ownership.
#[derive(Debug)]
pub struct Configuration {
    path: PathBuf,
    phase: Phase,
    source: PathBuf,
    source_state: SourceState,
    output: PathBuf,
    sections: [PathBuf; 3],
}
impl Configuration {
    /// Canonical location of the configuration file actually read.
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// The declared project phase.
    pub fn phase(&self) -> Phase {
        self.phase
    }
    /// Resolved Book source directory, which may be missing or not a directory.
    pub fn source(&self) -> &Path {
        &self.source
    }
    /// Whether the selected source was usable as a directory when loaded.
    pub fn source_state(&self) -> SourceState {
        self.source_state
    }
    /// Resolved generated-output location; loading never creates it.
    pub fn output(&self) -> &Path {
        &self.output
    }
    /// Resolved chapter path for a required role; existence and placement are unchecked.
    pub fn section(&self, role: SectionRole) -> &Path {
        &self.sections[match role {
            SectionRole::Design => 0,
            SectionRole::Authority => 1,
            SectionRole::ImplementationRoadmaps => 2,
        }]
    }
}

/// Read, validate, and resolve an explicit configuration file.
///
/// Relative arguments are resolved from the caller's working directory. Existing
/// symlinks in the configuration filename are resolved first; its actual parent
/// is the layout root. Source/output paths are relative to that root and section
/// paths to the source. The process working directory is never changed.
///
/// A missing/non-directory source is returned as data for later structural
/// checking. An unresolvable path, including a dangling symlink, is an error.
/// No Book, renderer, network operation, or subprocess is executed.
///
/// # Errors
/// Returns located, typed errors for unreadable or invalid configuration and
/// unsafe or unresolvable path relationships, preserving underlying causes.
///
/// ```no_run
/// use anchor_book::configuration::{load, SectionRole};
/// let config = load(std::path::Path::new("anchorbook.toml"))?;
/// let design_chapter = config.section(SectionRole::Design);
/// # Ok::<(), anchor_book::configuration::Error>(())
/// ```
pub fn load(path: &Path) -> Result<Configuration, Error> {
    let actual = fs::canonicalize(path).map_err(|cause| Error::io(path, None, cause))?;
    let text = fs::read_to_string(&actual).map_err(|cause| Error::io(path, None, cause))?;
    let settings = schema::parse(path, &text)?;
    // A readable file always has a parent after canonicalization.
    let root = actual.parent().ok_or_else(|| {
        Error::invalid(
            path,
            None,
            None,
            "configuration must have a parent directory",
        )
    })?;
    let resolve = |base: &Path, value: &str, field: &str, inspect| {
        paths::resolve(base, value, inspect)
            .map_err(|cause| Error::path_failure(path, field, settings.span(field), cause))
    };
    let source = resolve(root, &settings.source, "book.source", true)?;
    let output = resolve(root, &settings.output, "book.output", true)?;
    if source.starts_with(&output) || output.starts_with(&source) {
        return Err(Error::invalid_path(
            path,
            "book.output",
            settings.span("book.output"),
            "source and output must be separate; neither may equal or contain the other",
        ));
    }
    let source_state = match paths::missing_is_none(fs::metadata(&source))
        .map_err(|cause| Error::io(path, Some("book.source"), cause))?
    {
        Some(meta) if meta.is_dir() => SourceState::Directory,
        Some(_) => SourceState::NotDirectory,
        None => SourceState::Missing,
    };
    let mut sections = [PathBuf::new(), PathBuf::new(), PathBuf::new()];
    for (index, role) in SectionRole::ALL.into_iter().enumerate() {
        let field = format!("book.sections.{}", role.as_str());
        // No descendant can exist beneath a missing or non-directory source.
        // Retain intended paths; the checker must report the source defect first.
        sections[index] = resolve(
            &source,
            &settings.sections[index],
            &field,
            source_state == SourceState::Directory,
        )?;
        if sections[..index].contains(&sections[index]) {
            return Err(Error::invalid_path(
                path,
                &field,
                settings.span(&field),
                "required section roles must resolve to distinct chapter paths",
            ));
        }
    }
    Ok(Configuration {
        path: actual,
        phase: settings.phase,
        source,
        source_state,
        output,
        sections,
    })
}
