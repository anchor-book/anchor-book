//! Read-only observations. Structural decisions are made in rules.rs.
use super::{Finding, Group, Kind, Location, navigation};
use crate::configuration::{self, Configuration};
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

pub(super) struct InputError {
    code: &'static str,
    summary: &'static str,
    location: Box<Location>,
    message: String,
    cause: Option<Box<dyn Error + Send + Sync>>,
}
impl InputError {
    fn io(path: &Path, code: &'static str, summary: &'static str, error: io::Error) -> Self {
        Self {
            code,
            summary,
            location: Box::new(Location::file(path)),
            message: error.to_string(),
            cause: Some(Box::new(error)),
        }
    }
    pub fn finding(self, group: Group) -> Finding {
        let mut finding = Finding::new(
            group,
            Kind::Operation,
            self.code,
            self.summary,
            *self.location,
        )
        .details(
            "readable, supported input whose paths can be resolved",
            self.message,
            "Resolve the reported input or access problem, then rerun the same check.",
        );
        finding.cause = self.cause;
        finding
    }
}

pub(super) struct Input<T> {
    pub path: PathBuf,
    pub value: Result<Option<T>, InputError>,
}
pub(super) struct Setting {
    pub field: &'static str,
    pub anchor_field: &'static str,
    pub expected: PathBuf,
    pub resolved: io::Result<PathBuf>,
}
pub(super) struct Chapter {
    pub location: Location,
    pub depth: usize,
    pub target: Target,
}
pub(super) enum Target {
    Draft,
    External,
    Local {
        path: PathBuf,
        metadata: io::Result<fs::Metadata>,
    },
    Unresolved {
        path: PathBuf,
        error: io::Error,
    },
}

pub(super) fn text(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}
fn read(root: &Path, name: &str) -> Result<Option<String>, InputError> {
    let requested = root.join(name);
    let resolved = configuration::resolve_within(root, name, true).map_err(|error| {
        InputError::io(
            &requested,
            "input.unsupported",
            "Input path cannot be resolved within its root",
            error,
        )
    })?;
    match text(&resolved) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(InputError::io(
            &requested,
            "operation.read-failed",
            "Required input cannot be read",
            error,
        )),
    }
}

pub(super) fn renderer(config: &Configuration) -> Input<Vec<Setting>> {
    // Validated configuration has a canonical file path with a parent.
    let root = config.path().parent().unwrap_or(config.path());
    let path = root.join("book.toml");
    let value = read(root, "book.toml").and_then(|text| {
        let Some(text) = text else {
            return Ok(None);
        };
        // Pure deserialization uses mdBook defaults, never environment overrides.
        let settings = text
            .parse::<mdbook_core::config::Config>()
            .map_err(|error| InputError {
                code: "input.parse-failed",
                summary: "Renderer configuration cannot be parsed",
                location: Box::new(Location::file(&path)),
                message: format!("{error:#}"),
                cause: Some(error.into_boxed_dyn_error()),
            })?;
        Ok(Some(
            [
                (
                    "book.src",
                    settings.book.src,
                    config.source(),
                    "book.source",
                ),
                (
                    "build.build-dir",
                    settings.build.build_dir,
                    config.output(),
                    "book.output",
                ),
            ]
            .into_iter()
            .map(|(field, value, expected, anchor_field)| Setting {
                field,
                anchor_field,
                expected: expected.to_owned(),
                resolved: renderer_path(root, &value),
            })
            .collect(),
        ))
    });
    Input { path, value }
}

pub(super) fn summary(config: &Configuration) -> Input<Vec<Chapter>> {
    let path = config.source().join("SUMMARY.md");
    let value = read(config.source(), "SUMMARY.md").and_then(|text| {
        let Some(text) = text else {
            return Ok(None);
        };
        let entries = navigation::parse(&text).map_err(|error| InputError {
            code: error.code,
            summary: "Book navigation cannot be interpreted",
            location: Box::new(error.offset.map_or_else(
                || Location::file(&path),
                |offset| Location::at(&path, &text, offset),
            )),
            message: error.message,
            cause: error.cause,
        })?;
        Ok(Some(
            entries
                .into_iter()
                .map(|entry| {
                    let target = match entry.target {
                        None => Target::Draft,
                        Some(raw) if navigation::is_external(&raw.to_string_lossy()) => {
                            Target::External
                        }
                        Some(raw) => match configuration::resolve_within(
                            config.source(),
                            &raw.to_string_lossy(),
                            true,
                        ) {
                            Ok(path) => {
                                let metadata = fs::metadata(&path);
                                Target::Local { path, metadata }
                            }
                            Err(error) => Target::Unresolved {
                                path: config.source().join(raw),
                                error,
                            },
                        },
                    };
                    Chapter {
                        location: Location::at(&path, &text, entry.offset),
                        depth: entry.depth,
                        target,
                    }
                })
                .collect(),
        ))
    });
    Input { path, value }
}

/// mdBook permits relative/absolute settings; outside paths are drift rather
/// than implicit permission to read content. Only metadata is inspected here.
fn renderer_path(root: &Path, path: &Path) -> io::Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        root.join(path)
    };
    let base = absolute
        .ancestors()
        .last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no root"))?;
    let relative = absolute
        .strip_prefix(base)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let raw = relative
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "renderer path is not UTF-8"))?;
    configuration::resolve_within(base, if raw.is_empty() { "." } else { raw }, true)
}
