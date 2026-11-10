//! Separate acceptance check against the current Book, loaded only at runtime.
use anchor_book::records::{RecordKind, read_records};

#[path = "support/book.rs"]
mod book;

#[test]
#[ignore = "current-Book acceptance; run explicitly with --ignored after independent tests"]
fn current_book_records() {
    // Properties that hold for any valid Book; the Book's contents are not pinned here.
    // Independent fixtures specify parsing, methods, and backlinks against expected values.
    let sources = book::load();
    let chapters = book::chapters(&sources);
    let inventory = read_records(&chapters).unwrap_or_else(|e| {
        for d in e.diagnostics() {
            eprintln!("{d}");
        }
        panic!("current Book failed record acceptance")
    });
    assert!(!inventory.is_empty(), "the current Book defines records");

    // Every reference resolves to a record of its kind and appears in that record's backlinks.
    for record in inventory.records() {
        for reference in record.references() {
            let target = inventory
                .get(&reference.id)
                .unwrap_or_else(|| panic!("{} cites missing {}", record.id(), reference.id));
            assert_eq!(target.kind(), RecordKind::from(reference.kind));
            assert!(
                inventory
                    .backlinks(&reference.id)
                    .iter()
                    .any(|b| b == record.id()),
                "{} is missing from the backlinks of {}",
                record.id(),
                reference.id
            );
        }
    }
    // Every backlink comes from an authored reference.
    for record in inventory.records() {
        for source in inventory.backlinks(record.id()) {
            let citing = inventory.get(source).unwrap();
            assert!(
                citing.references().iter().any(|r| r.id == record.id()),
                "backlink {source} -> {} has no authored reference",
                record.id()
            );
        }
    }

    // Chapter order affects neither the inventory nor its backlinks.
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
        // Each record's source range is exactly its authored definition.
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
    // The real configuration loads and resolves within the checkout; its values
    // (phase, paths, roles) are project choices and are not pinned here.
    use anchor_book::configuration::{SectionRole, SourceState, load};
    let root = std::fs::canonicalize(env!("CARGO_MANIFEST_DIR")).unwrap();
    let config = load(&root.join("anchorbook.toml")).unwrap();
    assert_eq!(config.source_state(), SourceState::Directory);
    assert!(config.source().starts_with(&root));
    assert!(config.output().starts_with(&root));
    for role in [
        SectionRole::Design,
        SectionRole::Authority,
        SectionRole::ImplementationRoadmaps,
    ] {
        assert!(config.section(role).starts_with(config.source()));
        assert!(config.section(role).is_file(), "{role:?} chapter exists");
    }
}

#[test]
#[ignore = "current-Book structural acceptance; independent checker tests use disposable fixtures"]
fn current_book_structure() {
    use anchor_book::checking::{Outcome, State, check};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let report = check(&root.join("anchorbook.toml")).unwrap_or_else(|error| {
        panic!(
            "{}",
            anchor_book::adapters::check::format_report(error.report())
        )
    });
    assert_eq!(
        report.outcome(),
        Outcome::Passed,
        "{}",
        anchor_book::adapters::check::format_report(&report)
    );
    assert!(report.findings().is_empty());
    assert_eq!(report.groups().len(), 6);
    assert!(
        report
            .groups()
            .iter()
            .all(|group| group.state == State::Passed)
    );
}

/// anchorbook: render-roadmap-milestones#roadmap-validation
#[test]
#[ignore = "current-Book roadmap acceptance; run explicitly with --ignored after independent tests"]
fn current_book_roadmaps() {
    // This Book's roadmaps are the chapters under its implementation-roadmaps role.
    let sources = book::load();
    let chapters = book::chapters(&sources);
    let inventory = read_records(&chapters).expect("current Book records are valid");
    let roadmaps: Vec<_> = chapters
        .iter()
        .copied()
        .filter(|c| c.path.starts_with("implementation-roadmaps"))
        .collect();
    let milestones =
        anchor_book::roadmaps::read_roadmaps(&roadmaps, &inventory).unwrap_or_else(|e| {
            for d in e.diagnostics() {
                eprintln!("{d}");
            }
            panic!("current Book roadmaps failed validation")
        });
    assert!(
        !milestones.is_empty(),
        "the current Book has roadmap milestones"
    );
}
