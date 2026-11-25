//! Distribution acceptance for the public crate and its private proof workspace.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
#[allow(dead_code)]
#[path = "support/checking.rs"]
mod fixture;

fn run(directory: &Path, program: &str, arguments: &[&str]) -> String {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(directory)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{program} {arguments:?}\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn metadata(directory: &Path, locked: bool) -> Value {
    let mut args = vec!["metadata", "--format-version", "1", "--all-features"];
    if locked {
        args.push("--locked");
    }
    serde_json::from_str(&run(directory, "cargo", &args)).unwrap()
}
fn document(path: &Path) -> toml_edit::Document<String> {
    toml_edit::Document::parse(fs::read_to_string(path).unwrap()).unwrap()
}
fn verifier(name: &str) -> bool {
    name == "vstd" || name.starts_with("verus")
}
fn public_manifest(text: &str) -> bool {
    // Cargo metadata checks resolved packages as well; this rejects even optional,
    // renamed, target-specific and development verifier declarations in the archive.
    fn inspect(item: &toml_edit::Item) -> bool {
        match item {
            toml_edit::Item::Table(table) => table.iter().all(|(key, value)| {
                !(verifier(key) || key == "package" && value.as_str().is_some_and(verifier))
                    && inspect(value)
            }),
            toml_edit::Item::Value(toml_edit::Value::InlineTable(table)) => {
                table.iter().all(|(key, value)| {
                    !(verifier(key) || key == "package" && value.as_str().is_some_and(verifier))
                })
            }
            _ => true,
        }
    }
    let parsed = toml_edit::Document::parse(text).unwrap();
    inspect(parsed.as_item())
}

#[test]
fn dependency_audit_rejects_verifier_reintroductions() {
    assert!(public_manifest("[dependencies]\nserde = \"1\"\n"));
    for text in [
        "[dependencies]\nvstd = \"0.0.0\"",
        "[dependencies]\nvstd = { version = \"0.0.0\", optional = true }",
        "[dependencies]\nproof = { package = \"vstd\", version = \"0.0.0\" }",
        "[target.'cfg(unix)'.dependencies]\nverus_builtin = \"0.0.0\"",
        "[dev-dependencies]\nvstd = \"0.0.0\"",
        "[features]\nverus = []",
    ] {
        assert!(!public_manifest(text), "accepted {text}");
    }
}

fn source_files(root: &Path, relative: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root.join(relative)).unwrap() {
        let entry = entry.unwrap();
        let name = relative.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            source_files(root, &name, files);
        } else {
            files.push(name);
        }
    }
}
fn same_sources(production: &Path, package: &Path) -> bool {
    let mut paths = Vec::new();
    source_files(production, Path::new("src"), &mut paths);
    paths
        .iter()
        .all(|path| fs::read(production.join(path)).ok() == fs::read(package.join(path)).ok())
}

/// anchorbook: A-package-verified-bodies#packaged-source
#[test]
fn source_correspondence_rejects_altered_packaged_code() {
    let a = fixture::Book::new();
    let b = fixture::Book::new();
    a.write("src/lib.rs", "pub fn result() -> bool { true }");
    b.write("src/lib.rs", "pub fn result() -> bool { true }");
    assert!(same_sources(&a.root, &b.root));
    b.write("src/lib.rs", "pub fn result() -> bool { false }");
    assert!(!same_sources(&a.root, &b.root));
    fs::remove_file(b.path("src/lib.rs")).unwrap();
    assert!(!same_sources(&a.root, &b.root));
}

