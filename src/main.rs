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
    if args.first().is_some_and(|a| a == "md") {
        return md(&args[1..]);
    }
    if args.len() == 1 && (args[0] == "--version" || args[0] == "-V") {
        return if writeln!(
            io::stdout().lock(),
            "anchorbook {}",
            env!("CARGO_PKG_VERSION")
        )
        .is_ok()
        {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    let help = "Anchor Book engine prototype\n\nUsage: anchorbook check --config PATH | md [supports html] | --help | --version\n\nRecord reading is available through the Rust library.\nThe mdBook 0.5.2 HTML adapter is available through md.\nStructural checking and configuration loading are available.\nMaterialization and contract generation are not implemented.\n";
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
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut request = String::new();
        io::stdin().read_to_string(&mut request)?;
        let output = anchor_book::adapters::mdbook::preprocess(&request)?;
        writeln!(io::stdout().lock(), "{output}")?;
        Ok(())
    })();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "{error}");
            ExitCode::FAILURE
        }
    }
}

fn check(args: &[std::ffi::OsString]) -> ExitCode {
    if args.len() != 2 || args[0] != "--config" || args[1].is_empty() {
        let _ = writeln!(io::stderr().lock(), "Usage: anchorbook check --config PATH");
        return ExitCode::from(2);
    }
    let report = match anchor_book::checking::check(std::path::Path::new(&args[1])) {
        Ok(report) => report,
        Err(error) => error.into_report(),
    };
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
