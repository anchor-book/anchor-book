//! Independent record-schema and relationship fixtures.
use std::{error::Error, path::Path};

use anchor_book::records::{
    Chapter, DiagnosticCode, Problem, RecordData, RecordErrors, RecordKind, Status,
    VerificationMethod, read_records,
};

const FIXTURE: &str = include_str!("fixtures/records.md");

fn requirement() -> String {
    FIXTURE
        .split("```anchorbook-decision")
        .next()
        .unwrap()
        .to_owned()
}
fn prefix_lines(text: &str, prefix: &str) -> String {
    let mut result = String::new();
    for line in text.lines() {
        result.push_str(prefix);
        result.push_str(line);
        result.push('\n');
    }
    result
}
fn inspect(text: &str) -> Result<anchor_book::records::RecordInventory, RecordErrors> {
    read_records(&[Chapter::new(Path::new("fixture.md"), text)])
}
fn failures(text: &str) -> RecordErrors {
    inspect(text).unwrap_err()
}
fn has_problem(text: &str, problem: Problem) {
    let errors = failures(text);
    assert!(
        errors.diagnostics().iter().any(|d| d.problem() == problem),
        "{text}\n{errors:?}"
    );
    let expected = match problem {
        Problem::InvalidInput => DiagnosticCode::InvalidInput,
        Problem::InvalidDefinition => DiagnosticCode::InvalidDefinition,
        Problem::DuplicateId => DiagnosticCode::DuplicateId,
        Problem::IncompatibleStatus => DiagnosticCode::IncompatibleStatus,
        _ => DiagnosticCode::InvalidReference,
    };
    for diagnostic in errors
        .diagnostics()
        .iter()
        .filter(|d| d.problem() == problem)
    {
        assert_eq!(diagnostic.code(), expected);
    }
}
fn snapshot(errors: &RecordErrors) -> Vec<String> {
    errors
        .diagnostics()
        .iter()
        .map(|d| format!("{} {:?} {:?}", d, d.problem(), d.related_locations()))
        .collect()
}

#[test]
fn reads_typed_records_preserves_ranges_and_derives_direct_backlinks() {
    let original = FIXTURE.to_owned();
    let inventory = inspect(&original).unwrap();
    assert_eq!(inventory.len(), 3);
    assert!(!inventory.is_empty());
    assert_eq!(
        inventory.records().map(|r| r.id()).collect::<Vec<_>>(),
        ["immutable-input", "preserve-input", "pure-reader"]
    );
    let anchor = inventory.get("immutable-input").unwrap();
    assert_eq!(anchor.kind(), RecordKind::Anchor);
    assert_eq!(anchor.status(), Status::Accepted);
    assert_eq!(anchor.title(), "Inputs do not change");
    let RecordData::Anchor(data) = anchor.data() else {
        panic!()
    };
    assert_eq!(
        data.property,
        "Parsing observes the input without changing it."
    );
    assert_eq!(data.criteria[0].id, "source-comparison");
    assert_eq!(
        data.criteria[0].statement,
        "Compare input before and after validation."
    );
    assert_eq!(data.authorities[0].id, "preserve-input");
    assert_eq!(
        inventory.backlinks("preserve-input"),
        ["immutable-input", "pure-reader"]
    );
    assert_eq!(inventory.backlinks("pure-reader"), ["immutable-input"]);
    assert!(inventory.backlinks("immutable-input").is_empty());
    assert!(inventory.get("not-present").is_none());
    let block = &original[anchor.location().range()];
    assert!(block.starts_with("```anchorbook-anchor\n"));
    assert!(block.trim_end_matches('\n').ends_with("\n```"));
    let body = &original[anchor.body_location().range()];
    assert!(body.starts_with("schema-version = 1\n"));
    assert!(!body.contains("```"));
    for reference in anchor.references() {
        assert_eq!(
            &original[reference.location.range()],
            format!("\"{}\"", reference.id)
        );
    }
    let RecordData::Requirement(req) = inventory.get("preserve-input").unwrap().data() else {
        panic!()
    };
    assert_eq!(req.source, "The reader is observational.");
    let RecordData::Decision(decision) = inventory.get("pure-reader").unwrap().data() else {
        panic!()
    };
    assert_eq!(decision.choice, "Accept supplied chapters.");
    assert_eq!(decision.rationale, "Separate acquisition from validation.");
    assert_eq!(decision.alternatives, ["Read paths inside the validator."]);
    assert_eq!(
        decision.reconsider_when,
        "A new adapter needs to acquire input differently."
    );
    assert_eq!(original, FIXTURE);
}

