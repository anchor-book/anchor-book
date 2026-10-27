//! Presentation and actual-process mdBook protocol checks using independent input.
use anchor_book::{
    adapters::mdbook::preprocess,
    records::Chapter,
    rendering::{RenderChapter, render_chapters},
};
use serde_json::{Value, json};
use std::{
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};
const RECORDS: &str = include_str!("fixtures/records.md");
fn chapter(source: &str, path: &str, content: &str) -> Value {
    json!({"Chapter":{"name":"Fixture", "content":content, "number":null,
        "path":path,"source_path":source,"sub_items":[],"parent_names":[],"extra":"preserved"}})
}
// A structurally valid Book, so the process boundary reaches rendering.
fn request(items: Vec<Value>) -> Value {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/valid-book");
    json!([{"root":root,"config":{"preprocessor":{"anchorbook":{"config":"anchorbook.toml"}}},
        "renderer":"html","mdbook_version":"0.5.4"}, {"items":items,"custom":{"preserve":true}}])
}
fn run(args: &[&str], input: &str) -> Output {
    let mut process = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    process.wait_with_output().unwrap()
}
#[test]
fn renders_all_fields_and_preserves_unrelated_content() {
    let prefix = "# Chapter\n\nProse remains.\n\n```mermaid\ngraph LR\nA-->B\n```\n\n";
    let literal = format!("\n````text\n{RECORDS}````\n");
    let source = format!("{prefix}{RECORDS}{literal}");
    let output = render_chapters(&[RenderChapter {
        source: Chapter::new(Path::new("a.md"), &source),
        output_path: Path::new("a.md"),
    }])
    .unwrap()
    .remove(0);
    assert!(output.starts_with(prefix));
    assert!(output.ends_with(&literal));
    assert_eq!(output.matches("<section class=\"ab-record ").count(), 3);
    for field in [
        "Preserve input",
        "Reading leaves the supplied source unchanged.",
        "The reader is observational.",
        "Use an in-memory reader",
        "Accept supplied chapters.",
        "Separate acquisition from validation.",
        "Read paths inside the validator.",
        "A new adapter needs to acquire input differently.",
        "Inputs do not change",
        "Parsing observes the input without changing it.",
        "source-comparison",
        "Compare input before and after validation.",
        "Accepted",
        "Referenced by",
        "<details",
        "<summary>",
    ] {
        assert!(output.contains(field), "missing {field}");
    }
    assert!(output.contains("href=\"#preserve-input\""));
    assert!(output.contains("href=\"#immutable-input\""));
    for (identity, heading) in [
        ("immutable-input", "Authority"),
        ("pure-reader", "Requirements"),
    ] {
        let section = output
            .split(&format!("id=\"{identity}\""))
            .nth(1)
            .unwrap()
            .split("</section>")
            .next()
            .unwrap();
        assert!(section.contains(&format!("<h4>{heading}</h4>")));
    }
}
#[test]
fn escaped_fields_are_inert_and_empty_requirements_are_explicit() {
    let text = RECORDS
        .replace(
            "Preserve input",
            "<script>alert('x')</script> & <img onerror='x'>",
        )
        .replace("requirements = [\"preserve-input\"]", "requirements = []")
        .replace("status = \"accepted\"", "status = \"draft\"")
        .replace(
            "source = \"The reader is observational.\"",
            "source = '''line one\n\n**not Markdown**\n<script>bad()</script>'''",
        );
    let response = preprocess(&request(vec![chapter("a.md", "a.md", &text)]).to_string()).unwrap();
    let book: Value = serde_json::from_str(&response).unwrap();
    let html = book["items"][0]["Chapter"]["content"].as_str().unwrap();
    assert!(!html.contains("<script>"));
    assert!(!html.contains("<img"));
    assert!(html.contains("&lt;script&gt;"));
    assert!(html.contains("&amp;"));
    assert!(html.contains("&#10;&#10;**not Markdown**"));
    assert!(html.contains("No recorded requirements."));
    assert!(html.contains(">Draft<"));
}
#[test]
fn nested_chapters_routes_and_metadata_survive_process_boundary() {
    let split = RECORDS.find("```anchorbook-decision").unwrap();
    let mut parent = chapter("original.md", "authority/é & rules.md", &RECORDS[..split]);
    parent["Chapter"]["sub_items"] = json!([chapter(
        "child-source.md",
        "design/nested/anchor.md",
        &RECORDS[split..]
    )]);
    let input = request(vec![
        json!({"PartTitle":"Part"}),
        json!("Separator"),
        parent,
    ]);
    let original = input.to_string();
    let result = run(&["md"], &original);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(result.stderr.is_empty());
    let mut actual: Value = serde_json::from_slice(&result.stdout).unwrap();
    let first = actual["items"][2]["Chapter"]["content"].as_str().unwrap();
    assert!(first.contains("href=\"../design/nested/anchor.html#immutable-input\""));
    let second = actual["items"][2]["Chapter"]["sub_items"][0]["Chapter"]["content"]
        .as_str()
        .unwrap();
    assert!(second.contains("href=\"../../authority/%C3%A9%20%26%20rules.html#preserve-input\""));
    actual["items"][2]["Chapter"]["content"] = input[1]["items"][2]["Chapter"]["content"].clone();
    actual["items"][2]["Chapter"]["sub_items"][0]["Chapter"]["content"] =
        input[1]["items"][2]["Chapter"]["sub_items"][0]["Chapter"]["content"].clone();
    assert_eq!(actual, input[1]);
    assert_eq!(input.to_string(), original);
}
#[test]
fn protocol_failure_has_no_partial_output() {
    let valid = request(vec![chapter("a.md", "a.md", RECORDS)]);
    let mut cases = vec!["invalid json".into(), "[]".into(), "[{},{}]".into()];
    for pointer in [
        "/0/renderer",
        "/0/mdbook_version",
        "/0/root",
        "/0/config/preprocessor/anchorbook/config",
        "/1/items",
        "/1/items/0/Chapter/content",
        "/1/items/0/Chapter/path",
        "/1/items/0/Chapter/source_path",
        "/1/items/0/Chapter/sub_items",
    ] {
        let mut input = valid.clone();
        *input.pointer_mut(pointer).unwrap() = Value::Null;
        cases.push(input.to_string());
    }
    for version in ["0.4.52", "0.5.2"] {
        let mut wrong_version = valid.clone();
        wrong_version[0]["mdbook_version"] = json!(version);
        cases.push(wrong_version.to_string());
    }
    for route in [
        "../outside.md",
        "/absolute.md",
        "https://site/path.md",
        "scheme:chapter.md",
        "a\\b.md",
        "a\nb.md",
        "a\rb.md",
        "a\tb.md",
        "",
        "a/./b.md",
    ] {
        let mut input = valid.clone();
        input[1]["items"][0]["Chapter"]["path"] = json!(route);
        cases.push(input.to_string());
    }
    cases.push(
        request(vec![
            chapter("a.md", "a.md", RECORDS),
            chapter("b.md", "a.html", "plain"),
        ])
        .to_string(),
    );
    cases.push(
        request(vec![
            chapter("a.md", "a.md", RECORDS),
            chapter("a.md", "b.md", "plain"),
        ])
        .to_string(),
    );
    cases.push(
        request(vec![chapter(
            "a.md",
            "a.md",
            &RECORDS.replace("schema-version = 1", "schema-version = 9"),
        )])
        .to_string(),
    );
    cases.push(
        request(vec![chapter(
            "a.md",
            "a.md",
            &RECORDS.replace(
                "requirements = [\"preserve-input\"]",
                "requirements = [\"missing\"]",
            ),
        )])
        .to_string(),
    );
    cases.push(request(vec![json!({"Unexpected":{}})]).to_string());
    for input in cases {
        let result = run(&["md"], &input);
        assert!(!result.status.success(), "accepted {input}");
        assert!(result.stdout.is_empty());
        assert!(!result.stderr.is_empty());
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(
            stderr.contains("render.invalid-input") || stderr.contains("record."),
            "{stderr}"
        );
    }
}
#[test]
fn capability_probe_and_arguments_have_defined_statuses() {
    for (args, code) in [
        (vec!["md", "supports", "html"], 0),
        (vec!["md", "supports", "pdf"], 1),
        (vec!["md", "supports"], 2),
        (vec!["md", "extra"], 2),
        (vec!["md", "supports", "html", "extra"], 2),
    ] {
        let output = run(&args, "");
        assert_eq!(output.status.code(), Some(code));
        assert!(output.stdout.is_empty());
    }
    let empty = run(&["md"], &request(vec![]).to_string());
    assert!(empty.status.success());
}

