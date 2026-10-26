//! Resolve Book configuration and read, validate, and render canonical design records.
//!
//! Record and rendering operations use supplied Markdown without filesystem access.
//! The configuration loader reads explicit files and resolves paths without writes.
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

pub mod adapters;
pub mod configuration;
pub mod records;
pub mod rendering;

// Preserve the initial API while offering a capability-scoped namespace.
pub use records::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, Decision, Diagnostic,
    DiagnosticCode, Problem, Record, RecordData, RecordErrors, RecordInventory, RecordKind,
    Requirement, SourceLocation, Status, VerificationMethod, read_records,
};
