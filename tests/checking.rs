//! Independent structural fixtures, actual-process CLI checks, and repairs.
#[path = "support/checking.rs"]
mod fixture;
use anchor_book::{
    adapters::check::format_report,
    checking::{CheckReport, Group, Kind, Outcome, State, check},
};
use fixture::{Book, CONFIG, RENDERER, SUMMARY};
use std::{
    error::Error as _,
    fs,
    process::{Command, Output},
};

fn report(book: &Book) -> CheckReport {
    check(&book.config()).unwrap_or_else(|e| e.into_report())
}
fn run(book: &Book) -> Output {
    Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .args(["check", "--config"])
        .arg(book.config())
        .output()
        .unwrap()
}
fn state(report: &CheckReport, group: Group) -> State {
    report
        .groups()
        .iter()
        .find(|g| g.group == group)
        .unwrap()
        .state
}
fn codes(report: &CheckReport) -> Vec<&'static str> {
    report.findings().iter().map(|f| f.code).collect()
}

#[test]
fn real_cli_and_library_agree_for_pass_failure_and_incomplete_with_preserved_files() {
    let book = Book::new();
    for (summary, expected) in [
        (SUMMARY.to_owned(), Outcome::Passed),
        (
            format!("{SUMMARY}\n- [Missing](missing.md)\n"),
            Outcome::Failed,
        ),
        (format!("{SUMMARY}\nordinary prose\n"), Outcome::Incomplete),
    ] {
        book.write("docs/SUMMARY.md", &summary);
        let before = book.snapshot();
        let r = report(&book);
        let result = run(&book);
        assert_eq!(r.outcome(), expected);
        assert_eq!(result.status.code(), Some(i32::from(expected.exit_code())));
        let (written, empty) = if expected == Outcome::Incomplete {
            (&result.stderr, &result.stdout)
        } else {
            (&result.stdout, &result.stderr)
        };
        assert!(empty.is_empty());
        assert_eq!(String::from_utf8_lossy(written), format_report(&r));
        assert_eq!(book.snapshot(), before);
        assert!(!book.path("generated").exists());
        assert!(format_report(&r).contains("files changed by check: none"));
    }
}

#[test]
fn missing_and_non_file_chapters_are_located_and_correctable() {
    let book = Book::new();
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n[Prefix](missing.md)\n"),
    );
    let r = report(&book);
    assert_eq!(codes(&r), ["structure.chapter-missing"]);
    let f = &r.findings()[0];
    assert_eq!(f.location.path, book.path("docs/missing.md"));
    assert_eq!(f.location.position, None);
    assert_eq!(f.related[0].position, Some((7, 1)));
    assert_eq!(f.kind, Kind::Structure);
    assert!(!f.expected.is_empty());
    assert!(!f.help.is_empty());
    book.write("docs/missing.md", "# Restored");
    assert_eq!(report(&book).outcome(), Outcome::Passed);
    fs::remove_file(book.path("docs/missing.md")).unwrap();
    fs::create_dir(book.path("docs/missing.md")).unwrap();
    assert_eq!(codes(&report(&book)), ["structure.chapter-invalid"]);
    fs::remove_dir(book.path("docs/missing.md")).unwrap();
    book.write("docs/SUMMARY.md", SUMMARY);
    assert_eq!(report(&book).outcome(), Outcome::Passed);
}

#[test]
fn required_roles_must_be_local_top_level_entries_not_titles_drafts_or_external_links() {
    let book = Book::new();
    for (replacement, code) in [
        (
            "  - [Authority](authority.md)",
            "structure.role-not-top-level",
        ),
        ("# Authority", "structure.role-missing"),
        ("---", "structure.role-missing"),
        ("- [Authority]()", "structure.role-missing"),
        (
            "- [Authority](https://example.invalid/authority.md)",
            "structure.role-missing",
        ),
        ("", "structure.role-missing"),
    ] {
        book.write(
            "docs/SUMMARY.md",
            &SUMMARY.replace("- [Authority](authority.md)", replacement),
        );
        let r = report(&book);
        assert_eq!(
            r.outcome(),
            Outcome::Failed,
            "{replacement}: {}",
            format_report(&r)
        );
        assert_eq!(codes(&r), [code]);
        let f = &r.findings()[0];
        assert_eq!(
            f.related[0].field.as_deref(),
            Some("book.sections.authority")
        );
        assert!(f.expected.contains("authority"));
        book.write("docs/SUMMARY.md", SUMMARY);
        assert_eq!(report(&book).outcome(), Outcome::Passed);
    }
}

