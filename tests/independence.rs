//! The engine stays buildable and testable when this Book is broken.
use std::process::Command;

/// anchorbook: A-check-independent-build#broken-book
#[test]
#[ignore = "copies the engine and runs its tests several times; run explicitly with --ignored"]
fn the_engine_builds_and_tests_with_a_broken_or_absent_book() {
    let script = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/scripts/check-engine-independence.sh"
    );
    let output = Command::new(script).output().unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Engine independence and Book acceptance experiments passed."));
}
