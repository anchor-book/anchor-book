//! Roadmap milestones are validated and rendered from the Markdown roadmap format.
use anchor_book::{
    Chapter, read_records,
    rendering::{RenderChapter, RenderError, render_chapters},
    roadmaps::read_roadmaps,
};
use serde_json::{Value, json};
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

#[allow(dead_code)]
#[path = "support/checking.rs"]
mod fixture;

const RECORDS: &str = r#"# Design

```anchorbook-requirement
schema-version = 1
id = "requirement-fixture"
title = "Fixture requirement"
status = "draft"
statement = "Fixture."
source = "Fixture."
```

```anchorbook-anchor
schema-version = 1
id = "fixture-anchor"
title = "Fixture anchor"
status = "draft"
property = "Fixture."
authorities = [{ kind = "requirement", id = "requirement-fixture" }]

[[criteria]]
id = "first"
verification = ["test"]
statement = "First."

[[criteria]]
id = "second"
verification = ["test"]
statement = "Second."
```
"#;

fn milestone(id: &str, criterion: &str, coverage: &str) -> String {
    format!(
        "## Milestone 1. Work happens {{#{id}}}\n\n**Outcome:** Something works.\n\n**Status:** Proposed\n\n**Identifier:** `{id}`\n\n**Tasks**\n\n- [ ] Do it.\n\n### Completion criteria\n\n#### It `works` {{#{id}-{criterion}}}\n\nThe condition.\n\n- **Anchor coverage:** {coverage}\n- **Evidence:** Pending.\n\n### Completion record\n\nPending.\n"
    )
}

const COVERAGE: &str = "[fixture-anchor](design.md#fixture-anchor) · `first`, `second` — Full; `fixture-anchor` · `first` — Partial: x";

fn codes(roadmaps: &[(&str, &str)]) -> Vec<(String, usize, String)> {
    let records = read_records(&[Chapter::new(Path::new("design.md"), RECORDS)]).unwrap();
    let chapters: Vec<_> = roadmaps
        .iter()
        .map(|(path, text)| Chapter::new(Path::new(path), text))
        .collect();
    match read_roadmaps(&chapters, &records) {
        Ok(_) => Vec::new(),
        Err(errors) => errors
            .diagnostics()
            .iter()
            .map(|d| (d.code().to_owned(), d.line(), d.message().to_owned()))
            .collect(),
    }
}

fn only(roadmap: &str) -> Vec<(String, usize)> {
    codes(&[("roadmap.md", roadmap)])
        .into_iter()
        .map(|(code, line, _)| (code, line))
        .collect()
}

#[test]
fn valid_milestones_resolve_their_identifiers_and_coverage() {
    let records = read_records(&[Chapter::new(Path::new("design.md"), RECORDS)]).unwrap();
    let text = milestone("road-1", "works", COVERAGE);
    let milestones =
        read_roadmaps(&[Chapter::new(Path::new("roadmap.md"), &text)], &records).unwrap();
    assert_eq!(milestones.len(), 1);
    assert_eq!(milestones[0].id, "road-1");
    let ids: Vec<_> = milestones[0]
        .criteria
        .iter()
        .map(|c| c.id.as_str())
        .collect();
    assert_eq!(ids, ["road-1-works"]);
    assert_eq!(milestones[0].criteria[0].title, "It `works`");
    let heading = &text[milestones[0].criteria[0].heading.clone()];
    assert_eq!(heading, "#### It `works` {#road-1-works}");
    let end = milestones[0].criteria[0].end;
    assert!(text[end..].starts_with("### Completion record"));
}

#[test]
fn malformed_headings_and_fields_are_located() {
    let good = milestone("road-1", "works", COVERAGE);
    // Heading without a fragment, at line 1.
    let no_fragment = good.replacen(" {#road-1}", "", 1);
    assert_eq!(
        only(&no_fragment),
        [("roadmap.invalid-milestone".into(), 1)]
    );
    // Missing Outcome is reported at the heading.
    let missing = good.replacen("**Outcome:** Something works.\n\n", "", 1);
    assert_eq!(only(&missing), [("roadmap.invalid-milestone".into(), 1)]);
    // Identifier that does not match the fragment, at its own line.
    let mismatch = good.replacen("**Identifier:** `road-1`", "**Identifier:** `road-2`", 1);
    assert_eq!(only(&mismatch), [("roadmap.invalid-milestone".into(), 7)]);
    // Unknown status.
    let status = good.replacen("**Status:** Proposed", "**Status:** Finished", 1);
    assert_eq!(only(&status), [("roadmap.invalid-milestone".into(), 5)]);
    // Criterion heading without a fragment, and with the wrong prefix.
    let criterion = good.replacen(" {#road-1-works}", "", 1);
    assert_eq!(only(&criterion), [("roadmap.invalid-milestone".into(), 15)]);
    let prefix = good.replacen("{#road-1-works}", "{#other-works}", 1);
    assert_eq!(only(&prefix), [("roadmap.invalid-milestone".into(), 15)]);
}

