//! Native Verus execution shared by criterion progress and the strict CI gate.
use super::{
    FunctionId, Inputs, Mode, Plan, ProofOutcome, ProofResult, ProofRun, Scope, Target,
    VerifyError, identity, nextest, verus_report,
};
use crate::VerificationMethod;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

/// The measured Verus report adapter's release.
pub const VERUS_VERSION: &str = "0.2026.10.18.7d82994";
const RUST_VERSION: &str = "1.99.1";
const Z3_VERSION: &str = "4.16.0";

/// A function result without source or proof-note payloads.
#[derive(Clone, Debug, Serialize)]
pub struct FunctionEvidence {
    /// Verus's fully qualified function identity.
    pub function: String,
    /// Verus's `exec`, `proof`, or `spec` mode.
    pub mode: String,
    /// Whether this function verified.
    pub success: bool,
}

/// Evidence from one fresh complete-target invocation.
#[derive(Clone, Debug, Serialize)]
pub struct TargetEvidence {
    /// Package selected from the Book's proof links.
    pub package: String,
    /// Cargo target kind.
    pub target: String,
    /// Cargo features used for this invocation.
    pub features: Vec<String>,
    /// Argument vector with project-relative paths.
    pub command: Vec<String>,
    /// Process exit code; absent when terminated by a signal or not started.
    pub exit_code: Option<i32>,
    /// Whole target and each expected linked function verified.
    pub complete: bool,
    /// Functions expected from resolved links, including their modes.
    pub expected: Vec<String>,
    /// Functions reported by the verifier.
    pub functions: Vec<FunctionEvidence>,
    /// Reasons this target does not pass.
    pub reasons: Vec<String>,
    /// Located diagnostic references, without source excerpts.
    pub locations: Vec<String>,
    /// Project-relative native log paths for local diagnosis; logs are not public artifacts.
    pub logs: Vec<String>,
}

/// Public-safe strict proof-gate result, distinct from criterion satisfaction.
#[derive(Clone, Debug, Serialize)]
pub struct ProofReport {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Nonempty selected scope has current complete proof-target evidence.
    pub complete: bool,
    /// Inputs evaluated, including dirty checkout content identities.
    pub inputs: Inputs,
    /// Actual tool identities.
    pub tools: Vec<String>,
    /// Per-target native results.
    pub targets: Vec<TargetEvidence>,
    /// Reasons preventing a successful gate.
    pub reasons: Vec<String>,
    /// What target success establishes and leaves outstanding.
    pub limits: String,
}
impl ProofReport {
    /// A gate passes only with current complete evidence.
    pub fn exit_code(&self) -> u8 {
        u8::from(!self.complete)
    }
}

/// Run only required proof targets, returning the same native evidence used by
/// [`super::verify`]. Criterion assessments and guards are not implied.
///
/// # Errors
/// Invalid configuration, Book, records, or requested anchors, as [`super::plan`].
pub fn proof(config: &Path, anchors: Vec<String>) -> Result<ProofReport, VerifyError> {
    let plan = super::plan(
        config,
        Scope {
            anchors,
            methods: vec![VerificationMethod::Proof],
        },
    )?;
    Ok(run(&plan).1)
}

fn mode(mode: Mode) -> &'static str {
    match mode {
        Mode::Proof => "proof",
        Mode::Verified => "exec",
        Mode::Test => "test",
    }
}

struct Tools {
    path: OsString,
    toolchain: String,
}
impl Tools {
    fn command(&self, name: impl AsRef<std::ffi::OsStr>) -> Command {
        let mut cmd = Command::new(name);
        cmd.env("PATH", &self.path)
            .env("RUSTUP_TOOLCHAIN", RUST_VERSION);
        cmd
    }
}

