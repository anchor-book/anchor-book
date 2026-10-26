//! Filesystem and caller integration for the configuration capability.
use anchor_book::configuration::{ErrorKind, Phase, SectionRole, SourceState, load};
use std::{
    error::Error as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
const INPUT: &str = include_str!("fixtures/configuration.toml");
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "anchorbook-config-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
    fn config(&self, text: &str) -> PathBuf {
        let path = self.0.join("anchorbook.toml");
        fs::write(&path, text).unwrap();
        path
    }
    fn dir(&self, path: &str) {
        fs::create_dir_all(self.0.join(path)).unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn selects_alternative_layouts_and_resolves_all_roles_without_a_rendered_book() {
    for name in ["book", "docs", "md"] {
        let f = Fixture::new();
        f.dir(name);
        let path = f.config(&INPUT.replace("\"docs\"", &format!("\"{name}\"")));
        let actual = load(&path).unwrap();
        assert_eq!(actual.path(), path);
        assert_eq!(actual.source(), f.0.join(name));
        assert_eq!(actual.source_state(), SourceState::Directory);
        assert_eq!(actual.output(), f.0.join("generated/site"));
        assert_eq!(actual.phase(), Phase::Prototyping);
        for (role, chapter, key) in [
            (SectionRole::Design, "overview.md", "design"),
            (SectionRole::Authority, "rules.md", "authority"),
            (
                SectionRole::ImplementationRoadmaps,
                "work.md",
                "implementation-roadmaps",
            ),
        ] {
            assert_eq!(actual.section(role), f.0.join(name).join(chapter));
            assert_eq!(role.as_str(), key);
        }
        assert_eq!(SectionRole::ALL.len(), 3);
        assert!(!actual.output().exists());
        assert!(!actual.section(SectionRole::Design).exists());
        assert!(!f.0.join("generated").exists());
    }
}

#[test]
fn missing_and_non_directory_sources_are_reportable_structural_observations() {
    let f = Fixture::new();
    let path = f.config(INPUT);
    let value = load(&path).unwrap();
    assert_eq!(value.source_state(), SourceState::Missing);
    assert_eq!(
        value.section(SectionRole::Design),
        f.0.join("docs/overview.md")
    );
    assert!(!f.0.join("docs").exists());
    fs::write(f.0.join("docs"), "not a directory").unwrap();
    let value = load(&path).unwrap();
    assert_eq!(value.source_state(), SourceState::NotDirectory);
    assert_eq!(
        value.section(SectionRole::Authority),
        f.0.join("docs/rules.md")
    );
    assert_eq!(
        fs::read_to_string(f.0.join("docs")).unwrap(),
        "not a directory"
    );
}

#[test]
fn invalid_paths_are_located_and_corrections_resolve() {
    let f = Fixture::new();
    f.dir("docs");
    for (field, original, values) in [
        (
            "book.source",
            "\"docs\"",
            vec![
                "",
                " ",
                "../outside",
                "/absolute",
                "generated",
                "generated/site",
                "generated/site/nested",
            ],
        ),
        (
            "book.output",
            "\"generated/site\"",
            vec!["", "../outside", "/absolute", "docs", "docs/site", "."],
        ),
        (
            "book.sections.design",
            "\"overview.md\"",
            vec!["", "../outside", "/absolute"],
        ),
    ] {
        for value in values {
            let source = INPUT.replace(original, &format!("\"{value}\""));
            let path = f.config(&source);
            let error = load(&path).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidPath, "{source}");
            // Overlap is attributed to the output setting, with both paths described.
            assert!(error.field() == Some(field) || error.field() == Some("book.output"));
            assert_eq!(error.path(), path);
            let span = error.span().unwrap();
            assert!(source[span].starts_with('"'));
            assert!(error.to_string().contains("config.path-invalid"));
            f.config(INPUT);
            assert!(load(&path).is_ok());
        }
    }
    let path = f.config(&INPUT.replace("\"docs\"", "\"./unused/../docs\""));
    assert_eq!(load(&path).unwrap().source(), f.0.join("docs"));
    assert!(!f.0.join("unused").exists());
}

#[test]
fn duplicate_roles_are_rejected_after_resolution() {
    let f = Fixture::new();
    f.dir("docs");
    for (old, replacement, key) in [
        ("rules.md", "overview.md", "book.sections.authority"),
        (
            "work.md",
            "./overview.md",
            "book.sections.implementation-roadmaps",
        ),
        (
            "work.md",
            "unused/../rules.md",
            "book.sections.implementation-roadmaps",
        ),
    ] {
        let source = INPUT.replace(old, replacement);
        let path = f.config(&source);
        let error = load(&path).unwrap_err();
        assert_eq!(error.field(), Some(key));
        assert_eq!(error.kind(), ErrorKind::InvalidPath);
        assert!(error.message().contains("distinct chapter paths"));
    }
}

#[test]
fn filesystem_errors_preserve_causes_and_never_create_missing_paths() {
    let f = Fixture::new();
    let missing = f.0.join("absent.toml");
    let error = load(&missing).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Read);
    assert_eq!(error.path(), missing);
    assert_eq!(error.field(), None);
    assert_eq!(error.span(), None);
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<std::io::Error>()
            .is_some()
    );
    assert!(error.to_string().contains("operation.read-failed"));
    let path = f.config(INPUT);
    fs::write(&path, [0xff]).unwrap();
    assert_eq!(load(&path).unwrap_err().kind(), ErrorKind::Read);
    assert_eq!(load(&f.0).unwrap_err().kind(), ErrorKind::Read);
    fs::write(f.0.join("file"), "untouched").unwrap();
    let path = f.config(&INPUT.replace("generated/site", "file/site"));
    let error = load(&path).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::InvalidPath);
    assert!(error.source().is_some());
    let path = f.config(&INPUT.replace("generated/site", "file/../out"));
    assert_eq!(load(&path).unwrap_err().kind(), ErrorKind::InvalidPath);
    assert_eq!(fs::read_to_string(f.0.join("file")).unwrap(), "untouched");
    assert!(!missing.exists());
    assert!(!f.0.join("out").exists());
}

