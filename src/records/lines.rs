//! The line-grammar record reader.
//!
//! A record is one top-level Markdown blockquote whose first line is its heading,
//! ``**`<id>`** (<status>): **<Title>**``, followed by labeled field lines, in the
//! order its kind requires. Every line of a record is accounted for: a line that
//! is not the heading, a field, a continuation paragraph, a list item, a
//! proof-scope item, or a blank quoted line makes the definition invalid at that
//! line. Markdown parsing is used only to find code blocks, whose text is never
//! a record.

use std::{cmp::Ordering, ops::Range};

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

use super::{
    Anchor, AuthorityKind, AuthorityReference, Chapter, Criterion, CriterionReference, Decision,
    Diagnostic, Problem, ProofScope, Record, RecordData, RecordKind, Requirement, SourceLocation,
    Status, VerificationMethod,
};

/// A recognized definition, decoded or not, as the inventory sees it.
pub(super) struct Candidate {
    /// Identity recovered from a well-formed heading, even if later lines are invalid.
    pub id: Option<String>,
    pub record: Option<Record>,
    pub location: SourceLocation,
}

/// The kind a record identifier's code states.
pub(super) fn kind_of(id: &str) -> Option<RecordKind> {
    let (code, name) = id.split_at_checked(2)?;
    let kind = match code {
        "A-" => RecordKind::Anchor,
        "R-" => RecordKind::Requirement,
        "D-" => RecordKind::Decision,
        _ => return None,
    };
    is_name(name).then_some(kind)
}

