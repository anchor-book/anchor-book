//! The line grammar's guarantees to agents: searches find records and references,
//! every line is accounted for, and identifiers state their kind. Fixtures are
//! written from the record syntax in Anchor Definitions, not from engine output.
use std::path::Path;

use anchor_book::records::{
    Chapter, DiagnosticCode, Problem, RecordData, RecordErrors, RecordInventory, RecordKind,
    Status, read_records,
};

/// An independent fixture Book: three kinds, prose, and a decision citing no requirement.
const BOOK: &str = "# Fixture

Prose before the records names `R-inputs-unchanged` in passing.

> **`R-inputs-unchanged`** (accepted): **Inputs stay unchanged**
> Statement: Reading must not change its input.
> Source: The reader only observes.

> **`R-fast-reads`** (draft): **Reads are fast**
> Statement: Reading takes linear time.
> Source: Large Books.

> **`D-in-memory-reader`** (accepted): **Read supplied text**
> Choice: The reader takes chapter text from its caller.
> Requirements: `R-inputs-unchanged`
> Rationale: Acquisition stays outside validation.
> Alternatives:
> - Open files inside the reader.
> Reconsider when: A caller cannot supply text.

> **`D-no-index-cache`** (draft): **Keep no index cache**
> Choice: Every read starts fresh.
> Requirements: none
> Rationale: Nothing to invalidate.
> Alternatives:
> - Cache the index between reads.
> Reconsider when: Reads become slow.

> **`A-reads-preserve-input`** (draft): **Reading preserves its input**
> Property: The reader returns records without changing the supplied text.
> Authority: `R-inputs-unchanged`; `D-in-memory-reader`; `R-fast-reads`
> - Criterion `byte-compare` [test]: Compare the text before and after reading.
> - Criterion `formal-preservation` [proof]: The reader never writes its input.
>   - Domain: Every chapter text.
>   - Boundary: The line scanner.
>   - Assumptions: The verifier is trusted.
>   - Exclusions: Allocation, covered by `byte-compare` and `A-reads-are-linear#timing`.

> **`A-reads-are-linear`** (accepted): **Reading is linear**
> Property: Reading time grows linearly with the text.
> Authority: `R-inputs-unchanged`
> - Criterion `timing` [test, review]: Time reads of growing Books.
";

fn read(text: &str) -> Result<RecordInventory, RecordErrors> {
    read_records(&[Chapter::new(Path::new("book.md"), text)])
}

/// The heading of a record, parsed from the matched line alone.
fn heading(line: &str) -> Option<(&str, &str, &str)> {
    let rest = line.strip_prefix("> **`")?;
    let (id, rest) = rest.split_once("`** (")?;
    let (status, rest) = rest.split_once("): **")?;
    Some((id, status, rest.strip_suffix("**")?))
}

/// anchorbook: A-records-searchable-lines#heading-search
#[test]
fn a_heading_search_returns_every_record_with_its_kind_identity_status_and_title() {
    let inventory = read(BOOK).unwrap();
    // The search `^> \*\*`[ARD]-`, applied line by line.
    let found: Vec<_> = BOOK
        .lines()
        .filter(|l| {
            l.strip_prefix("> **`")
                .is_some_and(|r| ["A-", "R-", "D-"].iter().any(|c| r.starts_with(c)))
        })
        .map(|l| heading(l).expect("a matched line is a whole heading"))
        .collect();
    assert_eq!(found.len(), inventory.len());
    for (id, status, title) in found {
        let record = inventory.get(id).expect("every match is a record");
        let kind = match &id[..2] {
            "A-" => RecordKind::Anchor,
            "R-" => RecordKind::Requirement,
            _ => RecordKind::Decision,
        };
        assert_eq!(record.kind(), kind, "{id}");
        assert_eq!(record.title(), title);
        let status = if status == "draft" {
            Status::Draft
        } else {
            Status::Accepted
        };
        assert_eq!(record.status(), status);
    }
    // A search for one kind's drafts answers from the matched lines.
    let draft_decisions: Vec<_> = BOOK
        .lines()
        .filter(|l| l.starts_with("> **`D-") && l.contains("`** (draft)"))
        .collect();
    assert_eq!(draft_decisions.len(), 1);
    assert!(draft_decisions[0].contains("D-no-index-cache"));
}

/// anchorbook: A-records-searchable-lines#reference-search
#[test]
fn an_identifier_search_returns_its_definition_and_exactly_its_references() {
    let inventory = read(BOOK).unwrap();
    for record in inventory.records() {
        let needle = format!("`{}`", record.id());
        let (mut definitions, mut references, mut mentions) = (0, 0, 0);
        for line in BOOK.lines().filter(|l| l.contains(&needle)) {
            // Each matched line is classified from its own text.
            if line.starts_with(&format!("> **{needle}**")) {
                definitions += 1;
            } else if line.starts_with("> Authority: ") || line.starts_with("> Requirements: ") {
                references += 1;
            } else if line.starts_with(">   - Exclusions: ") || !line.starts_with('>') {
                mentions += 1;
            } else {
                panic!("unclassifiable match for {needle}: {line}");
            }
        }
        assert_eq!(definitions, 1, "{needle}");
        assert_eq!(
            references,
            inventory.backlinks(record.id()).len(),
            "{needle}: one reference line for each citing record"
        );
        let expected_mentions = usize::from(record.id() == "R-inputs-unchanged");
        assert_eq!(mentions, expected_mentions, "{needle}");
    }
}

