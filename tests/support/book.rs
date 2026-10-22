//! Runtime-only acquisition for this repository's Book acceptance tests.
use anchor_book::Chapter;
use pulldown_cmark::{Event, Parser, Tag};
use std::{fs, path::PathBuf};

pub fn load() -> Vec<(PathBuf, String)> {
    let root = std::env::var_os("ANCHORBOOK_TEST_BOOK")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("book"));
    let summary = fs::read_to_string(root.join("SUMMARY.md")).expect("read Book summary");
    Parser::new(&summary)
        .filter_map(|event| match event {
            Event::Start(Tag::Link { dest_url, .. }) => {
                let path = PathBuf::from(dest_url.as_ref());
                let text = fs::read_to_string(root.join(&path))
                    .unwrap_or_else(|error| panic!("read chapter {}: {error}", path.display()));
                Some((path, text))
            }
            _ => None,
        })
        .collect()
}

pub fn chapters(sources: &[(PathBuf, String)]) -> Vec<Chapter<'_>> {
    sources
        .iter()
        .map(|(path, text)| Chapter::new(path, text))
        .collect()
}
