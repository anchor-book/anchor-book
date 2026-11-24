//! Common contract retrieval and configuration-selected phase guidance.
use std::{collections::BTreeSet, fs, path::Path, process::Command};

#[allow(dead_code)]
#[path = "support/checking.rs"]
mod fixture;

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

/// anchorbook: A-project-agent-bootstrap#contract-retrieval
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
fn contract_rejects_invalid_arguments_without_partial_output() {
    let dir = temp("arguments");
    for args in [
        vec!["contract", "--config"],
        vec!["contract", "--config", ""],
        vec!["contract", "--phase", "port"],
        vec!["contract", "unexpected"],
        vec!["contract", "--config", "a", "--config", "b"],
    ] {
        let output = run_in(&dir, &args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Usage: anchorbook contract"));
    }
    fs::remove_dir_all(dir).unwrap();
}

/// anchorbook: A-project-agent-bootstrap#phase-guidance
#[test]
fn configured_phase_selects_port_guidance_and_preserves_project_files() {
    use anchor_book::configuration::Phase;
    let book = fixture::Book::new();
    fs::create_dir(book.path("nested")).unwrap();
    book.write(
        "AGENTS.md",
        "Project-specific instructions must stay in the Book.\n",
    );
    for (name, phase) in [
        ("port", Phase::Port),
        ("prototyping", Phase::Prototyping),
        ("building", Phase::Building),
        ("complete", Phase::Complete),
    ] {
        book.write(
            "anchorbook.toml",
            &fixture::CONFIG.replace("prototyping", name),
        );
        let before = book.snapshot();
        let configuration = anchor_book::configuration::load(&book.config()).unwrap();
        assert_eq!(configuration.phase(), phase);
        for (directory, argument) in [
            (book.root.clone(), "anchorbook.toml"),
            (book.path("nested"), "../anchorbook.toml"),
        ] {
            let output = run_in(&directory, &["contract", "--config", argument]);
            assert!(output.status.success(), "{:?}", output.stderr);
            assert!(output.stderr.is_empty());
            let text = String::from_utf8(output.stdout).unwrap();
            assert!(text.starts_with(anchor_book::contract::text()));
            assert_eq!(
                text.matches("## Work in the port phase\n").count(),
                usize::from(name == "port")
            );
            assert!(!text.contains("Project-specific instructions must stay in the Book."));
            assert_eq!(text, anchor_book::contract::for_phase(phase));
            if name != "port" {
                assert_eq!(text, anchor_book::contract::text());
            }
            let check = run_in(&directory, &["check", "--config", argument]);
            assert!(check.status.success(), "{:?}", check.stderr);
        }
        assert_eq!(before, book.snapshot());
    }
}

/// anchorbook: A-project-agent-bootstrap#phase-guidance
#[test]
fn configured_contract_needs_valid_configuration_but_no_valid_book() {
    let book = fixture::Book::new();
    let config = fixture::CONFIG.replace("prototyping", "port");
    book.write("anchorbook.toml", &config);
    fs::remove_dir_all(book.path("docs")).unwrap();
    fs::remove_file(book.path("book.toml")).unwrap();
    let before = book.snapshot();
    let output = run_in(&book.root, &["contract", "--config", "anchorbook.toml"]);
    assert!(output.status.success(), "{:?}", output.stderr);
    assert!(output.stderr.is_empty());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("## Work in the port phase\n")
    );
    let check = run_in(&book.root, &["check", "--config", "anchorbook.toml"]);
    assert_eq!(
        check.status.code(),
        Some(1),
        "port preserves structural checks"
    );
    assert_eq!(before, book.snapshot());

    for invalid in [
        "this is not toml [".to_owned(),
        config.replace("\"port\"", "\"porting\""),
        config.replace("phase = \"port\"", ""),
        config.replace("\"generated/site\"", "\"docs/output\""),
    ] {
        book.write("anchorbook.toml", &invalid);
        let before = book.snapshot();
        let output = run_in(&book.root, &["contract", "--config", "anchorbook.toml"]);
        assert_eq!(output.status.code(), Some(2));
        assert!(
            output.stdout.is_empty(),
            "no fallback to common text on an error"
        );
        assert!(!output.stderr.is_empty());
        let common = run_in(&book.root, &["contract"]);
        assert!(common.status.success());
        assert_eq!(common.stdout, anchor_book::contract::text().as_bytes());
        assert_eq!(before, book.snapshot());
    }
    let missing = run_in(&book.root, &["contract", "--config", "missing.toml"]);
    assert_eq!(missing.status.code(), Some(2));
    assert!(missing.stdout.is_empty());
    assert!(!missing.stderr.is_empty());
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
        .chain(std::iter::once("port.md".to_owned()))
        .collect();
    assert_eq!(
        listed.len(),
        anchor_book::contract::SECTIONS.len() + 1,
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
    let port = fs::read_to_string(dir.join("port.md")).unwrap();
    assert_eq!(port, anchor_book::contract::PORT);
    assert!(port.starts_with("## "));
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
    check_links(anchor_book::contract::text());
    check_links(&anchor_book::contract::for_phase(
        anchor_book::configuration::Phase::Port,
    ));
}

fn check_links(text: &str) {
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
