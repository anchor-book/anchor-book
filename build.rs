//! Records the engine's source revision so `anchorbook --version` can name it.
//!
//! A packaged crate carries its commit in `.cargo_vcs_info.json`. Otherwise, when
//! the package directory is the root of a Git checkout, as in a `cargo install
//! --git` checkout or this repository, `git` supplies `HEAD`. Source inside some
//! other repository, or without Git, has no known revision. The revision names a
//! commit; it does not describe uncommitted changes.
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("set by Cargo"));
    let revision = packaged_revision(&dir)
        .or_else(|| git_revision(&dir))
        .unwrap_or_default();
    println!("cargo:rustc-env=ANCHORBOOK_REVISION={revision}");
}

fn is_commit(text: &str) -> bool {
    text.len() == 40 && text.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The `git.sha1` value Cargo records when packaging.
fn packaged_revision(dir: &Path) -> Option<String> {
    let path = dir.join(".cargo_vcs_info.json");
    let text = fs::read_to_string(&path).ok()?;
    println!("cargo:rerun-if-changed={}", path.display());
    let after = &text[text.find("\"sha1\"")? + "\"sha1\"".len()..];
    let value = after.trim_start().strip_prefix(':')?.trim_start();
    let value = value.strip_prefix('"')?;
    let sha = &value[..value.find('"')?];
    is_commit(sha).then(|| sha.to_owned())
}

fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).ok())
        .flatten()
        .map(|text| text.trim().to_owned())
}

/// `HEAD` of the checkout whose root is the package directory.
fn git_revision(dir: &Path) -> Option<String> {
    let root = PathBuf::from(git(dir, &["rev-parse", "--show-toplevel"])?);
    if fs::canonicalize(root).ok()? != fs::canonicalize(dir).ok()? {
        return None;
    }
    let revision = git(dir, &["rev-parse", "HEAD"]).filter(|r| is_commit(r))?;
    // Rebuild when HEAD moves: its own file, the branch it names, and packed refs.
    let mut watched = vec!["HEAD".to_owned(), "packed-refs".to_owned()];
    watched.extend(git(dir, &["symbolic-ref", "-q", "HEAD"]));
    for name in watched {
        if let Some(path) = git(dir, &["rev-parse", "--git-path", &name]) {
            let path = dir.join(path);
            if path.exists() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
    Some(revision)
}
