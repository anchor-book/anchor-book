//! CLI invocation, streams, and exit statuses for engine adapters.
use std::{
    io::{self, Read, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
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
    let help = "Anchor Book engine prototype\n\nUsage: anchorbook md [supports html] | --help | --version\n\nRecord reading is available through the Rust library.\nThe mdBook 0.5.2 HTML adapter is available through md.\nThe structural checker and configuration reader are not implemented.\n";
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
