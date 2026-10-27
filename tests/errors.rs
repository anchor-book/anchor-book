//! Public error presentation and cause chains, independent of the current Book.
use anchor_book::adapters::mdbook::preprocess;
use anchor_book::rendering::{RenderChapter, RenderError, render_chapters};
use anchor_book::{Chapter, Diagnostic, RecordErrors, read_records};
use serde_json::json;
use std::{error::Error, path::Path};

#[test]
fn record_errors_expose_a_summary_and_the_first_diagnostic() {
    let errors = read_records(&[Chapter::new(Path::new("a.md"), "{anchor}`old`")]).unwrap_err();
    assert_eq!(
        errors.to_string(),
        "record validation failed with 1 diagnostic(s)"
    );
    let cause = errors
        .source()
        .unwrap()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert!(std::ptr::eq(cause, &errors.diagnostics()[0]));
    assert!(cause.source().is_none());
}

#[test]
fn rendered_diagnostics_are_separate_lines_without_extra_newlines() {
    let errors = render_chapters(&[RenderChapter {
        source: Chapter::new(Path::new("a.md"), "{anchor}`one`\n\n{anchor}`two`"),
        output_path: Path::new("a.md"),
    }])
    .unwrap_err();
    let rendered = errors.to_string();
    let lines: Vec<_> = rendered.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].starts_with("a.md:1:1: record.invalid-definition: legacy {anchor}"));
    assert!(lines[1].starts_with("a.md:3:1: record.invalid-definition: legacy {anchor}"));
    assert!(!rendered.ends_with('\n'));
    assert_eq!(
        errors
            .source()
            .unwrap()
            .downcast_ref::<RecordErrors>()
            .unwrap()
            .diagnostics()
            .len(),
        2
    );
}

#[test]
fn adapter_preserves_json_and_record_parser_causes() {
    let error = preprocess("[").unwrap_err();
    let json = error
        .source()
        .unwrap()
        .downcast_ref::<serde_json::Error>()
        .unwrap();
    assert!(json.is_eof());

    let request = json!([
        {"renderer":"html", "mdbook_version":"0.5.4"},
        {"items":[{"Chapter":{
            "source_path":"a.md", "path":"a.md", "sub_items":[],
            "content":"```anchorbook-requirement\nid = @\n```\n"
        }}]}
    ]);
    let error = preprocess(&request.to_string()).unwrap_err();
    let render = error
        .source()
        .unwrap()
        .downcast_ref::<RenderError>()
        .unwrap();
    let records = render
        .source()
        .unwrap()
        .downcast_ref::<RecordErrors>()
        .unwrap();
    let diagnostic = records
        .source()
        .unwrap()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert_eq!(diagnostic.code().as_str(), "record.invalid-definition");
    assert!(
        diagnostic.source().is_some(),
        "preserve underlying TOML parser cause"
    );

    let protocol = preprocess("[{},{}]").unwrap_err();
    assert!(
        protocol.source().is_none(),
        "protocol rejection has no parser cause"
    );
    let route = render_chapters(&[RenderChapter {
        source: Chapter::new(Path::new("a.md"), "ordinary prose"),
        output_path: Path::new("../outside.md"),
    }])
    .unwrap_err();
    assert!(
        route.source().is_none(),
        "invalid route has no record parser cause"
    );
}