/// A name: lowercase ASCII words of letters and digits, separated by single
/// hyphens, starting with a letter.
pub(super) fn is_name(name: &str) -> bool {
    name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && name.split('-').all(|word| {
            !word.is_empty()
                && word
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

const IDENTIFIER_FORM: &str = "a record identifier is a kind code (A, R, or D), a hyphen, and a name of lowercase letters and digits in hyphen-separated words starting with a letter, such as `D-utf8-paths`";

struct Line<'a> {
    /// Byte range of the line, without its line ending.
    range: Range<usize>,
    text: &'a str,
}

fn lines(content: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut start = 0;
    for piece in content.split_inclusive('\n') {
        let text = piece.strip_suffix('\n').unwrap_or(piece);
        let text = text.strip_suffix('\r').unwrap_or(text);
        out.push(Line {
            range: start..start + text.len(),
            text,
        });
        start += piece.len();
    }
    out
}

/// Code blocks, fenced blocks labeled as records in the old TOML form, and legacy
/// `{anchor}` markers in text outside code.
type Regions = (Vec<Range<usize>>, Vec<Range<usize>>, Vec<Range<usize>>);
fn code_blocks(content: &str) -> Regions {
    let mut options = Options::empty();
    for extension in [
        Options::ENABLE_TABLES,
        Options::ENABLE_FOOTNOTES,
        Options::ENABLE_STRIKETHROUGH,
        Options::ENABLE_TASKLISTS,
    ] {
        options.insert(extension);
    }
    let mut code = Vec::new();
    let mut toml = Vec::new();
    let mut legacy = Vec::new();
    for (event, range) in Parser::new_ext(content, options).into_offset_iter() {
        if let Event::Text(text) = &event
            && text.contains("{anchor}")
            && !code.iter().any(|r: &Range<usize>| r.contains(&range.start))
        {
            legacy.push(range.clone());
        }
        if let Event::Start(Tag::CodeBlock(kind)) = event {
            if let CodeBlockKind::Fenced(info) = &kind
                && info
                    .split_whitespace()
                    .next()
                    .is_some_and(|label| label.starts_with("anchorbook-"))
            {
                toml.push(range.clone());
            }
            // From the start of its first line, so an indented code block's lines
            // count as code from their indentation on.
            let line_start = content[..range.start].rfind('\n').map_or(0, |i| i + 1);
            code.push(line_start..range.end);
        }
    }
    (code, toml, legacy)
}

pub(super) fn read(chapter: Chapter<'_>, diagnostics: &mut Vec<Diagnostic>) -> Vec<Candidate> {
    let content = chapter.content;
    let (code, toml, legacy) = code_blocks(content);
    let in_code = |line: &Line<'_>| code.iter().any(|r| r.contains(&line.range.start));
    let location = |range: Range<usize>| SourceLocation::new(chapter.path, content, range);
    let mut candidates = Vec::new();
    let reject = |range: Range<usize>,
                  message: &str,
                  candidates: &mut Vec<Candidate>,
                  diagnostics: &mut Vec<Diagnostic>| {
        let location = location(range);
        diagnostics.push(Diagnostic::new(
            Problem::InvalidDefinition,
            location.clone(),
            message,
        ));
        candidates.push(Candidate {
            id: None,
            record: None,
            location,
        });
    };
    for range in toml {
        reject(
            range,
            "TOML record blocks are not supported; write the record as a blockquote whose first line is ``**`<id>`** (<status>): **<Title>**``",
            &mut candidates,
            diagnostics,
        );
    }
    for range in legacy {
        reject(
            range,
            "legacy {anchor} markers are unsupported; write the record as a line-grammar blockquote",
            &mut candidates,
            diagnostics,
        );
    }
    let all = lines(content);
    let mut rest: &[Line<'_>] = &all;
    while let Some(line) = rest.first() {
        if in_code(line) || !line.text.starts_with('>') {
            if !in_code(line) && line.text.trim_start().starts_with("> **`") {
                reject(
                    line.range.clone(),
                    "a record must be a top-level blockquote, not nested in a list or indented",
                    &mut candidates,
                    diagnostics,
                );
            }
            rest = &rest[1..];
            continue;
        }
        let length = rest
            .iter()
            .position(|l| !l.text.starts_with('>') || in_code(l))
            .unwrap_or(rest.len());
        let (block, tail) = rest.split_at(length);
        rest = tail;
        fn quoted<'s>(l: &Line<'s>) -> &'s str {
            l.text.strip_prefix("> ").unwrap_or(&l.text[1..])
        }
        if !quoted(&block[0]).starts_with("**`") {
            for l in block {
                let nested = l.text[1..].trim_start_matches(' ').starts_with('>');
                let deep = l.text.trim_start_matches(['>', ' ']);
                if deep.starts_with("**`") {
                    let message = if nested {
                        "a record must be a top-level blockquote, not nested in another quote"
                    } else {
                        "a record heading must start its own blockquote; separate it from the text above with a blank line"
                    };
                    reject(l.range.clone(), message, &mut candidates, diagnostics);
                }
            }
            continue;
        }
        let end = block[block.len() - 1].range.end;
        let end = if content[end..].starts_with("\r\n") {
            end + 2
        } else if content[end..].starts_with('\n') {
            end + 1
        } else {
            end
        };
        let record_range = block[0].range.start..end;
        let mut reader = RecordReader {
            chapter,
            lines: block,
        };
        let follows = rest.first().filter(|l| !l.text.trim().is_empty());
        let (id, result) = reader.read(record_range.clone());
        let result = match (result, follows) {
            (Ok(_), Some(next)) => Err(Box::new(Diagnostic::new(
                Problem::InvalidDefinition,
                location(next.range.clone()),
                "a record ends at its last `>` line; leave a blank line after it",
            ))),
            (result, _) => result,
        };
        let record = match result {
            Ok(record) => Some(record),
            Err(diagnostic) => {
                diagnostics.push(*diagnostic);
                None
            }
        };
        candidates.push(Candidate {
            id,
            record,
            location: location(record_range),
        });
    }
    candidates
}

/// The labels each kind's fields use, in their required order.
fn labels(kind: RecordKind) -> &'static [&'static str] {
    match kind {
        RecordKind::Anchor => &["Property", "Authority"],
        RecordKind::Requirement => &["Statement", "Source"],
        RecordKind::Decision => &[
            "Choice",
            "Requirements",
            "Rationale",
            "Alternatives",
            "Reconsider when",
        ],
    }
}
const ALL_LABELS: [&str; 9] = [
    "Property",
    "Authority",
    "Statement",
    "Source",
    "Choice",
    "Requirements",
    "Rationale",
    "Alternatives",
    "Reconsider when",
];
const SCOPE_ITEMS: [&str; 4] = ["Domain", "Boundary", "Assumptions", "Exclusions"];

