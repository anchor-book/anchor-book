//! CLI entry point reserved for the forthcoming engine adapters.
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
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
    let help = "Anchor Book engine prototype\n\nUsage: anchorbook --help | --version\n\nRecord reading is available through the Rust library.\nThe mdBook adapter, structural checker, and configuration reader are not implemented.\n";
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        return if writeln!(io::stdout().lock(), "{help}").is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    let _ = writeln!(
        io::stderr().lock(),
        "No engine operation is available in this CLI yet.\n{help}"
    );
    ExitCode::from(2)
}