#[test]
fn missing_prerequisites_block_dependents_without_cascaded_missing_roles() {
    let book = Book::new();
    fs::remove_file(book.path("docs/SUMMARY.md")).unwrap();
    let r = report(&book);
    assert_eq!(codes(&r), ["structure.summary-missing"]);
    assert_eq!(r.outcome(), Outcome::Failed);
    assert_eq!(state(&r, Group::Summary), State::Failed);
    assert_eq!(state(&r, Group::Chapters), State::Blocked);
    assert_eq!(state(&r, Group::Roles), State::Blocked);
    assert_eq!(
        r.groups()
            .iter()
            .find(|g| g.group == Group::Roles)
            .unwrap()
            .blocked_by,
        Some("structure.summary-missing")
    );
    fs::remove_dir_all(book.path("docs")).unwrap();
    let r = report(&book);
    assert_eq!(codes(&r), ["structure.source-invalid"]);
    assert_eq!(state(&r, Group::Summary), State::Blocked);
    assert_eq!(state(&r, Group::Renderer), State::Passed);
    fs::write(book.path("docs"), "not a directory").unwrap();
    assert_eq!(codes(&report(&book)), ["structure.source-invalid"]);
}

#[test]
fn malformed_and_ignored_navigation_is_incomplete_while_empty_navigation_is_structurally_missing_roles()
 {
    let book = Book::new();
    for source in [
        "plain text",
        "## Design\n",
        "```md\n- [Ignored](ignored.md)\n```",
        "- [Broken](broken.md",
        "- [One](one.md)\n- [Two](one.md)",
        "- [One](one.md) [Discarded](two.md)",
    ] {
        book.write("docs/SUMMARY.md", source);
        let error = check(&book.config()).unwrap_err();
        let r = error.report();
        assert_eq!(r.outcome(), Outcome::Incomplete);
        assert_eq!(state(r, Group::Summary), State::Error);
        assert_eq!(state(r, Group::Chapters), State::Blocked);
        assert_eq!(state(r, Group::Roles), State::Blocked);
        assert_eq!(r.findings().len(), 1);
        assert_eq!(r.findings()[0].location.path, book.path("docs/SUMMARY.md"));
        if r.findings()[0].code == "input.parse-failed" {
            assert!(error.source().is_some());
        }
    }
    book.write("docs/SUMMARY.md", "");
    let r = report(&book);
    assert_eq!(r.outcome(), Outcome::Failed);
    assert_eq!(codes(&r), ["structure.role-missing"; 3]);
}

#[test]
fn renderer_alignment_uses_defaults_resolved_paths_and_file_settings_only() {
    let book = Book::new();
    book.write(
        "book.toml",
        "[book]\nsrc=\"./docs\"\n[build]\nbuild-dir=\"generated/unused/../site\"\n",
    );
    assert_eq!(report(&book).outcome(), Outcome::Passed);
    book.write("book.toml", "");
    let r = report(&book);
    assert_eq!(codes(&r), ["structure.renderer-path-mismatch"; 2]);
    assert_eq!(r.findings()[0].location.field.as_deref(), Some("book.src"));
    assert_eq!(r.findings()[0].expected, "docs");
    assert_eq!(r.findings()[0].found, "src");
    book.write("book.toml", RENDERER);
    let result = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .args(["check", "--config"])
        .arg(book.config())
        .env("MDBOOK_BOOK__SRC", "wrong")
        .output()
        .unwrap();
    assert!(result.status.success());
    fs::remove_file(book.path("book.toml")).unwrap();
    assert_eq!(codes(&report(&book)), ["structure.renderer-missing"]);
    book.write("book.toml", "[book\n");
    let error = check(&book.config()).unwrap_err();
    assert_eq!(codes(error.report()), ["input.parse-failed"]);
    assert!(error.source().is_some());
    book.write(
        "book.toml",
        &format!(
            "[book]\nsrc={:?}\n[build]\nbuild-dir={:?}\n",
            book.path("docs"),
            book.path("generated/site")
        ),
    );
    assert_eq!(report(&book).outcome(), Outcome::Passed);
}

