//! The native proof gate through actual CLI processes and independently varied reports.
use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};
#[path = "support/checking.rs"]
mod fixture;

const PASS: &str = include_str!("fixtures/verus/modes.json");
const LIB: &str = r#"#[cfg(feature = "verus")]
use vstd::prelude::*;
/// anchorbook: A-exec#formal
#[cfg_attr(feature = "verus", verus_spec(r => ensures r == 1))]
pub fn linked() -> u32 { 1 }
/// anchorbook: A-lemma#formal
#[cfg(feature = "verus")]
#[verifier::proof]
#[verus_spec(ensures true)]
pub fn lemma() { }
"#;

fn book() -> fixture::Book {
    let b = fixture::Book::new();
    b.write("docs/authority.md", "# Authority\n\n> **`R-goal`** (draft): **Goal**\n> Statement: A verified function respects its contract.\n> Source: Independent fixture.\n");
    let mut design = "# Design\n\n".to_string();
    for id in ["A-exec", "A-lemma"] {
        design.push_str(&format!("> **`{id}`** (draft): **A verified function**\n> Property: It respects its contract.\n> Authority: `R-goal`\n> - Criterion `formal` [proof]: The function respects its contract.\n>   - Domain: All inputs.\n>   - Boundary: The function.\n>   - Assumptions: Supported tools.\n>   - Exclusions: None.\n\n"));
    }
    b.write("docs/design.md", &design);
    b.write("src/lib.rs", LIB);
    b.write("Cargo.toml", "[package]\nname = \"modes\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[features]\nverus = [\"dep:vstd\"]\n[dependencies]\nvstd = { version = \"=0.0.0-2026-10-18-0147\", optional = true }\n[package.metadata.verus]\nverify = true\nschema-version = 1\n");
    b.write("Cargo.lock", "# fixture input\n");
    b.write("anchorbook.toml", &format!("{}\n[verification]\nsources = [\"src\"]\noutput = \"target/proof\"\nrunners = [\"verus\"]\n[verification.verus]\nversion = \"0.2026.10.18.7d82994\"\nrust-toolchain = \"1.99.1\"\ninstallation = \"tools\"\ntargets = [{{manifest = \"Cargo.toml\", package = \"modes\", target = \"lib\", features = [\"verus\"]}}]\n", fixture::CONFIG));
    b
}

fn executable(book: &fixture::Book, name: &str, body: &str) {
    use std::os::unix::fs::PermissionsExt;
    book.write(name, body);
    fs::set_permissions(book.path(name), fs::Permissions::from_mode(0o755)).unwrap();
}

fn recording(b: &fixture::Book) {
    executable(b, "tools/cargo-verus", "#!/bin/sh\nexit 1\n");
    executable(
        b,
        "tools/verus",
        "#!/bin/sh\nprintf 'Verus\\n  Version: %s\\n  Toolchain: 1.99.1-x86_64-unknown-linux-gnu\\n' \"${TOOL_VERSION:-0.2026.10.18.7d82994}\"\n",
    );
    executable(
        b,
        "tools/z3",
        "#!/bin/sh\necho 'Z3 version 4.16.0 - 64 bit'\n",
    );
    executable(
        b,
        "tools/rustc",
        "#!/bin/sh\necho 'rustc 1.99.1 (fixture)'\n",
    );
    executable(
        b,
        "tools/cargo",
        r#"#!/bin/sh
printf '%s\n' "$@" >> "$RECORD"
previous=
for argument; do
  if [ "$previous" = --target-dir ]; then
    if [ -e "$argument" ]; then exit 91; fi
    mkdir -p "$argument"
  fi
  previous=$argument
done
if [ -n "$CHANGE" ]; then printf '\n// changed\n' >> "$CHANGE"; fi
if [ -n "$DIAGNOSTIC" ]; then printf '%s\n' "$DIAGNOSTIC" >&2; fi
if [ "$NO_OUTPUT" != yes ]; then cat "$REPORT"; fi
exit "${STATUS:-0}"
"#,
    );
    b.write("tools/report.json", PASS);
}

