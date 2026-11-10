# Changes to specification functions and contracts of formal-authority; code and proofs unchanged.
# Each: (id, spec item, old, new). "kind" in the id: R = alters a rule (not monotone under the
# biconditional), W = pure weakening (always expected to verify).
CHANGES = [
 ("SR1","target_is_suitable_authority","    &&& (!source_record.design_accepted || target_record.design_accepted)\n}","    &&& true\n}"),
 ("SR2","target_is_suitable_authority","    &&& target_record.decoded\n","    &&& true\n"),
 ("SR3","target_is_suitable_authority","    &&& kind_matches(target_record.kind, reference.kind)\n","    &&& true\n"),
 ("SR4","reference_is_not_repeated","record.references[earlier_reference_index].id@ != reference.id@\n}","true\n}"),
 ("SR5","resolves_uniquely_to","    &&& (forall|other_index: int| #![auto] 0 <= other_index < candidates.len()\n        && identified_by(candidates[other_index], id) ==> other_index == target_index)\n","    &&& true\n"),
 ("SR6","record_satisfies_authority_rules","        // Anchors need authority. A decision may have an empty requirements list.\n        (record.kind == Kind::Anchor ==> record.references.len() > 0)","        // Anchors need authority. A decision may have an empty requirements list.\n        true"),
 ("SR7","record_satisfies_authority_rules","    !record.decoded || (","    false || ("),
 ("SR8","kind_matches","        TargetKind::Requirement => kind == Kind::Requirement,","        TargetKind::Requirement => kind != Kind::Anchor,"),
 ("SR9","identified_by","        Some(value) => value@ == id,\n        None => false,","        Some(value) => value@ == id,\n        None => true,"),
 ("SR10","target_is_suitable_authority","    &&& (!source_record.design_accepted || target_record.design_accepted)\n}","    &&& (source_record.design_accepted ==> target_record.design_accepted || source_record.kind == Kind::Decision)\n}"),
 ("SW1","validate_authorities ensures","        (violations.len() == 0) == records_satisfy_authority_rules(\n            candidates@, candidates.len() as int),\n        forall","        (violations.len() == 0) ==> records_satisfy_authority_rules(\n            candidates@, candidates.len() as int),\n        forall"),
 ("SW2","violation_is_located","    &&& candidates[violation.record_index as int].decoded\n","    &&& true\n"),
]