/// The one invalid-definition diagnostic for `text`, as its line and source text.
/// References to the invalid record may also report it as an invalid target.
fn single_error(text: &str) -> (usize, String) {
    let errors = read(text).expect_err("the fixture is invalid");
    let definitions: Vec<_> = errors
        .diagnostics()
        .iter()
        .filter(|d| d.problem() == Problem::InvalidDefinition)
        .collect();
    assert_eq!(definitions.len(), 1, "{errors:?}");
    assert!(
        errors
            .diagnostics()
            .iter()
            .all(|d| d.problem() == Problem::InvalidDefinition
                || d.problem() == Problem::InvalidTarget),
        "{errors:?}"
    );
    let diagnostic = definitions[0];
    assert_eq!(diagnostic.code(), DiagnosticCode::InvalidDefinition);
    (
        diagnostic.location().line(),
        text[diagnostic.location().range()].to_owned(),
    )
}

/// The line of the last occurrence of `needle`, where a repeated line is reported.
fn line_of(text: &str, needle: &str) -> usize {
    text[..text.rfind(needle).unwrap()].matches('\n').count() + 1
}

/// anchorbook: A-records-every-line#line-accounting
#[test]
fn stray_misordered_repeated_unknown_and_missing_lines_are_located() {
    let source = "> Source: The reader only observes.";
    let choice = "> Choice: Every read starts fresh.";
    let cases = [
        // A stray, unlabeled line where no text field can continue.
        (
            BOOK.replace(
                "> Requirements: none",
                "> Requirements: none\n> A stray line.",
            ),
            "> A stray line.",
        ),
        // Misordered: `Rationale:` before `Requirements:`.
        (
            BOOK.replace(
                &format!("{choice}\n> Requirements: none\n> Rationale: Nothing to invalidate."),
                &format!("{choice}\n> Rationale: Nothing to invalidate.\n> Requirements: none"),
            ),
            "> Rationale: Nothing to invalidate.",
        ),
        // Repeated `Source:`.
        (BOOK.replace(source, &format!("{source}\n{source}")), source),
        // A field of another kind.
        (
            BOOK.replace(
                source,
                &format!("{source}\n> Property: Not a requirement field."),
            ),
            "> Property: Not a requirement field.",
        ),
        // Missing `Source:`, reported where the record ends.
        (
            BOOK.replace(&format!("\n{source}"), ""),
            "> Statement: Reading must not change its input.",
        ),
        // An indented line that is not a proof-scope item.
        (
            BOOK.replace(source, &format!("{source}\n>    deeper")),
            ">    deeper",
        ),
    ];
    for (text, at) in &cases {
        let (line, found) = single_error(text);
        assert_eq!(line, line_of(text, at), "{at}");
        assert_eq!(found, *at);
    }
    // The corrected text is the expected record.
    let RecordData::Requirement(r) = read(BOOK)
        .unwrap()
        .get("R-inputs-unchanged")
        .unwrap()
        .data()
        .clone()
    else {
        panic!()
    };
    assert_eq!(r.source, "The reader only observes.");
}