#[cfg(unix)]
#[test]
fn symlinks_preserve_identity_and_do_not_hide_escapes_or_dangling_targets() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    let outside = Fixture::new();
    f.dir("docs/sub");
    f.dir("generated");
    fs::write(f.0.join("docs/overview.md"), "# Design").unwrap();
    symlink("docs", f.0.join("alias")).unwrap();
    symlink("docs/sub", f.0.join("nested-alias")).unwrap();
    symlink(&outside.0, f.0.join("escape")).unwrap();
    symlink("nonexistent", f.0.join("dangling")).unwrap();
    symlink("loop", f.0.join("loop")).unwrap();
    symlink("overview.md", f.0.join("docs/duplicate.md")).unwrap();
    symlink(&outside.0, f.0.join("docs/escape")).unwrap();
    let path = f.config(&INPUT.replace("\"docs\"", "\"alias\""));
    assert_eq!(load(&path).unwrap().source(), f.0.join("docs"));
    let path = f.config(&INPUT.replace("\"docs\"", "\"nested-alias/..\""));
    assert_eq!(load(&path).unwrap().source(), f.0.join("docs"));
    for source in [
        INPUT.replace("generated/site", "alias/site"),
        INPUT.replace("\"docs\"", "\"escape\""),
        INPUT.replace("generated/site", "escape/site"),
        INPUT.replace("\"docs\"", "\"dangling\""),
        INPUT.replace("generated/site", "dangling/site"),
        INPUT.replace("\"docs\"", "\"loop\""),
        INPUT.replace("rules.md", "duplicate.md"),
        INPUT.replace("overview.md", "escape/chapter.md"),
    ] {
        let path = f.config(&source);
        assert_eq!(
            load(&path).unwrap_err().kind(),
            ErrorKind::InvalidPath,
            "{source}"
        );
    }
    let path = f.config(INPUT);
    symlink(&path, outside.0.join("selected.toml")).unwrap();
    let via_link = load(&outside.0.join("selected.toml")).unwrap();
    assert_eq!(via_link.path(), path);
    assert_eq!(via_link.source(), f.0.join("docs"));
    assert!(!outside.0.join("site").exists());
}