#[test]
fn parses_all_supported_line_endings_and_fence_at_eof() {
    for text in [
        FIXTURE.to_owned(),
        FIXTURE.replace('\n', "\r\n"),
        FIXTURE.trim_end_matches('\n').to_owned(),
    ] {
        assert_eq!(inspect(&text).unwrap().len(), 3);
    }
}

#[test]
fn verification_methods_are_explicit_typed_obligations_in_authored_order() {
    use VerificationMethod::{Proof, Review, Test};
    for (authored, expected) in [
        (r#"["proof"]"#, vec![Proof]),
        (r#"["test"]"#, vec![Test]),
        (r#"["review"]"#, vec![Review]),
        (r#"["review", "proof", "test"]"#, vec![Review, Proof, Test]),
    ] {
        let text = FIXTURE.replace(
            r#"verification = ["test"]"#,
            &format!("verification = {authored}"),
        );
        let inventory = inspect(&text).unwrap();
        let RecordData::Anchor(anchor) = inventory.get("immutable-input").unwrap().data() else {
            panic!()
        };
        assert_eq!(anchor.criteria[0].verification, expected);
        // Selecting a proof method does not require proof artifacts or change design status.
        assert_eq!(
            inventory.get("immutable-input").unwrap().status(),
            Status::Accepted
        );
    }
}

#[test]
fn rejects_invalid_verification_on_each_criterion_with_located_diagnostics() {
    let original = "verification = [\"test\"]";
    for replacement in [
        "",
        "verification = []",
        "verification = [\"proof\", \"proof\"]",
        "verification = [\"test\", \"test\"]",
        "verification = [\"review\", \"proof\", \"review\"]",
        "verification = [\"verus\"]",
        "verification = [\"Proof\"]",
        "verification = [\"\"]",
        "verification = [\" test\"]",
        "verification = [\"test\", false]",
        "verification = \"test\"",
        "verification = true",
        "verification = { test = true }",
        "methods = [\"test\"]",
        "verification = [\"test\"]\nverification = [\"review\"]",
    ] {
        for second in [false, true] {
            let text = if second {
                FIXTURE.replace(
                    "statement = \"Compare input before and after validation.\"",
                    &format!("statement = \"Compare input before and after validation.\"\n\n[[criteria]]\nid = \"second\"\nstatement = \"Another obligation.\"\n{replacement}"),
                )
            } else {
                FIXTURE.replace(original, replacement)
            };
            let errors = failures(&text);
            assert_eq!(errors.diagnostics().len(), 1, "{text}\n{errors:?}");
            let diagnostic = &errors.diagnostics()[0];
            assert_eq!(diagnostic.code(), DiagnosticCode::InvalidDefinition);
            assert_eq!(diagnostic.problem(), Problem::InvalidDefinition);
            assert_eq!(diagnostic.location().path(), Path::new("fixture.md"));
            assert!(diagnostic.location().range().start >= text.find("[[criteria]]").unwrap());
            assert!(!diagnostic.message().is_empty());
        }
    }
}

#[test]
fn only_real_definitions_enter_inventory() {
    let examples = format!(
        "# Examples\n\n````text\n{FIXTURE}\n````\n\n```rust\n{{anchor}}`example`\n```\n\n`{{anchor}}` is a literal.\n\n    ordinary code\n"
    );
    assert!(inspect(&examples).unwrap().is_empty());
    assert!(inspect("# prose\n\nNo records here.").unwrap().is_empty());
    assert!(read_records(&[]).unwrap().is_empty());
}

#[test]
fn rejects_noncanonical_fences_and_nesting() {
    let base = requirement();
    let mutations = [
        base.replace("```anchorbook-requirement", "````anchorbook-requirement"),
        base.replace("```", "~~~"),
        base.replace("```anchorbook-requirement", "``` anchorbook-requirement"),
        base.replace(
            "```anchorbook-requirement",
            "```anchorbook-requirement extra",
        ),
        base.replace("```anchorbook-requirement", "```anchorbook-requirement "),
        base.replace("\n```\n", "\n````\n"),
        base.replace("\n```\n", "\n``` \n"),
        base.replace("\n```\n", "\n"),
        prefix_lines(&base, "> "),
        prefix_lines(&base, "  "),
        prefix_lines(&base, "    "),
        format!("- item\n\n{}", prefix_lines(&base, "  ")),
        base.replace("anchorbook-requirement", "anchorbook-unknown"),
        "{anchor}`old-name`".to_owned(),
    ];
    for text in mutations {
        has_problem(&text, Problem::InvalidDefinition);
    }
}

#[test]
fn rejects_missing_unknown_duplicate_wrong_type_and_empty_fields() {
    let base = requirement();
    let mutations = [
        base.replace("schema-version = 1\n", ""),
        base.replace("schema-version = 1", "schema-version = 2"),
        base.replace("schema-version = 1", "schema-version = \"1\""),
        base.replace("schema-version = 1", "schema-version = true"),
        base.replace("schema-version = 1", "schema-version = 1\nunknown = 2"),
        base.replace("title = \"Preserve input\"", "title = \"\""),
        base.replace("title = \"Preserve input\"", "title = \"  \""),
        base.replace("title = \"Preserve input\"", "title = 10"),
        base.replace("status = \"accepted\"", "status = \"complete\""),
        base.replace("id = \"preserve-input\"", "id = \"Uppercase\""),
        base.replace("id = \"preserve-input\"", "id = \"bad--id\""),
        base.replace("id = \"preserve-input\"", "id = \"2-start\""),
        base.replace("id = \"preserve-input\"", "id = \"trailing-\""),
        base.replace("id = \"preserve-input\"", "id = \"nön-ascii\""),
        base.replace("id = \"preserve-input\"", "id = \"x\"\nid = \"y\""),
        base.replace("source = \"The reader is observational.\"", ""),
        base.replace("source = \"The reader is observational.\"", "source = \"\""),
        base.replace(
            "statement = \"Reading leaves the supplied source unchanged.\"",
            "statement = \"\t\"",
        ),
    ];
    for text in mutations {
        has_problem(&text, Problem::InvalidDefinition);
    }
}

#[test]
fn rejects_anchor_and_decision_schema_errors() {
    let mutations = [
        FIXTURE.replace("kind = \"decision\"", "kind = \"anchor\""),
        FIXTURE.replace("id = \"pure-reader\" },", "id = \"pure-reader\", extra = true },"),
        FIXTURE.replace("[[criteria]]", "[[criteria]]\nextra = true"),
        FIXTURE.replace("statement = \"Compare input before and after validation.\"", "statement = \"\""),
        FIXTURE.replace("alternatives = [\"Read paths inside the validator.\"]", "alternatives = []"),
        FIXTURE.replace("alternatives = [\"Read paths inside the validator.\"]", "alternatives = [\" \" ]"),
        FIXTURE.replace("rationale = \"Separate acquisition from validation.\"", "rationale = \"\""),
        FIXTURE.replace("choice = \"Accept supplied chapters.\"", "choice = \"\""),
        FIXTURE.replace("reconsider-when = \"A new adapter needs to acquire input differently.\"", "reconsider-when = \"\""),
        FIXTURE.replace("requirements = [\"preserve-input\"]", ""),
        FIXTURE.replace("[[criteria]]\nid = \"source-comparison\"\nverification = [\"test\"]\nstatement = \"Compare input before and after validation.\"", "criteria = []"),
        FIXTURE.replace("[[criteria]]\nid = \"source-comparison\"\nverification = [\"test\"]\nstatement = \"Compare input before and after validation.\"", "criteria = [{id = \"source-comparison\", statement = \"Compare inputs\", verification = [\"test\"]}]"),
        FIXTURE.replace("statement = \"Compare input before and after validation.\"", "statement = \"Compare inputs\"\n[[criteria]]\nid = \"source-comparison\"\nverification = [\"test\"]\nstatement = \"Duplicate\""),
        FIXTURE.replace("id = \"source-comparison\"", "id = \"bad_id\""),
    ];
    for text in mutations {
        has_problem(&text, Problem::InvalidDefinition);
    }
    let start = FIXTURE.find("authorities = [").unwrap();
    let end = start + FIXTURE[start..].find("]\n").unwrap() + 1;
    for replacement in [
        "authorities = []",
        "[[authorities]]\nkind = \"requirement\"\nid = \"preserve-input\"",
    ] {
        let mut text = FIXTURE.to_owned();
        text.replace_range(start..end, replacement);
        has_problem(&text, Problem::InvalidDefinition);
    }
    assert!(
        inspect(&FIXTURE.replace("requirements = [\"preserve-input\"]", "requirements = []"))
            .is_ok()
    );
}

#[test]
fn validates_all_edges_including_accepted_decisions() {
    has_problem(
        &FIXTURE.replace(
            "requirements = [\"preserve-input\"]",
            "requirements = [\"missing\"]",
        ),
        Problem::MissingTarget,
    );
    has_problem(
        &FIXTURE.replace(
            "requirements = [\"preserve-input\"]",
            "requirements = [\"immutable-input\"]",
        ),
        Problem::WrongTargetKind,
    );
    has_problem(
        &FIXTURE.replace(
            "requirements = [\"preserve-input\"]",
            "requirements = [\"preserve-input\", \"preserve-input\"]",
        ),
        Problem::RepeatedReference,
    );
    has_problem(
        &FIXTURE.replace(
            "{ kind = \"decision\", id = \"pure-reader\" }",
            "{ kind = \"requirement\", id = \"preserve-input\" }",
        ),
        Problem::RepeatedReference,
    );
    let draft_target = FIXTURE.replacen("status = \"accepted\"", "status = \"draft\"", 1);
    let errors = failures(&draft_target);
    assert_eq!(
        errors
            .diagnostics()
            .iter()
            .filter(|d| d.problem() == Problem::IncompatibleStatus)
            .count(),
        2
    );
    assert!(inspect(&FIXTURE.replace("status = \"accepted\"", "status = \"draft\"")).is_ok());
    has_problem(
        &FIXTURE.replace(
            "requirements = [\"preserve-input\"]",
            "requirements = [\"bad_id\"]",
        ),
        Problem::InvalidDefinition,
    );
}

#[test]
fn duplicates_have_no_winner_and_bad_authorities_do_not_look_missing() {
    let duplicated = format!("{FIXTURE}\n{}", requirement());
    let errors = failures(&duplicated);
    assert_eq!(
        errors
            .diagnostics()
            .iter()
            .filter(|d| d.problem() == Problem::DuplicateId)
            .count(),
        2
    );
    assert!(
        errors
            .diagnostics()
            .iter()
            .any(|d| d.problem() == Problem::AmbiguousTarget)
    );
    let duplicates: Vec<_> = errors
        .diagnostics()
        .iter()
        .filter(|d| d.problem() == Problem::DuplicateId)
        .collect();
    for (index, diagnostic) in duplicates.iter().enumerate() {
        assert_eq!(diagnostic.code(), DiagnosticCode::DuplicateId);
        assert_eq!(
            diagnostic.related_locations(),
            &[duplicates[1 - index].location().clone()]
        );
    }
    let cross_kind = FIXTURE.replacen("id = \"pure-reader\"", "id = \"preserve-input\"", 1);
    has_problem(&cross_kind, Problem::DuplicateId);
    let invalid = FIXTURE.replacen("source = \"The reader is observational.\"", "", 1);
    let errors = failures(&invalid);
    assert!(
        errors
            .diagnostics()
            .iter()
            .any(|d| d.problem() == Problem::InvalidDefinition)
    );
    assert_eq!(
        errors
            .diagnostics()
            .iter()
            .filter(|d| d.problem() == Problem::InvalidTarget)
            .count(),
        2
    );
    assert!(
        !errors
            .diagnostics()
            .iter()
            .any(|d| d.problem() == Problem::MissingTarget)
    );
    let malformed = FIXTURE.replacen("source = \"The reader is observational.\"", "source = [", 1);
    has_problem(&malformed, Problem::InvalidTarget);
}

#[test]
fn diagnostics_have_field_locations_and_preserve_parser_causes() {
    let text = requirement().replace("status = \"accepted\"", "status = \"wrong\"");
    let errors = failures(&text);
    let d = &errors.diagnostics()[0];
    assert_eq!(d.code(), DiagnosticCode::InvalidDefinition);
    assert_eq!(d.location().path(), Path::new("fixture.md"));
    assert_eq!(d.location().line(), 7);
    assert!(d.source().is_some());
    assert!(d.location().range().start >= text.find("status =").unwrap());
    let text = "é😀\n\n{anchor}`old`";
    let d = failures(text);
    assert_eq!(d.diagnostics()[0].location().line(), 3);
    assert_eq!(d.diagnostics()[0].location().column(), 1);
    let text = FIXTURE.replace(
        "requirements = [\"preserve-input\"]",
        "requirements = [\"absent\"]",
    );
    let err = failures(&text);
    let d = err
        .diagnostics()
        .iter()
        .find(|d| d.problem() == Problem::MissingTarget)
        .unwrap();
    assert_eq!(&text[d.location().range()], "\"absent\"");
}

#[test]
fn chapter_order_does_not_change_records_backlinks_or_diagnostics() {
    let split = FIXTURE.find("```anchorbook-decision").unwrap();
    let chapters = [
        Chapter::new(Path::new("z.md"), &FIXTURE[..split]),
        Chapter::new(Path::new("a.md"), &FIXTURE[split..]),
    ];
    let first = read_records(&chapters).unwrap();
    let reverse = read_records(&[chapters[1], chapters[0]]).unwrap();
    assert_eq!(
        first.records().collect::<Vec<_>>(),
        reverse.records().collect::<Vec<_>>()
    );
    assert_eq!(
        first.backlinks("preserve-input"),
        reverse.backlinks("preserve-input")
    );
    let invalid = FIXTURE.replace(
        "requirements = [\"preserve-input\"]",
        "requirements = [\"missing\"]",
    );
    let chapters = [
        Chapter::new(Path::new("z.md"), &invalid),
        Chapter::new(Path::new("a.md"), &invalid),
    ];
    assert_eq!(
        snapshot(&read_records(&chapters).unwrap_err()),
        snapshot(&read_records(&[chapters[1], chapters[0]]).unwrap_err())
    );
}

#[test]
fn chapter_identity_is_required_and_unique() {
    let errors = read_records(&[Chapter::new(Path::new(""), "")]).unwrap_err();
    assert_eq!(errors.diagnostics()[0].code(), DiagnosticCode::InvalidInput);
    assert_eq!(
        read_records(&[Chapter::new(Path::new(""), "")])
            .unwrap_err()
            .diagnostics()[0]
            .problem(),
        Problem::InvalidInput
    );
    let chapter = Chapter::new(Path::new("same.md"), "");
    assert_eq!(
        read_records(&[chapter, chapter]).unwrap_err().diagnostics()[0].problem(),
        Problem::InvalidInput
    );
}

#[test]
fn diagnostics_identify_unicode_columns_and_toml_error_ranges() {
    let text = "é😀 **{anchor}**";
    let errors = failures(text);
    let diagnostic = &errors.diagnostics()[0];
    assert_eq!(diagnostic.location().line(), 1);
    assert_eq!(diagnostic.location().column(), 6);
    assert_eq!(&text[diagnostic.location().range()], "{anchor}");
    assert!(diagnostic.message().contains("canonical definition fence"));

    let text = "# Context\n\n```anchorbook-requirement\nid = @\n```\n";
    let errors = failures(text);
    let diagnostic = &errors.diagnostics()[0];
    assert_eq!(diagnostic.location().line(), 4);
    assert_eq!(diagnostic.location().column(), 6);
    assert_eq!(&text[diagnostic.location().range()], "@");
    assert!(diagnostic.source().is_some());
}

#[test]
fn accepted_authority_conflicts_have_a_distinct_diagnostic_family() {
    let text = FIXTURE.replacen("status = \"accepted\"", "status = \"draft\"", 1);
    has_problem(&text, Problem::IncompatibleStatus);
}

#[test]
fn markdown_containers_and_code_boundaries_keep_their_diagnostic_meaning() {
    let nested = format!(
        "> ```text\n> example\n> ```\n>\n{}",
        prefix_lines(&requirement(), "> ")
    );
    let errors = failures(&nested);
    assert_eq!(errors.diagnostics().len(), 1);
    assert!(
        errors.diagnostics()[0]
            .message()
            .contains("must not be nested")
    );

    let text = "```text\n{anchor}`literal`\n```\n\n{anchor}`unsupported`\n";
    let errors = failures(text);
    assert_eq!(errors.diagnostics().len(), 1);
    assert_eq!(errors.diagnostics()[0].location().line(), 5);
    assert!(
        errors.diagnostics()[0]
            .message()
            .contains("legacy {anchor}")
    );
}

#[test]
fn markdown_extensions_preserve_legacy_marker_locations() {
    for text in [
        "| One | Two |\n| --- | --- |\n| {anchor} | {anchor} |\n",
        "[^note]: {anchor}\n",
        "~~struck~~{anchor}\n",
        "- [ ] {anchor}\n",
    ] {
        let errors = failures(text);
        let expected: Vec<_> = text
            .match_indices("{anchor}")
            .map(|(start, marker)| start..start + marker.len())
            .collect();
        assert_eq!(
            errors
                .diagnostics()
                .iter()
                .map(|d| d.location().range())
                .collect::<Vec<_>>(),
            expected,
            "{text}"
        );
        assert!(
            errors
                .diagnostics()
                .iter()
                .all(|d| d.message().contains("legacy {anchor}"))
        );
    }
}

#[test]
fn truncated_and_unicode_inputs_never_panic() {
    let text = FIXTURE.replace("Preserve input", "Preserve é😀 input");
    for (end, _) in text
        .char_indices()
        .chain(std::iter::once((text.len(), '\0')))
    {
        let _ = inspect(&text[..end]);
    }
    for malformed in [
        "é = [",
        "title = \"😀",
        "id = \"x\"\n\0",
        "id = true\nstatus = [\"é\"]",
    ] {
        has_problem(
            &format!("```anchorbook-requirement\n{malformed}\n```\n"),
            Problem::InvalidDefinition,
        );
    }
}
