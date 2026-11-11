//! Independent record-syntax and relationship fixtures.
use std::path::Path;

use anchor_book::records::{
    Chapter, DiagnosticCode, Problem, RecordData, RecordErrors, RecordKind, Status,
    VerificationMethod, read_records,
};

const FIXTURE: &str = include_str!("fixtures/records.md");
const CRITERION: &str =
    "> - Criterion `source-comparison` [test]: Compare input before and after validation.";
const SCOPE: &str = ">   - Domain: Every input.\n>   - Boundary: The reader.\n>   - Assumptions: None.\n>   - Exclusions: None.";

fn requirement() -> String {
    FIXTURE
        .split("> **`D-pure-reader`")
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

/// anchorbook: A-records-valid-authority#production-integration
#[test]
fn reads_typed_records_preserves_ranges_and_derives_direct_backlinks() {
    let original = FIXTURE.to_owned();
    let inventory = inspect(&original).unwrap();
    assert_eq!(inventory.len(), 3);
    assert!(!inventory.is_empty());
    assert_eq!(
        inventory.records().map(|r| r.id()).collect::<Vec<_>>(),
        ["A-immutable-input", "D-pure-reader", "R-preserve-input"]
    );
    let anchor = inventory.get("A-immutable-input").unwrap();
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
    assert_eq!(data.criteria[0].scope, None);
    assert_eq!(data.authorities[0].id, "R-preserve-input");
    assert_eq!(
        inventory.backlinks("R-preserve-input"),
        ["A-immutable-input", "D-pure-reader"]
    );
    assert_eq!(inventory.backlinks("D-pure-reader"), ["A-immutable-input"]);
    assert!(inventory.backlinks("A-immutable-input").is_empty());
    assert!(inventory.get("not-present").is_none());
    let block = &original[anchor.location().range()];
    assert!(block.starts_with("> **`A-immutable-input`** (accepted)"));
    assert!(block.ends_with("validation.\n"));
    let body = &original[anchor.body_location().range()];
    assert!(body.starts_with("> Property: "));
    assert!(!body.contains("**`A-"));
    for reference in anchor.references() {
        assert_eq!(
            &original[reference.location.range()],
            format!("`{}`", reference.id)
        );
    }
    let RecordData::Requirement(req) = inventory.get("R-preserve-input").unwrap().data() else {
        panic!()
    };
    assert_eq!(
        req.statement,
        "Reading leaves the supplied source unchanged."
    );
    assert_eq!(req.source, "The reader is observational.");
    let RecordData::Decision(decision) = inventory.get("D-pure-reader").unwrap().data() else {
        panic!()
    };
    assert_eq!(decision.choice, "Accept supplied chapters.");
    assert_eq!(decision.requirements[0].id, "R-preserve-input");
    assert_eq!(decision.rationale, "Separate acquisition from validation.");
    assert_eq!(decision.alternatives, ["Read paths inside the validator."]);
    assert_eq!(
        decision.reconsider_when,
        "A new adapter needs to acquire input differently."
    );
    assert_eq!(original, FIXTURE);
}

/// anchorbook: A-records-valid-authority#production-integration
#[test]
fn paragraphs_proof_scope_and_explicit_none_are_read_as_authored() {
    let text = FIXTURE
        .replace(
            "> Property: Parsing observes the input without changing it.",
            "> Property: Parsing observes the input.\n> It never changes it.\n>\n> Not even *once*.",
        )
        .replace(
            CRITERION,
            &format!(
                "{CRITERION}\n> - Criterion `formal` [proof, test]: The reader returns its input.\n{}",
                SCOPE.replace(
                    "Exclusions: None.",
                    "Exclusions: Conversion, covered by `source-comparison`."
                )
            ),
        )
        .replace("> Requirements: `R-preserve-input`", "> Requirements: none");
    let inventory = inspect(&text).unwrap();
    let RecordData::Anchor(anchor) = inventory.get("A-immutable-input").unwrap().data() else {
        panic!()
    };
    assert_eq!(
        anchor.property,
        "Parsing observes the input.\n\nIt never changes it.\n\nNot even *once*."
    );
    let formal = &anchor.criteria[1];
    assert_eq!(formal.statement, "The reader returns its input.");
    assert_eq!(
        formal.verification,
        [VerificationMethod::Proof, VerificationMethod::Test]
    );
    let scope = formal.scope.as_ref().unwrap();
    assert_eq!(scope.domain, "Every input.");
    assert_eq!(scope.boundary, "The reader.");
    assert_eq!(scope.assumptions, "None.");
    assert_eq!(
        scope.exclusions,
        "Conversion, covered by `source-comparison`."
    );
    assert_eq!(scope.covered_by.len(), 1);
    assert_eq!(scope.covered_by[0].anchor, "A-immutable-input");
    assert_eq!(scope.covered_by[0].criterion, "source-comparison");
    assert_eq!(
        &text[scope.covered_by[0].location.range()],
        "`source-comparison`"
    );
    let RecordData::Decision(decision) = inventory.get("D-pure-reader").unwrap().data() else {
        panic!()
    };
    assert!(decision.requirements.is_empty());
}

#[test]
fn parses_all_supported_line_endings_and_a_record_at_eof() {
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
        ("proof", vec![Proof]),
        ("test", vec![Test]),
        ("review", vec![Review]),
        ("review, proof, test", vec![Review, Proof, Test]),
    ] {
        let mut criterion = CRITERION.replace("[test]", &format!("[{authored}]"));
        if authored.contains("proof") {
            criterion = format!("{criterion}\n{SCOPE}");
        }
        let text = FIXTURE.replace(CRITERION, &criterion);
        let inventory = inspect(&text).unwrap();
        let RecordData::Anchor(anchor) = inventory.get("A-immutable-input").unwrap().data() else {
            panic!()
        };
        assert_eq!(anchor.criteria[0].verification, expected);
        // Selecting a proof method does not require proof artifacts or change design status.
        assert_eq!(
            inventory.get("A-immutable-input").unwrap().status(),
            Status::Accepted
        );
    }
}