fn kind_name(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Anchor => "an anchor",
        RecordKind::Requirement => "a requirement",
        RecordKind::Decision => "a decision",
    }
}

/// `Label: text` or `Label:` for a known label.
fn split_label(text: &str) -> Option<(&'static str, &str)> {
    ALL_LABELS.iter().find_map(|&label| {
        let rest = text.strip_prefix(label)?.strip_prefix(':')?;
        if rest.is_empty() {
            Some((label, rest))
        } else {
            rest.strip_prefix(' ').map(|rest| (label, rest))
        }
    })
}

/// Code spans in `text` as (content, byte range of the span including backticks).
fn code_spans(text: &str) -> Vec<(&str, Range<usize>)> {
    let mut out = Vec::new();
    let mut at = 0;
    while let Some(open) = text[at..].find('`').map(|i| at + i) {
        let Some(close) = text[open + 1..].find('`').map(|i| open + 1 + i) else {
            break;
        };
        out.push((&text[open + 1..close], open..close + 1));
        at = close + 1;
    }
    out
}

struct Field {
    label: &'static str,
    paragraphs: Vec<String>,
    /// Text after the label on the field's own line, with its byte offset.
    first: (usize, String),
}

struct PendingCriterion {
    criterion: Criterion,
    line: Range<usize>,
    items: Vec<(&'static str, String, Range<usize>, usize)>,
}

struct RecordReader<'a, 'b> {
    chapter: Chapter<'a>,
    lines: &'b [Line<'a>],
}

