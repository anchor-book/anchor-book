//! Project-owned working instructions have their own mapped, checked Book page.
#[allow(dead_code)]
#[path = "support/checking.rs"]
mod fixture;

use anchor_book::{checking::Outcome, configuration};
use fixture::{Book, CONFIG, SUMMARY};
use std::{
    fs,
    io::Write,
    process::{Command, Output, Stdio},
};

const GUIDANCE: &str = "# Team handbook\n\n## Reviewing changes\n\nRun the project checks and retain the engineer's agreed workflow.\n";

fn mapped(book: &Book, target: &str) {
    book.write(
        "anchorbook.toml",
        &format!("{CONFIG}project-guidance = {target:?}\n"),
    );
}

fn run(book: &Book, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .current_dir(&book.root)
        .args(args)
        .output()
        .unwrap()
}

/// anchorbook: A-check-configuration-validity#schema-rejection
/// anchorbook: A-check-configuration-validity#path-boundaries
#[test]
fn guidance_mapping_is_explicit_distinct_and_confined_to_book_sources() {
    let book = Book::new();
    assert!(
        configuration::load(&book.config())
            .unwrap()
            .project_guidance()
            .is_none()
    );
    for target in ["workflow.md", "guides/developing.md"] {
        mapped(&book, target);
        let config = configuration::load(&book.config()).unwrap();
        assert_eq!(
            config.project_guidance(),
            Some(book.path(&format!("docs/{target}")).as_path())
        );
        assert_eq!(config.mapped_sections().count(), 4);
        assert!(!config.project_guidance().unwrap().exists());
    }
    for target in [
        "",
        " ",
        "../outside.md",
        "/absolute.md",
        "design.md",
        "./authority.md",
        "unused/../roadmap.md",
    ] {
        mapped(&book, target);
        let before = book.snapshot();
        let error = configuration::load(&book.config()).unwrap_err();
        assert_eq!(
            error.kind(),
            configuration::ErrorKind::InvalidPath,
            "{target}"
        );
        assert_eq!(error.field(), Some("book.sections.project-guidance"));
        assert!(error.span().is_some());
        assert_eq!(
            run(&book, &["check", "--config", "anchorbook.toml"])
                .status
                .code(),
            Some(2)
        );
        assert_eq!(book.snapshot(), before);
    }
    for value in ["7", "[]", "false"] {
        book.write(
            "anchorbook.toml",
            &format!("{CONFIG}project-guidance = {value}\n"),
        );
        assert_eq!(
            configuration::load(&book.config()).unwrap_err().kind(),
            configuration::ErrorKind::InvalidValue
        );
    }
    book.write(
        "anchorbook.toml",
        &format!("{CONFIG}project-guidence = \"workflow.md\"\n"),
    );
    assert_eq!(
        configuration::load(&book.config()).unwrap_err().kind(),
        configuration::ErrorKind::UnknownField
    );
}

/// anchorbook: A-check-configuration-validity#path-boundaries
#[cfg(unix)]
#[test]
fn guidance_cannot_alias_another_role_or_escape_through_a_symlink() {
    use std::os::unix::fs::symlink;
    let book = Book::new();
    symlink("roadmap.md", book.path("docs/review.md")).unwrap();
    mapped(&book, "review.md");
    assert_eq!(
        configuration::load(&book.config()).unwrap_err().kind(),
        configuration::ErrorKind::InvalidPath
    );
    book.write("outside.md", GUIDANCE);
    symlink("../outside.md", book.path("docs/outside.md")).unwrap();
    mapped(&book, "outside.md");
    assert_eq!(
        configuration::load(&book.config()).unwrap_err().kind(),
        configuration::ErrorKind::InvalidPath
    );
}

