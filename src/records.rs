//! Canonical Book records: parsing, typed data, identity, and authority validation.
//!
//! This capability accepts supplied chapter text and returns a validated inventory
//! or located errors. Its implementation is independent of filesystem access,
//! presentation, CLI invocation, and mdBook's protocol.

mod diagnostics;
mod inventory;
mod markdown;
mod model;
mod schema;

pub use diagnostics::{Diagnostic, DiagnosticCode, Problem, RecordErrors, SourceLocation};
pub use inventory::{RecordInventory, read_records};
pub use model::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, Decision, Record, RecordData,
    RecordKind, Requirement, Status,
};
