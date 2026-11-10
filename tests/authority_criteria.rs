//! Behavioral criteria of `records-valid-authority`, exercised through the record
//! API with fixtures built from the criteria rather than from engine output.
use std::path::Path;

use anchor_book::records::{Chapter, DiagnosticCode, Problem, RecordErrors, read_records};

const PATH: &str = "authority.md";

fn requirement(id: &str, status: &str) -> String {
    format!(
        "```anchorbook-requirement\nschema-version = 1\nid = \"{id}\"\ntitle = \"Requirement {id}\"\n\
         status = \"{status}\"\nstatement = \"A required outcome.\"\nsource = \"A fixture.\"\n```\n\n"
    )
}

fn decision(id: &str, status: &str) -> String {
    format!(
        "```anchorbook-decision\nschema-version = 1\nid = \"{id}\"\ntitle = \"Decision {id}\"\n\
         status = \"{status}\"\nchoice = \"A choice.\"\nrequirements = []\nrationale = \"A reason.\"\n\
         alternatives = [\"Another choice.\"]\nreconsider-when = \"Never in fixtures.\"\n```\n\n"
    )
}

/// An anchor whose authority field is `authorities`, written verbatim; `None` omits the field.
fn anchor(id: &str, status: &str, authorities: Option<&str>) -> String {
    let field = authorities.map_or(String::new(), |list| format!("authorities = {list}\n"));
    format!(
        "```anchorbook-anchor\nschema-version = 1\nid = \"{id}\"\ntitle = \"Anchor {id}\"\n\
         status = \"{status}\"\nproperty = \"A property.\"\n{field}\n[[criteria]]\nid = \"check\"\n\
         verification = [\"test\"]\nstatement = \"Check it.\"\n```\n\n"
    )
}

fn cite(kind: &str, id: &str) -> String {
    format!("[{{ kind = \"{kind}\", id = \"{id}\" }}]")
}

fn read(text: &str) -> Result<anchor_book::records::RecordInventory, RecordErrors> {
    read_records(&[Chapter::new(Path::new(PATH), text)])
}

/// Asserts the text fails with exactly the expected authority problems, each
/// reported at the source text named for it, and that no inventory is returned.
fn assert_fails(text: &str, expected: &[(Problem, DiagnosticCode, &str)]) {
    let errors = read(text).expect_err("invalid authority must not produce an inventory");
    let mut found: Vec<_> = errors
        .diagnostics()
        .iter()
        .map(|d| {
            assert_eq!(d.location().path(), Path::new(PATH));
            (d.problem(), d.code(), &text[d.location().range()])
        })
        .collect();
    let mut expected = expected.to_vec();
    found.sort_by_key(|(_, _, at)| at.to_owned());
    expected.sort_by_key(|(_, _, at)| at.to_owned());
    assert_eq!(found, expected, "{text}");
}

/// anchorbook: records-valid-authority#required-authority
#[test]
fn anchors_need_a_requirement_or_decision_reference() {
    let authorities = format!(
        "{}{}",
        requirement("need", "accepted"),
        decision("choice", "accepted")
    );
    for list in [cite("requirement", "need"), cite("decision", "choice")] {
        let text = format!(
            "{authorities}{}",
            anchor("property", "accepted", Some(&list))
        );
        let inventory = read(&text).expect("one valid reference suffices");
        assert!(inventory.get("property").is_some());
    }

    for missing in [None, Some("[]")] {
        let text = format!("{authorities}{}", anchor("property", "draft", missing));
        let errors = read(&text).expect_err("an anchor without authority is invalid");
        let diagnostic = errors
            .diagnostics()
            .iter()
            .find(|d| d.problem() == Problem::InvalidDefinition)
            .expect("an invalid-definition diagnostic");
        assert_eq!(diagnostic.code(), DiagnosticCode::InvalidDefinition);
        let anchor_start = text.find("```anchorbook-anchor").unwrap();
        assert!(
            diagnostic.location().range().start >= anchor_start,
            "the diagnostic is located in the anchor: {diagnostic}"
        );
    }
}