fn tools(plan: &Plan, report: &mut ProofReport) -> Result<Tools, ProofResult> {
    let settings = plan.settings.verus.as_ref().ok_or_else(|| {
        ProofResult::NotRun("configure [verification.verus] to run proofs".into())
    })?;
    if settings.version != VERUS_VERSION || settings.rust_toolchain != RUST_VERSION {
        return Err(ProofResult::Unusable(format!(
            "supported tools are Verus {VERUS_VERSION} and Rust {RUST_VERSION}"
        )));
    }
    let mut paths = Vec::new();
    if let Some(dir) = &settings.installation {
        for name in ["verus", "cargo-verus", "z3"] {
            if !dir.join(name).is_file() {
                return Err(ProofResult::NotRun(format!(
                    "{name} is absent from the configured installation; no PATH fallback is used"
                )));
            }
        }
        paths.push(dir.clone());
    }
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let path = std::env::join_paths(paths)
        .map_err(|_| ProofResult::Unusable("invalid installation path".into()))?;
    let mut tools = Tools {
        path,
        toolchain: String::new(),
    };
    let mut versions = Vec::new();
    for name in ["verus", "z3", "rustc"] {
        let output = tools
            .command(name)
            .arg("--version")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .ok_or_else(|| {
                ProofResult::NotRun(format!(
                    "{name} is unavailable; install the pinned proof tools"
                ))
            })?;
        let text = String::from_utf8(output.stdout)
            .map_err(|_| ProofResult::Unusable(format!("invalid {name} identity")))?;
        versions.push(text);
    }
    let field = |name: &str| {
        versions[0]
            .lines()
            .find_map(|l| l.trim().strip_prefix(name))
            .map(str::trim)
    };
    if field("Version:") != Some(VERUS_VERSION)
        || !versions[1].starts_with(&format!("Z3 version {Z3_VERSION} "))
        || !versions[2].starts_with(&format!("rustc {RUST_VERSION} "))
    {
        return Err(ProofResult::Unusable(
            "actual Verus, Z3, or Rust identity differs from the supported selection".into(),
        ));
    }
    tools.toolchain = field("Toolchain:")
        .filter(|s| s.starts_with(&format!("{RUST_VERSION}-")))
        .ok_or_else(|| ProofResult::Unusable("unsupported verifier toolchain".into()))?
        .into();
    report.tools = vec![
        format!("Verus {VERUS_VERSION} ({})", tools.toolchain),
        versions[1].trim().into(),
        versions[2].trim().into(),
    ];
    Ok(tools)
}