#[test]
fn rejects_invalid_verification_on_each_criterion_with_located_diagnostics() {
    for replacement in [
        "[]",
        "[proof, proof]",
        "[test, test]",
        "[review, proof, review]",
        "[verus]",
        "[Proof]",
        "[ test]",
        "[test,review]",
        "[test , review]",
        "[test, ]",
        "test",
        "[test] ",
    ] {
        for second in [false, true] {
            let text = if second {
                FIXTURE.replace(
                    CRITERION,
                    &format!(
                        "{CRITERION}\n> - Criterion `second` {replacement}: Another obligation."
                    ),
                )
            } else {
                FIXTURE.replace(CRITERION, &CRITERION.replace("[test]", replacement))
            };
            let errors = failures(&text);
            assert_eq!(errors.diagnostics().len(), 1, "{text}\n{errors:?}");
            let diagnostic = &errors.diagnostics()[0];
            assert_eq!(diagnostic.code(), DiagnosticCode::InvalidDefinition);
            assert_eq!(diagnostic.problem(), Problem::InvalidDefinition);
            assert_eq!(diagnostic.location().path(), Path::new("fixture.md"));
            let line = if second {
                "> - Criterion `second`"
            } else {
                "> - Criterion "
            };
            assert_eq!(
                diagnostic.location().range().start,
                text.find(line).unwrap()
            );
            assert!(!diagnostic.message().is_empty());
        }
    }
}

#[test]
fn only_real_definitions_enter_inventory() {
    let examples = format!(
        "# Examples\n\n````text\n{FIXTURE}\n````\n\n```markdown\n{FIXTURE}```\n\n    > **`R-indented-code`** (draft): **Code**\n\n> A note quoting **`R-not-a-heading`**.\n> Still a note.\n\n`{{anchor}}` is a literal, and so is `> **`.\n"
    );
    assert!(inspect(&examples).unwrap().is_empty());
    assert!(inspect("# prose\n\nNo records here.").unwrap().is_empty());
    assert!(read_records(&[]).unwrap().is_empty());
}

/// anchorbook: A-render-validated-records#invalid-inventory
#[test]
fn rejects_noncanonical_records_and_nesting() {
    let base = requirement();
    let source = "> Source: The reader is observational.\n";
    let toml = "```anchorbook-requirement\nschema-version = 1\nid = \"requirement-x\"\ntitle = \"X\"\nstatus = \"draft\"\nstatement = \"S.\"\nsource = \"S.\"\n```\n";
    let mutations = [
        toml.to_owned(),
        format!("> Note:\n>\n{}", prefix_lines(toml, "> ")),
        format!("- item\n\n{}", prefix_lines(toml, "  ")),
        prefix_lines(&base, "> "),
        prefix_lines(&base, "  "),
        format!("- item\n\n{}", prefix_lines(&base, "  ")),
        format!(
            "> A note.\n{}",
            base.trim_start_matches("# A tiny independent specification\n\n")
        ),
        base.replace("> Statement:", ">Statement:"),
        base.replace(source, &format!("{source}A lazy continuation.\n")),
        base.replace(
            source,
            &format!(
                "{source}> **`R-second`** (draft): **Second**\n> Statement: S.\n> Source: S.\n"
            ),
        ),
        base.replace(source, &format!("{source}> > Quoted.\n")),
        "{anchor}`old-name`".to_owned(),
    ];
    for text in mutations {
        has_problem(&text, Problem::InvalidDefinition);
    }
}

