//! Evaluate observations without filesystem, process, or network access.
use super::{
    CheckReport, Finding, Group, Kind, Location,
    inspect::{Chapter, Input, Setting, Target},
};
use crate::configuration::{Configuration, SectionRole};
use std::{io, path::Path};

fn operation(path: &Path, group: Group, error: io::Error) -> Finding {
    let mut finding = Finding::new(
        group,
        Kind::Operation,
        "operation.read-failed",
        "Path could not be inspected",
        Location::file(path),
    )
    .details(
        "an accessible path whose identity and type can be observed",
        error.to_string(),
        "Resolve the access or path problem, then rerun the check.",
    );
    finding.cause = Some(Box::new(error));
    finding
}

pub(super) fn renderer(
    config: &Configuration,
    input: Input<Vec<Setting>>,
    report: &mut CheckReport,
) {
    report.pass(Group::Renderer);
    match input.value {
        Err(error) => report.push(error.finding(Group::Renderer)),
        Ok(None) => report.push(
            Finding::new(
                Group::Renderer,
                Kind::Structure,
                "structure.renderer-missing",
                "mdBook configuration is missing",
                Location::file(&input.path),
            )
            .details(
                "book.toml beside anchorbook.toml",
                "file is missing",
                "Restore the intended mdBook configuration, then rerun the check.",
            ),
        ),
        Ok(Some(settings)) => {
            for setting in settings {
                match setting.resolved {
                    Ok(observed) if observed == setting.expected => {}
                    Ok(observed) => {
                        let base = config.path().parent().unwrap_or(config.path());
                        let mut finding=Finding::new(Group::Renderer,Kind::Structure,"structure.renderer-path-mismatch","mdBook path differs from the intended Book layout",Location::key(&input.path,setting.field)).details(
                        display_path(&setting.expected,base),display_path(&observed,base),"Align book.toml with the intended Anchor Book configuration; if the intended layout changed, reconcile that design first.");
                        finding
                            .related
                            .push(Location::key(config.path(), setting.anchor_field));
                        report.push(finding);
                    }
                    Err(error) => {
                        let mut finding = operation(&input.path, Group::Renderer, error);
                        finding.location.field = Some(setting.field.into());
                        report.push(finding);
                    }
                }
            }
        }
    }
}