/// anchorbook: records-valid-authority#resolved-authority
#[test]
fn anchor_authority_must_resolve_to_one_valid_record_of_the_declared_kind() {
    use DiagnosticCode::{DuplicateId, InvalidReference};
    use Problem::{
        AmbiguousTarget, InvalidDefinition, InvalidTarget, MissingTarget, WrongTargetKind,
    };
    let need = requirement("need", "accepted");
    let choice = decision("choice", "accepted");

    let missing = format!(
        "{need}{}",
        anchor("property", "draft", Some(&cite("requirement", "absent")))
    );
    assert_fails(&missing, &[(MissingTarget, InvalidReference, "\"absent\"")]);

    let wrong_kind = format!(
        "{need}{choice}{}",
        anchor("property", "draft", Some(&cite("decision", "need")))
    );
    assert_fails(
        &wrong_kind,
        &[(WrongTargetKind, InvalidReference, "\"need\"")],
    );

    let invalid_need = need.replace("source = \"A fixture.\"\n", "");
    let invalid = format!(
        "{invalid_need}{}",
        anchor("property", "draft", Some(&cite("requirement", "need")))
    );
    let errors = read(&invalid).expect_err("an invalid target is not usable authority");
    let problems: Vec<_> = errors.diagnostics().iter().map(|d| d.problem()).collect();
    assert!(problems.contains(&InvalidDefinition), "{errors}");
    let target = errors
        .diagnostics()
        .iter()
        .find(|d| d.problem() == InvalidTarget)
        .expect("the reference reports an invalid target, not a missing one");
    assert_eq!(target.code(), InvalidReference);
    assert_eq!(&invalid[target.location().range()], "\"need\"");
    assert!(
        invalid[target.related_locations()[0].range()].starts_with("```anchorbook-requirement")
    );
    assert!(!problems.contains(&MissingTarget));

    let ambiguous = format!(
        "{need}{}{}",
        requirement("need", "accepted"),
        anchor("property", "draft", Some(&cite("requirement", "need")))
    );
    let errors = read(&ambiguous).expect_err("duplicate identities give no target");
    let reference = errors
        .diagnostics()
        .iter()
        .find(|d| d.problem() == AmbiguousTarget)
        .expect("an ambiguous-target diagnostic");
    assert_eq!(reference.code(), InvalidReference);
    assert_eq!(&ambiguous[reference.location().range()], "\"need\"");
    assert_eq!(
        errors
            .diagnostics()
            .iter()
            .filter(|d| d.code() == DuplicateId)
            .count(),
        2
    );
}

/// anchorbook: records-valid-authority#production-integration
/// anchorbook: records-valid-authority#resolved-authority
#[test]
fn each_failing_reference_is_reported_at_its_own_location() {
    let authorities = format!(
        "{}{}",
        requirement("need", "accepted"),
        decision("choice", "accepted")
    );
    let list = "[\n  { kind = \"requirement\", id = \"need\" },\n  { kind = \"decision\", id = \"absent\" },\n]";
    let text = format!("{authorities}{}", anchor("property", "draft", Some(list)));
    assert_fails(
        &text,
        &[(
            Problem::MissingTarget,
            DiagnosticCode::InvalidReference,
            "\"absent\"",
        )],
    );
}

/// anchorbook: records-valid-authority#compatible-status
#[test]
fn accepted_anchors_cite_only_accepted_authority() {
    for (kind, target) in [("requirement", "need"), ("decision", "choice")] {
        for anchor_status in ["draft", "accepted"] {
            for target_status in ["draft", "accepted"] {
                let authority = if kind == "requirement" {
                    requirement(target, target_status)
                } else {
                    decision(target, target_status)
                };
                let text = format!(
                    "{authority}{}",
                    anchor("property", anchor_status, Some(&cite(kind, target)))
                );
                if anchor_status == "accepted" && target_status == "draft" {
                    assert_fails(
                        &text,
                        &[(
                            Problem::IncompatibleStatus,
                            DiagnosticCode::IncompatibleStatus,
                            &format!("\"{target}\""),
                        )],
                    );
                } else {
                    assert!(
                        read(&text).is_ok(),
                        "{anchor_status} anchor citing {target_status} {kind} is valid"
                    );
                }
            }
        }
    }
}
