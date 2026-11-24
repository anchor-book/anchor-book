//! The reusable AI Working Contract packaged with this engine version.
//!
//! The contract is maintained as section files under `contract/` and joined at
//! compile time in the order below, so the printed text is exactly what this engine
//! version was built with. It contains no project-specific content; projects keep
//! their state, paths, commands, and policies in their own Book.

/// The common contract text, identical for every project.
pub const TEXT: &str = concat!(
    include_str!("../contract/session.md"),
    include_str!("../contract/guidance.md"),
    include_str!("../contract/collaboration.md"),
    include_str!("../contract/reverse-engineering.md"),
    include_str!("../contract/roadmaps.md"),
    include_str!("../contract/testing.md"),
    include_str!("../contract/alignment.md"),
    include_str!("../contract/verification.md"),
    include_str!("../contract/writing.md"),
    include_str!("../contract/project.md"),
);

/// Section files joined into [`TEXT`], in order, relative to `contract/`.
pub const SECTIONS: [&str; 10] = [
    "session.md",
    "guidance.md",
    "collaboration.md",
    "reverse-engineering.md",
    "roadmaps.md",
    "testing.md",
    "alignment.md",
    "verification.md",
    "writing.md",
    "project.md",
];

/// Additional guidance selected by the port phase, following the common contract.
pub const PORT: &str = include_str!("../contract/port.md");

/// The contract for this engine version. It reads no files.
///
/// ```
/// assert!(anchor_book::contract::text().starts_with("# AI Working Contract"));
/// ```
pub fn text() -> &'static str {
    TEXT
}

/// The common contract plus guidance for the selected phase. It reads no files.
pub fn for_phase(phase: crate::configuration::Phase) -> String {
    let mut text = TEXT.to_owned();
    if phase == crate::configuration::Phase::Port {
        text.push('\n');
        text.push_str(PORT);
    }
    text
}