/// anchorbook: A-render-validated-records#invalid-inventory
#[test]
fn rejects_malformed_headings_and_missing_repeated_misordered_or_empty_fields() {
    let base = requirement();
    let heading = "> **`R-preserve-input`** (accepted): **Preserve input**";
    let statement = "> Statement: Reading leaves the supplied source unchanged.";
    let source = "> Source: The reader is observational.";
    let mutations = [
        base.replace(
            heading,
            "> **`R-preserve-input`** (complete): **Preserve input**",
        ),
        base.replace(heading, "> **`R-preserve-input`** (accepted): ****"),
        base.replace(heading, "> **`R-preserve-input`** (accepted): **  **"),
        base.replace(
            heading,
            "> **`R-preserve-input`** (accepted): Preserve input",
        ),
        base.replace(
            heading,
            "> **`R-preserve-input`** accepted: **Preserve input**",
        ),
        base.replace(
            heading,
            "> **`R-preserve-input` (accepted): **Preserve input**",
        ),
        base.replace("R-preserve-input", "R-Uppercase"),
        base.replace("R-preserve-input", "R-bad--id"),
        base.replace("R-preserve-input", "R-2-start"),
        base.replace("R-preserve-input", "R-trailing-"),
        base.replace("R-preserve-input", "R-nön-ascii"),
        base.replace("R-preserve-input", "preserve-input"),
        base.replace(&format!("{statement}\n"), ""),
        base.replace(&format!("{source}\n"), ""),
        base.replace(source, "> Source: "),
        base.replace(source, "> Source:"),
        base.replace(statement, "> Statement: \t"),
        base.replace(source, &format!("{source}\n{statement}")),
        base.replace(
            &format!("{statement}\n{source}"),
            &format!("{source}\n{statement}"),
        ),
        base.replace(
            source,
            &format!("{source}\n> Choice: Not a requirement field."),
        ),
        base.replace(source, &format!("{source}\n>  indented")),
        base.replace(source, &format!("{source}\n>  ")),
        base.replace(source, &format!("{source}\n> - A list item.")),
        base.replace(
            source,
            &format!("{source}\n> - Criterion `x` [test]: Not here."),
        ),
    ];
    for text in mutations {
        has_problem(&text, Problem::InvalidDefinition);
    }
}

/// anchorbook: A-render-validated-records#invalid-inventory
#[test]
fn rejects_anchor_and_decision_field_errors() {
    let authority = "> Authority: `R-preserve-input`; `D-pure-reader`";
    let requirements = "> Requirements: `R-preserve-input`";
    let alternative = "> - Read paths inside the validator.";
    let mutations = [
        FIXTURE.replace(authority, "> Authority: R-preserve-input"),
        FIXTURE.replace(authority, "> Authority: `R-preserve-input` `D-pure-reader`"),
        FIXTURE.replace(authority, "> Authority: `A-immutable-input`"),
        FIXTURE.replace(authority, "> Authority: "),
        FIXTURE.replace(&format!("{authority}\n"), ""),
        FIXTURE.replace(&format!("\n{CRITERION}"), ""),
        FIXTURE.replace(CRITERION, &format!("{CRITERION}\n> Property: Again.")),
        FIXTURE.replace(CRITERION, &format!("{CRITERION}\n> A trailing paragraph.")),
        FIXTURE.replace(
            CRITERION,
            &CRITERION.replace("Compare input before and after validation.", ""),
        ),
        FIXTURE.replace(CRITERION, &format!("{CRITERION}\n{CRITERION}")),
        FIXTURE.replace(CRITERION, &CRITERION.replace("source-comparison", "bad_id")),
        FIXTURE.replace(
            CRITERION,
            &CRITERION.replace("`source-comparison`", "source-comparison"),
        ),
        FIXTURE.replace(CRITERION, &CRITERION.replace("]: ", "] ")),
        FIXTURE.replace(authority, &format!("{authority}\n> Continued.")),
        FIXTURE.replace("> Alternatives:\n", "> Alternatives: Inline.\n"),
        FIXTURE.replace(&format!("{alternative}\n"), ""),
        FIXTURE.replace(alternative, "> - "),
        FIXTURE.replace(alternative, "> - One.\n> Continued."),
        FIXTURE.replace(
            "> Rationale: Separate acquisition from validation.",
            "> Rationale: ",
        ),
        FIXTURE.replace("> Choice: Accept supplied chapters.", "> Choice: "),
        FIXTURE.replace(
            "> Reconsider when: A new adapter needs to acquire input differently.",
            "> Reconsider when: ",
        ),
        FIXTURE.replace(&format!("{requirements}\n"), ""),
        FIXTURE.replace(requirements, "> Requirements: "),
        FIXTURE.replace(requirements, "> Requirements: `D-pure-reader`"),
        FIXTURE.replace(requirements, "> Requirements: None"),
    ];
    for text in mutations {
        has_problem(&text, Problem::InvalidDefinition);
    }
    assert!(inspect(&FIXTURE.replace(requirements, "> Requirements: none")).is_ok());
}