impl<'a> RecordReader<'a, '_> {
    fn location(&self, range: Range<usize>) -> SourceLocation {
        SourceLocation::new(self.chapter.path, self.chapter.content, range)
    }
    fn invalid(&self, range: Range<usize>, message: impl Into<String>) -> Box<Diagnostic> {
        Box::new(Diagnostic::new(
            Problem::InvalidDefinition,
            self.location(range),
            message,
        ))
    }

    /// Read one record, returning its identity when the heading states one.
    fn read(&mut self, range: Range<usize>) -> (Option<String>, Result<Record, Box<Diagnostic>>) {
        let heading = &self.lines[0];
        let text = heading.text.strip_prefix("> ").unwrap_or("");
        let Some((id, status, title)) = parse_heading(text) else {
            return (
                None,
                Err(self.invalid(
                    heading.range.clone(),
                    "record heading must be exactly ``**`<id>`** (draft|accepted): **<Title>**``",
                )),
            );
        };
        let Some(kind) = kind_of(id) else {
            return (
                None,
                Err(self.invalid(heading.range.clone(), IDENTIFIER_FORM)),
            );
        };
        let id = id.to_owned();
        let status = match status {
            "draft" => Status::Draft,
            "accepted" => Status::Accepted,
            _ => {
                return (
                    Some(id),
                    Err(self.invalid(
                        heading.range.clone(),
                        "record status must be exactly `draft` or `accepted`",
                    )),
                );
            }
        };
        if title.trim().is_empty() {
            return (
                Some(id),
                Err(self.invalid(heading.range.clone(), "record title must not be empty")),
            );
        }
        let result = self.body(kind, &id).map(|data| Record {
            id: id.clone(),
            title: title.to_owned(),
            status,
            data,
            location: self.location(range.clone()),
            body: self
                .location(self.lines.get(1).map_or(range.end, |line| line.range.start)..range.end),
        });
        (Some(id), result)
    }

    fn body(&self, kind: RecordKind, id: &str) -> Result<RecordData, Box<Diagnostic>> {
        let expected = labels(kind);
        let mut fields: Vec<Field> = Vec::new();
        let mut criteria: Vec<PendingCriterion> = Vec::new();
        let mut alternatives: Vec<String> = Vec::new();
        for line in &self.lines[1..] {
            let text = if line.text == ">" {
                ""
            } else if let Some(text) = line.text.strip_prefix("> ") {
                text
            } else {
                return Err(self.invalid(
                    line.range.clone(),
                    "a record line is `>` alone or `> ` followed by its text",
                ));
            };
            let offset = line.range.start + 2;
            if text.trim().is_empty() {
                if !text.is_empty() {
                    return Err(
                        self.invalid(line.range.clone(), "a blank record line is `>` alone")
                    );
                }
                continue;
            }
            if text.starts_with('>') {
                return Err(self.invalid(line.range.clone(), "a record holds no nested quotes"));
            }
            if text.starts_with("**`") {
                return Err(self.invalid(
                    line.range.clone(),
                    "a record heading must start its own blockquote; separate records with a blank line",
                ));
            }
            if let Some(rest) = text.strip_prefix("- Criterion ") {
                if kind != RecordKind::Anchor {
                    return Err(self.invalid(
                        line.range.clone(),
                        format!("criteria belong to anchors, not {}", kind_name(kind)),
                    ));
                }
                if fields.len() < expected.len() {
                    return Err(self.missing(expected[fields.len()], line.range.clone()));
                }
                let criterion = self.criterion(rest, line.range.clone())?;
                if criteria.iter().any(|c| c.criterion.id == criterion.id) {
                    return Err(self.invalid(
                        line.range.clone(),
                        format!(
                            "criterion `{}` is repeated; criterion identifiers are unique within their anchor",
                            criterion.id
                        ),
                    ));
                }
                if let Some(previous) = criteria.last() {
                    self.finish_scope(previous)?;
                }
                criteria.push(PendingCriterion {
                    criterion,
                    line: line.range.clone(),
                    items: Vec::new(),
                });
                continue;
            }
            if let Some(rest) = text.strip_prefix("  - ") {
                let Some(pending) = criteria.last_mut() else {
                    return Err(self.invalid(
                        line.range.clone(),
                        "a proof-scope item must follow its criterion",
                    ));
                };
                if !pending
                    .criterion
                    .verification
                    .contains(&VerificationMethod::Proof)
                {
                    return Err(self.invalid(
                        line.range.clone(),
                        "only a criterion that lists `proof` has proof-scope items",
                    ));
                }
                let next = SCOPE_ITEMS.get(pending.items.len()).copied();
                let item = SCOPE_ITEMS.iter().find_map(|&label| {
                    rest.strip_prefix(label)
                        .and_then(|r| r.strip_prefix(':'))
                        .map(|r| (label, r))
                });
                match (item, next) {
                    (Some((label, value)), Some(expected)) if label == expected => {
                        let value = value.strip_prefix(' ').unwrap_or(value);
                        if value.trim().is_empty() {
                            return Err(self.invalid(
                                line.range.clone(),
                                format!("`{label}:` must not be empty"),
                            ));
                        }
                        let value_offset = offset + 4 + label.len() + 2;
                        pending.items.push((
                            label,
                            value.to_owned(),
                            line.range.clone(),
                            value_offset,
                        ));
                    }
                    _ => {
                        return Err(self.invalid(
                            line.range.clone(),
                            "a proof criterion's scope items are `Domain:`, `Boundary:`, `Assumptions:`, and `Exclusions:`, once each, in that order",
                        ));
                    }
                }
                continue;
            }
            if let Some(item) = text.strip_prefix("- ") {
                if fields.last().is_some_and(|f| f.label == "Alternatives") {
                    if item.trim().is_empty() {
                        return Err(
                            self.invalid(line.range.clone(), "an alternative must not be empty")
                        );
                    }
                    alternatives.push(item.to_owned());
                    continue;
                }
                return Err(self.invalid(
                    line.range.clone(),
                    "a list item here must be a criterion, ``- Criterion `<id>` [<methods>]: <statement>``",
                ));
            }
            if text.starts_with(' ') {
                return Err(self.invalid(
                    line.range.clone(),
                    "record lines are not indented, except proof-scope items (`  - Domain: ...`)",
                ));
            }
            if let Some((label, rest)) = split_label(text) {
                let position = expected.iter().position(|&l| l == label);
                let Some(position) = position else {
                    return Err(self.invalid(
                        line.range.clone(),
                        format!("`{label}:` is not a field of {}", kind_name(kind)),
                    ));
                };
                match (criteria.is_empty(), position.cmp(&fields.len())) {
                    (true, Ordering::Equal) => {
                        if label == "Alternatives" {
                            if !rest.is_empty() {
                                return Err(self.invalid(
                                    line.range.clone(),
                                    "`Alternatives:` stands alone; list each alternative as `- <alternative>` below it",
                                ));
                            }
                        } else if rest.trim().is_empty() {
                            return Err(self.invalid(
                                line.range.clone(),
                                format!("`{label}:` must not be empty"),
                            ));
                        }
                        if fields.last().is_some_and(|f| f.label == "Alternatives")
                            && alternatives.is_empty()
                        {
                            return Err(self.invalid(
                                line.range.clone(),
                                "`Alternatives:` needs at least one `- <alternative>` item",
                            ));
                        }
                        fields.push(Field {
                            label,
                            paragraphs: vec![rest.to_owned()],
                            first: (offset + label.len() + 2, rest.to_owned()),
                        });
                    }
                    (true, Ordering::Greater) => {
                        return Err(self.missing(expected[fields.len()], line.range.clone()));
                    }
                    _ => {
                        return Err(self.invalid(
                            line.range.clone(),
                            format!("`{label}:` is repeated or out of order"),
                        ));
                    }
                }
                continue;
            }
            // An unlabeled line continues the text field above as a new paragraph.
            match fields.last_mut() {
                Some(field)
                    if criteria.is_empty()
                        && !matches!(
                            field.label,
                            "Authority" | "Requirements" | "Alternatives"
                        ) =>
                {
                    field.paragraphs.push(text.to_owned());
                }
                Some(field) if field.label == "Alternatives" || !criteria.is_empty() => {
                    return Err(self.invalid(
                        line.range.clone(),
                        "unexpected line; criteria and alternatives are one line each",
                    ));
                }
                Some(field) => {
                    return Err(self.invalid(
                        line.range.clone(),
                        format!("`{}:` is one line; it cannot continue", field.label),
                    ));
                }
                None => return Err(self.missing(expected[0], line.range.clone())),
            }
        }
        let end = self.lines[self.lines.len() - 1].range.clone();
        if fields.len() < expected.len() {
            return Err(self.missing(expected[fields.len()], end));
        }
        if let Some(last) = criteria.last() {
            self.finish_scope(last)?;
        }
        let text = |label: &str| {
            fields
                .iter()
                .find(|f| f.label == label)
                .map(|f| f.paragraphs.join("\n\n"))
                .expect("every expected field was read")
        };
        let first = |label: &str| {
            &fields
                .iter()
                .find(|f| f.label == label)
                .expect("every expected field was read")
                .first
        };
        Ok(match kind {
            RecordKind::Anchor => {
                if criteria.is_empty() {
                    return Err(self.invalid(
                        end,
                        "an anchor needs at least one criterion, ``- Criterion `<id>` [<methods>]: <statement>``",
                    ));
                }
                let (at, value) = first("Authority");
                let authorities = self.references(*at, value, "Authority", false)?;
                if !super::authority::has_required_authority(true, authorities.len()) {
                    return Err(self.invalid(end, "`Authority:` needs at least one reference"));
                }
                let criteria = criteria
                    .into_iter()
                    .map(|pending| self.with_scope(pending, id))
                    .collect::<Result<_, _>>()?;
                RecordData::Anchor(Anchor {
                    property: text("Property"),
                    authorities,
                    criteria,
                })
            }
            RecordKind::Requirement => RecordData::Requirement(Requirement {
                statement: text("Statement"),
                source: text("Source"),
            }),
            RecordKind::Decision => {
                if alternatives.is_empty() {
                    return Err(self.invalid(
                        end,
                        "`Alternatives:` needs at least one `- <alternative>` item",
                    ));
                }
                let (at, value) = first("Requirements");
                let requirements = if value == "none" {
                    Vec::new()
                } else {
                    self.references(*at, value, "Requirements", true)?
                };
                RecordData::Decision(Decision {
                    choice: text("Choice"),
                    requirements,
                    rationale: text("Rationale"),
                    alternatives,
                    reconsider_when: text("Reconsider when"),
                })
            }
        })
    }

    fn missing(&self, label: &str, range: Range<usize>) -> Box<Diagnostic> {
        self.invalid(
            range,
            format!("expected `{label}:` here; fields are required in their defined order"),
        )
    }

    /// References separated by `;`, each a code-spanned identifier.
    fn references(
        &self,
        at: usize,
        value: &str,
        field: &str,
        requirements_only: bool,
    ) -> Result<Vec<AuthorityReference>, Box<Diagnostic>> {
        let mut out = Vec::new();
        let mut offset = 0;
        for part in value.split(';') {
            let lead = part.len() - part.trim_start().len();
            let item = part.trim();
            let range = at + offset + lead..at + offset + lead + item.len();
            offset += part.len() + 1;
            let Some(id) = item
                .strip_prefix('`')
                .and_then(|i| i.strip_suffix('`'))
                .filter(|i| !i.contains('`'))
            else {
                return Err(self.invalid(
                    range,
                    format!(
                        "`{field}:` lists code-spanned identifiers separated by `;`{}",
                        if requirements_only {
                            ", or is exactly `none`"
                        } else {
                            ""
                        }
                    ),
                ));
            };
            let kind = match kind_of(id) {
                Some(RecordKind::Requirement) => AuthorityKind::Requirement,
                Some(RecordKind::Decision) if !requirements_only => AuthorityKind::Decision,
                Some(_) => {
                    return Err(self.invalid(
                        range,
                        if requirements_only {
                            format!("`{field}:` cites only requirements (`R-`); `{id}` is not one")
                        } else {
                            format!(
                                "`{field}:` cites only requirements (`R-`) and decisions (`D-`); `{id}` is neither"
                            )
                        },
                    ));
                }
                None => return Err(self.invalid(range, IDENTIFIER_FORM)),
            };
            out.push(AuthorityReference {
                kind,
                id: id.to_owned(),
                location: self.location(range),
            });
        }
        Ok(out)
    }

    /// ``- Criterion `<id>` [<methods>]: <statement>`` after its `- Criterion ` prefix.
    fn criterion(&self, rest: &str, line: Range<usize>) -> Result<Criterion, Box<Diagnostic>> {
        let form = "a criterion is ``- Criterion `<id>` [<methods>]: <statement>``";
        let parsed = rest.strip_prefix('`').and_then(|r| {
            let (id, r) = r.split_once('`')?;
            let r = r.strip_prefix(" [")?;
            let (methods, statement) = r.split_once("]: ")?;
            Some((id, methods, statement))
        });
        let Some((id, methods, statement)) = parsed else {
            return Err(self.invalid(line, form));
        };
        if !is_name(id) {
            return Err(self.invalid(
                line,
                "a criterion identifier is lowercase letters and digits in hyphen-separated words, starting with a letter",
            ));
        }
        let mut verification = Vec::new();
        for method in methods.split(", ") {
            let method = match method {
                "proof" => VerificationMethod::Proof,
                "test" => VerificationMethod::Test,
                "review" => VerificationMethod::Review,
                _ => {
                    return Err(self.invalid(
                        line,
                        "criterion methods are distinct values from `proof`, `test`, and `review`, separated by `, `",
                    ));
                }
            };
            if verification.contains(&method) {
                return Err(self.invalid(line, "a criterion's methods must be distinct"));
            }
            verification.push(method);
        }
        if statement.trim().is_empty() {
            return Err(self.invalid(line, "a criterion's statement must not be empty"));
        }
        Ok(Criterion {
            id: id.to_owned(),
            statement: statement.to_owned(),
            verification,
            scope: None,
        })
    }

    fn finish_scope(&self, pending: &PendingCriterion) -> Result<(), Box<Diagnostic>> {
        let proof = pending
            .criterion
            .verification
            .contains(&VerificationMethod::Proof);
        if proof && pending.items.len() != SCOPE_ITEMS.len() {
            let range = pending
                .items
                .last()
                .map_or(pending.line.clone(), |item| item.2.clone());
            return Err(self.invalid(
                range,
                format!(
                    "criterion `{}` requires proof, so it needs `Domain:`, `Boundary:`, `Assumptions:`, and `Exclusions:` items",
                    pending.criterion.id
                ),
            ));
        }
        Ok(())
    }

    fn with_scope(
        &self,
        pending: PendingCriterion,
        anchor: &str,
    ) -> Result<Criterion, Box<Diagnostic>> {
        let mut criterion = pending.criterion;
        if pending.items.is_empty() {
            return Ok(criterion);
        }
        let value = |label: &str| {
            pending
                .items
                .iter()
                .find(|item| item.0 == label)
                .expect("finish_scope checked every item")
        };
        let exclusions = value("Exclusions");
        let mut covered_by = Vec::new();
        for (span, range) in code_spans(&exclusions.1) {
            let range = exclusions.3 + range.start..exclusions.3 + range.end;
            let reference = match span.split_once('#') {
                None if is_name(span) => CriterionReference {
                    anchor: anchor.to_owned(),
                    criterion: span.to_owned(),
                    location: self.location(range),
                },
                Some((other, name))
                    if kind_of(other) == Some(RecordKind::Anchor) && is_name(name) =>
                {
                    CriterionReference {
                        anchor: other.to_owned(),
                        criterion: name.to_owned(),
                        location: self.location(range),
                    }
                }
                _ => {
                    return Err(self.invalid(
                        range,
                        "`Exclusions:` names covering criteria as `<criterion-id>` for this anchor or `<anchor-id>#<criterion-id>` for another",
                    ));
                }
            };
            covered_by.push(reference);
        }
        criterion.scope = Some(ProofScope {
            domain: value("Domain").1.clone(),
            boundary: value("Boundary").1.clone(),
            assumptions: value("Assumptions").1.clone(),
            exclusions: exclusions.1.clone(),
            covered_by,
        });
        Ok(criterion)
    }
}