fn fresh_directory(output: &Path) -> std::io::Result<PathBuf> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    fs::create_dir_all(output)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let dir = output.join(format!(
        "verus-{stamp}-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&dir)?;
    Ok(dir)
}

fn display(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(Path::new("[external]"))
        .to_string_lossy()
        .into_owned()
}

fn execute(plan: &Plan, report: &mut ProofReport) -> ProofResult {
    if !plan.settings.runners.iter().any(|r| r == "verus") {
        return ProofResult::NotRun("verus is not listed in [verification] runners".into());
    }
    if plan.proof_targets.is_empty() {
        return ProofResult::NotRun("no linked proof targets in the selected scope".into());
    }
    let tools = match tools(plan, report) {
        Ok(tools) => tools,
        Err(result) => return result,
    };
    let directory = match fresh_directory(&plan.settings.output) {
        Ok(dir) => dir,
        Err(_) => {
            return ProofResult::Unusable("cannot create fresh proof output directory".into());
        }
    };
    let mut inputs = nextest::input_paths(plan);
    // The complete authored Book matters, including criteria and completion claims.
    // plan() retains the actual config and Book paths, not assumed project names.
    inputs.extend(plan.proof_inputs.iter().cloned());
    let before = identity::snapshot(&inputs);
    if identity::snapshot(&plan.proof_inputs) != plan.proof_snapshot {
        return ProofResult::Unusable("Book or configuration changed after planning".into());
    }
    if identity::snapshot(&nextest::input_paths(plan)) != plan.inputs.code {
        return ProofResult::Unusable("code inputs changed after planning".into());
    }
    let mut functions = BTreeMap::<FunctionId, ProofOutcome>::new();
    let mut failures = Vec::new();
    let mut all_targets_verified = true;
    let settings = plan
        .settings
        .verus
        .as_ref()
        .expect("tools require settings");
    for (index, (package, target)) in plan.proof_targets.iter().enumerate() {
        let Some(configured) = settings.targets.iter().find(|t| {
            &t.package == package
                && *target == Target::Lib
                && plan.manifests.get(package) == Some(&t.manifest)
        }) else {
            return ProofResult::Unusable(format!(
                "no matching configured library target for `{package}` ({target})"
            ));
        };
        let manifest_text = match fs::read_to_string(&configured.manifest) {
            Ok(s) => s,
            Err(_) => return ProofResult::Unusable("cannot read target manifest".into()),
        };
        let manifest = match toml_edit::Document::parse(manifest_text.as_str()) {
            Ok(m) => m,
            Err(_) => return ProofResult::Unusable("cannot parse target manifest".into()),
        };
        let crate_name = manifest
            .get("lib")
            .and_then(|l| l.get("name"))
            .and_then(|n| n.as_str())
            .map(str::to_owned)
            .unwrap_or_else(|| package.replace('-', "_"));
        let links: Vec<_> = plan
            .links
            .iter()
            .filter(|l| {
                l.method == VerificationMethod::Proof
                    && l.function.package == *package
                    && l.function.target == *target
            })
            .collect();
        let expected: BTreeMap<_, _> = links
            .iter()
            .map(|l| {
                (
                    format!("{crate_name}::{}", l.function.path()),
                    (l.function.clone(), mode(l.mode)),
                )
            })
            .collect();
        let target_dir = directory.join(format!("target-{index}"));
        let mut args = vec![
            "verus".into(),
            "verify".into(),
            "--fwd-verus-args-to".into(),
            "roots".into(),
            "--locked".into(),
            "--manifest-path".into(),
            display(&configured.manifest, &plan.base),
            "--package".into(),
            package.clone(),
            "--target-dir".into(),
            display(&target_dir, &plan.base),
        ];
        if !configured.features.is_empty() {
            args.extend(["--features".into(), configured.features.join(",")]);
        }
        args.push("--lib".into());
        args.extend([
            "--".into(),
            "--no-cheating".into(),
            "--output-json".into(),
            "--time-expanded".into(),
        ]);
        let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let output = tools
            .command(cargo)
            .current_dir(&plan.base)
            .args(&args)
            .output();
        let mut target_report = TargetEvidence {
            package: package.clone(),
            target: target.to_string(),
            features: configured.features.clone(),
            command: std::iter::once("cargo".into()).chain(args).collect(),
            exit_code: None,
            complete: false,
            expected: expected
                .iter()
                .map(|(n, (_, m))| format!("{n} ({m})"))
                .collect(),
            functions: Vec::new(),
            reasons: Vec::new(),
            locations: Vec::new(),
            logs: vec![
                display(
                    &directory.join(format!("target-{index}.stdout")),
                    &plan.base,
                ),
                display(
                    &directory.join(format!("target-{index}.stderr")),
                    &plan.base,
                ),
            ],
        };
        let mut target_verified = false;
        match output {
            Err(_) => target_report
                .reasons
                .push("cannot start cargo verus".into()),
            Ok(output) => {
                target_report.exit_code = output.status.code();
                let log = directory.join(format!("target-{index}"));
                if fs::write(log.with_extension("stdout"), &output.stdout)
                    .and_then(|()| fs::write(log.with_extension("stderr"), &output.stderr))
                    .is_err()
                {
                    return ProofResult::Unusable("cannot retain native proof logs".into());
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                target_report.locations = stderr
                    .lines()
                    .filter_map(|l| l.trim().strip_prefix("--> "))
                    .filter(|l| !Path::new(l).is_absolute())
                    .map(str::to_owned)
                    .collect();
                let trust = stderr.contains("not allowed with --no-cheating");
                if trust {
                    for (id, _) in expected.values() {
                        functions.insert(id.clone(), ProofOutcome::Failed);
                    }
                    target_report
                        .reasons
                        .push("project trust-policy violation (--no-cheating)".into());
                }
                match String::from_utf8(output.stdout)
                    .map_err(|_| "invalid UTF-8 in Verus report".into())
                    .and_then(|raw| {
                        verus_report::parse(&raw, &tools.toolchain, output.status.success())
                    }) {
                    Err(reason) => target_report.reasons.push(reason),
                    Ok(parsed) => {
                        target_verified = parsed.complete && !trust;
                        target_report.complete = parsed.complete && !trust;
                        failures.extend(parsed.failures.iter().cloned());
                        if !parsed.complete {
                            target_report
                                .reasons
                                .push("target verification is incomplete or failed".into());
                        }
                        for (name, (id, mode)) in &expected {
                            match parsed
                                .functions
                                .iter()
                                .find(|f| &f.function == name && &f.mode == mode)
                            {
                                Some(f) if !trust => {
                                    functions.insert(
                                        id.clone(),
                                        if f.success {
                                            ProofOutcome::Verified
                                        } else {
                                            ProofOutcome::Failed
                                        },
                                    );
                                }
                                Some(_) => {}
                                None => {
                                    target_report.complete = false;
                                    target_report.reasons.push(format!("expected function `{name}` is absent or in the wrong mode ({mode})"));
                                }
                            }
                        }
                        target_report.functions = parsed.functions;
                    }
                }
            }
        }
        all_targets_verified &= target_verified;
        report.targets.push(target_report);
    }
    if identity::snapshot(&inputs) != before {
        return ProofResult::Unusable(
            "Book, configuration, code, manifests, or lockfiles changed during verification".into(),
        );
    }
    ProofResult::Ran(ProofRun {
        complete: all_targets_verified,
        functions,
        failures_elsewhere: failures,
        guards: BTreeMap::new(),
    })
}

pub(super) fn run(plan: &Plan) -> (ProofResult, ProofReport) {
    let mut report = ProofReport { schema_version: 1, complete: false, inputs: plan.inputs.clone(), tools: Vec::new(), targets: Vec::new(), reasons: Vec::new(), limits: "Target contracts only. Criterion specification assessments, sensitivity and witness guards remain outstanding. Verifier, solver, compiler, standard-library and dependency contracts are trusted; excluded integration boundaries retain their test and review obligations.".into() };
    let result = execute(plan, &mut report);
    match &result {
        ProofResult::NotRun(reason) | ProofResult::Unusable(reason) => {
            report.reasons.push(reason.clone());
            for target in &mut report.targets {
                target.complete = false;
                target.reasons.push(reason.clone());
            }
        }
        ProofResult::Ran(run) => {
            report.complete = run.complete
                && !report.targets.is_empty()
                && report.targets.iter().all(|t| t.complete)
        }
    }
    if !plan.findings.is_empty() {
        report.complete = false;
        report
            .reasons
            .push("unresolved evidence-link findings; run verify for locations".into());
    }
    if plan
        .criteria
        .iter()
        .filter(|c| c.methods.contains(&VerificationMethod::Proof))
        .any(|c| {
            !plan.links.iter().any(|l| {
                l.anchor == c.anchor
                    && l.criterion == c.criterion
                    && l.method == VerificationMethod::Proof
            })
        })
    {
        report.complete = false;
        report
            .reasons
            .push("a selected proof criterion has no resolved evidence links".into());
    }
    if !report.complete && report.reasons.is_empty() {
        report
            .reasons
            .push("one or more required proof targets did not pass".into());
    }
    (result, report)
}