/// anchorbook: A-render-validated-records#invalid-inventory
#[test]
fn validates_all_edges_including_accepted_decisions() {
    let requirements = "> Requirements: `R-preserve-input`";
    has_problem(
        &FIXTURE.replace(requirements, "> Requirements: `R-missing`"),
        Problem::MissingTarget,
    );
    has_problem(
        &FIXTURE.replace(
            requirements,
            "> Requirements: `R-preserve-input`; `R-preserve-input`",
        ),
        Problem::RepeatedReference,
    );
    has_problem(
        &FIXTURE.replace(
            "> Authority: `R-preserve-input`; `D-pure-reader`",
            "> Authority: `R-preserve-input`; `R-preserve-input`",
        ),
        Problem::RepeatedReference,
    );
    let draft_target = FIXTURE.replacen("(accepted)", "(draft)", 1);
    let errors = failures(&draft_target);
    assert_eq!(
        errors
            .diagnostics()
            .iter()
            .filter(|d| d.problem() == Problem::IncompatibleStatus)
            .count(),
        2
    );
    assert!(inspect(&FIXTURE.replace("(accepted)", "(draft)")).is_ok());
    has_problem(
        &FIXTURE.replace(requirements, "> Requirements: `R-bad_id`"),
        Problem::InvalidDefinition,
    );
}