fn command(b: &fixture::Book, operation: &str) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_anchorbook"));
    c.args([operation, "--config"])
        .arg(b.config())
        .env("CARGO", b.path("tools/cargo"))
        .env("REPORT", b.path("tools/report.json"))
        .env("RECORD", b.path("tools/calls"));
    c
}
fn report(output: &Output) -> Value {
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
fn run(b: &fixture::Book) -> Output {
    command(b, "proof").output().unwrap()
}
fn bad(b: &fixture::Book, value: &Value) {
    b.write("tools/report.json", &value.to_string());
    let output = run(b);
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(report(&output)["complete"], false);
}

/// anchorbook: A-verify-runs-linked-checks#selected-run
/// anchorbook: A-verify-runs-linked-checks#source-preservation
#[test]
fn gate_uses_current_targets_links_modes_and_native_arguments() {
    let b = book();
    recording(&b);
    let before = b.snapshot();
    let out = run(&b);
    let r = report(&out);
    assert_eq!(out.status.code(), Some(0), "{r}");
    assert_eq!(r["complete"], true);
    assert_eq!(r["schema_version"], 1);
    assert_eq!(r["targets"].as_array().unwrap().len(), 1);
    assert_eq!(
        r["targets"][0]["expected"],
        json!(["modes::lemma (proof)", "modes::linked (exec)"])
    );
    let args = fs::read_to_string(b.path("tools/calls")).unwrap();
    for expected in [
        "--locked\n",
        "--no-cheating\n",
        "--lib\n",
        "--package\nmodes\n",
        "--features\nverus\n",
        "--fwd-verus-args-to\nroots\n",
        "--output-json\n",
        "--time-expanded\n",
    ] {
        assert!(args.contains(expected), "{args}");
    }
    assert_eq!(args.matches("verify\n").count(), 1);
    for (path, value) in before {
        assert_eq!(
            b.snapshot().get(&path),
            Some(&value),
            "source changed: {path:?}"
        );
    }
    assert!(!String::from_utf8_lossy(&out.stdout).contains(b.root.to_str().unwrap()));
    assert!(!String::from_utf8_lossy(&out.stdout).contains("proof_notes"));
    let progress = command(&b, "verify").output().unwrap();
    let text = String::from_utf8(progress.stdout).unwrap();
    assert!(text.contains("formal [proof]: no-evidence"), "{text}");
    assert!(text.contains("specification assessment"), "{text}");
    assert!(
        text.contains("proof targets: verified against contracts"),
        "{text}"
    );
    let calls = fs::read_to_string(b.path("tools/calls")).unwrap();
    let directories = calls
        .lines()
        .zip(calls.lines().skip(1))
        .filter(|(a, _)| *a == "--target-dir")
        .map(|(_, b)| b)
        .collect::<Vec<_>>();
    assert_eq!(directories.len(), 2);
    assert_ne!(directories[0], directories[1]);
}

/// anchorbook: A-verify-criterion-states#state-per-method
#[test]
fn linked_failure_supporting_failure_and_trust_have_distinct_states() {
    let b = book();
    recording(&b);
    for (raw, state, diagnostic) in [
        (
            include_str!("fixtures/verus/linked-failure.json"),
            "failed",
            "",
        ),
        (
            include_str!("fixtures/verus/supporting-failure.json"),
            "inconclusive",
            "",
        ),
        (
            include_str!("fixtures/verus/assumption.json"),
            "failed",
            "error: assume/admit not allowed with --no-cheating",
        ),
        (
            include_str!("fixtures/verus/trusted-body.json"),
            "failed",
            "error: external_body/assume_specification not allowed with --no-cheating",
        ),
    ] {
        b.write("tools/report.json", raw);
        let out = command(&b, "verify")
            .args(["--anchor", "A-exec"])
            .env("STATUS", "1")
            .env("DIAGNOSTIC", diagnostic)
            .output()
            .unwrap();
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(text.contains(&format!("formal [proof]: {state}")), "{text}");
        let gate = command(&b, "proof")
            .args(["--anchor", "A-exec"])
            .env("STATUS", "1")
            .env("DIAGNOSTIC", diagnostic)
            .output()
            .unwrap();
        assert_eq!(gate.status.code(), Some(1));
        assert_eq!(report(&gate)["complete"], false);
    }
}

/// anchorbook: A-verify-links-resolve#link-findings
#[test]
fn invalid_incomplete_ambiguous_or_wrong_mode_reports_never_pass() {
    let b = book();
    recording(&b);
    let original: Value = serde_json::from_str(PASS).unwrap();
    for (pointer, value) in [
        ("/verus/version", json!("future")),
        ("/verus/toolchain", json!("wrong")),
        ("/verification-results/success", json!(false)),
        ("/verification-results/encountered-error", json!(true)),
        ("/verification-results/encountered-vir-error", json!(true)),
        (
            "/verification-results/is-verifying-entire-crate",
            json!(false),
        ),
        ("/verification-results/verified", json!(0)),
        ("/verification-results/verified", json!("2")),
        ("/verification-results/errors", json!(1)),
        (
            "/times-ms/smt/smt-run-module-times/0/function-breakdown/0/mode:",
            json!("exec"),
        ),
        (
            "/times-ms/smt/smt-run-module-times/0/function-breakdown/0/success",
            Value::Null,
        ),
        (
            "/times-ms/smt/smt-run-module-times/0/function-breakdown/0/function",
            json!("other::lemma"),
        ),
        ("/times-ms/smt/smt-run-module-times", json!([])),
    ] {
        let mut value_original = original.clone();
        *value_original.pointer_mut(pointer).unwrap() = value;
        bad(&b, &value_original);
    }
    let mut repeated = original.clone();
    let fs = repeated
        .pointer_mut("/times-ms/smt/smt-run-module-times/0/function-breakdown")
        .unwrap()
        .as_array_mut()
        .unwrap();
    fs.push(fs[0].clone());
    bad(&b, &repeated);
    for raw in [
        "",
        "{}",
        "not json",
        &format!("{PASS}{PASS}"),
        &format!("unrecognized output\n{PASS}"),
    ] {
        b.write("tools/report.json", raw);
        assert_eq!(run(&b).status.code(), Some(1));
    }
    b.write(
        "tools/report.json",
        &format!("verification results:: 2 verified, 0 errors\n{PASS}"),
    );
    assert_eq!(run(&b).status.code(), Some(0));
    assert_eq!(
        command(&b, "proof")
            .env("STATUS", "1")
            .output()
            .unwrap()
            .status
            .code(),
        Some(1)
    );
}

/// anchorbook: A-verify-runs-linked-checks#selected-run
#[test]
fn stale_changed_missing_and_unavailable_evidence_fails_the_gate() {
    let b = book();
    recording(&b);
    assert!(run(&b).status.success());
    let stale = command(&b, "proof")
        .env("NO_OUTPUT", "yes")
        .output()
        .unwrap();
    assert_eq!(stale.status.code(), Some(1));
    let build = command(&b, "verify")
        .env("NO_OUTPUT", "yes")
        .env("STATUS", "101")
        .output()
        .unwrap();
    assert!(
        String::from_utf8(build.stdout)
            .unwrap()
            .contains("formal [proof]: inconclusive")
    );
    for path in [
        "src/lib.rs",
        "Cargo.lock",
        "Cargo.toml",
        "anchorbook.toml",
        "docs/design.md",
    ] {
        let original = fs::read_to_string(b.path(path)).unwrap();
        let changed = command(&b, "proof")
            .env("CHANGE", b.path(path))
            .output()
            .unwrap();
        assert_eq!(
            changed.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&changed.stdout)
        );
        assert!(report(&changed)["reasons"].to_string().contains("changed"));
        b.write(path, &original);
    }
    assert_eq!(
        command(&b, "proof")
            .env("TOOL_VERSION", "future")
            .output()
            .unwrap()
            .status
            .code(),
        Some(1)
    );
    executable(&b, "tools/verus", "#!/bin/sh\nexit 1\n");
    let out = run(&b);
    assert_eq!(out.status.code(), Some(1));
    assert!(report(&out)["reasons"].to_string().contains("unavailable"));
    fs::remove_file(b.path("tools/cargo-verus")).unwrap();
    let out = run(&b);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        report(&out)["reasons"]
            .to_string()
            .contains("no PATH fallback")
    );
}