pub(super) fn navigation(
    config: &Configuration,
    input: Input<Vec<Chapter>>,
    report: &mut CheckReport,
) {
    report.pass(Group::Summary);
    let entries = match input.value {
        Err(error) => {
            let finding = error.finding(Group::Summary);
            let code = finding.code;
            report.push(finding);
            block_navigation(report, code);
            return;
        }
        Ok(None) => {
            report.push(
                Finding::new(
                    Group::Summary,
                    Kind::Structure,
                    "structure.summary-missing",
                    "Book navigation is missing",
                    Location::file(&input.path),
                )
                .details(
                    "SUMMARY.md in the configured source directory",
                    "file is missing",
                    "Restore the intended Book navigation, then rerun the check.",
                ),
            );
            block_navigation(report, "structure.summary-missing");
            return;
        }
        Ok(Some(entries)) => entries,
    };
    report.pass(Group::Chapters);
    report.pass(Group::Roles);
    let mut local = Vec::new();
    let mut unknown_targets = false;
    for entry in entries {
        match entry.target {
            Target::Draft | Target::External => {}
            Target::Unresolved { path, error } => {
                let (kind, code) = match error.kind() {
                    io::ErrorKind::InvalidInput => (Kind::Structure, "structure.chapter-invalid"),
                    io::ErrorKind::NotFound => (Kind::Structure, "structure.chapter-missing"),
                    _ => {
                        unknown_targets = true;
                        (Kind::Operation, "operation.read-failed")
                    }
                };
                let mut finding=Finding::new(Group::Chapters,kind,code,"Chapter target cannot be resolved",Location::file(path)).details(
                    "an existing chapter file within the source directory",error.to_string(),"Restore or correct the intended chapter target while preserving required section mappings.");
                finding.related.push(entry.location);
                finding.cause = Some(Box::new(error));
                report.push(finding);
            }
            Target::Local { path, metadata } => {
                let finding=match metadata {
                    Ok(meta) if meta.is_file()=>None,
                    Ok(_)=>Some(Finding::new(Group::Chapters,Kind::Structure,"structure.chapter-invalid","Chapter target is not a file",Location::file(&path)).details(
                        "an existing chapter file","target exists but is not a regular file","Correct the navigation target or restore the intended chapter file.")),
                    Err(error) if error.kind()==io::ErrorKind::NotFound=>Some(Finding::new(Group::Chapters,Kind::Structure,"structure.chapter-missing","Chapter file is missing",Location::file(&path)).details(
                        "an existing chapter file within the source directory","file is missing","Restore the intended chapter or correct its target if moved; preserve required section mappings.")),
                    Err(error)=>Some(operation(&path,Group::Chapters,error)),
                };
                if let Some(mut finding) = finding {
                    finding.related.push(entry.location.clone());
                    report.push(finding);
                }
                local.push((path, entry.depth, entry.location));
            }
        }
    }
    // Unresolvable identities cannot establish that a required role is absent.
    if unknown_targets
        && SectionRole::ALL.iter().any(|role| {
            !local
                .iter()
                .any(|(path, depth, _)| path == config.section(*role) && *depth == 0)
        })
    {
        report.block(Group::Roles, "operation.read-failed");
        return;
    }
    for role in SectionRole::ALL {
        let matches: Vec<_> = local
            .iter()
            .filter(|(target, _, _)| target == config.section(role))
            .collect();
        if matches.iter().any(|(_, depth, _)| *depth == 0) {
            continue;
        }
        let (code, location, found) = if let Some((_, _, location)) = matches.first() {
            (
                "structure.role-not-top-level",
                location.clone(),
                "configured chapter appears only below the top level",
            )
        } else {
            (
                "structure.role-missing",
                Location::file(&input.path),
                "no local chapter entry matches the configured role",
            )
        };
        let mut finding=Finding::new(Group::Roles,Kind::Structure,code,"Required section is not at its configured top-level location",location).details(
            format!("{} at {} as a top-level linked chapter",role.as_str(),display_path(config.section(role),config.source())),found,
            "Restore the required navigation entry at the top level, preserving its intended content and children.");
        finding.related.push(Location::key(
            config.path(),
            &format!("book.sections.{}", role.as_str()),
        ));
        report.push(finding);
    }
}
fn block_navigation(report: &mut CheckReport, reason: &'static str) {
    for group in [Group::Chapters, Group::Roles] {
        report.block(group, reason);
    }
}
fn display_path(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
#[path = "../../tests/support/checking.rs"]
mod fixture;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checking::{Outcome, State};

    fn observed_roles(config: &Configuration) -> Vec<Chapter> {
        [
            SectionRole::Design,
            SectionRole::Authority,
            SectionRole::ImplementationRoadmaps,
        ]
        .into_iter()
        .map(|role| Chapter {
            location: Location::file(config.source().join("SUMMARY.md")),
            depth: 0,
            target: Target::Local {
                path: config.section(role).to_owned(),
                metadata: std::fs::metadata(config.section(role)),
            },
        })
        .collect()
    }

    fn evaluate(config: &Configuration, chapters: Vec<Chapter>) -> CheckReport {
        let mut report = CheckReport::new(config.path());
        for group in Group::ALL {
            report.pass(group);
        }
        navigation(
            config,
            Input {
                path: config.source().join("SUMMARY.md"),
                value: Ok(Some(chapters)),
            },
            &mut report,
        );
        report
    }

    fn group_state(report: &CheckReport, group: Group) -> State {
        report
            .groups()
            .iter()
            .find(|g| g.group == group)
            .unwrap()
            .state
    }

    #[test]
    fn metadata_failures_after_resolution_are_not_conflated_with_missing_files() {
        let book = fixture::Book::new();
        let config = crate::configuration::load(&book.config()).unwrap();
        let before = book.snapshot();
        // Supply observations directly: a target can become inaccessible after
        // resolution. No filesystem race or privileged permission test is needed.
        for (kind, expected_code, outcome, state) in [
            (
                io::ErrorKind::NotFound,
                "structure.chapter-missing",
                Outcome::Failed,
                State::Failed,
            ),
            (
                io::ErrorKind::PermissionDenied,
                "operation.read-failed",
                Outcome::Incomplete,
                State::Error,
            ),
            (
                io::ErrorKind::Interrupted,
                "operation.read-failed",
                Outcome::Incomplete,
                State::Error,
            ),
        ] {
            let mut chapters = observed_roles(&config);
            chapters[1].target = Target::Local {
                path: config.section(SectionRole::Authority).to_owned(),
                metadata: Err(io::Error::new(kind, "metadata inspection failed")),
            };
            let report = evaluate(&config, chapters);
            assert_eq!(report.outcome(), outcome);
            assert_eq!(group_state(&report, Group::Chapters), state);
            assert_eq!(group_state(&report, Group::Roles), State::Passed);
            assert_eq!(report.findings().len(), 1);
            let finding = &report.findings()[0];
            assert_eq!(finding.code, expected_code);
            assert_eq!(
                finding.location.path,
                config.section(SectionRole::Authority)
            );
            if kind != io::ErrorKind::NotFound {
                assert_eq!(finding.kind, Kind::Operation);
                assert_eq!(
                    finding
                        .cause()
                        .unwrap()
                        .downcast_ref::<io::Error>()
                        .unwrap()
                        .kind(),
                    kind
                );
                assert_eq!(finding.found, "metadata inspection failed");
            }
        }
        assert_eq!(book.snapshot(), before);
    }

    #[test]
    fn unknown_targets_only_block_role_conclusions_that_are_not_already_established() {
        let book = fixture::Book::new();
        let config = crate::configuration::load(&book.config()).unwrap();
        // A role present only below the top level is also unresolved here: the
        // unknown target could supply its top-level entry.
        for (missing_authority, nested_authority, expected) in [
            (false, false, State::Passed),
            (true, false, State::Blocked),
            (false, true, State::Blocked),
        ] {
            let mut chapters = observed_roles(&config);
            if missing_authority {
                chapters.remove(1);
            } else if nested_authority {
                chapters[1].depth = 1;
            }
            chapters.push(Chapter {
                location: Location::file(config.source().join("SUMMARY.md")),
                depth: 0,
                target: Target::Unresolved {
                    path: config.source().join("unknown.md"),
                    error: io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "target identity unavailable",
                    ),
                },
            });
            let report = evaluate(&config, chapters);
            assert_eq!(report.outcome(), Outcome::Incomplete);
            assert_eq!(group_state(&report, Group::Chapters), State::Error);
            assert_eq!(group_state(&report, Group::Roles), expected);
            assert_eq!(
                report.findings().iter().map(|f| f.code).collect::<Vec<_>>(),
                ["operation.read-failed"]
            );
            let role_result = report
                .groups()
                .iter()
                .find(|g| g.group == Group::Roles)
                .unwrap();
            assert_eq!(
                role_result.blocked_by,
                if expected == State::Blocked {
                    Some("operation.read-failed")
                } else {
                    None
                }
            );
        }
    }
}