/// ``**`<id>`** (<status>): **<Title>**`` into its parts; validation is the caller's.
fn parse_heading(text: &str) -> Option<(&str, &str, &str)> {
    let rest = text.strip_prefix("**`")?;
    let (id, rest) = rest.split_once("`** (")?;
    let (status, rest) = rest.split_once("): **")?;
    let title = rest.strip_suffix("**")?;
    Some((id, status, title))
}

#[cfg(test)]
mod tests {
    use super::{code_spans, is_name, kind_of, parse_heading, split_label};
    use crate::records::RecordKind;

    #[test]
    fn identifiers_state_their_kind_and_name() {
        assert_eq!(
            kind_of("A-check-observation-only"),
            Some(RecordKind::Anchor)
        );
        assert_eq!(kind_of("R-observation-only"), Some(RecordKind::Requirement));
        assert_eq!(kind_of("D-utf8-paths"), Some(RecordKind::Decision));
        for bad in [
            "D-12",
            "R-2026-01",
            "a-x",
            "X-name",
            "D-",
            "D-a--b",
            "D-a-",
            "D-Ab",
            "D",
        ] {
            assert_eq!(kind_of(bad), None, "{bad}");
        }
        assert!(is_name("file-preservation") && !is_name("1st") && !is_name(""));
    }

    #[test]
    fn headings_labels_and_code_spans_split_exactly() {
        assert_eq!(
            parse_heading("**`A-x`** (draft): **A title**"),
            Some(("A-x", "draft", "A title"))
        );
        assert_eq!(parse_heading("**`A-x`** (draft) **A title**"), None);
        assert_eq!(
            split_label("Reconsider when: x"),
            Some(("Reconsider when", "x"))
        );
        assert_eq!(split_label("Alternatives:"), Some(("Alternatives", "")));
        assert_eq!(split_label("Propertyx: y"), None);
        assert_eq!(split_label("Property:y"), None);
        assert_eq!(
            code_spans("by `a` and `A-b#c`"),
            [("a", 3..6), ("A-b#c", 11..18)]
        );
    }
}
