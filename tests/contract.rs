//! `anchorbook contract` prints the packaged contract, the same for every project.
use std::{collections::BTreeSet, fs, path::Path, process::Command};

fn run_in(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn temp(name: &str) -> std::path::PathBuf {
    let dir =
        std::env::temp_dir().join(format!("anchorbook-contract-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// anchorbook: project-agent-bootstrap#contract-retrieval
#[test]
fn contract_output_is_the_packaged_text_for_every_project() {
    let empty = temp("empty");
    let other = temp("other");
    // A project with a broken configuration and its own guidance: neither is read.
    fs::write(other.join("anchorbook.toml"), "this is not toml [").unwrap();
    fs::write(other.join("AGENTS.md"), "Project-specific instructions.\n").unwrap();
    let first = run_in(&empty, &["contract"]);
    let second = run_in(&other, &["contract"]);
    for output in [&first, &second] {
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
    }
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(first.stdout, anchor_book::contract::text().as_bytes());
    let text = String::from_utf8(first.stdout).unwrap();
    assert!(text.starts_with("# AI Working Contract\n"));
    assert!(!text.contains("Project-specific instructions."));
    for dir in [empty, other] {
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn contract_takes_no_arguments() {
    let dir = temp("arguments");
    let output = run_in(&dir, &["contract", "--config", "anchorbook.toml"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Usage: anchorbook contract"));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn every_section_file_is_joined_once_in_order() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("contract");
    let on_disk: BTreeSet<String> = fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name != "README.md")
        .collect();
    let listed: BTreeSet<String> = anchor_book::contract::SECTIONS
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    assert_eq!(
        listed.len(),
        anchor_book::contract::SECTIONS.len(),
        "no file listed twice"
    );
    assert_eq!(
        on_disk, listed,
        "every section file, and only those, is joined"
    );
    let joined: String = anchor_book::contract::SECTIONS
        .iter()
        .map(|name| fs::read_to_string(dir.join(name)).unwrap())
        .collect();
    assert_eq!(joined, anchor_book::contract::TEXT);
    for (index, name) in anchor_book::contract::SECTIONS.iter().enumerate() {
        let text = fs::read_to_string(dir.join(name)).unwrap();
        let expected = if index == 0 {
            "# AI Working Contract\n"
        } else {
            "## "
        };
        assert!(
            text.starts_with(expected),
            "{name} starts with a whole section"
        );
    }
}

#[test]
fn links_within_the_contract_name_its_own_sections() {
    let text = anchor_book::contract::text();
    let fragments: BTreeSet<String> = text
        .lines()
        .filter_map(|line| {
            line.strip_prefix("## ")
                .or_else(|| line.strip_prefix("### "))
        })
        .map(|heading| {
            heading
                .to_lowercase()
                .chars()
                .filter_map(|c| match c {
                    'a'..='z' | '0'..='9' | '-' => Some(c),
                    ' ' => Some('-'),
                    _ => None,
                })
                .collect()
        })
        .collect();
    let mut rest = text;
    let mut links = 0;
    while let Some(start) = rest.find("](#") {
        let after = &rest[start + 3..];
        let end = after.find(')').unwrap();
        assert!(
            fragments.contains(&after[..end]),
            "unresolved #{}",
            &after[..end]
        );
        links += 1;
        rest = &after[end..];
    }
    assert!(links > 0);
    assert!(
        !text.contains("](../") && !text.contains(".md)"),
        "no links into a Book"
    );
}
