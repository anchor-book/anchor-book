//! The reusable AI Working Contract packaged with this engine version.
//!
//! The contract is maintained as section files under `contract/` and joined at
//! compile time in the order below, so the printed text is exactly what this engine
//! version was built with. It contains no project-specific content; projects keep
//! their state, paths, commands, and policies in their own Book.

/// The complete contract text, identical for every project.
pub const TEXT: &str = concat!(
    include_str!("../contract/session.md"),
    include_str!("../contract/collaboration.md"),
    include_str!("../contract/roadmaps.md"),
    include_str!("../contract/testing.md"),
    include_str!("../contract/alignment.md"),
    include_str!("../contract/verification.md"),
    include_str!("../contract/writing.md"),
    include_str!("../contract/project.md"),
);

/// Section files joined into [`TEXT`], in order, relative to `contract/`.
pub const SECTIONS: [&str; 8] = [
    "session.md",
    "collaboration.md",
    "roadmaps.md",
    "testing.md",
    "alignment.md",
    "verification.md",
    "writing.md",
    "project.md",
];

/// The contract for this engine version. It reads no files.
///
/// ```
/// assert!(anchor_book::contract::text().starts_with("# AI Working Contract"));
/// ```
pub fn text() -> &'static str {
    TEXT
}