/// anchorbook: A-project-no-imposed-dependencies#verifier-isolation
/// anchorbook: A-package-verified-bodies#packaged-source
#[test]
#[ignore = "packages and installs the crate; requires mdBook and Rust 1.88.0"]
fn packaged_engine_is_usable_and_contains_no_verifier_dependency() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_meta = metadata(root, true);
    let public = document(&root.join("Cargo.toml"));
    let private = document(&root.join("verification/Cargo.toml"));
    assert_eq!(private["package"]["publish"].as_bool(), Some(false));
    assert_eq!(private["lib"]["path"].as_str(), Some("../src/lib.rs"));
    let public_values: Value =
        toml_edit::de::from_str(&fs::read_to_string(root.join("Cargo.toml")).unwrap()).unwrap();
    let private_values: Value =
        toml_edit::de::from_str(&fs::read_to_string(root.join("verification/Cargo.toml")).unwrap())
            .unwrap();
    assert_eq!(
        public_values["package"]["name"],
        private_values["package"]["name"]
    );
    for (name, dependency) in public_values["dependencies"].as_object().unwrap() {
        assert_eq!(
            dependency, &private_values["dependencies"][name],
            "private dependency drift: {name}"
        );
    }
    for (name, _) in private["dependencies"].as_table().unwrap().iter() {
        assert!(
            name == "vstd" || public["dependencies"].get(name).is_some(),
            "extra proof dependency: {name}"
        );
    }
    let version = public["package"]["version"].as_str().unwrap();
    let mut arguments = vec!["package", "--locked", "--all-features"];
    if std::env::var_os("ANCHORBOOK_PACKAGE_ALLOW_DIRTY").is_some() {
        arguments.push("--allow-dirty");
    }
    run(root, "cargo", &arguments);
    let package = Path::new(root_meta["target_directory"].as_str().unwrap())
        .join("package")
        .join(format!("anchor-book-{version}"));
    assert!(!package.join("verification").exists());
    assert!(public_manifest(
        &fs::read_to_string(package.join("Cargo.toml")).unwrap()
    ));
    for asset in [
        "contract/guidance.md",
        "templates/project-guidance.md",
        "LICENSE-MIT",
        "LICENSE-APACHE",
        "build.rs",
    ] {
        assert!(
            package.join(asset).is_file(),
            "missing packaged asset: {asset}"
        );
    }
    assert!(
        same_sources(root, &package),
        "packaged source differs from verified production source"
    );
    let graph = metadata(&package, true);
    assert!(
        graph["packages"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| !verifier(p["name"].as_str().unwrap()))
    );
    let lock = document(&package.join("Cargo.lock"));
    assert!(
        lock["package"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .all(|p| !verifier(p["name"].as_str().unwrap()))
    );
    run(
        &package,
        "cargo",
        &["+1.88.0", "build", "--locked", "--all-features"],
    );
    let trial = fixture::Book::new();
    let installed = trial.path("installed");
    run(
        &trial.root,
        "cargo",
        &[
            "install",
            "--path",
            package.to_str().unwrap(),
            "--locked",
            "--all-features",
            "--root",
            installed.to_str().unwrap(),
        ],
    );
    let binary = installed.join("bin/anchorbook");
    let binary = binary.to_str().unwrap();
    let reported = run(&trial.root, binary, &["--version"]);
    let vcs: Value =
        serde_json::from_str(&fs::read_to_string(package.join(".cargo_vcs_info.json")).unwrap())
            .unwrap();
    let revision = vcs["git"]["sha1"].as_str().unwrap();
    assert_eq!(revision, run(root, "git", &["rev-parse", "HEAD"]).trim());
    assert!(reported.contains(version) && reported.contains(revision));
    assert_eq!(
        run(&trial.root, binary, &["contract"]),
        anchor_book::contract::text()
    );
    assert_eq!(
        run(&trial.root, binary, &["template", "project-guidance"]).trim(),
        anchor_book::templates::PROJECT_GUIDANCE.trim()
    );
    run(
        &trial.root,
        binary,
        &["check", "--config", "anchorbook.toml"],
    );
    let renderer = format!(
        "{}\n[preprocessor.anchorbook]\ncommand = {}\nconfig = \"anchorbook.toml\"\n",
        fixture::RENDERER,
        serde_json::to_string(&format!("{binary} md")).unwrap()
    );
    trial.write("book.toml", &renderer);
    run(&trial.root, "mdbook", &["build"]);
    assert!(
        fs::read_to_string(trial.path("generated/site/design.html"))
            .unwrap()
            .contains("Arbitrary authored content.")
    );
    let manifest = format!(
        "[package]\nname = \"package-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[dependencies]\nanchor-book = {{ path = {} }}\n",
        serde_json::to_string(package.to_str().unwrap()).unwrap()
    );
    trial.write("consumer/Cargo.toml", &manifest);
    trial.write("consumer/src/main.rs", "use anchor_book::{Chapter, read_records};\nfn main() { assert!(read_records(&[Chapter::new(std::path::Path::new(\"design.md\"), \"# Design\")]).unwrap().is_empty()); }\n");
    let consumer = trial.path("consumer");
    let graph = metadata(&consumer, false);
    assert!(
        graph["packages"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| !verifier(p["name"].as_str().unwrap()))
    );
    run(&consumer, "cargo", &["run", "--locked"]);
    run(&consumer, "cargo", &["run", "--locked", "--all-features"]);
    run(&consumer, "cargo", &["+1.88.0", "run", "--locked"]);
    run(
        &consumer,
        "cargo",
        &["+1.88.0", "run", "--locked", "--all-features"],
    );
    for version in ["0.0.0-2026-10-11-0230", "0.0.0-2026-11-22-0344"] {
        trial.write(
            "consumer/Cargo.toml",
            &format!("{manifest}vstd = \"={version}\"\n"),
        );
        run(&consumer, "cargo", &["check"]);
    }
}

#[path = "support/proof_source.rs"]
mod proof_source;

/// anchorbook: A-package-verified-bodies#feature-boundary
#[test]
fn verification_configuration_cannot_change_executable_source() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut paths = Vec::new();
    source_files(root, Path::new("src"), &mut paths);
    for path in paths
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
    {
        assert!(
            proof_source::ghost_only(&fs::read_to_string(root.join(path)).unwrap()),
            "configuration can change executable source: {}",
            path.display()
        );
    }
    for altered in [
        "#[cfg(anchorbook_verification)] fn different() {}",
        "fn answer() -> bool { cfg!(anchorbook_verification) }",
        "fn answer() -> bool { cfg!(verus_keep_ghost) }",
        "#[cfg(verus_only)] fn different() {}",
        "#[cfg(verus_keep_ghost_body)] fn different() {}",
        "#[cfg(not(anchorbook_verification))] fn different() {}",
        "#[cfg_attr(anchorbook_verification, path = \"other.rs\")] mod implementation;",
        "fn answer() { #[cfg(anchorbook_verification)] return; }",
        "#[cfg(anchorbook_verification)] verus! { pub fn hidden_runtime() {} }",
    ] {
        assert!(
            !proof_source::ghost_only(altered),
            "accepted divergent source: {altered}"
        );
    }
}