#[test]
fn invalid_configuration_and_mixed_failures_never_pass_and_preserve_causes() {
    let book = Book::new();
    book.write(
        "anchorbook.toml",
        &CONFIG.replace("schema-version = 1", "schema-version = \"one\""),
    );
    let error = check(&book.config()).unwrap_err();
    let r = error.report();
    assert_eq!(state(r, Group::Configuration), State::Error);
    assert_eq!(r.findings()[0].kind, Kind::Configuration);
    assert!(r.groups()[1..].iter().all(|g| g.state == State::Blocked));
    assert!(error.source().is_some());
    book.write("anchorbook.toml", CONFIG);
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n- [Missing](missing.md)"),
    );
    book.write("book.toml", "[book");
    let r = report(&book);
    assert_eq!(r.outcome(), Outcome::Incomplete);
    assert_eq!(
        codes(&r),
        ["input.parse-failed", "structure.chapter-missing"]
    );
    assert!(format_report(&r).contains("findings are partial"));
    fs::remove_file(book.config()).unwrap();
    let error = check(&book.config()).unwrap_err();
    assert_eq!(codes(error.report()), ["operation.read-failed"]);
    assert!(error.source().is_some());
}

#[test]
fn external_links_drafts_and_configured_commands_do_not_execute_or_create_anything() {
    let book = Book::new();
    book.write("docs/SUMMARY.md",&format!("{SUMMARY}\n- [Draft]()\n- [External](http://127.0.0.1:1/never-fetch)\n- [Mail](mailto:user@example.invalid)\n"));
    book.write("book.toml",&format!("{RENDERER}\n[preprocessor.trap]\ncommand = \"touch should-not-exist\"\n[output.trap]\ncommand = \"touch also-should-not-exist\"\n"));
    let before = book.snapshot();
    assert_eq!(report(&book).outcome(), Outcome::Passed);
    assert!(run(&book).status.success());
    assert_eq!(book.snapshot(), before);
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n- [Missing](not-created.md)"),
    );
    let before = book.snapshot();
    assert_eq!(report(&book).outcome(), Outcome::Failed);
    assert_eq!(book.snapshot(), before);
}

#[test]
fn alternative_layouts_and_invocation_directories_keep_the_same_rules() {
    for layout in ["book", "md", "other-name"] {
        let book = Book::new();
        fs::rename(book.path("docs"), book.path(layout)).unwrap();
        book.write(
            "anchorbook.toml",
            &CONFIG.replace("\"docs\"", &format!("\"{layout}\"")),
        );
        book.write(
            "book.toml",
            &RENDERER.replace("\"docs\"", &format!("\"{layout}\"")),
        );
        book.write("caller/keep", "unchanged");
        for (cwd, arg) in [
            (book.root.clone(), "anchorbook.toml"),
            (book.path("caller"), "../anchorbook.toml"),
        ] {
            let result = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
                .args(["check", "--config", arg])
                .current_dir(cwd)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert!(String::from_utf8_lossy(&result.stdout).contains("result: passed"));
        }
    }
}

#[test]
fn invalid_check_invocations_only_write_stderr_and_exit_two() {
    for args in [
        vec!["check"],
        vec!["check", "--config"],
        vec!["check", "wrong"],
        vec!["check", "--config", ""],
        vec!["check", "--config", "a", "extra"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("Usage: anchorbook check --config PATH")
        );
    }
}

#[test]
fn passing_report_has_the_documented_scope_and_complete_group_results() {
    let book = Book::new();
    let output = run(&book);
    let expected = format!(
        "anchorbook check: passed\nconfig: {}\npaths: relative to the configuration directory\nscope: structural/v1\n\nchecks: configuration passed; source passed; summary passed; renderer passed; chapters passed; roles passed;\nresult: passed (0 errors)\nnot evaluated: anchor semantics, implementation, verification evidence, roadmap completion, or links within chapter prose\nfiles changed by check: none\n",
        book.config().display()
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n- [Missing](missing.md)"),
    );
    let text = String::from_utf8(run(&book).stdout).unwrap();
    for field in [
        "error[structure.chapter-missing] (structure): Chapter file is missing",
        "  at: docs/missing.md",
        "  declared by: docs/SUMMARY.md:7:3",
        "  expected: an existing chapter file within the source directory",
        "  found: file is missing",
        "  help: Restore the intended chapter",
        "chapters failed",
        "result: failed (1 errors)",
    ] {
        assert!(text.contains(field), "missing {field}: {text}");
    }
}

