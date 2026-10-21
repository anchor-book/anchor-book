//! Read, validate, and render the canonical design records in an Anchor Book.
//!
//! Callers supply Markdown chapters. This library performs no filesystem access,
//! subprocess execution, or verification of the system being described.
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
pub mod records;
pub mod rendering;

// Preserve the initial API while offering a capability-scoped namespace.
pub use records::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, Decision, Diagnostic,
    DiagnosticCode, Problem, Record, RecordData, RecordErrors, RecordInventory, RecordKind,
    Requirement, SourceLocation, Status, read_records,
};
