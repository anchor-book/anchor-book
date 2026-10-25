//! Separate acceptance check against the current Book, loaded only at runtime.
use anchor_book::records::{RecordData, RecordKind, VerificationMethod, read_records};

#[path = "support/book.rs"]
mod book;

#[test]
#[ignore = "current-Book acceptance; run explicitly with --ignored after independent tests"]
fn current_book_records() {
    let sources = book::load();
    let chapters = book::chapters(&sources);
    let inventory = read_records(&chapters).unwrap_or_else(|e| {
        for d in e.diagnostics() {
            eprintln!("{d}");
        }
        panic!("current Book failed record acceptance")
    });
    assert_eq!(inventory.len(), 25);
    for (kind, expected) in [
        (RecordKind::Anchor, 12),
        (RecordKind::Requirement, 8),
        (RecordKind::Decision, 5),
    ] {
        assert_eq!(
            inventory.records().filter(|r| r.kind() == kind).count(),
            expected
        );
    }
    assert_eq!(
        inventory
            .records()
            .map(|r| r.references().len())
            .sum::<usize>(),
        35
    );
    let expected = [
        "check-actionable-diagnostics",
        "check-book-selection",
        "check-configuration-validity",
        "check-independent-build",
        "check-navigation-conformance",
        "check-observation-only",
        "check-renderer-alignment",
        "check-shared-typed-results",
        "check-truthful-results",
        "decision-explicit-configuration",
        "decision-mdbook-structure",
        "decision-separate-check-and-repair",
        "decision-shared-record-rendering",
        "decision-typed-check-results",
        "engine-valid-authority",
        "render-records-in-context",
        "render-validated-records",
        "requirement-configured-structure",
        "requirement-independent-engine",
        "requirement-observation-only",
        "requirement-readable-records",
        "requirement-repair-feedback",
        "requirement-shared-checker",
        "requirement-traceable-properties",
        "requirement-truthful-results",
    ];
    assert_eq!(
        inventory.records().map(|r| r.id()).collect::<Vec<_>>(),
        expected
    );
    assert_eq!(
        inventory.backlinks("requirement-traceable-properties"),
        ["engine-valid-authority"]
    );
    assert_eq!(
        inventory.backlinks("requirement-readable-records"),
        [
            "decision-shared-record-rendering",
            "render-records-in-context",
            "render-validated-records"
        ]
    );
    let RecordData::Anchor(anchor) = inventory.get("render-validated-records").unwrap().data()
    else {
        panic!()
    };
    assert_eq!(
        anchor
            .criteria
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["invalid-inventory", "protocol-and-independence"]
    );
    let RecordData::Anchor(authority) = inventory.get("engine-valid-authority").unwrap().data()
    else {
        panic!()
    };
    assert_eq!(
        authority
            .criteria
            .iter()
            .map(|c| (c.id.as_str(), c.verification.as_slice()))
            .collect::<Vec<_>>(),
        [
            ("required-authority", &[VerificationMethod::Test][..]),
            ("resolved-authority", &[VerificationMethod::Test][..]),
            ("compatible-status", &[VerificationMethod::Test][..]),
            ("formal-authority", &[VerificationMethod::Proof][..]),
            (
                "production-integration",
                &[VerificationMethod::Test, VerificationMethod::Review][..]
            ),
        ]
    );
    let reversed: Vec<_> = chapters.iter().rev().copied().collect();
    let again = read_records(&reversed).unwrap();
    assert_eq!(
        inventory.records().collect::<Vec<_>>(),
        again.records().collect::<Vec<_>>()
    );
    for record in inventory.records() {
        assert_eq!(
            inventory.backlinks(record.id()),
            again.backlinks(record.id())
        );
        let source = chapters
            .iter()
            .find(|c| c.path == record.location().path())
            .unwrap()
            .content;
        assert!(source[record.location().range()].starts_with("```anchorbook-"));
    }
}

#[test]
#[ignore = "current-project configuration acceptance; independent tests use disposable fixtures"]
fn current_book_configuration() {
    use anchor_book::configuration::{Phase, SectionRole, SourceState, load};
    let root = std::fs::canonicalize(env!("CARGO_MANIFEST_DIR")).unwrap();
    let config = load(&root.join("anchorbook.toml")).unwrap();
    assert_eq!(config.phase(), Phase::Prototyping);
    assert_eq!(config.source(), root.join("book"));
    assert_eq!(config.output(), root.join("target/anchorbook"));
    assert_eq!(config.source_state(), SourceState::Directory);
    for (role, path) in [
        (SectionRole::Design, "design/index.md"),
        (SectionRole::Authority, "authority/index.md"),
        (
            SectionRole::ImplementationRoadmaps,
            "implementation-roadmaps/index.md",
        ),
    ] {
        assert_eq!(config.section(role), root.join("book").join(path));
    }
}
