//! Configure and check Book structure, read, validate, and render design records,
//! validate and render roadmap milestones, and report evidence for anchor criteria.
//!
//! Record and rendering operations use supplied Markdown without filesystem access.
//! Configuration and structural checking inspect explicit files without writes.
//! [`verification`] reads sources, asks `git` for the revision, and runs linked
//! tests with cargo-nextest, writing only under its configured output directory;
//! no other operation writes files or runs the project's tools.
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
pub mod verification;

// Preserve the initial API while offering a capability-scoped namespace.
pub use records::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, Decision, Diagnostic,
    DiagnosticCode, Problem, Record, RecordData, RecordErrors, RecordInventory, RecordKind,
    Requirement, SourceLocation, Status, VerificationMethod, read_records,
};
