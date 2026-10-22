use std::collections::{BTreeMap, BTreeSet};

use super::{
    Chapter, Diagnostic, Problem, Record, RecordErrors, SourceLocation, Status, authority, markdown,
};

/// An immutable, validated collection. Construction is all-or-nothing.
#[derive(Clone, Debug)]
pub struct RecordInventory {
    records: BTreeMap<String, Record>,
    backlinks: BTreeMap<String, Vec<String>>,
}
impl RecordInventory {
    /// Look up an anchor, requirement, or decision by its Book-wide identity.
    pub fn get(&self, id: &str) -> Option<&Record> {
        self.records.get(id)
    }
    /// Records in ascending identity order, independent of chapter ordering.
    pub fn records(&self) -> impl ExactSizeIterator<Item = &Record> {
        self.records.values()
    }
    /// Number of validated definitions.
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Whether no definitions were supplied.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// IDs of records directly citing this ID, sorted and unique.
    /// Unknown and unreferenced IDs both yield an empty slice; use `get` to distinguish them.
    pub fn backlinks(&self, id: &str) -> &[String] {
        self.backlinks.get(id).map_or(&[], Vec::as_slice)
    }
}

/// Read the full chapter inventory, validate canonical definitions, and resolve
/// direct authority edges. Inputs are borrowed; the successful inventory owns
/// its data and retains definition/body ranges in the supplied sources.
///
/// # Errors
/// Returns located, sorted diagnostics for malformed input, invalid definitions,
/// duplicate identities, unresolved or repeated references, and acceptance-state
/// conflicts. There is no partially successful inventory. Does not inspect files
/// or evaluate anchor criteria, project structure, or implementation evidence.
pub fn read_records(chapters: &[Chapter<'_>]) -> Result<RecordInventory, RecordErrors> {
    let mut diagnostics = Vec::new();
    let mut ordered: Vec<_> = chapters.iter().collect();
    ordered.sort_by(|a, b| a.path.cmp(b.path).then(a.content.cmp(b.content)));
    let mut candidates = Vec::new();
    let mut paths = BTreeSet::new();
    for chapter in ordered {
        if chapter.path.as_os_str().is_empty() || !paths.insert(chapter.path) {
            diagnostics.push(Diagnostic::new(
                Problem::InvalidInput,
                SourceLocation::new(chapter.path, chapter.content, 0..0),
                "chapter paths must be nonempty and unique",
            ));
        }
        candidates.extend(markdown::read(*chapter, &mut diagnostics));
    }
    let mut identities: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, candidate) in candidates.iter().enumerate() {
        if let Some(id) = candidate.id.as_deref() {
            identities.entry(id).or_default().push(i);
        }
    }
    for (id, indices) in &identities {
        if indices.len() > 1 {
            for &i in indices {
                let mut diagnostic = Diagnostic::new(
                    Problem::DuplicateId,
                    candidates[i].location.clone(),
                    format!("record identity `{id}` is defined more than once"),
                );
                diagnostic.related = indices
                    .iter()
                    .filter(|&&j| j != i)
                    .map(|&j| candidates[j].location.clone())
                    .collect();
                diagnostics.push(diagnostic);
            }
        }
    }
    let unknown_invalid: Vec<_> = candidates
        .iter()
        .filter(|c| c.id.is_none())
        .map(|c| c.location.clone())
        .collect();
    // This conversion preserves parser candidates, including invalid definitions
    // and identities that could not be recovered. It is tested, not formally proved.
    let input: Vec<_> = candidates
        .iter()
        .map(|candidate| {
            let record = candidate.record.as_ref();
            authority::Candidate {
                id: candidate.id.clone(),
                decoded: record.is_some(),
                kind: match record.map(Record::kind) {
                    Some(super::RecordKind::Anchor) => authority::Kind::Anchor,
                    Some(super::RecordKind::Decision) => authority::Kind::Decision,
                    _ => authority::Kind::Requirement,
                },
                design_accepted: record.is_some_and(|r| r.status() == Status::Accepted),
                references: record
                    .map(|r| {
                        r.references()
                            .iter()
                            .map(|reference| authority::Reference {
                                id: reference.id.clone(),
                                kind: match reference.kind {
                                    super::AuthorityKind::Requirement => {
                                        authority::TargetKind::Requirement
                                    }
                                    super::AuthorityKind::Decision => {
                                        authority::TargetKind::Decision
                                    }
                                },
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        })
        .collect();
    // The same function bodies are executed here and verified by Verus.
    for violation in authority::validate_authorities(&input) {
        let candidate = &candidates[violation.record_index];
        let record = candidate
            .record
            .as_ref()
            .expect("validator reports decoded source candidates");
        let Some(reference_index) = violation.reference_index else {
            diagnostics.push(Diagnostic::new(
                Problem::InvalidDefinition,
                candidate.location.clone(),
                "authorities must be a nonempty array of inline tables",
            ));
            continue;
        };
        let reference = &record.references()[reference_index];
        let (problem, explanation) = match violation.failure {
            authority::Failure::RepeatedReference => (
                Problem::RepeatedReference,
                "authority identity is repeated in this record",
            ),
            authority::Failure::MissingTarget => (
                Problem::MissingTarget,
                "authority definition does not exist",
            ),
            authority::Failure::InvalidTarget
                if !identities.contains_key(reference.id.as_str()) =>
            {
                (
                    Problem::InvalidTarget,
                    "authority cannot be resolved while definitions with unknown identities are invalid",
                )
            }
            authority::Failure::InvalidTarget => {
                (Problem::InvalidTarget, "authority definition is invalid")
            }
            authority::Failure::AmbiguousTarget => (
                Problem::AmbiguousTarget,
                "authority has duplicate definitions; no target is selected",
            ),
            authority::Failure::WrongTargetKind => (
                Problem::WrongTargetKind,
                "authority exists with the wrong record kind",
            ),
            authority::Failure::IncompatibleStatus => (
                Problem::IncompatibleStatus,
                "accepted records must cite accepted authority",
            ),
            authority::Failure::MissingAuthority => {
                unreachable!("missing authority has no reference index")
            }
        };
        let mut diagnostic = Diagnostic::new(
            problem,
            reference.location.clone(),
            format!("{}: {explanation}", reference.id),
        );
        if violation.failure != authority::Failure::RepeatedReference {
            diagnostic.related = identities
                .get(reference.id.as_str())
                .map(|indices| {
                    indices
                        .iter()
                        .map(|&index| candidates[index].location.clone())
                        .collect()
                })
                .unwrap_or_else(|| unknown_invalid.clone());
        }
        diagnostics.push(diagnostic);
    }
    if !diagnostics.is_empty() {
        diagnostics.sort_by(|a, b| {
            a.location
                .path
                .cmp(&b.location.path)
                .then(a.location.range.start.cmp(&b.location.range.start))
                .then(a.code().as_str().cmp(b.code().as_str()))
                .then(a.message.cmp(&b.message))
        });
        return Err(RecordErrors(diagnostics));
    }
    let mut backlinks: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for candidate in &candidates {
        if let Some(record) = &candidate.record {
            for reference in record.references() {
                backlinks
                    .entry(reference.id.clone())
                    .or_default()
                    .push(record.id().to_owned());
            }
        }
    }
    for sources in backlinks.values_mut() {
        sources.sort();
        sources.dedup();
    }
    let records = candidates
        .into_iter()
        .filter_map(|c| c.record)
        .map(|r| (r.id().to_owned(), r))
        .collect();
    Ok(RecordInventory { records, backlinks })
}
