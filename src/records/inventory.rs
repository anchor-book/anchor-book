use std::collections::{BTreeMap, BTreeSet};

use super::{Chapter, Diagnostic, Problem, Record, RecordErrors, SourceLocation, Status, markdown};

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
    let mut backlinks: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for candidate in &candidates {
        let Some(record) = &candidate.record else {
            continue;
        };
        let mut seen = BTreeSet::new();
        for reference in record.references() {
            let mut related = Vec::new();
            let problem = if !seen.insert(reference.id.as_str()) {
                Some((
                    Problem::RepeatedReference,
                    "authority identity is repeated in this record",
                ))
            } else if let Some(indices) = identities.get(reference.id.as_str()) {
                related = indices
                    .iter()
                    .map(|&i| candidates[i].location.clone())
                    .collect();
                if indices.len() != 1 {
                    Some((
                        Problem::AmbiguousTarget,
                        "authority has duplicate definitions; no target is selected",
                    ))
                } else if let Some(target) = &candidates[indices[0]].record {
                    if target.kind() != reference.kind.into() {
                        Some((
                            Problem::WrongTargetKind,
                            "authority exists with the wrong record kind",
                        ))
                    } else if record.status() == Status::Accepted
                        && target.status() != Status::Accepted
                    {
                        Some((
                            Problem::IncompatibleStatus,
                            "accepted records must cite accepted authority",
                        ))
                    } else {
                        None
                    }
                } else {
                    Some((Problem::InvalidTarget, "authority definition is invalid"))
                }
            } else if !unknown_invalid.is_empty() {
                related.clone_from(&unknown_invalid);
                Some((
                    Problem::InvalidTarget,
                    "authority cannot be resolved while definitions with unknown identities are invalid",
                ))
            } else {
                Some((
                    Problem::MissingTarget,
                    "authority definition does not exist",
                ))
            };
            if let Some((problem, explanation)) = problem {
                let mut diagnostic = Diagnostic::new(
                    problem,
                    reference.location.clone(),
                    format!("{}: {explanation}", reference.id),
                );
                diagnostic.related = related;
                diagnostics.push(diagnostic);
            } else {
                backlinks
                    .entry(reference.id.clone())
                    .or_default()
                    .push(record.id().to_owned());
            }
        }
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