/// anchorbook: A-render-validated-records#invalid-inventory
/// anchorbook: A-records-valid-authority#production-integration
#[test]
fn duplicates_have_no_winner_and_bad_authorities_do_not_look_missing() {
    let duplicated = format!(
        "{FIXTURE}\n{}",
        requirement().trim_start_matches("# A tiny independent specification\n\n")
    );
    let errors = failures(&duplicated);
    assert_eq!(
        errors
            .diagnostics()
            .iter()
            .filter(|d| d.problem() == Problem::DuplicateId)
            .count(),
        2,
        "{errors:?}"
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
    // The kind code is part of the identifier, so equal names of different kinds
    // are different records.
    let same_name = format!(
        "{FIXTURE}\n> **`D-preserve-input`** (draft): **Same name**\n> Choice: C.\n> Requirements: none\n> Rationale: R.\n> Alternatives:\n> - A.\n> Reconsider when: W.\n"
    );
    assert_eq!(inspect(&same_name).unwrap().len(), 4);
    let invalid = FIXTURE.replacen("> Source: The reader is observational.\n", "", 1);
    let errors = failures(&invalid);
    for diagnostic in errors
        .diagnostics()
        .iter()
        .filter(|d| d.problem() == Problem::InvalidTarget)
    {
        assert!(
            diagnostic
                .message()
                .contains("authority definition is invalid")
        );
        assert_eq!(diagnostic.related_locations().len(), 1);
        assert!(
            invalid[diagnostic.related_locations()[0].range()]
                .starts_with("> **`R-preserve-input`**")
        );
    }
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
    let malformed = FIXTURE.replacen(
        "> **`R-preserve-input`** (accepted): **Preserve input**",
        "> **`R-preserve-input** (accepted): **Preserve input**",
        1,
    );
    has_problem(&malformed, Problem::InvalidTarget);
    let errors = failures(&malformed);
    for diagnostic in errors
        .diagnostics()
        .iter()
        .filter(|d| d.problem() == Problem::InvalidTarget)
    {
        assert!(
            diagnostic
                .message()
                .contains("definitions with unknown identities")
        );
        assert_eq!(diagnostic.related_locations().len(), 1);
    }
}

/// anchorbook: A-records-valid-authority#production-integration
#[test]
fn diagnostics_are_located_at_the_offending_line() {
    let text = requirement().replace("(accepted)", "(wrong)");
    let errors = failures(&text);
    let d = &errors.diagnostics()[0];
    assert_eq!(d.code(), DiagnosticCode::InvalidDefinition);
    assert_eq!(d.location().path(), Path::new("fixture.md"));
    assert_eq!(d.location().line(), 3);
    assert_eq!(d.location().column(), 1);
    let text = requirement().replace("> Source: The reader is observational.", "> Source: ");
    let errors = failures(&text);
    let d = &errors.diagnostics()[0];
    assert_eq!(d.location().line(), 5);
    assert_eq!(&text[d.location().range()], "> Source: ");
    let text = "é😀\n\n{anchor}`old`";
    let d = failures(text);
    assert_eq!(d.diagnostics()[0].location().line(), 3);
    assert_eq!(d.diagnostics()[0].location().column(), 1);
    let text = FIXTURE.replace(
        "> Requirements: `R-preserve-input`",
        "> Requirements: `R-absent`",
    );
    let err = failures(&text);
    let d = err
        .diagnostics()
        .iter()
        .find(|d| d.problem() == Problem::MissingTarget)
        .unwrap();
    assert_eq!(&text[d.location().range()], "`R-absent`");
}

/// anchorbook: A-records-valid-authority#production-integration
#[test]
fn chapter_order_does_not_change_records_backlinks_or_diagnostics() {
    let split = FIXTURE.find("> **`D-pure-reader`").unwrap();
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
        first.backlinks("R-preserve-input"),
        reverse.backlinks("R-preserve-input")
    );
    let invalid = FIXTURE.replace(
        "> Requirements: `R-preserve-input`",
        "> Requirements: `R-missing`",
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
fn diagnostics_identify_unicode_columns_and_toml_blocks() {
    let text = "é😀 **{anchor}**";
    let errors = failures(text);
    let diagnostic = &errors.diagnostics()[0];
    assert_eq!(diagnostic.location().line(), 1);
    assert_eq!(diagnostic.location().column(), 6);
    assert_eq!(&text[diagnostic.location().range()], "{anchor}");
    assert!(diagnostic.message().contains("legacy {anchor}"));

    let text = "# Context é😀\n\n```anchorbook-requirement\nid = \"requirement-x\"\n```\n";
    let errors = failures(text);
    assert_eq!(errors.diagnostics().len(), 1);
    let diagnostic = &errors.diagnostics()[0];
    assert_eq!(diagnostic.location().line(), 3);
    assert_eq!(diagnostic.location().column(), 1);
    assert!(
        diagnostic
            .message()
            .contains("TOML record blocks are not supported")
    );
}

/// anchorbook: A-render-validated-records#invalid-inventory
#[test]
fn accepted_authority_conflicts_have_a_distinct_diagnostic_family() {
    let text = FIXTURE.replacen("(accepted)", "(draft)", 1);
    has_problem(&text, Problem::IncompatibleStatus);
}

#[test]
fn markdown_containers_and_code_boundaries_keep_their_diagnostic_meaning() {
    let nested = format!(
        "> ```text\n> example\n> ```\n>\n{}",
        prefix_lines(
            requirement().trim_start_matches("# A tiny independent specification\n\n"),
            "> "
        )
    );
    let errors = failures(&nested);
    assert_eq!(errors.diagnostics().len(), 1, "{errors:?}");
    assert!(
        errors.diagnostics()[0]
            .message()
            .contains("not nested in another quote")
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

/// anchorbook: A-check-shared-typed-results#library-boundary
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
        "> **`",
        "> **`R-😀",
        "> **`R-x`** (draft): **😀",
        "> **`R-x`** (draft): **T**\n> Statement: \0\n> Source:",
        "> **`A-x`** (draft): **T**\n> Property: é\n> Authority: `R-",
        "> **`A-x`** (draft): **T**\n> Property: é\n> Authority: `R-x`\n> - Criterion `",
    ] {
        has_problem(malformed, Problem::InvalidDefinition);
    }
}

/// anchorbook: A-records-valid-authority#production-integration
#[test]
fn records_next_to_code_and_indented_prose_are_read_exactly() {
    let record = requirement().replace("# A tiny independent specification\n\n", "");
    // A record right after a closing fence, and indented prose and nested list
    // items, are not code and not records.
    let text = format!(
        "```text\n> **`R-in-code`** (draft): **Code**\n```\n{record}\nProse.\n\n    > **`R-indented-code`** (draft): **Indented code**\n\n- A list\n  continued with **`text`**.\n  - Nested.\n"
    );
    let inventory = inspect(&text).unwrap();
    assert_eq!(
        inventory.records().map(|r| r.id()).collect::<Vec<_>>(),
        ["R-preserve-input"]
    );
    // A record must end before a fence that follows it; the error is at the fence,
    // not inside the record.
    let text = format!(
        "{}>\n```text\nx\n```\n",
        record.trim_end_matches('\n').to_owned() + "\n"
    );
    let errors = failures(&text);
    let d = &errors.diagnostics()[0];
    assert!(d.message().contains("leave a blank line after it"), "{d}");
    assert_eq!(&text[d.location().range()], "```text");
    // CRLF line endings stay inside the record's range, so rendering replaces
    // exactly the record.
    let crlf = FIXTURE.replace('\n', "\r\n");
    let inventory = inspect(&crlf).unwrap();
    for record in inventory.records() {
        let range = record.location().range();
        assert!(crlf[range.clone()].ends_with("\r\n"), "{}", record.id());
        assert!(
            crlf[range.end..].is_empty()
                || crlf[range.end..].starts_with("\r\n> **`")
                || crlf[range.end..].starts_with("\r\n")
        );
    }
}

#[test]
fn diagnostic_messages_name_the_rule_a_line_breaks() {
    let base = requirement();
    let source = "> Source: The reader is observational.";
    let statement = "> Statement: Reading leaves the supplied source unchanged.";
    let message = |text: &str| {
        let errors = failures(text);
        errors
            .diagnostics()
            .iter()
            .find(|d| d.problem() == Problem::InvalidDefinition)
            .unwrap()
            .message()
            .to_owned()
    };
    for (text, expected) in [
        (
            base.replace(source, &format!("{source}\n> Property: x")),
            "`Property:` is not a field of a requirement",
        ),
        (
            base.replace(source, &format!("{source}\n> - Criterion `x` [test]: y")),
            "criteria belong to anchors, not a requirement",
        ),
        (
            base.replace(&format!("{statement}\n"), ""),
            "expected `Statement:` here",
        ),
        (
            base.replace(source, &format!("{source}\n{statement}")),
            "`Statement:` is repeated or out of order",
        ),
        (
            FIXTURE.replace(
                "> - Read paths inside the validator.",
                "> - One.\n> Continued.",
            ),
            "criteria and alternatives are one line each",
        ),
        (
            FIXTURE.replace(
                "> Requirements: `R-preserve-input`",
                "> Requirements: `R-preserve-input`\n> Continued.",
            ),
            "`Requirements:` is one line",
        ),
        (
            FIXTURE.replace(CRITERION, &format!("{CRITERION}\n> Continued.")),
            "criteria and alternatives are one line each",
        ),
        (
            format!(
                "> A note.\n{}",
                base.replace("# A tiny independent specification\n\n", "")
            ),
            "must start its own blockquote",
        ),
        (
            prefix_lines(
                &base.replace("# A tiny independent specification\n\n", ""),
                "> ",
            ),
            "not nested in another quote",
        ),
        (
            prefix_lines(
                &base.replace("# A tiny independent specification\n\n", ""),
                "  ",
            ),
            "not nested in a list or indented",
        ),
    ] {
        let found = message(&text);
        assert!(found.contains(expected), "{expected}: {found}\n{text}");
    }
    // A missing field before the criteria is reported at the first criterion.
    let text = FIXTURE
        .replace("> Authority: `R-preserve-input`; `D-pure-reader`\n", "")
        .replace(
            CRITERION,
            &format!("{CRITERION}\n> - Criterion `second` [test]: Another."),
        );
    let errors = failures(&text);
    let d = &errors.diagnostics()[0];
    assert!(d.message().contains("expected `Authority:` here"), "{d}");
    assert_eq!(&text[d.location().range()], CRITERION);
}
