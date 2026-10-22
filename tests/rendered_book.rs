//! Explicit acceptance of an already-built mdBook; never part of ordinary tests.
use anchor_book::{Record, RecordData, Status, VerificationMethod, read_records};
use percent_encoding::percent_decode_str;
use scraper::{ElementRef, Html, Selector};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use url::Url;

#[path = "support/book.rs"]
mod book;

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn text(element: ElementRef<'_>) -> String {
    normalize(&element.text().collect::<Vec<_>>().join(" "))
}

fn selector(css: &str) -> Selector {
    Selector::parse(css).expect("valid test selector")
}

struct Page {
    html: Html,
    ids: BTreeMap<String, usize>,
}

impl Page {
    fn read(path: &Path) -> Self {
        let source = fs::read_to_string(path).expect("read rendered page");
        let html = Html::parse_document(&source);
        let mut ids = BTreeMap::new();
        for element in html.select(&selector("[id]")) {
            *ids.entry(element.attr("id").unwrap().to_owned())
                .or_default() += 1;
        }
        Self { html, ids }
    }

    fn records(&self) -> BTreeMap<&str, ElementRef<'_>> {
        let mut records = BTreeMap::new();
        for element in self.html.select(&selector("section.ab-record")) {
            let id = element.attr("id").expect("record has identity");
            assert!(
                records.insert(id, element).is_none(),
                "duplicate record {id}"
            );
            assert_eq!(self.ids.get(id), Some(&1), "unique record target {id}");
        }
        records
    }
}

fn read_pages(directory: &Path, pages: &mut BTreeMap<PathBuf, Page>) {
    for entry in fs::read_dir(directory).expect("read rendered directory") {
        let entry = entry.expect("read rendered entry");
        let kind = entry.file_type().expect("read rendered entry type");
        let path = entry.path();
        if kind.is_dir() {
            read_pages(&path, pages);
        } else if kind.is_file() && path.extension().is_some_and(|ext| ext == "html") {
            pages.insert(path, Page::read(&entry.path()));
        }
    }
}

fn check_record(record: &Record, element: ElementRef<'_>) {
    let rendered = text(element);
    let has = |value: &str| {
        assert!(
            rendered.contains(&normalize(value)),
            "record {} missing field: {value}",
            record.id()
        );
    };
    has(record.id());
    has(record.title());
    let status: Vec<_> = element.select(&selector(".ab-status")).map(text).collect();
    assert_eq!(
        status,
        [match record.status() {
            Status::Draft => "Draft",
            Status::Accepted => "Accepted",
        }],
        "design status for {}",
        record.id()
    );
    for reference in record.references() {
        has(&reference.id);
    }
    match record.data() {
        RecordData::Anchor(anchor) => {
            has(&anchor.property);
            let criteria: Vec<_> = element.select(&selector(".ab-criteria > li")).collect();
            assert_eq!(
                criteria.len(),
                anchor.criteria.len(),
                "criteria for {}",
                record.id()
            );
            for (criterion, item) in anchor.criteria.iter().zip(criteria) {
                let ids: Vec<_> = item.select(&selector("code")).map(text).collect();
                assert_eq!(ids, [criterion.id.as_str()], "criterion identity");
                let statement: Vec<_> = item.select(&selector(".ab-text")).map(text).collect();
                assert_eq!(
                    statement,
                    [normalize(&criterion.statement)],
                    "criterion statement"
                );
                // Independent expected labels, not the production renderer's mapping.
                let methods = criterion
                    .verification
                    .iter()
                    .map(|method| match method {
                        VerificationMethod::Proof => "proof",
                        VerificationMethod::Test => "test",
                        VerificationMethod::Review => "review",
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let labels: Vec<_> = item.select(&selector("p")).map(text).collect();
                assert_eq!(
                    labels,
                    [format!("Required verification: {methods}")],
                    "criterion methods"
                );
            }
        }
        RecordData::Requirement(requirement) => {
            has(&requirement.statement);
            has(&requirement.source);
        }
        RecordData::Decision(decision) => {
            has(&decision.choice);
            has(&decision.rationale);
            for alternative in &decision.alternatives {
                has(alternative);
            }
            has(&decision.reconsider_when);
        }
    }
}

fn check_links(pages: &BTreeMap<PathBuf, Page>) -> usize {
    let mut checked = 0;
    for (path, page) in pages {
        let base = Url::from_file_path(path).expect("absolute rendered path");
        for link in page.html.select(&selector("a[href]")) {
            let href = link.attr("href").unwrap();
            // Match the previous check's scope: relative local links only.
            // External and site-root URLs need a deployed-site check, not network access here.
            if href.starts_with('/') || Url::parse(href).is_ok() {
                continue;
            }
            let resolved = base
                .join(href)
                .unwrap_or_else(|error| panic!("{href}: {error}"));
            let target = resolved.to_file_path().expect("relative local link");
            assert!(target.exists(), "{}: missing target {href}", path.display());
            if let (Some(target_page), Some(fragment)) = (pages.get(&target), resolved.fragment()) {
                let fragment = percent_decode_str(fragment)
                    .decode_utf8()
                    .expect("UTF-8 fragment");
                assert!(
                    target_page.ids.contains_key(fragment.as_ref()),
                    "{}: missing fragment {href}",
                    path.display()
                );
            }
            checked += 1;
        }
    }
    checked
}

#[test]
#[ignore = "rendered-Book acceptance; run mdbook build, then this test with --ignored"]
fn rendered_book_records_and_links() {
    let output = std::env::var_os("ANCHORBOOK_TEST_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/anchorbook"))
        .canonicalize()
        .expect("rendered Book directory; run mdbook build first");
    let sources = book::load();
    let chapters = book::chapters(&sources);
    let inventory = read_records(&chapters).expect("valid Book records");
    assert!(!inventory.is_empty(), "expected records in this Book");
    let mut pages = BTreeMap::new();
    read_pages(&output, &mut pages);
    assert!(!pages.is_empty(), "no rendered HTML");

    for chapter in &chapters {
        let path = output.join(chapter.path).with_extension("html");
        let page = pages
            .get(&path)
            .unwrap_or_else(|| panic!("missing page {}", chapter.path.display()));
        let expected: BTreeMap<_, _> = inventory
            .records()
            .filter(|record| record.location().path() == chapter.path)
            .map(|record| (record.id(), record))
            .collect();
        let actual = page.records();
        assert_eq!(
            actual.keys().collect::<Vec<_>>(),
            expected.keys().collect::<Vec<_>>(),
            "records in {}",
            chapter.path.display()
        );
        for (id, record) in expected {
            check_record(record, actual[id]);
        }
    }

    let print = pages
        .get(&output.join("print.html"))
        .expect("combined print page");
    let actual = print.records();
    let expected: BTreeSet<_> = inventory.records().map(Record::id).collect();
    assert_eq!(
        actual.keys().copied().collect::<BTreeSet<_>>(),
        expected,
        "print inventory"
    );
    for record in inventory.records() {
        check_record(record, actual[record.id()]);
    }
    for link in print.html.select(&selector("a.ab-record-link")) {
        let href = link.attr("href").expect("record link destination");
        let id = href
            .strip_prefix('#')
            .unwrap_or_else(|| panic!("print record link leaves view: {href}"));
        assert!(expected.contains(id), "print link target {href}");
    }
    let links = check_links(&pages);
    println!(
        "Rendered acceptance passed: {} complete records, unique print targets, {links} local links.",
        inventory.len()
    );
}
