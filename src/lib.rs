//! Configure and check Book structure, read, validate, and render design records,
//! and validate and render roadmap milestones.
//!
//! Record and rendering operations use supplied Markdown without filesystem access.
//! Configuration and structural checking inspect explicit files without writes.
//! No operation executes subprocesses or verifies the system being described.
//! A successful inventory establishes schema and authority-reference consistency.
//!
//! ```
//! use anchor_book::{Chapter, read_records};
//! use std::path::Path;
//!
//! let chapters = [Chapter::new(Path::new("intro.md"), "# Introduction")];
//! let inventory = read_records(&chapters)?;
//! assert!(inventory.is_empty());
//! # Ok::<(), anchor_book::RecordErrors>(())
//! ```
// Verus's attributes on loops need this; only verification enables the feature.
#![cfg_attr(feature = "verus", feature(proc_macro_hygiene))]

pub mod adapters;
pub mod checking;
pub mod configuration;
pub mod contract;
pub mod records;
pub mod rendering;
pub mod roadmaps;

// Preserve the initial API while offering a capability-scoped namespace.
pub use records::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, Decision, Diagnostic,
    DiagnosticCode, Problem, Record, RecordData, RecordErrors, RecordInventory, RecordKind,
    Requirement, SourceLocation, Status, VerificationMethod, read_records,
};

/// Deliberately untested helper for the required-checks demonstration; never merged.
pub fn demonstration_offset(value: u32) -> u32 {
    value.saturating_add(1)
}
