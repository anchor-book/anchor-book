//! Identities of the inputs a plan was made from, so a report states what it describes.
use super::PlannedCriterion;
use crate::configuration::Configuration;
use crate::records::{RecordData, RecordInventory};
use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Content identities of a run's inputs, and the Git state when available.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Inputs {
    /// Identity of the evaluated criteria: their anchors, identifiers, methods, and statements.
    pub criteria: String,
    /// Identity of the configuration file's contents.
    pub configuration: String,
    /// Identity of the linked code: every file in the configured sources, and
    /// the manifests and lockfiles of the packages with links.
    pub code: String,
    /// The commit checked out, when the configuration is in a Git checkout.
    pub revision: Option<String>,
    /// Whether the checkout has uncommitted changes, when Git can tell.
    pub dirty: Option<bool>,
}

/// 64-bit FNV-1a: a stable content identity, not a security hash.
fn fnv1a(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

pub(super) fn inputs(
    config: &Configuration,
    criteria: &[PlannedCriterion],
    inventory: &RecordInventory,
    code: &[PathBuf],
) -> Inputs {
    let mut text = String::new();
    for criterion in criteria {
        let statement = inventory
            .get(&criterion.anchor)
            .and_then(|r| match r.data() {
                RecordData::Anchor(anchor) => anchor
                    .criteria
                    .iter()
                    .find(|c| c.id == criterion.criterion)
                    .map(|c| c.statement.as_str()),
                _ => None,
            });
        let methods: Vec<&str> = criterion.methods.iter().map(|m| m.as_str()).collect();
        // Each field ends with a separator that cannot occur inside identifiers.
        let _ = writeln!(
            text,
            "{}#{}\u{1f}{}\u{1f}{}",
            criterion.anchor,
            criterion.criterion,
            methods.join(","),
            statement.unwrap_or_default()
        );
    }
    let configuration = fs::read(config.path()).map_or_else(|_| String::new(), |b| fnv1a(&b));
    let directory = config.path().parent().unwrap_or(Path::new("."));
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(directory)
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok())
    };
    let revision = git(&["rev-parse", "HEAD"]).map(|r| r.trim().to_owned());
    let dirty = revision
        .as_ref()
        .and(git(&["status", "--porcelain", "--untracked-files=no"]))
        .map(|status| !status.trim().is_empty());
    Inputs {
        criteria: fnv1a(text.as_bytes()),
        configuration,
        code: snapshot(code),
        revision,
        dirty,
    }
}

fn files(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                files(&entry.path(), out);
            }
        }
    } else if path.is_file() {
        out.push(path.to_owned());
    }
}

/// One identity for the contents of every file under `paths`, independent of
/// enumeration order; a file that cannot be read contributes its path only.
pub(super) fn snapshot(paths: &[PathBuf]) -> String {
    let mut all = Vec::new();
    for path in paths {
        files(path, &mut all);
    }
    all.sort();
    all.dedup();
    let mut bytes = Vec::new();
    for file in all {
        bytes.extend(file.to_string_lossy().as_bytes());
        bytes.push(0);
        bytes.extend(fs::read(&file).unwrap_or_default());
        bytes.push(0);
    }
    fnv1a(&bytes)
}

#[cfg(test)]
mod tests {
    use super::fnv1a;

    #[test]
    fn identities_are_the_published_fnv1a_values() {
        // Reference values for 64-bit FNV-1a.
        assert_eq!(fnv1a(b""), "cbf29ce484222325");
        assert_eq!(fnv1a(b"a"), "af63dc4c8601ec8c");
        assert_eq!(fnv1a(b"foobar"), "85944171f73967e8");
    }
}