#[test]
fn empty_scope_bad_links_and_invalid_configuration_cannot_remove_obligations() {
    let b = book();
    recording(&b);
    b.write(
        "src/lib.rs",
        &LIB.replace("/// anchorbook: A-exec#formal\n", ""),
    );
    let out = run(&b);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        report(&out)["reasons"]
            .to_string()
            .contains("no resolved evidence links")
    );
    b.write(
        "src/lib.rs",
        &LIB.replace("A-exec#formal", "A-missing#formal"),
    );
    assert_eq!(run(&b).status.code(), Some(1));
    b.write("src/lib.rs", "pub fn plain() {}\n");
    assert_eq!(run(&b).status.code(), Some(1));
    b.write("src/lib.rs", LIB);
    let config = fs::read_to_string(b.config()).unwrap();
    for (from, to) in [
        ("target = \"lib\"", "target = \"bin\""),
        ("package = \"modes\"", "package = \"wrong\""),
        (
            "manifest = \"Cargo.toml\"",
            "manifest = \"../outside.toml\"",
        ),
        ("features = [\"verus\"]", "features = [\"--skip\"]"),
    ] {
        b.write("anchorbook.toml", &config.replace(from, to));
        let out = run(&b);
        assert_eq!(
            out.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(out.stdout.is_empty());
    }
}

