//! Process-boundary checks for the deliberately limited CLI entry point.
use std::process::Command;

#[test]
fn help_and_version_describe_the_actual_scope() {
    let help = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    let text = String::from_utf8(help.stdout).unwrap();
    assert!(text.contains("Rust library"));
    assert!(text.contains("not implemented"));
    let version = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
        .arg("--version")
        .output()
        .unwrap();
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        format!("anchorbook {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn unavailable_operations_never_report_success() {
    for args in [
        vec![],
        vec!["check"],
        vec!["md", "supports", "html"],
        vec!["materialize"],
        vec!["--help", "check"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_anchorbook"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("No engine operation is available")
        );
    }
}