/// anchorbook: A-check-navigation-conformance#required-roles
/// anchorbook: A-check-observation-only#file-preservation
#[test]
fn mapped_guidance_requires_a_real_top_level_chapter_and_preserves_local_content() {
    let book = Book::new();
    // Existing Books remain valid, even if an unmapped guidance page happens to exist.
    book.write("docs/team.md", GUIDANCE);
    assert_eq!(
        anchor_book::checking::check(&book.config())
            .unwrap()
            .outcome(),
        Outcome::Passed
    );
    mapped(&book, "team.md");
    for (navigation, expected, code) in [
        (
            SUMMARY.to_owned(),
            Outcome::Failed,
            Some("structure.role-missing"),
        ),
        (
            format!("{SUMMARY}  - [Team handbook](team.md)\n"),
            Outcome::Failed,
            Some("structure.role-not-top-level"),
        ),
        (
            format!("{SUMMARY}\n# Team handbook\n"),
            Outcome::Failed,
            Some("structure.role-missing"),
        ),
        (
            format!("{SUMMARY}\n---\n"),
            Outcome::Failed,
            Some("structure.role-missing"),
        ),
        (
            format!("{SUMMARY}\n- [Team handbook]()\n"),
            Outcome::Failed,
            Some("structure.role-missing"),
        ),
        (
            format!("{SUMMARY}\n- [Team handbook](https://example.invalid/team.md)\n"),
            Outcome::Failed,
            Some("structure.role-missing"),
        ),
        (
            format!("{SUMMARY}\n[Team handbook](team.md)\n"),
            Outcome::Passed,
            None,
        ),
    ] {
        book.write("docs/SUMMARY.md", &navigation);
        let before = book.snapshot();
        let report = anchor_book::checking::check(&book.config()).unwrap();
        assert_eq!(report.outcome(), expected, "{navigation}");
        if let Some(code) = code {
            let finding = report.findings().iter().find(|f| f.code == code).unwrap();
            assert!(finding.expected.contains("project-guidance"));
            assert!(
                finding
                    .related
                    .iter()
                    .any(|l| l.field.as_deref() == Some("book.sections.project-guidance"))
            );
        }
        let cli = run(&book, &["check", "--config", "anchorbook.toml"]);
        assert_eq!(cli.status.code(), Some(i32::from(expected.exit_code())));
        assert_eq!(book.snapshot(), before);
    }
    fs::remove_file(book.path("docs/team.md")).unwrap();
    let report = anchor_book::checking::check(&book.config()).unwrap();
    assert_eq!(report.outcome(), Outcome::Failed);
    assert!(
        report
            .findings()
            .iter()
            .any(|f| f.code == "structure.chapter-missing")
    );
}

#[test]
fn packaged_starter_works_without_a_project_and_never_replaces_authored_guidance() {
    let book = Book::new();
    book.write("anchorbook.toml", "broken configuration [");
    book.write("docs/team.md", GUIDANCE);
    let before = book.snapshot();
    for directory in [&book.root, &book.path("docs")] {
        let output = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
            .current_dir(directory)
            .args(["template", "project-guidance"])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert_eq!(
            output.stdout,
            anchor_book::templates::PROJECT_GUIDANCE.as_bytes()
        );
    }
    for args in [
        vec!["template"],
        vec!["template", "unknown"],
        vec!["template", "project-guidance", "docs/team.md"],
    ] {
        let output = run(&book, &args);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }
    assert_eq!(book.snapshot(), before);
}

/// anchorbook: A-check-navigation-conformance#required-roles
#[test]
fn mdbook_gates_missing_guidance_and_preserves_its_prose_when_restored() {
    let book = Book::new();
    mapped(&book, "team.md");
    let request = serde_json::json!([
        {"root":book.root,"renderer":"html","mdbook_version":"0.5.4",
         "config":{"preprocessor":{"anchorbook":{"config":"anchorbook.toml"}}}},
        {"items":[{"Chapter":{"name":"Team handbook","content":GUIDANCE,
          "number":null,"path":"team.md","source_path":"team.md","sub_items":[],"parent_names":[]}}]}
    ]).to_string();
    let invoke = || {
        let mut process = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
            .args(["md"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        process
            .stdin
            .take()
            .unwrap()
            .write_all(request.as_bytes())
            .unwrap();
        process.wait_with_output().unwrap()
    };
    let output = invoke();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("project-guidance"));
    book.write("docs/team.md", GUIDANCE);
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n[Team handbook](team.md)\n"),
    );
    let before = book.snapshot();
    let output = invoke();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rendered: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let content = rendered["items"][0]["Chapter"]["content"].as_str().unwrap();
    assert!(content.contains(GUIDANCE));
    assert!(!content.contains("ab-milestone"));
    assert_eq!(book.snapshot(), before);
}
