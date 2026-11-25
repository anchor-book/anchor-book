//! Engine-owned starters for project-owned Book content.
//!
//! Templates are packaged text, not initialized project policy. Retrieval performs
//! no I/O; callers reconcile existing content rather than overwriting it.

/// Markdown starter for the configured project-guidance page.
pub const PROJECT_GUIDANCE: &str = include_str!("../templates/project-guidance.md");