#[test]
fn statuses_may_carry_an_explanation() {
    let text = milestone("road-1", "works", COVERAGE).replacen(
        "**Status:** Proposed",
        "**Status:** In progress — tests pending",
        1,
    );
    assert!(only(&text).is_empty());
}

#[test]
fn identifiers_are_unique_across_chapters() {
    let first = milestone("road-1", "works", COVERAGE);
    // The duplicate is located in the chapter that repeats it, after its preamble.
    let second = format!("# Roadmap\n\nIntro.\n\n{first}");
    let duplicate = codes(&[("a.md", &first), ("b.md", &second)]);
    let found: Vec<_> = duplicate
        .iter()
        .map(|(c, l, m)| (c.as_str(), *l, m.contains("already defined in a.md")))
        .collect();
    assert_eq!(
        found,
        [
            ("roadmap.duplicate-id", 5, true),
            ("roadmap.duplicate-id", 19, true)
        ]
    );
}

#[test]
fn milestones_end_at_the_next_section_heading() {
    let records = read_records(&[Chapter::new(Path::new("design.md"), RECORDS)]).unwrap();
    let text = format!(
        "# Roadmap\n\n## Goal\n\nGoal text.\n\n{}\n{}\n## Later work\n\n#### Not a criterion\n",
        milestone("road-1", "works", COVERAGE),
        milestone("road-2", "holds", "None — preparation"),
    );
    let milestones =
        read_roadmaps(&[Chapter::new(Path::new("roadmap.md"), &text)], &records).unwrap();
    let ids: Vec<_> = milestones.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(ids, ["road-1", "road-2"]);
    for milestone in &milestones {
        assert_eq!(milestone.criteria.len(), 1);
        assert!(text[milestone.criteria[0].end..].starts_with("### Completion record"));
    }
    let second = text.find("{#road-2}\n").unwrap();
    assert!(milestones[1].criteria[0].heading.start > second);
    assert!(milestones[0].criteria[0].heading.start < second);
}

#[test]
fn coverage_must_cite_existing_anchors_and_criteria() {
    let missing_anchor = milestone("road-1", "works", "`no-such-anchor` · `first` — Full");
    let wrong_kind = milestone("road-1", "works", "`requirement-fixture` · `first` — Full");
    let missing_criterion = milestone("road-1", "works", "`fixture-anchor` · `third` — Full");
    for (text, message) in [
        (missing_anchor, "not a record"),
        (wrong_kind, "not an anchor"),
        (missing_criterion, "no criterion `third`"),
    ] {
        let found = codes(&[("roadmap.md", &text)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].0, "roadmap.invalid-reference");
        assert_eq!(found[0].1, 19);
        assert!(found[0].2.contains(message), "{}", found[0].2);
    }
    let none = milestone("road-1", "works", "None — design preparation");
    assert!(only(&none).is_empty());
}