#[test]
fn renders_required_methods_per_criterion_without_claiming_results() {
    let text = RECORDS.replace(
        "verification = [\"test\"]",
        "verification = [\"review\", \"proof\", \"test\"]",
    ).replace(
        "statement = \"Compare input before and after validation.\"",
        "statement = \"Compare input before and after validation.\"\n\n[[criteria]]\nid = \"second\"\nstatement = \"Review the assumptions.\"\nverification = [\"review\"]",
    );
    let output = run(
        &["md"],
        &request(vec![chapter("a.md", "a.md", &text)]).to_string(),
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let book: Value = serde_json::from_slice(&output.stdout).unwrap();
    let html = book["items"][0]["Chapter"]["content"].as_str().unwrap();
    assert!(html.contains("<li><code>source-comparison</code><div class=\"ab-text\">Compare input before and after validation.</div><p>Required verification: review, proof, test</p></li>"));
    assert!(html.contains("<li><code>second</code><div class=\"ab-text\">Review the assumptions.</div><p>Required verification: review</p></li>"));
    assert!(!html.contains("Verified"));
    assert!(!html.contains("Passed"));
    assert!(html.contains(">Accepted<"));
}

#[test]
fn invalid_verification_fails_preprocessing_without_success_json() {
    for replacement in [
        "",
        "verification = []",
        "verification = [\"test\", \"test\"]",
        "verification = [\"verus\"]",
    ] {
        let text = RECORDS.replace("verification = [\"test\"]", replacement);
        let output = run(
            &["md"],
            &request(vec![chapter("a.md", "a.md", &text)]).to_string(),
        );
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        let diagnostics = String::from_utf8_lossy(&output.stderr);
        assert!(
            diagnostics.contains("record.invalid-definition"),
            "{diagnostics}"
        );
        assert!(diagnostics.contains("a.md"), "{diagnostics}");
    }
}

#[test]
fn uncited_decision_authority_failure_reaches_the_real_adapter() {
    let source = RECORDS
        .split("```anchorbook-anchor")
        .next()
        .unwrap()
        .replace(
            "requirements = [\"preserve-input\"]",
            "requirements = [\"missing\"]",
        );
    let result = run(
        &["md"],
        &request(vec![chapter("a.md", "a.md", &source)]).to_string(),
    );
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    let diagnostics = String::from_utf8_lossy(&result.stderr);
    assert!(diagnostics.contains("record.invalid-reference"));
    assert!(diagnostics.contains("missing: authority definition does not exist"));
}
