use std::{collections::BTreeSet, ops::Range};

use serde::Deserialize;
use toml_edit::{Document, Item};

use super::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, Decision, Diagnostic, Problem,
    Record, RecordData, RecordKind, Requirement, SourceLocation, Status,
};

pub(super) struct Candidate {
    pub id: Option<String>,
    pub record: Option<Record>,
    pub location: SourceLocation,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RawAnchor {
    schema_version: i64,
    id: String,
    title: String,
    status: Status,
    property: String,
    authorities: Vec<RawAuthority>,
    criteria: Vec<RawCriterion>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAuthority {
    kind: AuthorityKind,
    id: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCriterion {
    id: String,
    statement: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RawRequirement {
    schema_version: i64,
    id: String,
    title: String,
    status: Status,
    statement: String,
    source: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RawDecision {
    schema_version: i64,
    id: String,
    title: String,
    status: Status,
    choice: String,
    requirements: Vec<String>,
    rationale: String,
    alternatives: Vec<String>,
    reconsider_when: String,
}

pub(super) fn identifier(value: &str) -> bool {
    value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && value.split('-').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

struct Context<'a> {
    chapter: Chapter<'a>,
    body: Range<usize>,
    document: Document<&'a str>,
}
impl Context<'_> {
    fn location(&self, span: Option<Range<usize>>) -> SourceLocation {
        let span = span.unwrap_or(0..0);
        SourceLocation::new(
            self.chapter.path,
            self.chapter.content,
            self.body.start + span.start..self.body.start + span.end,
        )
    }
    fn field_location(&self, field: &str) -> SourceLocation {
        self.location(self.document.get(field).and_then(Item::span))
    }
    fn invalid(&self, field: &str, message: impl Into<String>) -> Box<Diagnostic> {
        Box::new(Diagnostic::new(
            Problem::InvalidDefinition,
            self.field_location(field),
            message,
        ))
    }
    fn nonempty(&self, field: &str, value: &str) -> Result<(), Box<Diagnostic>> {
        if value.trim().is_empty() {
            Err(self.invalid(field, format!("`{field}` must not be empty")))
        } else {
            Ok(())
        }
    }
    fn common(&self, version: i64, id: &str, title: &str) -> Result<(), Box<Diagnostic>> {
        if version != 1 {
            return Err(self.invalid("schema-version", "supported schema-version is 1"));
        }
        if !identifier(id) {
            return Err(self.invalid(
                "id",
                "record id must use lowercase words separated by single hyphens",
            ));
        }
        self.nonempty("title", title)
    }
    fn decode<T: serde::de::DeserializeOwned>(&self) -> Result<T, Box<Diagnostic>> {
        T::deserialize(toml_edit::de::Deserializer::from(self.document.clone())).map_err(|error| {
            let mut diagnostic = Diagnostic::new(
                Problem::InvalidDefinition,
                self.location(error.span()),
                error.message(),
            );
            diagnostic.cause = Some(Box::new(error));
            Box::new(diagnostic)
        })
    }
    fn reference(
        &self,
        field: &str,
        index: usize,
        id: String,
        kind: AuthorityKind,
    ) -> Result<AuthorityReference, Box<Diagnostic>> {
        let value = self
            .document
            .get(field)
            .and_then(Item::as_array)
            .and_then(|a| a.get(index));
        let location = self.location(value.and_then(|v| {
            v.as_inline_table()
                .and_then(|t| t.get("id"))
                .and_then(|v| v.span())
                .or_else(|| v.span())
        }));
        if !identifier(&id) {
            return Err(Box::new(Diagnostic::new(
                Problem::InvalidDefinition,
                location,
                format!("`{field}` contains an invalid authority identifier"),
            )));
        }
        Ok(AuthorityReference { kind, id, location })
    }
    fn record(
        &self,
        kind: RecordKind,
        location: SourceLocation,
    ) -> Result<Record, Box<Diagnostic>> {
        let (id, title, status, data) = match kind {
            RecordKind::Anchor => {
                let raw: RawAnchor = self.decode()?;
                self.common(raw.schema_version, &raw.id, &raw.title)?;
                self.nonempty("property", &raw.property)?;
                let inline = self
                    .document
                    .get("authorities")
                    .and_then(Item::as_array)
                    .is_some_and(|a| a.iter().all(|v| v.is_inline_table()));
                if raw.authorities.is_empty() || !inline {
                    return Err(self.invalid(
                        "authorities",
                        "authorities must be a nonempty array of inline tables",
                    ));
                }
                if raw.criteria.is_empty()
                    || !self
                        .document
                        .get("criteria")
                        .is_some_and(Item::is_array_of_tables)
                {
                    return Err(self.invalid(
                        "criteria",
                        "criteria must be a nonempty sequence of [[criteria]] tables",
                    ));
                }
                let mut seen = BTreeSet::new();
                let mut criteria = Vec::new();
                for c in raw.criteria {
                    if !identifier(&c.id) || !seen.insert(c.id.clone()) {
                        return Err(self.invalid(
                            "criteria",
                            "criterion identifiers must be valid and unique within the anchor",
                        ));
                    }
                    self.nonempty("criteria", &c.statement)?;
                    criteria.push(Criterion {
                        id: c.id,
                        statement: c.statement,
                    });
                }
                let authorities = raw
                    .authorities
                    .into_iter()
                    .enumerate()
                    .map(|(i, a)| self.reference("authorities", i, a.id, a.kind))
                    .collect::<Result<_, _>>()?;
                (
                    raw.id,
                    raw.title,
                    raw.status,
                    RecordData::Anchor(Anchor {
                        property: raw.property,
                        authorities,
                        criteria,
                    }),
                )
            }
            RecordKind::Requirement => {
                let raw: RawRequirement = self.decode()?;
                self.common(raw.schema_version, &raw.id, &raw.title)?;
                self.nonempty("statement", &raw.statement)?;
                self.nonempty("source", &raw.source)?;
                (
                    raw.id,
                    raw.title,
                    raw.status,
                    RecordData::Requirement(Requirement {
                        statement: raw.statement,
                        source: raw.source,
                    }),
                )
            }
            RecordKind::Decision => {
                let raw: RawDecision = self.decode()?;
                self.common(raw.schema_version, &raw.id, &raw.title)?;
                for (field, value) in [
                    ("choice", &raw.choice),
                    ("rationale", &raw.rationale),
                    ("reconsider-when", &raw.reconsider_when),
                ] {
                    self.nonempty(field, value)?;
                }
                if raw.alternatives.is_empty() {
                    return Err(self.invalid("alternatives", "alternatives must not be empty"));
                }
                for alternative in &raw.alternatives {
                    self.nonempty("alternatives", alternative)?;
                }
                let requirements = raw
                    .requirements
                    .into_iter()
                    .enumerate()
                    .map(|(i, id)| {
                        self.reference("requirements", i, id, AuthorityKind::Requirement)
                    })
                    .collect::<Result<_, _>>()?;
                (
                    raw.id,
                    raw.title,
                    raw.status,
                    RecordData::Decision(Decision {
                        choice: raw.choice,
                        requirements,
                        rationale: raw.rationale,
                        alternatives: raw.alternatives,
                        reconsider_when: raw.reconsider_when,
                    }),
                )
            }
        };
        Ok(Record {
            id,
            title,
            status,
            data,
            location,
            body: SourceLocation::new(self.chapter.path, self.chapter.content, self.body.clone()),
        })
    }
}

pub(super) fn parse(
    chapter: Chapter<'_>,
    kind: RecordKind,
    range: Range<usize>,
    body: Range<usize>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Candidate {
    let location = SourceLocation::new(chapter.path, chapter.content, range);
    let document = match Document::parse(&chapter.content[body.clone()]) {
        Ok(doc) => doc,
        Err(error) => {
            let span = error.span().unwrap_or(0..0);
            let mut diagnostic = Diagnostic::new(
                Problem::InvalidDefinition,
                SourceLocation::new(
                    chapter.path,
                    chapter.content,
                    body.start + span.start..body.start + span.end,
                ),
                error.message(),
            );
            diagnostic.cause = Some(Box::new(error));
            diagnostics.push(diagnostic);
            return Candidate {
                id: None,
                record: None,
                location,
            };
        }
    };
    // Retain a trustworthy identity even when other fields are invalid, so a
    // dependent edge reports invalid authority instead of a fabricated absence.
    let id = document
        .get("id")
        .and_then(Item::as_str)
        .filter(|s| identifier(s))
        .map(str::to_owned);
    let context = Context {
        chapter,
        body,
        document,
    };
    let record = match context.record(kind, location.clone()) {
        Ok(record) => Some(record),
        Err(error) => {
            diagnostics.push(*error);
            None
        }
    };
    Candidate {
        id,
        record,
        location,
    }
}
