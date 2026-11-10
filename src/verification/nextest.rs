//! The test runner: runs exactly the linked tests in scope with cargo-nextest and
//! reads that run's JUnit report.
//!
//! Each package with linked tests is run once with an exact filter, ignored tests
//! included. A tool configuration under `[verification] output` selects the
//! profile and puts nextest's report store there, so the project's own nextest
//! configuration is untouched. The `cargo` named by the `CARGO` environment
//! variable is used, as Cargo itself does for subcommands, or else `cargo`.
use super::{FunctionId, Plan, RunnerResult, Target, TestOutcome, identity};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// The nextest release this engine is tested against.
pub const NEXTEST_VERSION: &str = "0.9.151";
const PROFILE: &str = "anchorbook-verify";

/// nextest's binary identity for a crate target.
fn binary_id(function: &FunctionId) -> String {
    match &function.target {
        Target::Lib => function.package.clone(),
        Target::Bin(name) => format!("{}::bin/{name}", function.package),
        Target::Test(name) => format!("{}::{name}", function.package),
    }
}

/// A filterset selecting exactly `tests`.
pub(super) fn filter(tests: &[&FunctionId]) -> String {
    tests
        .iter()
        .map(|f| format!("(binary_id(={}) & test(={}))", binary_id(f), f.path()))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Unescape the XML entities nextest writes in attribute values.
fn unescape(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

fn attribute(tag: &str, name: &str) -> Option<String> {
    let start = tag.find(&format!(" {name}=\""))? + name.len() + 3;
    let end = start + tag[start..].find('"')?;
    Some(unescape(&tag[start..end]))
}

/// Test cases in a JUnit report, by binary identity and test name. A case with
/// a `failure` or `error` element failed, one with `skipped` was skipped, and
/// any other case passed. A case reported more than once is left out, so it
/// has no evidence.
pub(super) fn parse_junit(xml: &str) -> BTreeMap<(String, String), TestOutcome> {
    let mut cases: BTreeMap<(String, String), Option<TestOutcome>> = BTreeMap::new();
    for chunk in xml.split("<testcase ").skip(1) {
        let Some((tag, after)) = chunk.split_once('>') else {
            continue;
        };
        // A case's children run to its closing tag; a self-closing case has none.
        let body = if tag.ends_with('/') {
            ""
        } else {
            after.split("</testcase>").next().unwrap_or_default()
        };
        let outcome = if body.contains("<failure") || body.contains("<error") {
            TestOutcome::Failed
        } else if body.contains("<skipped") {
            TestOutcome::Skipped
        } else {
            TestOutcome::Passed
        };
        let tag = format!(" {tag}");
        if let (Some(class), Some(name)) = (attribute(&tag, "classname"), attribute(&tag, "name")) {
            cases
                .entry((class, name))
                .and_modify(|seen| *seen = None)
                .or_insert(Some(outcome));
        }
    }
    cases
        .into_iter()
        .filter_map(|(key, outcome)| outcome.map(|o| (key, o)))
        .collect()
}

/// Files whose contents define the run's inputs: the configured sources and the
/// manifests and lockfiles of the packages with linked tests.
pub(super) fn input_paths(plan: &Plan) -> Vec<PathBuf> {
    let mut paths = plan.settings.sources.clone();
    for manifest in plan.manifests.values() {
        paths.push(manifest.clone());
        let mut directory = manifest.parent();
        while let Some(dir) = directory {
            let lock = dir.join("Cargo.lock");
            if lock.is_file() {
                paths.push(lock);
                break;
            }
            if dir == plan.base {
                break;
            }
            directory = dir.parent();
        }
    }
    paths
}

fn cargo() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}

/// The installed nextest's version line, or `None` when it cannot run.
pub(super) fn version() -> Option<String> {
    let output = Command::new(cargo())
        .args(["nextest", "--version"])
        .output()
        .ok()
        .filter(|o| o.status.success())?;
    String::from_utf8(output.stdout)
        .ok()?
        .lines()
        .next()
        .map(str::to_owned)
}

/// Run the plan's linked tests. Returns the result and the nextest version used.
pub(super) fn run(plan: &Plan) -> (RunnerResult<TestOutcome>, Option<String>) {
    if !plan.settings.runners.iter().any(|r| r == "nextest") {
        return (
            RunnerResult::NotRun("nextest is not listed in [verification] runners".into()),
            None,
        );
    }
    if plan.tests.is_empty() {
        return (RunnerResult::Ran(BTreeMap::new()), None);
    }
    let Some(version) = version() else {
        return (
            RunnerResult::NotRun(format!(
                "cargo-nextest is not installed; install it with `cargo install cargo-nextest --version {NEXTEST_VERSION} --locked`"
            )),
            None,
        );
    };
    let result = run_packages(plan);
    (result, Some(version))
}

fn run_packages(plan: &Plan) -> RunnerResult<TestOutcome> {
    let directory = plan.settings.output.join("nextest");
    let store = directory.join("store");
    let tool = directory.join("tool.toml");
    let report = store.join(PROFILE).join("junit.xml");
    let written = fs::create_dir_all(&directory).and_then(|()| {
        fs::write(
            &tool,
            format!(
                "# Written by anchorbook verify for its own runs.\n[store]\ndir = {}\n\n[profile.{PROFILE}]\nfail-fast = false\n\n[profile.{PROFILE}.junit]\npath = \"junit.xml\"\n",
                toml_string(&store.to_string_lossy())
            ),
        )
    });
    if let Err(error) = written {
        return RunnerResult::Unusable(format!(
            "cannot write runner output under {}: {error}",
            directory.display()
        ));
    }
    let inputs = input_paths(plan);
    let before = identity::snapshot(&inputs);
    let mut by_package: BTreeMap<&str, Vec<&FunctionId>> = BTreeMap::new();
    for test in &plan.tests {
        by_package.entry(&test.package).or_default().push(test);
    }
    let mut outcomes = BTreeMap::new();
    for (package, tests) in by_package {
        let Some(manifest) = plan.manifests.get(package) else {
            continue;
        };
        // An earlier report must never be read as this run's.
        let _ = fs::remove_file(&report);
        let log = directory.join(format!("{package}.log"));
        let mut tool_option = OsString::from("anchorbook:");
        tool_option.push(&tool);
        let output = Command::new(cargo())
            .current_dir(&plan.base)
            .args(["nextest", "run", "--manifest-path"])
            .arg(manifest)
            .args([
                "--locked",
                "--no-fail-fast",
                "--no-tests=pass",
                "--run-ignored",
                "all",
                "--profile",
                PROFILE,
                "--tool-config-file",
            ])
            .arg(&tool_option)
            .arg("-E")
            .arg(filter(&tests))
            .output();
        let output = match output {
            Ok(output) => output,
            Err(error) => {
                return RunnerResult::Unusable(format!("cannot start cargo nextest: {error}"));
            }
        };
        let mut text = output.stdout.clone();
        text.extend(&output.stderr);
        let _ = fs::write(&log, &text);
        let Ok(xml) = fs::read_to_string(&report) else {
            return RunnerResult::Unusable(format!(
                "cargo nextest for package `{package}` produced no report (exit {}); see {}",
                output
                    .status
                    .code()
                    .map_or_else(|| "by signal".to_owned(), |c| c.to_string()),
                display(&log, &plan.base)
            ));
        };
        let cases = parse_junit(&xml);
        for test in tests {
            if let Some(outcome) = cases.get(&(binary_id(test), test.path())) {
                outcomes.insert(test.clone(), *outcome);
            }
        }
    }
    if identity::snapshot(&inputs) != before {
        return RunnerResult::Unusable(
            "sources, manifests, or lockfiles changed during the run, so its results may not describe the current files".into(),
        );
    }
    RunnerResult::Ran(outcomes)
}

/// A TOML basic string holding `value`.
fn toml_string(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{:04X}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn display(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{TestOutcome, parse_junit};

    #[test]
    fn paths_become_toml_strings_that_read_back_unchanged() {
        for path in ["/tmp/a b", "C:\\dir\\x", "quote\"d", "tab\tnew\nline", "é"] {
            let text = format!("dir = {}\n", super::toml_string(path));
            let document = toml_edit::Document::parse(text.as_str()).unwrap();
            assert_eq!(document.get("dir").unwrap().as_str(), Some(path), "{text}");
        }
    }

    #[test]
    fn junit_cases_map_to_outcomes_and_repeated_cases_have_none() {
        let xml = r#"<testsuites><testsuite name="p::t">
<testcase name="a::passes" classname="p::t" time="0.1"/>
<testcase name="a::fails" classname="p::t"><failure type="test failure">boom</failure></testcase>
<testcase name="errors" classname="p::t"><error message="x"/></testcase>
<testcase name="skipped" classname="p"><skipped/></testcase>
<testcase name="output" classname="p"><system-out>ok</system-out></testcase>
<testcase name="after-closed" classname="p"/><skipped/>
<testcase name="twice" classname="p"/>
<testcase name="twice" classname="p"/>
<testcase name="escaped&amp;" classname="p&lt;x&gt;"/>
</testsuite></testsuites>"#;
        let cases = parse_junit(xml);
        let get =
            |class: &str, name: &str| cases.get(&(class.to_owned(), name.to_owned())).copied();
        assert_eq!(get("p::t", "a::passes"), Some(TestOutcome::Passed));
        assert_eq!(get("p::t", "a::fails"), Some(TestOutcome::Failed));
        assert_eq!(get("p::t", "errors"), Some(TestOutcome::Failed));
        assert_eq!(get("p", "skipped"), Some(TestOutcome::Skipped));
        assert_eq!(get("p", "output"), Some(TestOutcome::Passed));
        assert_eq!(get("p", "twice"), None);
        assert_eq!(get("p<x>", "escaped&"), Some(TestOutcome::Passed));
        // A self-closing case owns no following elements.
        assert_eq!(get("p", "after-closed"), Some(TestOutcome::Passed));
        assert_eq!(cases.len(), 7);
        assert!(parse_junit("").is_empty());
        assert!(parse_junit("<testcase name=\"x\"").is_empty());
    }
}