#[test]
fn renamed_libraries_and_attribute_modes_are_resolved_from_sources() {
    let b = book();
    recording(&b);
    let manifest = fs::read_to_string(b.path("Cargo.toml")).unwrap();
    b.write(
        "Cargo.toml",
        &(manifest + "\n[lib]\nname = \"actual_library\"\n"),
    );
    b.write(
        "tools/report.json",
        &PASS.replace("modes::", "actual_library::"),
    );
    let source = LIB.replace(
        "#[verifier::proof]",
        "#[cfg_attr(feature = \"verus\", verifier::proof)]",
    );
    b.write("src/lib.rs", &source);
    let out = run(&b);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let p = anchor_book::verification::plan(&b.config(), Default::default()).unwrap();
    assert!(p.links.iter().any(|l| l.function.name == "lemma" && l.mode == anchor_book::verification::Mode::Proof));
    let fake = source.replace(
        "#[cfg_attr(feature = \"verus\", verus_spec(r => ensures r == 1))]",
        "#[cfg_attr(feature = \"verus\", doc = \"verus_spec verifier :: proof\")]",
    );
    b.write("src/lib.rs", &fake);
    assert_eq!(run(&b).status.code(), Some(1));
    let unlinked = command(&b, "proof")
        .args(["--anchor", "missing"])
        .output()
        .unwrap();
    assert_eq!(unlinked.status.code(), Some(2));
    assert_eq!(
        command(&b, "proof")
            .args(["--method", "test"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
}

#[test]
fn absent_functions_are_no_evidence_and_an_empty_book_is_not_a_proof_gate() {
    let b = book();
    recording(&b);
    let mut value: Value = serde_json::from_str(PASS).unwrap();
    value
        .pointer_mut("/times-ms/smt/smt-run-module-times/0/function-breakdown")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .retain(|f| f["mode:"] == "proof");
    b.write("tools/report.json", &value.to_string());
    let out = command(&b, "verify")
        .args(["--anchor", "A-exec"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8(out.stdout)
            .unwrap()
            .contains("formal [proof]: no-evidence")
    );
    assert_eq!(run(&b).status.code(), Some(1));
    b.write("docs/design.md", "# Design\n\nNo anchors yet.\n");
    b.write("src/lib.rs", "pub fn plain() {}\n");
    assert_eq!(run(&b).status.code(), Some(1));
}

#[test]
#[ignore = "native Verus acceptance: set ANCHORBOOK_TEST_VERUS to the pinned tool directory"]
fn native_proof_failure_support_failure_trust_and_correction() {
    let b = book();
    let tools =
        PathBuf::from(std::env::var_os("ANCHORBOOK_TEST_VERUS").expect("pinned Verus directory"));
    let config = fs::read_to_string(b.config()).unwrap().replace(
        "installation = \"tools\"",
        &format!(
            "installation = {}",
            serde_json::to_string(&tools.to_string_lossy()).unwrap()
        ),
    );
    b.write("anchorbook.toml", &config);
    fs::remove_file(b.path("Cargo.lock")).unwrap();
    assert!(
        Command::new("cargo")
            .args(["generate-lockfile", "--offline", "--manifest-path"])
            .arg(b.path("Cargo.toml"))
            .status()
            .unwrap()
            .success()
    );
    let native = |operation: &str| {
        let out = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
            .args([operation, "--config"])
            .arg(b.config())
            .args(["--anchor", "A-exec"])
            .output()
            .unwrap();
        assert!(
            out.stderr.is_empty(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        out
    };
    let out = native("proof");
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    for (source, state) in [
        (LIB.replace("{ 1 }", "{ 2 }"), "failed"),
        (LIB.replace("ensures true", "ensures false"), "inconclusive"),
        (
            LIB.replace("pub fn lemma() { }", "pub fn lemma() { assume_(false); }"),
            "failed",
        ),
        (
            LIB.replace(
                "#[verifier::proof]",
                "#[verifier::external_body]\n#[verifier::proof]",
            ),
            "failed",
        ),
    ] {
        b.write("src/lib.rs", &source);
        let out = native("proof");
        assert_eq!(out.status.code(), Some(1));
        assert_eq!(report(&out)["complete"], false);
        let out = native("verify");
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(text.contains(&format!("formal [proof]: {state}")), "{text}");
    }
    b.write("src/lib.rs", LIB);
    assert!(native("proof").status.success());
}
