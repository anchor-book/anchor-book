//! CLI invocation, streams, and exit statuses for engine adapters.
use std::{
    io::{self, Read, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_some_and(|a| a == "check") {
        return check(&args[1..]);
    }
    if args.first().is_some_and(|a| a == "verify") {
        return verify(&args[1..]);
    }
    if args.first().is_some_and(|a| a == "md") {
        return md(&args[1..]);
    }
    if args.first().is_some_and(|a| a == "contract") {
        if args.len() != 1 {
            let _ = writeln!(io::stderr().lock(), "Usage: anchorbook contract");
            return ExitCode::from(2);
        }
        return if io::stdout()
            .lock()
            .write_all(anchor_book::contract::text().as_bytes())
            .is_ok()
        {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    if args.len() == 1 && (args[0] == "--version" || args[0] == "-V") {
        return if writeln!(
            io::stdout().lock(),
            "anchorbook {} ({})",
            env!("CARGO_PKG_VERSION"),
            match env!("ANCHORBOOK_REVISION") {
                "" => "revision unknown",
                revision => revision,
            }
        )
        .is_ok()
        {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    let help = "Anchor Book engine prototype\n\nUsage: anchorbook check --config PATH | md [supports html] | contract | --help | --version\n\nRecord reading is available through the Rust library.\nThe mdBook 0.5.4 HTML adapter is available through md; it checks structure before rendering.\nStructural checking and configuration loading are available.\ncontract prints the AI Working Contract packaged with this engine version.\nMaterialization is not implemented.\n";
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        return if writeln!(io::stdout().lock(), "{help}").is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    let _ = writeln!(io::stderr().lock(), "Unknown or missing operation.\n{help}");
    ExitCode::from(2)
}

fn md(args: &[std::ffi::OsString]) -> ExitCode {
    if args.len() == 2 && args[0] == "supports" {
        return if args[1] == "html" {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    if !args.is_empty() {
        let _ = writeln!(io::stderr().lock(), "Usage: anchorbook md [supports html]");
        return ExitCode::from(2);
    }
    // stdout carries only Book JSON; mdBook stops the build on any failure status.
    let result = (|| -> Result<ExitCode, Box<dyn std::error::Error>> {
        let mut request = String::new();
        io::stdin().read_to_string(&mut request)?;
        let config = anchor_book::adapters::mdbook::configuration_path(&request)?;
        let report = structural_report(&config);
        let outcome = report.outcome();
        if outcome != anchor_book::checking::Outcome::Passed {
            let text = anchor_book::adapters::check::format_report(&report);
            io::stderr().lock().write_all(text.as_bytes())?;
            return Ok(ExitCode::from(outcome.exit_code()));
        }
        // The check passed, so the configuration loads; its roadmap role names the
        // chapter whose subtree holds the Book's roadmaps.
        let configuration = anchor_book::configuration::load(&config)?;
        let roadmaps = configuration
            .section(anchor_book::configuration::SectionRole::ImplementationRoadmaps)
            .strip_prefix(configuration.source())
            .ok()
            .map(std::path::Path::to_path_buf);
        let output =
            anchor_book::adapters::mdbook::preprocess_with_roadmaps(&request, roadmaps.as_deref())?;
        writeln!(io::stdout().lock(), "{output}")?;
        Ok(ExitCode::SUCCESS)
    })();
    match result {
        Ok(code) => code,
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "{error}");
            ExitCode::FAILURE
        }
    }
}

fn structural_report(config: &std::path::Path) -> anchor_book::checking::CheckReport {
    match anchor_book::checking::check(config) {
        Ok(report) => report,
        Err(error) => error.into_report(),
    }
}

fn check(args: &[std::ffi::OsString]) -> ExitCode {
    if args.len() != 2 || args[0] != "--config" || args[1].is_empty() {
        let _ = writeln!(io::stderr().lock(), "Usage: anchorbook check --config PATH");
        return ExitCode::from(2);
    }
    let report = structural_report(std::path::Path::new(&args[1]));
    let outcome = report.outcome();
    let text = anchor_book::adapters::check::format_report(&report);
    let written = if outcome == anchor_book::checking::Outcome::Incomplete {
        io::stderr().lock().write_all(text.as_bytes())
    } else {
        io::stdout().lock().write_all(text.as_bytes())
    };
    if written.is_err() {
        ExitCode::from(2)
    } else {
        ExitCode::from(outcome.exit_code())
    }
}

fn verify(args: &[std::ffi::OsString]) -> ExitCode {
    use anchor_book::{VerificationMethod, verification};
    let usage =
        "Usage: anchorbook verify --config PATH [--anchor ID]... [--method test|proof|review]...";
    let mut config = None;
    let mut scope = verification::Scope::default();
    let mut rest = args.iter();
    while let Some(flag) = rest.next() {
        let value = rest
            .next()
            .and_then(|v| v.to_str())
            .filter(|v| !v.is_empty());
        match (flag.to_str(), value) {
            (Some("--config"), Some(path)) if config.is_none() => config = Some(path.to_owned()),
            (Some("--anchor"), Some(id)) => scope.anchors.push(id.to_owned()),
            (Some("--method"), Some(method)) => match method {
                "test" => scope.methods.push(VerificationMethod::Test),
                "proof" => scope.methods.push(VerificationMethod::Proof),
                "review" => scope.methods.push(VerificationMethod::Review),
                _ => {
                    let _ = writeln!(io::stderr().lock(), "{usage}");
                    return ExitCode::from(2);
                }
            },
            _ => {
                let _ = writeln!(io::stderr().lock(), "{usage}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(config) = config else {
        let _ = writeln!(io::stderr().lock(), "{usage}");
        return ExitCode::from(2);
    };
    match verification::verify(std::path::Path::new(&config), scope) {
        Ok(report) => {
            let text = anchor_book::adapters::verify::format_report(&report);
            if io::stdout().lock().write_all(text.as_bytes()).is_err() {
                return ExitCode::from(2);
            }
            ExitCode::from(report.exit_code())
        }
        Err(error) => {
            let _ = writeln!(
                io::stderr().lock(),
                "anchorbook verify: incomplete\n{error}"
            );
            ExitCode::from(2)
        }
    }
}
