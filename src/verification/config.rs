//! The `[verification]` section of `anchorbook.toml`, read only by `verify`.
use super::VerifyError;
use crate::configuration::{self, Configuration};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml_edit::Document;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Raw {
    sources: Vec<String>,
    output: String,
    runners: Vec<String>,
}

/// Runners `verify` supports.
const RUNNERS: [&str; 2] = ["nextest", "verus"];

/// Validated `[verification]` settings with resolved paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// Directories scanned for evidence links, resolved, in declared order.
    pub sources: Vec<PathBuf>,
    /// Where runner output is written; separate from the Book and the sources.
    pub output: PathBuf,
    /// Runners the project uses.
    pub runners: Vec<String>,
}

fn invalid(field: &str, message: impl std::fmt::Display) -> VerifyError {
    VerifyError::Configuration(format!("[verification] {field}: {message}"))
}

/// Read and validate `[verification]` from the configuration's file.
pub(super) fn load(config: &Configuration) -> Result<Settings, VerifyError> {
    let text = fs::read_to_string(config.path())
        .map_err(|e| VerifyError::Read(format!("{}: {e}", config.path().display())))?;
    let document = Document::parse(text.as_str())
        .map_err(|e| VerifyError::Configuration(e.message().to_owned()))?;
    let Some(item) = document.get("verification") else {
        return Err(VerifyError::Configuration(
            "verify needs a [verification] section with sources, output, and runners".into(),
        ));
    };
    let table = item
        .as_table_like()
        .ok_or_else(|| invalid("section", "must be a table"))?;
    for (key, _) in table.iter() {
        if !["sources", "output", "runners"].contains(&key) {
            return Err(invalid(
                key,
                "unknown field; expected only sources, output, runners",
            ));
        }
    }
    let value = item
        .clone()
        .into_value()
        .map_err(|_| invalid("section", "must be a table"))?;
    let raw = Raw::deserialize(serde::de::IntoDeserializer::into_deserializer(value))
        .map_err(|e| invalid("section", e.message()))?;
    let root = config
        .path()
        .parent()
        .ok_or_else(|| invalid("section", "configuration has no directory"))?;
    let resolve = |field: &str, value: &str| {
        configuration::resolve_within(root, value, true).map_err(|e| invalid(field, e))
    };
    if raw.sources.is_empty() {
        return Err(invalid("sources", "must list at least one directory"));
    }
    let mut sources = Vec::with_capacity(raw.sources.len());
    for value in &raw.sources {
        let path = resolve("sources", value)?;
        if !path.is_dir() {
            return Err(invalid("sources", format!("`{value}` is not a directory")));
        }
        if sources.contains(&path) {
            return Err(invalid("sources", format!("`{value}` is listed twice")));
        }
        sources.push(path);
    }
    let output = resolve("output", &raw.output)?;
    let overlaps = |a: &Path, b: &Path| a.starts_with(b) || b.starts_with(a);
    if overlaps(&output, config.source()) || overlaps(&output, config.output()) {
        return Err(invalid(
            "output",
            "must be separate from the Book source and output; neither may contain the other",
        ));
    }
    if let Some(source) = sources.iter().find(|s| overlaps(&output, s)) {
        return Err(invalid(
            "output",
            format!(
                "must be separate from source directory {}",
                source.display()
            ),
        ));
    }
    for runner in &raw.runners {
        if !RUNNERS.contains(&runner.as_str()) {
            return Err(invalid(
                "runners",
                format!(
                    "`{runner}` is not supported; supported runners: {}",
                    RUNNERS.join(", ")
                ),
            ));
        }
    }
    let mut distinct = raw.runners.clone();
    distinct.sort();
    distinct.dedup();
    if distinct.len() != raw.runners.len() {
        return Err(invalid("runners", "lists a runner twice"));
    }
    Ok(Settings {
        sources,
        output,
        runners: raw.runners,
    })
}
