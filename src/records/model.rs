use std::path::Path;

use serde::Deserialize;

use super::SourceLocation;

/// A caller-supplied chapter. The path is an identity/location, never opened here.
#[derive(Clone, Copy, Debug)]
pub struct Chapter<'a> {
    /// Source path, preferably relative to the caller's Book source directory.
    pub path: &'a Path,
    /// Complete Markdown source, before record rendering.
    pub content: &'a str,
}

impl<'a> Chapter<'a> {
    /// Associate Markdown with its source path without reading or changing files.
    pub fn new(path: &'a Path, content: &'a str) -> Self {
        Self { path, content }
    }
}

/// The kind declared by a canonical definition fence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RecordKind {
    /// A property of the intended system.
    Anchor,
    /// A required outcome or constraint.
    Requirement,
    /// A design choice and its rationale.
    Decision,
}

/// Acceptance of a design record, not implementation or verification status.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Still under design review.
    Draft,
    /// Accepted into the intended design.
    Accepted,
}

/// Allowed authority targets of an anchor.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum AuthorityKind {
    /// A requirement record.
    Requirement,
    /// A decision record.
    Decision,
}

impl From<AuthorityKind> for RecordKind {
    fn from(value: AuthorityKind) -> Self {
        match value {
            AuthorityKind::Requirement => Self::Requirement,
            AuthorityKind::Decision => Self::Decision,
        }
    }
}

/// An authored authority edge. Its target is resolved before inventory creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityReference {
    /// Required target kind.
    pub kind: AuthorityKind,
    /// Target's Book-wide identity.
    pub id: String,
    /// Location of the authored reference, not the target.
    pub location: SourceLocation,
}

/// A verification obligation, identified within its anchor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Criterion {
    /// Local identity; pair it with the anchor ID for an unambiguous reference.
    pub id: String,
    /// What verification must establish.
    pub statement: String,
}

/// An anchor's kind-specific fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Anchor {
    /// The complete property, including qualifications.
    pub property: String,
    /// Direct authored authority references, in source order.
    pub authorities: Vec<AuthorityReference>,
    /// Verification criteria, in source order.
    pub criteria: Vec<Criterion>,
}

/// A requirement's kind-specific fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requirement {
    /// Required outcome or constraint.
    pub statement: String,
    /// Authored explanation of provenance, not proof of approval.
    pub source: String,
}

/// A decision's kind-specific fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decision {
    /// Selected design approach.
    pub choice: String,
    /// Direct requirement references; may be empty.
    pub requirements: Vec<AuthorityReference>,
    /// Reason for the choice.
    pub rationale: String,
    /// Alternatives and tradeoffs.
    pub alternatives: Vec<String>,
    /// Conditions under which to revisit the decision.
    pub reconsider_when: String,
}

/// Typed payload of a validated record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordData {
    /// An anchor definition.
    Anchor(Anchor),
    /// A requirement definition.
    Requirement(Requirement),
    /// A design decision definition.
    Decision(Decision),
}

/// A validated definition. Obtained only through a successful record inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) status: Status,
    pub(super) data: RecordData,
    pub(super) location: SourceLocation,
    pub(super) body: SourceLocation,
}

impl Record {
    /// Stable identity, unique across all three kinds in the inventory.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Human-readable title.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Authored design-acceptance state.
    pub fn status(&self) -> Status {
        self.status
    }
    /// Kind-specific data without parser or renderer dependencies.
    pub fn data(&self) -> &RecordData {
        &self.data
    }
    /// Full definition range, including the fence lines, for later replacement.
    pub fn location(&self) -> &SourceLocation {
        &self.location
    }
    /// TOML body range, excluding both fence lines.
    pub fn body_location(&self) -> &SourceLocation {
        &self.body
    }
    /// The record kind.
    pub fn kind(&self) -> RecordKind {
        match self.data {
            RecordData::Anchor(_) => RecordKind::Anchor,
            RecordData::Requirement(_) => RecordKind::Requirement,
            RecordData::Decision(_) => RecordKind::Decision,
        }
    }
    /// Direct authored relationships, without transitive or reverse edges.
    pub fn references(&self) -> &[AuthorityReference] {
        match &self.data {
            RecordData::Anchor(a) => &a.authorities,
            RecordData::Decision(d) => &d.requirements,
            RecordData::Requirement(_) => &[],
        }
    }
}
