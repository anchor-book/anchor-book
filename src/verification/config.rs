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
    verus: Option<VerusSettings>,
}

/// The supported native Verus installation and library targets.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerusSettings {
    /// Expected Verus release identity.
    pub version: String,
    /// Verification Rust toolchain, independent of the ordinary build.
    #[serde(rename = "rust-toolchain")]
    pub rust_toolchain: String,
    /// Optional directory containing Verus, cargo-verus, and Z3; otherwise PATH.
    pub installation: Option<PathBuf>,
    /// Complete library targets available to proof criteria.
    pub targets: Vec<VerusTarget>,
}

/// One complete crate selected for native verification.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerusTarget {
    /// Cargo manifest, resolved within the project.
    pub manifest: PathBuf,
    /// Package name, checked against the manifest.
    pub package: String,
    /// The supported target kind: `lib`.
    pub target: String,
    /// Explicit Cargo features used for verification.
    pub features: Vec<String>,
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
    /// Optional native proof settings; test-only projects need none.
    pub verus: Option<VerusSettings>,
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
        if !["sources", "output", "runners", "verus"].contains(&key) {
            return Err(invalid(
                key,
                "unknown field; expected only sources, output, runners, verus",
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
    let mut verus = raw.verus;
    if let Some(settings) = &mut verus {
        if settings.version.is_empty()
            || settings.rust_toolchain.is_empty()
            || settings.targets.is_empty()
        {
            return Err(invalid(
                "verus",
                "version, rust-toolchain, and a nonempty targets list are required",
            ));
        }
        if let Some(path) = &mut settings.installation {
            if path.as_os_str().is_empty() {
                return Err(invalid("verus.installation", "must not be empty"));
            }
            if path.is_relative() {
                *path = root.join(&*path);
            }
        }
        let mut seen = std::collections::BTreeSet::new();
        for target in &mut settings.targets {
            target.manifest =
                resolve("verus.targets.manifest", &target.manifest.to_string_lossy())?;
            if !target.manifest.is_file() || target.target != "lib" || target.package.is_empty() {
                return Err(invalid(
                    "verus.targets",
                    "each target needs an existing manifest, package, and target = \"lib\"",
                ));
            }
            if !seen.insert(target.package.clone()) {
                return Err(invalid("verus.targets", "a package is listed twice"));
            }
            if target.features.iter().any(|f| {
                f.is_empty()
                    || f.starts_with('-')
                    || f.contains(',')
                    || f.chars().any(char::is_whitespace)
            }) {
                return Err(invalid(
                    "verus.targets.features",
                    "features must be nonempty individual Cargo feature names",
                ));
            }
            let text = fs::read_to_string(&target.manifest)
                .map_err(|e| invalid("verus.targets.manifest", e))?;
            let manifest =
                Document::parse(text.as_str()).map_err(|e| invalid("verus.targets.manifest", e))?;
            if manifest
                .get("package")
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                != Some(target.package.as_str())
            {
                return Err(invalid(
                    "verus.targets.package",
                    "must match the manifest's package name",
                ));
            }
        }
    }
    Ok(Settings {
        sources,
        output,
        runners: raw.runners,
        verus,
    })
}