/// anchorbook: A-records-every-line#proof-scope
#[test]
fn proof_criteria_carry_their_scope_and_exclusions_name_real_criteria() {
    let domain = ">   - Domain: Every chapter text.";
    let boundary = ">   - Boundary: The line scanner.";
    let exclusions =
        ">   - Exclusions: Allocation, covered by `byte-compare` and `A-reads-are-linear#timing`.";
    let cases = [
        (BOOK.replace(&format!("{domain}\n"), ""), ">   - Boundary: The line scanner."),
        (BOOK.replace(domain, ">   - Domain: "), ">   - Domain: "),
        (BOOK.replace(domain, &format!("{domain}\n{domain}")), domain),
        (BOOK.replace(&format!("{domain}\n{boundary}"), &format!("{boundary}\n{domain}")), boundary),
        (BOOK.replace(exclusions, &exclusions.replace("`byte-compare`", "`byte-check`")), "`byte-check`"),
        (BOOK.replace(exclusions, &exclusions.replace("#timing", "#speed")), "`A-reads-are-linear#speed`"),
        (BOOK.replace(exclusions, &exclusions.replace("A-reads-are-linear", "A-missing")), "`A-missing#timing`"),
        (BOOK.replace(exclusions, &exclusions.replace("`byte-compare`", "`R-fast-reads`")), "`R-fast-reads`"),
        (BOOK.replace(exclusions, &exclusions.replace("`byte-compare`", "`R-fast-reads#timing`")), "`R-fast-reads#timing`"),
        (BOOK.replace(exclusions, &exclusions.replace("`byte-compare`", "`not a name`")), "`not a name`"),
        (BOOK.replace(exclusions, &exclusions.replace("#timing", "#Timing")), "`A-reads-are-linear#Timing`"),
        (BOOK.replace(&format!("\n{exclusions}"), ""), ">   - Assumptions: The verifier is trusted."),
        (
            BOOK.replace(&format!("{domain}\n{boundary}\n>   - Assumptions: The verifier is trusted.\n{exclusions}\n"), ""),
            "> - Criterion `formal-preservation` [proof]: The reader never writes its input.",
        ),
        (
            BOOK.replace(
                "> - Criterion `timing` [test, review]: Time reads of growing Books.",
                "> - Criterion `timing` [test, review]: Time reads of growing Books.\n>   - Domain: Not a proof criterion.",
            ),
            ">   - Domain: Not a proof criterion.",
        ),
    ];
    for (text, at) in &cases {
        let (line, found) = single_error(text);
        assert_eq!(line, line_of(text, at), "{at}");
        if at.starts_with('`') {
            assert_eq!(found, *at);
        }
    }
    let message = |text: &str| {
        read(text).unwrap_err().diagnostics()[0]
            .message()
            .to_owned()
    };
    assert!(
        message(&cases[7].0).contains("names covering criteria as"),
        "a span that is not a criterion reference names the form"
    );
    assert!(message(&cases[4].0).contains("which is not a criterion in this Book"));
    for form in [8, 9, 10] {
        assert!(
            message(&cases[form].0).contains("names covering criteria as"),
            "{}",
            cases[form].1
        );
    }
    assert!(
        message(&cases[12].0)
            .contains("needs `Domain:`, `Boundary:`, `Assumptions:`, and `Exclusions:`")
    );
    let inventory = read(BOOK).unwrap();
    let RecordData::Anchor(anchor) = inventory.get("A-reads-preserve-input").unwrap().data() else {
        panic!()
    };
    let scope = anchor.criteria[1].scope.as_ref().unwrap();
    assert_eq!(scope.domain, "Every chapter text.");
    let named: Vec<_> = scope
        .covered_by
        .iter()
        .map(|c| (c.anchor.as_str(), c.criterion.as_str()))
        .collect();
    assert_eq!(
        named,
        [
            ("A-reads-preserve-input", "byte-compare"),
            ("A-reads-are-linear", "timing")
        ]
    );
    assert!(anchor.criteria[0].scope.is_none());
}

/// anchorbook: A-records-named-identifiers#identifier-forms
#[test]
fn identifiers_state_their_kind_and_never_a_number() {
    let heading = "> **`R-fast-reads`** (draft): **Reads are fast**";
    for bad in [
        "fast-reads",
        "X-fast-reads",
        "r-fast-reads",
        "R-Fast-reads",
        "D-12",
        "R-2026-01",
        "R-fast_reads",
        "R-fast-",
        "R--fast",
    ] {
        let text = BOOK
            .replace(heading, &heading.replace("R-fast-reads", bad))
            .replace("`R-fast-reads`", &format!("`{bad}`"));
        let errors = read(&text).expect_err(bad);
        let at = line_of(&text, &format!("> **`{bad}`**"));
        assert!(
            errors.diagnostics().iter().any(|d| d.code() == DiagnosticCode::InvalidDefinition
                && d.location().line() == at),
            "{bad}: {errors:?}"
        );
        assert!(read(&text).is_err());
    }
    // A record's fields must be those of the kind its code states.
    let mismatch = BOOK
        .replace(heading, "> **`D-fast-reads`** (draft): **Reads are fast**")
        .replace("`R-fast-reads`", "`D-fast-reads`");
    let (line, found) = single_error(&mismatch);
    assert_eq!(found, "> Statement: Reading takes linear time.");
    assert_eq!(
        line,
        line_of(&mismatch, "> Statement: Reading takes linear time.")
    );
    // Digits inside a word are allowed.
    let digits = BOOK.replace("R-fast-reads", "R-utf8-reads");
    assert!(read(&digits).unwrap().get("R-utf8-reads").is_some());
}

/// anchorbook: A-records-named-identifiers#reference-codes
#[test]
fn reference_fields_accept_only_their_kinds_codes_even_when_the_record_exists() {
    let cases = [
        (
            BOOK.replace(
                "> Authority: `R-inputs-unchanged`\n",
                "> Authority: `A-reads-preserve-input`\n",
            ),
            "`A-reads-preserve-input`",
            "> Authority: `A-reads-preserve-input`",
        ),
        (
            BOOK.replace(
                "> Requirements: `R-inputs-unchanged`",
                "> Requirements: `D-no-index-cache`",
            ),
            "`D-no-index-cache`",
            "> Requirements: `D-no-index-cache`",
        ),
    ];
    for (text, at, field) in &cases {
        let (line, found) = single_error(text);
        assert_eq!(found, *at);
        assert_eq!(line, line_of(text, field));
    }
}