#[test]
fn every_role_and_chapter_depth_is_checked_and_display_titles_do_not_define_roles() {
    let book = Book::new();
    book.write("docs/container.md", "# Container");
    for (line, role) in [
        ("- [Design](design.md)", "design"),
        ("- [Authority](authority.md)", "authority"),
        ("- [Work](roadmap.md)", "implementation-roadmaps"),
    ] {
        for (replacement, code) in [
            (String::new(), "structure.role-missing"),
            (
                format!("- [Container](container.md)\n  {line}"),
                "structure.role-not-top-level",
            ),
        ] {
            book.write("docs/SUMMARY.md", &SUMMARY.replace(line, &replacement));
            let r = report(&book);
            assert_eq!(codes(&r), [code]);
            assert_eq!(
                r.findings()[0].related[0].field.as_deref(),
                Some(format!("book.sections.{role}").as_str())
            );
        }
    }
    for summary in [
        format!("[Unnumbered](missing.md)\n\n{SUMMARY}"),
        SUMMARY.replace(
            "- [Design](design.md)",
            "- [Different title](design.md)\n  - [Nested](missing.md)",
        ),
    ] {
        book.write("docs/SUMMARY.md", &summary);
        assert_eq!(codes(&report(&book)), ["structure.chapter-missing"]);
    }
    book.write(
        "docs/SUMMARY.md",
        &SUMMARY
            .replace("Design", "随意标题")
            .replace("Authority", "Another title"),
    );
    assert_eq!(report(&book).outcome(), Outcome::Passed);
}

#[test]
fn unicode_reference_links_spaces_and_literal_fragments_keep_precise_locations() {
    let book = Book::new();
    let summary = format!("{SUMMARY}\n- [章节][ref]\n\n[ref]: extra%20chapter.md\n");
    book.write("docs/SUMMARY.md", &summary);
    let r = report(&book);
    assert_eq!(codes(&r), ["structure.chapter-missing"]);
    assert_eq!(r.findings()[0].related[0].position, Some((7, 3)));
    assert_eq!(
        r.findings()[0].location.path,
        book.path("docs/extra chapter.md")
    );
    book.write("docs/extra chapter.md", "# Extra");
    assert_eq!(report(&book).outcome(), Outcome::Passed);
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n- [Fragment](design.md#heading)\n"),
    );
    assert_eq!(codes(&report(&book)), ["structure.chapter-missing"]);
    book.write("docs/design.md#heading", "# Literal filename");
    assert_eq!(report(&book).outcome(), Outcome::Passed);
}

#[test]
fn findings_are_stably_sorted_and_control_characters_do_not_reach_terminal_output() {
    let book = Book::new();
    let mut orders = Vec::new();
    for links in ["- [Z](z.md)\n- [A](a.md)", "- [A](a.md)\n- [Z](z.md)"] {
        book.write("docs/SUMMARY.md", &format!("{SUMMARY}\n{links}"));
        let r = report(&book);
        orders.push(
            r.findings()
                .iter()
                .map(|f| f.location.path.clone())
                .collect::<Vec<_>>(),
        );
        assert_eq!(r.outcome(), Outcome::Failed);
    }
    assert_eq!(orders[0], orders[1]);
    assert_eq!(orders[0], [book.path("docs/a.md"), book.path("docs/z.md")]);
    book.write(
        "anchorbook.toml",
        &format!("\"bad\\u001bname\" = 1\n{CONFIG}"),
    );
    let output = run(&book);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.contains(&0x1b));
    assert!(String::from_utf8_lossy(&output.stderr).contains("\\u{1b}"));
}

#[cfg(unix)]
#[test]
fn chapter_escapes_symlinks_and_summary_aliases_cannot_hide_invalid_structure() {
    use std::os::unix::fs::symlink;
    let book = Book::new();
    let outside = Book::new();
    symlink(outside.path("docs/design.md"), book.path("docs/escape.md")).unwrap();
    symlink("absent.md", book.path("docs/dangling.md")).unwrap();
    symlink("loop.md", book.path("docs/loop.md")).unwrap();
    for (target, code, outcome) in [
        (
            "../anchorbook.toml",
            "structure.chapter-invalid",
            Outcome::Failed,
        ),
        ("/outside.md", "structure.chapter-invalid", Outcome::Failed),
        ("escape.md", "structure.chapter-invalid", Outcome::Failed),
        ("dangling.md", "structure.chapter-missing", Outcome::Failed),
        ("loop.md", "operation.read-failed", Outcome::Incomplete),
    ] {
        book.write(
            "docs/SUMMARY.md",
            &format!("{SUMMARY}\n- [Target]({target})"),
        );
        let r = report(&book);
        assert_eq!(r.outcome(), outcome);
        assert_eq!(codes(&r), [code]);
        // An unrelated unknown target does not invalidate already observed roles.
        assert_eq!(state(&r, Group::Roles), State::Passed);
    }
    book.write(
        "docs/SUMMARY.md",
        &format!(
            "{}\n- [Unknown](loop.md)",
            SUMMARY.replace("- [Authority](authority.md)", "")
        ),
    );
    let r = report(&book);
    assert_eq!(codes(&r), ["operation.read-failed"]);
    assert_eq!(state(&r, Group::Roles), State::Blocked);
    fs::remove_file(book.path("docs/SUMMARY.md")).unwrap();
    symlink(
        outside.path("docs/SUMMARY.md"),
        book.path("docs/SUMMARY.md"),
    )
    .unwrap();
    let r = report(&book);
    assert_eq!(r.outcome(), Outcome::Incomplete);
    assert_eq!(state(&r, Group::Summary), State::Error);
    assert_eq!(state(&r, Group::Roles), State::Blocked);
}

