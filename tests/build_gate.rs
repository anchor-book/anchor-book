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

// anchorbook: check-shared-typed-results#equivalent-callers
#[test]
fn library_cli_and_preprocessor_report_the_same_check() {
    use anchor_book::{
        adapters::check::format_report,
        checking::{Outcome, check},
    };
    let summary = fixture::SUMMARY;
    for (label, navigation, expected) in [
        ("passing", summary.to_owned(), Outcome::Passed),
        (
            "failing",
            format!("{summary}- [Missing](missing.md)\n"),
            Outcome::Failed,
        ),
        (
            "incomplete",
            format!("{summary}\nUnsupported prose outside navigation.\n"),
            Outcome::Incomplete,
        ),
    ] {
        let book = fixture::Book::new();
        book.write("docs/SUMMARY.md", &navigation);
        let report = check(&book.config()).unwrap_or_else(|error| error.into_report());
        assert_eq!(report.outcome(), expected, "{label}");
        let text = format_report(&report);
        let cli = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
            .args(["check", "--config"])
            .arg(book.config())
            .output()
            .unwrap();
        let preprocessor = preprocess(&book, "anchorbook.toml");
        let status = i32::from(expected.exit_code());
        assert_eq!(cli.status.code(), Some(status), "{label}: CLI status");
        assert_eq!(
            preprocessor.status.code(),
            Some(status),
            "{label}: preprocessor status"
        );
        let cli_report = if expected == Outcome::Incomplete {
            &cli.stderr
        } else {
            &cli.stdout
        };
        assert_eq!(
            String::from_utf8_lossy(cli_report),
            text,
            "{label}: CLI report"
        );
        if expected == Outcome::Passed {
            // A passing check lets the preprocessor render; it reports nothing.
            assert!(preprocessor.stderr.is_empty(), "{label}");
            assert!(
                serde_json::from_slice::<Value>(&preprocessor.stdout).is_ok(),
                "{label}"
            );
        } else {
            // A failing or incomplete check stops rendering with the same report.
            assert!(preprocessor.stdout.is_empty(), "{label}");
            assert_eq!(
                String::from_utf8_lossy(&preprocessor.stderr),
                text,
                "{label}: preprocessor report"
            );
        }
    }
}