// Separate processes exercise relative arguments without changing this test
// process's global working directory (which would race other tests).
#[test]
fn caller_working_directory_does_not_change_the_selected_book() {
    if let Some(root) = std::env::var_os("ANCHORBOOK_CONFIG_FIXTURE_ROOT") {
        let argument = std::env::var_os("ANCHORBOOK_CONFIG_FIXTURE_ARGUMENT").unwrap();
        let value = load(Path::new(&argument)).unwrap();
        assert_eq!(value.source(), Path::new(&root).join("docs"));
        assert_eq!(value.output(), Path::new(&root).join("generated/site"));
        return;
    }
    let f = Fixture::new();
    f.dir("nested");
    f.config(INPUT);
    for (cwd, argument) in [
        (f.0.clone(), "anchorbook.toml"),
        (f.0.join("nested"), "../anchorbook.toml"),
    ] {
        let result = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "caller_working_directory_does_not_change_the_selected_book",
                "--nocapture",
            ])
            .current_dir(cwd)
            .env("ANCHORBOOK_CONFIG_FIXTURE_ROOT", &f.0)
            .env("ANCHORBOOK_CONFIG_FIXTURE_ARGUMENT", argument)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stdout)
        );
        assert!(
            result.stderr.is_empty(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn loading_preserves_all_fixture_content_and_directory_entries() {
    use std::collections::BTreeMap;
    fn snapshot(root: &Path) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
        fn visit(root: &Path, dir: &Path, entries: &mut BTreeMap<PathBuf, Option<Vec<u8>>>) {
            for entry in fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                let relative = path.strip_prefix(root).unwrap().to_owned();
                if path.is_dir() {
                    entries.insert(relative, None);
                    visit(root, &path, entries);
                } else {
                    entries.insert(relative, Some(fs::read(path).unwrap()));
                }
            }
        }
        let mut entries = BTreeMap::new();
        visit(root, root, &mut entries);
        entries
    }
    let f = Fixture::new();
    f.dir("docs");
    fs::write(f.0.join("docs/keep.md"), "unchanged").unwrap();
    for source in [
        INPUT.to_owned(),
        INPUT.replace("generated/site", "docs"),
        INPUT.replace("prototyping", "unknown"),
        INPUT.replace("schema-version = 1", "schema-version = ["),
    ] {
        let path = f.config(&source);
        let before = snapshot(&f.0);
        let _ = load(&path);
        assert_eq!(snapshot(&f.0), before);
    }
}

#[cfg(unix)]
#[test]
fn inaccessible_descendants_are_errors_not_missing_paths() {
    use std::os::unix::fs::PermissionsExt;
    struct Restore(PathBuf, fs::Permissions);
    impl Drop for Restore {
        fn drop(&mut self) {
            fs::set_permissions(&self.0, self.1.clone()).unwrap();
        }
    }
    let f = Fixture::new();
    f.dir("docs/locked");
    let locked = f.0.join("docs/locked");
    let _restore = Restore(locked.clone(), fs::metadata(&locked).unwrap().permissions());
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
    // Privileged test runners may bypass mode bits; deterministic error
    // classification is also covered by the metadata-observation unit test.
    if fs::read_dir(&locked).is_ok() {
        return;
    }
    for source in [
        INPUT.replace("\"docs\"", "\"docs/locked/child\""),
        INPUT.replace("generated/site", "docs/locked/output"),
        INPUT.replace("overview.md", "locked/chapter.md"),
    ] {
        let path = f.config(&source);
        let error = load(&path).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidPath);
        assert_eq!(
            error
                .source()
                .unwrap()
                .downcast_ref::<std::io::Error>()
                .unwrap()
                .kind(),
            std::io::ErrorKind::PermissionDenied
        );
    }
}