#[cfg(unix)]
#[test]
fn access_failure_is_incomplete_and_keeps_known_structural_findings() {
    use std::os::unix::fs::PermissionsExt;
    struct Restore(std::path::PathBuf, fs::Permissions);
    impl Drop for Restore {
        fn drop(&mut self) {
            fs::set_permissions(&self.0, self.1.clone()).unwrap();
        }
    }
    let book = Book::new();
    book.write("docs/locked/child.md", "# Locked fixture");
    let locked = book.path("docs/locked");
    let _restore = Restore(locked.clone(), fs::metadata(&locked).unwrap().permissions());
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
    if fs::read_dir(&locked).is_ok() {
        return;
    }
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n- [Locked](locked/child.md)\n- [Missing](missing.md)"),
    );
    let r = report(&book);
    assert_eq!(r.outcome(), Outcome::Incomplete);
    assert!(codes(&r).contains(&"operation.read-failed"));
    assert!(codes(&r).contains(&"structure.chapter-missing"));
    let error = check(&book.config()).unwrap_err();
    assert!(error.source().is_some());
}

#[test]
fn unreadable_input_bytes_and_non_file_inputs_are_not_reported_as_missing() {
    for input in ["docs/SUMMARY.md", "book.toml"] {
        let book = Book::new();
        fs::write(book.path(input), [0xff]).unwrap();
        let error = check(&book.config()).unwrap_err();
        assert_eq!(error.report().outcome(), Outcome::Incomplete);
        assert!(codes(error.report()).contains(&"operation.read-failed"));
        assert!(error.source().is_some());
        fs::remove_file(book.path(input)).unwrap();
        fs::create_dir(book.path(input)).unwrap();
        let r = report(&book);
        assert_eq!(r.outcome(), Outcome::Incomplete);
        assert!(codes(&r).contains(&"operation.read-failed"));
    }
}

#[test]
fn phase_changes_do_not_skip_structural_rules_or_imply_anchor_verification() {
    let book = Book::new();
    for phase in ["prototyping", "building", "complete"] {
        book.write("anchorbook.toml", &CONFIG.replace("prototyping", phase));
        book.write("docs/SUMMARY.md", SUMMARY);
        book.write(
            "docs/design.md",
            "```anchorbook-anchor\ninvalid record content\n```\n",
        );
        assert_eq!(report(&book).outcome(), Outcome::Passed);
        book.write(
            "docs/SUMMARY.md",
            &SUMMARY.replace("- [Authority](authority.md)", ""),
        );
        assert_eq!(codes(&report(&book)), ["structure.role-missing"]);
    }
}

#[test]
fn inline_code_outside_navigation_is_incomplete_but_link_label_code_is_supported() {
    let book = Book::new();
    book.write(
        "docs/SUMMARY.md",
        &SUMMARY.replace("[Design]", "[`Design`]"),
    );
    assert_eq!(report(&book).outcome(), Outcome::Passed);
    book.write(
        "docs/SUMMARY.md",
        &format!("{SUMMARY}\n`ignored by mdBook`\n"),
    );
    let r = report(&book);
    assert_eq!(r.outcome(), Outcome::Incomplete);
    assert_eq!(codes(&r), ["input.unsupported"]);
    assert_eq!(state(&r, Group::Summary), State::Error);
    assert_eq!(state(&r, Group::Chapters), State::Blocked);
    assert_eq!(state(&r, Group::Roles), State::Blocked);
    assert_eq!(r.findings()[0].location.position, Some((7, 1)));
    let output = run(&book);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("input.unsupported"));
}