#[test]
fn coverage_levels_are_read_and_full_takes_no_qualifier() {
    use anchor_book::roadmaps::CoverageLevel;
    let records = read_records(&[Chapter::new(Path::new("design.md"), RECORDS)]).unwrap();
    let text = milestone("road-1", "works", COVERAGE);
    let milestones =
        read_roadmaps(&[Chapter::new(Path::new("roadmap.md"), &text)], &records).unwrap();
    let coverage: Vec<_> = milestones[0]
        .coverage
        .iter()
        .map(|c| {
            (
                c.completion_criterion.as_str(),
                c.anchor.as_str(),
                c.criteria.clone(),
                c.level,
                c.line,
            )
        })
        .collect();
    assert_eq!(
        coverage,
        [
            (
                "road-1-works",
                "fixture-anchor",
                vec!["first".to_owned(), "second".to_owned()],
                CoverageLevel::Full,
                19
            ),
            (
                "road-1-works",
                "fixture-anchor",
                vec!["first".to_owned()],
                CoverageLevel::Partial,
                19
            ),
        ]
    );
    for qualified in [
        "`fixture-anchor` · `first` — Full for the declared versions",
        "`fixture-anchor` · `first` — Full. Limited to one case.",
    ] {
        let found = codes(&[("roadmap.md", &milestone("road-1", "works", qualified))]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(
            (found[0].0.as_str(), found[0].1),
            ("roadmap.invalid-coverage", 19)
        );
        assert!(
            found[0].2.contains("Full with a qualifier"),
            "{}",
            found[0].2
        );
    }
    let missing = codes(&[(
        "roadmap.md",
        &milestone("road-1", "works", "`fixture-anchor` · `first`"),
    )]);
    assert_eq!(missing[0].0, "roadmap.invalid-coverage");
}

#[test]
fn examples_in_code_blocks_are_not_roadmap_structure() {
    let example = format!(
        "# Format\n\n```markdown\n{}```\n",
        milestone("road-1", "works", "`no-such-anchor` · `first` — Full").replacen(
            "**Outcome:** Something works.\n\n",
            "",
            1
        )
    );
    assert!(only(&example).is_empty());
}

fn render(roadmap: bool, text: &str) -> Result<Vec<String>, RenderError> {
    let design = RenderChapter {
        source: Chapter::new(Path::new("design.md"), RECORDS),
        output_path: Path::new("design.md"),
        roadmap: false,
    };
    let work = RenderChapter {
        source: Chapter::new(Path::new("roadmap.md"), text),
        output_path: Path::new("roadmap.md"),
        roadmap,
    };
    render_chapters(&[design, work])
}

const CRITERION_OPEN: &str = "<details class=\"ab-details ab-roadmap-criterion\" open>\n<summary><h4 id=\"road-1-works\">It <code>works</code> <span class=\"ab-link-id\">road-1-works</span></h4></summary>\n";

#[test]
fn milestones_render_as_cards_with_fields_and_open_sections() {
    let text = milestone("road-1", "works", COVERAGE);
    let page = &render(true, &text).unwrap()[1];
    let expected = [
        "<section class=\"ab-milestone\">\n\n",
        "## Milestone 1. Work happens {#road-1}",
        "\n\n<div class=\"ab-milestone-meta\"><span class=\"ab-status\">Proposed</span><span class=\"ab-link-id\">road-1</span></div>",
        "\n\n<div class=\"ab-milestone-field\"><div class=\"ab-field-label\">Outcome</div><div class=\"ab-field-value\">\n\nSomething works.\n\n</div></div>",
        // A bare status repeats the badge, and the identifier repeats the heading.
        "\n\n\n\n\n\n",
        "<details class=\"ab-details ab-milestone-tasks\" open>\n<summary>Tasks <span class=\"ab-task-progress\">0 of 1 done</span></summary>\n",
        "\n\n- [ ] Do it.\n</details>\n\n",
        "\n\n### Completion criteria\n\n",
        CRITERION_OPEN,
        "\n\nThe condition.\n\n- **Anchor coverage:** ",
        COVERAGE,
        "\n- **Evidence:** Pending.\n\n\n</details>\n\n",
        "### Completion record\n\nPending.\n",
        "\n</section>\n\n",
    ]
    .concat();
    let at = page
        .rfind("\n\n<style>\n")
        .expect("a rendered milestone ends with its presentation block");
    let (body, presentation) = page.split_at(at);
    assert_eq!(body, expected);
    assert!(presentation.contains(".ab-milestone {"));
    assert!(presentation.ends_with("</script>\n"));
}

#[test]
fn explained_statuses_tasks_and_evidence_render_in_order() {
    let text = milestone("road-1", "works", COVERAGE)
        .replacen(
            "**Status:** Proposed",
            "**Status:** In progress — tests pending",
            1,
        )
        .replacen(
            "- [ ] Do it.",
            "- [x] Plan it.\n- [x] Build it.\n- [ ] Ship it.",
            1,
        )
        .replacen(
            "### Completion record",
            "### Evidence {#road-1-evidence}\n\nRan the tests.\n\n### Completion record",
            1,
        );
    let second = milestone("road-2", "holds", "None — preparation");
    let chapter = format!("{text}\n{second}");
    let page = &render(true, &chapter).unwrap()[1];
    assert!(page.contains("<span class=\"ab-status\">In progress</span>"));
    assert!(page.contains("<div class=\"ab-field-label\">Status</div><div class=\"ab-field-value\">\n\nIn progress — tests pending\n\n</div></div>"));
    assert!(page.contains("<span class=\"ab-task-progress\">2 of 3 done</span>"));
    assert!(page.contains("- [ ] Ship it.\n</details>\n\n"));
    assert!(page.contains(
        "\n</details>\n\n<details class=\"ab-details ab-milestone-evidence\" open>\n<summary><h3 id=\"road-1-evidence\">Evidence</h3></summary>\n\n\nRan the tests.\n\n\n</details>\n\n### Completion record"
    ));
    // The first card closes before the second opens.
    assert!(page.contains("Pending.\n\n\n</section>\n\n<section class=\"ab-milestone\">\n\n## Milestone 1. Work happens {#road-2}"));
    assert_eq!(page.matches("<section").count(), 2);
    assert_eq!(page.matches("</section>").count(), 2);
    assert_eq!(
        page.matches("<details").count(),
        page.matches("</details>").count()
    );
}

#[test]
fn chapters_outside_the_roadmaps_role_are_not_read_as_roadmaps() {
    let invalid = milestone("road-1", "works", "`no-such-anchor` · `first` — Full");
    assert_eq!(render(false, &invalid).unwrap()[1], invalid);
    let error = render(true, &invalid).unwrap_err();
    assert!(matches!(error, RenderError::Roadmaps(_)));
    let cause = std::error::Error::source(&error).expect("roadmap errors are the cause");
    assert!(cause.to_string().starts_with("roadmap.invalid-reference"));
    assert!(
        error
            .to_string()
            .starts_with("roadmap.invalid-reference: roadmap.md:19:")
    );
}

fn adapter(book: &fixture::Book, roadmap: &str, nested: &str) -> std::process::Output {
    let chapter = |name: &str, path: &str, content: &str, sub: Value| {
        json!({"Chapter": {"name": name, "content": content, "number": null,
            "path": path, "source_path": path, "sub_items": sub, "parent_names": []}})
    };
    let request = json!([
        {"root": book.root, "config": {"preprocessor": {"anchorbook": {"config": "anchorbook.toml"}}},
         "renderer": "html", "mdbook_version": "0.5.4"},
        {"items": [
            chapter("Design", "design.md", RECORDS, json!([])),
            chapter("Authority", "authority.md", "# Authority\n", json!([])),
            chapter("Work", "roadmap.md", roadmap,
                json!([chapter("Nested", "nested.md", nested, json!([]))])),
        ]}
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
fn the_adapter_renders_the_roadmaps_role_and_its_nested_chapters() {
    let book = fixture::Book::new();
    let before = book.snapshot();
    let nested = milestone("nested-1", "works", COVERAGE);
    let output = adapter(&book, &milestone("road-1", "works", COVERAGE), &nested);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rendered: Value = serde_json::from_slice(&output.stdout).unwrap();
    let items = &rendered["items"];
    let work = items[2]["Chapter"]["content"].as_str().unwrap();
    let inner = items[2]["Chapter"]["sub_items"][0]["Chapter"]["content"]
        .as_str()
        .unwrap();
    assert!(work.contains("<h4 id=\"road-1-works\">"));
    assert!(inner.contains("<h4 id=\"nested-1-works\">"));
    let design = items[0]["Chapter"]["content"].as_str().unwrap();
    assert!(!design.contains("class=\"ab-details ab-roadmap-criterion\""));
    assert_eq!(book.snapshot(), before);
}

#[test]
fn the_adapter_refuses_invalid_roadmaps_without_output() {
    let book = fixture::Book::new();
    let invalid = milestone("road-1", "works", "`fixture-anchor` · `third` — Full");
    let output = adapter(&book, &invalid, "# Nested\n");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("roadmap.invalid-reference: roadmap.md:19: anchor `fixture-anchor` has no criterion `third`"),
        "{stderr}"
    );
}
