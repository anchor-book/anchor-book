//! Production authority checks and their Verus specification.
//!
//! Anchor: engine-valid-authority; criterion: formal-authority.
//!
//! Read the rule specifications first, then `validate_authorities` and its helpers.
//! `spec fn` defines what must hold; ordinary `fn` contains executable Rust.
//! `ensures` is a guarantee Verus proves, and `requires` is a caller obligation.
//! An `invariant` holds at loop boundaries; `decreases` shows progress toward termination.
//! `forall` means every value in the stated range; `exists` means at least one.
//! `==>` means "if ... then ..."; `&&&` joins conditions that must all hold.
//! `@` gives a mathematical view, such as a string's sequence of characters.
//! `int` and `Seq` are mathematical types. `#![auto]` selects quantifier triggers
//! for the solver; it does not assume that a condition is true.
//!
//! The proof establishes acceptance versus rejection. Candidate conversion and
//! the exact failure classifications, messages, and locations are tested separately.
use vstd::prelude::*;

verus! {

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Anchor,
    Requirement,
    Decision,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TargetKind {
    Requirement,
    Decision,
}

pub(super) struct Reference {
    pub id: String,
    pub kind: TargetKind,
}

pub(super) struct Candidate {
    // Keep any recovered identity even when the rest of the definition is invalid.
    pub id: Option<String>,
    // Schema decoding produced a record. Its authority relationships are unchecked.
    pub decoded: bool,
    pub kind: Kind,
    // Authored design acceptance, not implementation or verification status.
    pub design_accepted: bool,
    pub references: Vec<Reference>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Failure {
    MissingAuthority,
    RepeatedReference,
    MissingTarget,
    InvalidTarget,
    AmbiguousTarget,
    WrongTargetKind,
    IncompatibleStatus,
}

pub(super) struct Violation {
    // Position in the complete candidate collection.
    pub record_index: usize,
    // Position in that record's references; None means it has no required authority.
    pub reference_index: Option<usize>,
    pub failure: Failure,
}

enum Lookup {
    Missing,
    Unique(usize), // The matching candidate's index.
    Ambiguous,
}

// Authority rules: these mathematical definitions do not execute at runtime.

pub(super) open spec fn identified_by(candidate: Candidate, id: Seq<char>) -> bool {
    match candidate.id {
        Some(value) => value@ == id,
        None => false,
    }
}

pub(super) open spec fn kind_matches(kind: Kind, target: TargetKind) -> bool {
    match target {
        TargetKind::Requirement => kind == Kind::Requirement,
        TargetKind::Decision => kind == Kind::Decision,
    }
}

// "Distinct": this record has not already referenced the same ID.
// Different records may still share an authority.
pub(super) open spec fn reference_is_not_repeated(
    record: Candidate, reference_index: int,
) -> bool {
    let reference = record.references[reference_index];
    forall|earlier_reference_index: int| #![auto]
        0 <= earlier_reference_index < reference_index ==>
        record.references[earlier_reference_index].id@ != reference.id@
}

// "Resolve to one": the index exists, its candidate has the ID, and no other
// candidate claims that ID. Undecodable candidates still count toward ambiguity.
pub(super) open spec fn resolves_uniquely_to(
    candidates: Seq<Candidate>, id: Seq<char>, target_index: int,
) -> bool {
    &&& 0 <= target_index < candidates.len()
    &&& identified_by(candidates[target_index], id)
    &&& (forall|other_index: int| #![auto] 0 <= other_index < candidates.len()
        && identified_by(candidates[other_index], id) ==> other_index == target_index)
}

// "Suitable authority": decoded, of the declared kind, and accepted if required.
// Resolution above establishes identity; this predicate establishes suitability.
pub(super) open spec fn target_is_suitable_authority(
    source_record: Candidate, reference: Reference, target_record: Candidate,
) -> bool {
    &&& target_record.decoded
    &&& kind_matches(target_record.kind, reference.kind)
    &&& (!source_record.design_accepted || target_record.design_accepted)
}

// A reference must be distinct and resolve to one suitable authority.
pub(super) open spec fn reference_satisfies_authority_rules(
    candidates: Seq<Candidate>, record_index: int, reference_index: int,
) -> bool {
    let record = candidates[record_index];
    let reference = record.references[reference_index];
    &&& reference_is_not_repeated(record, reference_index)
    &&& (exists|target_index: int| #![auto]
        resolves_uniquely_to(candidates, reference.id@, target_index)
        && target_is_suitable_authority(record, reference, candidates[target_index]))
}

pub(super) open spec fn record_satisfies_authority_rules(
    candidates: Seq<Candidate>, record_index: int,
) -> bool {
    let record = candidates[record_index];
    // Undecodable sources already have parser errors, which read_records retains.
    // They impose no source-reference obligations here, but remain lookup candidates.
    !record.decoded || (
        // Anchors need authority. A decision may have an empty requirements list.
        (record.kind == Kind::Anchor ==> record.references.len() > 0)
        // Check every reference, including those of decisions no anchor cites.
        && forall|reference_index: int| #![auto]
            0 <= reference_index < record.references.len() ==>
            reference_satisfies_authority_rules(candidates, record_index, reference_index)
    )
}

// The first record_count records satisfy the rules. The index record_count is excluded.
// The validator's final contract uses the full length; its loops use processed counts.
pub(super) open spec fn records_satisfy_authority_rules(
    candidates: Seq<Candidate>, record_count: int,
) -> bool {
    forall|record_index: int| #![auto] 0 <= record_index < record_count ==>
        record_satisfies_authority_rules(candidates, record_index)
}

// Executable checks: contracts connect these Rust bodies to the rules above.

// Used by schema decoding as well as the complete validator: emptiness is not
// assumed away at the boundary of the proof.
pub(super) fn has_required_authority(is_anchor: bool, reference_count: usize) -> (allowed: bool)
    ensures allowed == (!is_anchor || reference_count > 0),
{
    !is_anchor || reference_count > 0
}

fn matches_identity(candidate: &Candidate, id: &str) -> (yes: bool)
    ensures yes == identified_by(*candidate, id@),
{
    match &candidate.id {
        Some(value) => value.as_str() == id,
        None => false,
    }
}

fn matches_target_kind(kind: Kind, target: TargetKind) -> (yes: bool)
    ensures yes == kind_matches(kind, target),
{
    matches!((kind, target),
        (Kind::Requirement, TargetKind::Requirement) | (Kind::Decision, TargetKind::Decision))
}

// Count identity matches even in undecodable candidates: otherwise an invalid
// definition could disappear and make an ambiguous authority look unique.
fn lookup(candidates: &[Candidate], id: &str) -> (result: Lookup)
    ensures match result {
        Lookup::Missing => forall|candidate_index: int| #![auto] 0 <= candidate_index < candidates.len() ==>
            !identified_by(candidates[candidate_index], id@),
        Lookup::Unique(target_index) =>
            resolves_uniquely_to(candidates@, id@, target_index as int),
        Lookup::Ambiguous => exists|first_index: int, second_index: int| #![auto]
            0 <= first_index < candidates.len() && 0 <= second_index < candidates.len()
            && first_index != second_index
            && identified_by(candidates[first_index], id@)
            && identified_by(candidates[second_index], id@),
    },
{
    let mut candidate_index: usize = 0;
    let mut matched_index: Option<usize> = None;
    // Among candidates already visited, every match is the recorded matched_index.
    // A second match exits as Ambiguous. The unvisited count decreases each time.
    while candidate_index < candidates.len()
        invariant
            candidate_index <= candidates.len(),
            matched_index.is_some() ==> matched_index.unwrap() < candidate_index
                && identified_by(candidates[matched_index.unwrap() as int], id@),
            forall|earlier_index: int| #![auto] 0 <= earlier_index < candidate_index
                && identified_by(candidates[earlier_index], id@)
                ==> matched_index == Some(earlier_index as usize),
        decreases candidates.len() - candidate_index,
    {
        if matches_identity(&candidates[candidate_index], id) {
            if matched_index.is_some() {
                return Lookup::Ambiguous;
            }
            matched_index = Some(candidate_index);
        }
        candidate_index += 1;
    }
    match matched_index {
        Some(target_index) => Lookup::Unique(target_index),
        None => Lookup::Missing,
    }
}

// An unrecoverable ID means we cannot describe an unresolved target as simply missing.
fn has_unknown_identity(candidates: &[Candidate]) -> (found: bool)
    ensures found == (exists|candidate_index: int| #![auto]
        0 <= candidate_index < candidates.len() && candidates[candidate_index].id.is_none()),
{
    let mut candidate_index: usize = 0;
    // Everything visited so far has a known ID; finding None is a witness for exists.
    while candidate_index < candidates.len()
        invariant candidate_index <= candidates.len(),
            forall|earlier_index: int| #![auto] 0 <= earlier_index < candidate_index ==>
                candidates[earlier_index].id.is_some(),
        decreases candidates.len() - candidate_index,
    {
        if candidates[candidate_index].id.is_none() {
            return true;
        }
        candidate_index += 1;
    }
    false
}

// requires supplies only safe indices, not an assumption that the reference is valid.
// ensures proves both directions: valid references succeed and invalid ones fail.
fn validate_reference(candidates: &[Candidate], record_index: usize, reference_index: usize)
    -> (failure: Option<Failure>)
    requires
        record_index < candidates.len(),
        reference_index < candidates[record_index as int].references.len(),
    ensures failure.is_none() == reference_satisfies_authority_rules(
        candidates@, record_index as int, reference_index as int),
{
    let record = &candidates[record_index];
    let reference = &record.references[reference_index];
    let mut earlier_reference_index: usize = 0;
    // None of the earlier references examined so far uses this reference's ID.
    while earlier_reference_index < reference_index
        invariant
            record_index < candidates.len(), reference_index < record.references.len(),
            earlier_reference_index <= reference_index,
            record == candidates[record_index as int],
            reference == record.references[reference_index as int],
            forall|checked_index: int| #![auto] 0 <= checked_index < earlier_reference_index ==>
                record.references[checked_index].id@ != reference.id@,
        decreases reference_index - earlier_reference_index,
    {
        if record.references[earlier_reference_index].id.as_str() == reference.id.as_str() {
            return Some(Failure::RepeatedReference);
        }
        earlier_reference_index += 1;
    }
    match lookup(candidates, reference.id.as_str()) {
        Lookup::Missing => {
            if has_unknown_identity(candidates) { Some(Failure::InvalidTarget) }
            else { Some(Failure::MissingTarget) }
        },
        Lookup::Ambiguous => Some(Failure::AmbiguousTarget),
        Lookup::Unique(index) => {
            let target_record = &candidates[index];
            if !target_record.decoded { Some(Failure::InvalidTarget) }
            else if !matches_target_kind(target_record.kind, reference.kind) { Some(Failure::WrongTargetKind) }
            else if record.design_accepted && !target_record.design_accepted { Some(Failure::IncompatibleStatus) }
            else { None }
        },
    }
}

// Main guarantee: no violations if and only if every decoded record's authority
// relationships satisfy the rules. Parser errors still prevent overall Book success.
// The contract does not prove the exact classification or location of each violation.
pub(super) fn validate_authorities(candidates: &[Candidate]) -> (violations: Vec<Violation>)
    ensures (violations.len() == 0) == records_satisfy_authority_rules(
        candidates@, candidates.len() as int),
{
    let mut violations: Vec<Violation> = Vec::new();
    let mut record_index: usize = 0;
    // At each loop head, the collected violations describe whether all earlier
    // records satisfy the rules. Reaching the full length gives the final guarantee.
    while record_index < candidates.len()
        invariant record_index <= candidates.len(),
            (violations.len() == 0) == records_satisfy_authority_rules(candidates@, record_index as int),
        decreases candidates.len() - record_index,
    {
        proof {
            // Extending the processed portion by one record means the old portion
            // AND the current record satisfy the rules. Verus proves this assertion;
            // it is not an assumption, and this block does not execute at runtime.
            assert(records_satisfy_authority_rules(candidates@, record_index as int + 1) ==
                (records_satisfy_authority_rules(candidates@, record_index as int)
                    && record_satisfies_authority_rules(candidates@, record_index as int)));
        }
        let record = &candidates[record_index];
        if record.decoded {
            if !has_required_authority(matches!(record.kind, Kind::Anchor), record.references.len()) {
                violations.push(Violation {
                    record_index,
                    reference_index: None,
                    failure: Failure::MissingAuthority,
                });
            }
            let mut reference_index: usize = 0;
            // No violations yet exactly when earlier records passed, authority
            // presence is satisfied, and every reference visited in this record passed.
            while reference_index < record.references.len()
                invariant
                    record_index < candidates.len(), record == candidates[record_index as int], record.decoded,
                    reference_index <= record.references.len(),
                    (violations.len() == 0) == (records_satisfy_authority_rules(candidates@, record_index as int)
                        && (record.kind == Kind::Anchor ==> record.references.len() > 0)
                        && forall|checked_index: int| #![auto] 0 <= checked_index < reference_index ==>
                            reference_satisfies_authority_rules(candidates@, record_index as int, checked_index)),
                decreases record.references.len() - reference_index,
            {
                if let Some(failure) = validate_reference(candidates, record_index, reference_index) {
                    violations.push(Violation { record_index, reference_index: Some(reference_index), failure });
                }
                reference_index += 1;
            }
        }
        record_index += 1;
    }
    violations
}
}

#[cfg(test)]
mod tests {
    use super::{
        Candidate, Failure, Kind, Reference, TargetKind, has_required_authority,
        validate_authorities,
    };

    fn record(
        id: &str,
        kind: Kind,
        design_accepted: bool,
        references: &[(&str, TargetKind)],
    ) -> Candidate {
        Candidate {
            id: Some(id.to_owned()),
            decoded: true,
            kind,
            design_accepted,
            references: references
                .iter()
                .map(|(id, kind)| Reference {
                    id: (*id).to_owned(),
                    kind: *kind,
                })
                .collect(),
        }
    }

    fn failures(input: Vec<Candidate>) -> Vec<(usize, Option<usize>, Failure)> {
        validate_authorities(&input)
            .into_iter()
            .map(|v| (v.record_index, v.reference_index, v.failure))
            .collect()
    }

    #[test]
    fn validates_authority_presence_and_all_acceptance_states() {
        assert!(validate_authorities(&[]).is_empty());
        for kind in [Kind::Requirement, Kind::Decision, Kind::Anchor] {
            assert_eq!(
                has_required_authority(kind == Kind::Anchor, 0),
                kind != Kind::Anchor
            );
            assert!(has_required_authority(kind == Kind::Anchor, 1));
            assert!(has_required_authority(kind == Kind::Anchor, usize::MAX));
            let actual = failures(vec![record("source", kind, true, &[])]);
            let expected = if kind == Kind::Anchor {
                vec![(0, None, Failure::MissingAuthority)]
            } else {
                vec![]
            };
            assert_eq!(actual, expected);
        }
        for source_accepted in [false, true] {
            for target_accepted in [false, true] {
                for source_kind in [Kind::Anchor, Kind::Decision] {
                    let actual = failures(vec![
                        record(
                            "source",
                            source_kind,
                            source_accepted,
                            &[("target", TargetKind::Requirement)],
                        ),
                        record("target", Kind::Requirement, target_accepted, &[]),
                    ]);
                    let expected = if source_accepted && !target_accepted {
                        vec![(0, Some(0), Failure::IncompatibleStatus)]
                    } else {
                        vec![]
                    };
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    #[test]
    fn resolves_invalid_ambiguous_and_unknown_candidates_without_losing_identity() {
        let source = || {
            record(
                "source",
                Kind::Anchor,
                false,
                &[("target", TargetKind::Requirement)],
            )
        };
        assert_eq!(
            failures(vec![source()]),
            [(0, Some(0), Failure::MissingTarget)]
        );
        let mut invalid = record("target", Kind::Requirement, true, &[]);
        invalid.decoded = false;
        assert_eq!(
            failures(vec![source(), invalid]),
            [(0, Some(0), Failure::InvalidTarget)]
        );
        let mut unknown = record("unused", Kind::Requirement, false, &[]);
        unknown.decoded = false;
        unknown.id = None;
        assert_eq!(
            failures(vec![source(), unknown]),
            [(0, Some(0), Failure::InvalidTarget)]
        );
        // Invalid candidates still contribute to identity ambiguity.
        let mut invalid = record("target", Kind::Decision, false, &[]);
        invalid.decoded = false;
        assert_eq!(
            failures(vec![
                source(),
                invalid,
                record("target", Kind::Requirement, true, &[])
            ]),
            [(0, Some(0), Failure::AmbiguousTarget)]
        );
        assert_eq!(
            failures(vec![source(), record("target", Kind::Decision, true, &[])]),
            [(0, Some(0), Failure::WrongTargetKind)]
        );
    }

    #[test]
    fn checks_every_edge_and_unreferenced_decision_including_repeated_ids() {
        let input = vec![
            record(
                "anchor",
                Kind::Anchor,
                true,
                &[
                    ("decision", TargetKind::Decision),
                    ("requirement", TargetKind::Requirement),
                    ("requirement", TargetKind::Decision),
                    ("absent", TargetKind::Requirement),
                ],
            ),
            record("decision", Kind::Decision, true, &[]),
            record("requirement", Kind::Requirement, true, &[]),
            record(
                "uncited",
                Kind::Decision,
                false,
                &[("absent", TargetKind::Requirement)],
            ),
        ];
        assert_eq!(
            failures(input),
            [
                (0, Some(2), Failure::RepeatedReference),
                (0, Some(3), Failure::MissingTarget),
                (3, Some(0), Failure::MissingTarget),
            ]
        );
    }

    #[test]
    fn string_identity_uses_the_complete_value() {
        let input = vec![
            record(
                "anchor",
                Kind::Anchor,
                false,
                &[("target-é", TargetKind::Requirement)],
            ),
            record("target-e", Kind::Requirement, true, &[]),
            record("target-é", Kind::Requirement, true, &[]),
        ];
        assert!(validate_authorities(&input).is_empty());
    }
}
