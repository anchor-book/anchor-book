//! Private adapter for the measured Verus 0.2026.10.18 report.
use super::verus::{FunctionEvidence, VERUS_VERSION};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) struct Parsed {
    pub complete: bool,
    pub functions: Vec<FunctionEvidence>,
    pub failures: Vec<String>,
}

/// Remove only the known dependency success summary. Everything else must be
/// exactly one JSON document, never a convenient JSON substring of a log.
pub(super) fn parse(raw: &str, toolchain: &str, success: bool) -> Result<Parsed, String> {
    let json = raw
        .lines()
        .filter(|line| {
            let summary = line
                .strip_prefix("verification results:: ")
                .and_then(|s| s.strip_suffix(" verified, 0 errors"));
            !summary.is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let value: Value = serde_json::from_str(&json)
        .map_err(|_| "Verus did not produce exactly one supported JSON report")?;
    if value.pointer("/verus/version").and_then(Value::as_str) != Some(VERUS_VERSION) {
        return Err("unsupported or missing Verus report version".into());
    }
    if value.pointer("/verus/toolchain").and_then(Value::as_str) != Some(toolchain) {
        return Err("report toolchain differs from the actual selected verifier".into());
    }
    let result = value
        .get("verification-results")
        .ok_or("missing verification-results")?;
    let boolean = |key: &str| {
        result
            .get(key)
            .and_then(Value::as_bool)
            .ok_or_else(|| format!("missing or invalid {key}"))
    };
    let declared_success = boolean("success")?;
    let error = boolean("encountered-error")?;
    let vir_error = boolean("encountered-vir-error")?;
    let mut functions = Vec::new();
    if let Some(modules) = value.pointer("/times-ms/smt/smt-run-module-times") {
        for module in modules.as_array().ok_or("invalid function modules")? {
            for entry in module
                .get("function-breakdown")
                .and_then(Value::as_array)
                .ok_or("missing function breakdown")?
            {
                let name = entry
                    .get("function")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                    .ok_or("missing function identity")?;
                let mode = entry
                    .get("mode:")
                    .and_then(Value::as_str)
                    .ok_or("missing function mode")?;
                if !["exec", "proof", "spec"].contains(&mode) {
                    return Err("unsupported function mode".into());
                }
                let success = entry
                    .get("success")
                    .and_then(Value::as_bool)
                    .ok_or("missing function outcome")?;
                functions.push(FunctionEvidence {
                    function: name.into(),
                    mode: mode.into(),
                    success,
                });
            }
        }
    }
    functions.sort_by(|a, b| a.function.cmp(&b.function).then(a.mode.cmp(&b.mode)));
    let mut names = BTreeSet::new();
    if functions.iter().any(|f| !names.insert(&f.function)) {
        return Err("duplicate function identities in Verus report".into());
    }
    let failures = functions
        .iter()
        .filter(|f| !f.success)
        .map(|f| f.function.clone())
        .collect::<Vec<_>>();
    let complete = success
        && declared_success
        && !error
        && !vir_error
        && result
            .get("is-verifying-entire-crate")
            .and_then(Value::as_bool)
            == Some(true)
        && result
            .get("verified")
            .and_then(Value::as_u64)
            .is_some_and(|n| n > 0)
        && result.get("errors").and_then(Value::as_u64) == Some(0)
        && !functions.is_empty()
        && failures.is_empty();
    Ok(Parsed {
        complete,
        functions,
        failures,
    })
}
