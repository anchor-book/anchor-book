//! The mdBook preprocessor refuses to render a Book that fails structural checking.
use serde_json::{Value, json};
use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

#[allow(dead_code)]
#[path = "support/checking.rs"]
mod fixture;

fn preprocess(book: &fixture::Book, config: &str) -> Output {
    let request = json!([
        {"root": book.root, "config": {"preprocessor": {"anchorbook": {"config": config}}},
         "renderer": "html", "mdbook_version": "0.5.4"},
        {"items": [{"Chapter": {"name": "Design", "content": "# Design\n", "number": null,
            "path": "design.md", "source_path": "design.md", "sub_items": [],
            "parent_names": []}}]}
    ]);
    let mut process = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .arg("md")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(request.to_string().as_bytes())
        .unwrap();
    process.wait_with_output().unwrap()
}

#[test]
fn valid_book_reaches_rendering() {
    let book = fixture::Book::new();
    let before = book.snapshot();
    let output = preprocess(&book, "anchorbook.toml");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let rendered: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        rendered["items"][0]["Chapter"]["content"].as_str(),
        Some("# Design\n")
    );
    assert_eq!(book.snapshot(), before);
}

struct Case {
    label: &'static str,
    config: &'static str,
    file: Option<(&'static str, String)>,
    status: i32,
}

#[test]
fn failed_or_incomplete_checks_prevent_rendering() {
    let summary = fixture::SUMMARY;
    let cases = [
        Case {
            label: "broken navigation",
            config: "anchorbook.toml",
            file: Some((
                "docs/SUMMARY.md",
                format!("{summary}- [Missing](missing.md)\n"),
            )),
            status: 1,
        },
        Case {
            label: "unsupported navigation",
            config: "anchorbook.toml",
            file: Some((
                "docs/SUMMARY.md",
                format!("{summary}\nUnsupported prose outside navigation.\n"),
            )),
            status: 2,
        },
        Case {
            label: "invalid configuration",
            config: "anchorbook.toml",
            file: Some(("anchorbook.toml", "schema-version = 9\n".into())),
            status: 2,
        },
        Case {
            label: "missing configuration",
            config: "absent.toml",
            file: None,
            status: 2,
        },
    ];
    for case in cases {
        let book = fixture::Book::new();
        if let Some((path, text)) = &case.file {
            book.write(path, text);
        }
        let before = book.snapshot();
        let output = preprocess(&book, case.config);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let label = case.label;
        assert_eq!(output.status.code(), Some(case.status), "{label}: {stderr}");
        assert!(output.stdout.is_empty(), "{label}: rendered output emitted");
        assert!(stderr.contains("anchorbook check: "), "{label}: {stderr}");
        assert_eq!(book.snapshot(), before, "{label}: files changed");
    }
}
