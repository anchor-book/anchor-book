//! Roadmap milestones written in the Book's plain Markdown roadmap format.
//!
//! Reads supplied chapter text, validates milestone and criterion identity and
//! anchor-coverage references against a validated record inventory, and reports
//! the source ranges a renderer replaces. Does not read files or render HTML.

use crate::records::{Chapter, RecordData, RecordInventory};
use pulldown_cmark::{Event, Options, Parser, Tag};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    ops::Range,
    path::{Path, PathBuf},
};

/// Milestone statuses defined by the roadmap model.
const STATUSES: [&str; 5] = ["Proposed", "Ready", "In progress", "Blocked", "Complete"];

/// A validated completion criterion and the source line its heading occupies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoadmapCriterion {
    /// Book-wide fragment, prefixed by its milestone identifier.
    pub id: String,
    /// Display title from the heading.
    pub title: String,
    /// Byte range of the heading line, excluding its line ending.
    pub heading: Range<usize>,
    /// Byte offset where the criterion's section ends.
    pub end: usize,
}

/// A validated milestone in one roadmap chapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    /// Stable identifier from the heading fragment.
    pub id: String,
    /// Chapter that defines it.
    pub path: PathBuf,
    /// Completion criteria in source order.
    pub criteria: Vec<RoadmapCriterion>,
}

/// A located roadmap format or reference problem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoadmapDiagnostic {
    code: &'static str,
    path: PathBuf,
    line: usize,
    message: String,
}
impl RoadmapDiagnostic {
    /// Stable code: `roadmap.invalid-milestone`, `roadmap.duplicate-id`, or
    /// `roadmap.invalid-reference`.
    pub fn code(&self) -> &'static str {
        self.code
    }
    /// Chapter containing the problem.
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// One-based source line.
    pub fn line(&self) -> usize {
        self.line
    }
    /// Explanation of the expected form.
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl fmt::Display for RoadmapDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}:{}: {}",
            self.code,
            self.path.display(),
            self.line,
            self.message
        )
    }
}

/// All roadmap diagnostics, sorted by path, line, code, and message.
#[derive(Debug)]
pub struct RoadmapErrors(Vec<RoadmapDiagnostic>);
impl RoadmapErrors {
    /// Every finding; never empty.
    pub fn diagnostics(&self) -> &[RoadmapDiagnostic] {
        &self.0
    }
}
impl fmt::Display for RoadmapErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, d) in self.0.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "{d}")?;
        }
        Ok(())
    }
}
impl Error for RoadmapErrors {}

struct Line<'a> {
    start: usize,
    end: usize,
    number: usize,
    text: &'a str,
}

fn lines(content: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut start = 0;
    for (index, raw) in content.split_inclusive('\n').enumerate() {
        let text = raw.trim_end_matches(['\n', '\r']);
        out.push(Line {
            start,
            end: start + text.len(),
            number: index + 1,
            text,
        });
        start += raw.len();
    }
    out
}

fn code_regions(content: &str) -> Vec<Range<usize>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    Parser::new_ext(content, options)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            matches!(event, Event::Start(Tag::CodeBlock(_))).then_some(range)
        })
        .collect()
}

fn is_identifier(text: &str) -> bool {
    !text.is_empty()
        && text.starts_with(|c: char| c.is_ascii_lowercase())
        && !text.ends_with('-')
        && !text.contains("--")
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Split `Title {#id}`; the fragment must close the line.
fn heading_fragment(rest: &str) -> Option<(&str, &str)> {
    let rest = rest.trim_end();
    let body = rest.strip_suffix('}')?;
    let open = body.rfind(" {#")?;
    let id = &body[open + 3..];
    let title = body[..open].trim();
    (!title.is_empty() && is_identifier(id)).then_some((title, id))
}

/// Parse `N. Title {#id}` after the `## Milestone ` marker.
fn milestone_heading(rest: &str) -> Option<&str> {
    let dot = rest.find(". ")?;
    if dot == 0 || !rest[..dot].bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    heading_fragment(&rest[dot + 2..]).map(|(_, id)| id)
}

struct Reader<'a> {
    path: &'a Path,
    diagnostics: Vec<RoadmapDiagnostic>,
}
impl Reader<'_> {
    fn error(&mut self, code: &'static str, line: usize, message: impl Into<String>) {
        self.diagnostics.push(RoadmapDiagnostic {
            code,
            path: self.path.to_owned(),
            line,
            message: message.into(),
        });
    }
}

/// Anchor and criterion identifiers cited by an anchor-coverage text.
fn coverage_references(text: &str) -> Vec<(String, Vec<String>)> {
    let mut references = Vec::new();
    for (at, separator) in text.match_indices(" · ") {
        let before = &text[..at];
        let anchor = if let Some(link) = before.strip_suffix(')') {
            link.rfind("](")
                .and_then(|close| link[..close].rfind('[').map(|open| &link[open + 1..close]))
        } else if let Some(code) = before.strip_suffix('`') {
            code.rfind('`').map(|open| &code[open + 1..])
        } else {
            None
        };
        let mut criteria = Vec::new();
        let mut after = &text[at + separator.len()..];
        while let Some(rest) = after.strip_prefix('`') {
            let Some(close) = rest.find('`') else { break };
            criteria.push(rest[..close].to_owned());
            after = &rest[close + 1..];
            match after.strip_prefix(", ") {
                Some(next) => after = next,
                None => break,
            }
        }
        if let Some(anchor) = anchor {
            references.push((anchor.to_owned(), criteria));
        }
    }
    references
}

fn read_chapter(
    chapter: Chapter<'_>,
    inventory: &RecordInventory,
    reader: &mut Reader<'_>,
) -> Vec<Milestone> {
    let code = code_regions(chapter.content);
    let in_code = |offset: usize| code.iter().any(|r| r.contains(&offset));
    let all = lines(chapter.content);
    let lines: Vec<_> = all.iter().filter(|l| !in_code(l.start)).collect();
    // A milestone's section runs to the next level-one or level-two heading.
    let sections: Vec<usize> = (0..lines.len())
        .filter(|&i| lines[i].text.starts_with("## ") || lines[i].text.starts_with("# "))
        .collect();
    let mut milestones = Vec::new();
    for (position, &start) in sections.iter().enumerate() {
        let line = lines[start];
        let Some(rest) = line.text.strip_prefix("## Milestone ") else {
            continue;
        };
        let Some(id) = milestone_heading(rest) else {
            reader.error(
                "roadmap.invalid-milestone",
                line.number,
                "expected `## Milestone N. Title {#milestone-id}` with a lowercase identifier",
            );
            continue;
        };
        let end = sections.get(position + 1).copied().unwrap_or(lines.len());
        // The section includes its heading line, which matches no field.
        let body = &lines[start..end];
        let section_end = lines.get(end).map_or(chapter.content.len(), |l| l.start);
        milestones.push(read_milestone(
            chapter,
            id,
            line.number,
            body,
            section_end,
            inventory,
            reader,
        ));
    }
    milestones
}

fn read_milestone(
    chapter: Chapter<'_>,
    id: &str,
    heading_line: usize,
    body: &[&Line<'_>],
    section_end: usize,
    inventory: &RecordInventory,
    reader: &mut Reader<'_>,
) -> Milestone {
    let field = |label: &str| {
        let prefix = format!("**{label}:**");
        body.iter()
            .find(|l| l.text.starts_with(&prefix))
            .map(|l| (l.number, l.text[prefix.len()..].trim()))
    };
    for label in ["Outcome", "Status", "Identifier"] {
        if field(label).is_none() {
            reader.error(
                "roadmap.invalid-milestone",
                heading_line,
                format!("milestone `{id}` needs a `**{label}:**` field"),
            );
        }
    }
    if let Some((number, value)) = field("Identifier")
        && value != format!("`{id}`")
    {
        reader.error(
            "roadmap.invalid-milestone",
            number,
            format!("Identifier must be `{id}`, matching the heading fragment"),
        );
    }
    if let Some((number, value)) = field("Status")
        && !STATUSES.iter().any(|s| {
            value == *s
                || value.starts_with(&format!("{s} —"))
                || value.starts_with(&format!("{s} -"))
        })
    {
        reader.error(
            "roadmap.invalid-milestone",
            number,
            format!("Status must start with one of {}", STATUSES.join(", ")),
        );
    }
    let mut criteria: Vec<RoadmapCriterion> = Vec::new();
    for (position, line) in body.iter().enumerate() {
        if let Some(rest) = line.text.strip_prefix("#### ") {
            match heading_fragment(rest) {
                Some((title, criterion)) if criterion.starts_with(&format!("{id}-")) => {
                    let end = body[position + 1..]
                        .iter()
                        .find(|l| l.text.starts_with('#'))
                        .map_or(section_end, |l| l.start);
                    criteria.push(RoadmapCriterion {
                        id: criterion.to_owned(),
                        title: title.to_owned(),
                        heading: line.start..line.end,
                        end,
                    });
                }
                Some((_, criterion)) => reader.error(
                    "roadmap.invalid-milestone",
                    line.number,
                    format!("criterion fragment `{criterion}` must start with `{id}-`"),
                ),
                None => reader.error(
                    "roadmap.invalid-milestone",
                    line.number,
                    format!("expected `#### Title {{#{id}-criterion-id}}`"),
                ),
            }
        }
        let coverage = line
            .text
            .strip_prefix("- **Anchor coverage:**")
            .or_else(|| line.text.strip_prefix("**Anchor coverage:**"));
        if let Some(text) = coverage {
            for (anchor, cited) in coverage_references(text) {
                check_reference(inventory, &anchor, &cited, line.number, reader);
            }
        }
    }
    Milestone {
        id: id.to_owned(),
        path: chapter.path.to_owned(),
        criteria,
    }
}

fn check_reference(
    inventory: &RecordInventory,
    anchor: &str,
    cited: &[String],
    line: usize,
    reader: &mut Reader<'_>,
) {
    let Some(record) = inventory.get(anchor) else {
        reader.error(
            "roadmap.invalid-reference",
            line,
            format!("anchor coverage cites `{anchor}`, which is not a record in this Book"),
        );
        return;
    };
    let RecordData::Anchor(data) = record.data() else {
        reader.error(
            "roadmap.invalid-reference",
            line,
            format!("anchor coverage cites `{anchor}`, which is not an anchor"),
        );
        return;
    };
    for criterion in cited {
        if !data.criteria.iter().any(|c| &c.id == criterion) {
            reader.error(
                "roadmap.invalid-reference",
                line,
                format!("anchor `{anchor}` has no criterion `{criterion}`"),
            );
        }
    }
}

/// Read and validate the milestones of the supplied roadmap chapters.
///
/// Milestone and criterion identifiers must be unique across all supplied
/// chapters, and anchor-coverage references must resolve to anchors and their
/// criteria in `inventory`. Text inside code blocks is never roadmap structure.
///
/// # Errors
/// Returns every located diagnostic; there is no partial result.
pub fn read_roadmaps(
    chapters: &[Chapter<'_>],
    inventory: &RecordInventory,
) -> Result<Vec<Milestone>, RoadmapErrors> {
    let mut diagnostics = Vec::new();
    let mut milestones = Vec::new();
    for chapter in chapters {
        let mut reader = Reader {
            path: chapter.path,
            diagnostics: Vec::new(),
        };
        milestones.extend(read_chapter(*chapter, inventory, &mut reader));
        diagnostics.append(&mut reader.diagnostics);
    }
    let mut seen: BTreeMap<&str, &Path> = BTreeMap::new();
    for milestone in &milestones {
        let ids = std::iter::once(milestone.id.as_str())
            .chain(milestone.criteria.iter().map(|c| c.id.as_str()));
        for id in ids {
            if let Some(first) = seen.insert(id, &milestone.path) {
                let content = chapters
                    .iter()
                    .find(|c| c.path == milestone.path)
                    .map_or("", |c| c.content);
                let line = content
                    .lines()
                    .position(|l| l.ends_with(&format!("{{#{id}}}")))
                    .map_or(1, |p| p + 1);
                diagnostics.push(RoadmapDiagnostic {
                    code: "roadmap.duplicate-id",
                    path: milestone.path.clone(),
                    line,
                    message: format!(
                        "identifier `{id}` is already defined in {}",
                        first.display()
                    ),
                });
            }
        }
    }
    if diagnostics.is_empty() {
        Ok(milestones)
    } else {
        diagnostics.sort_by(|a, b| {
            (&a.path, a.line, a.code, &a.message).cmp(&(&b.path, b.line, b.code, &b.message))
        });
        Err(RoadmapErrors(diagnostics))
    }
}

#[cfg(test)]
mod tests {
    use super::{coverage_references, heading_fragment, milestone_heading};

    #[test]
    fn headings_require_a_closing_identifier_fragment() {
        assert_eq!(milestone_heading("12. Do it {#road-12}"), Some("road-12"));
        assert_eq!(milestone_heading("100. A {#road-100}"), Some("road-100"));
        assert_eq!(milestone_heading("1. Do it"), None);
        assert_eq!(milestone_heading("one. Do it {#road-1}"), None);
        assert_eq!(milestone_heading(". Do it {#road-1}"), None);
        assert_eq!(milestone_heading("1. Do it {#Road-1}"), None);
        assert_eq!(heading_fragment("Title {#a-b} "), Some(("Title", "a-b")));
        assert_eq!(heading_fragment("{#a-b}"), None);
        assert_eq!(heading_fragment("Title {#a-}"), None);
    }

    #[test]
    fn coverage_lists_link_and_code_anchors_with_their_criteria() {
        let text =
            " [a-one](x.md#a-one) · `c1`, `c2` — Partial: x; `a-two` · `c3` — Full; None — y";
        assert_eq!(
            coverage_references(text),
            vec![
                ("a-one".to_owned(), vec!["c1".to_owned(), "c2".to_owned()]),
                ("a-two".to_owned(), vec!["c3".to_owned()]),
            ]
        );
        assert!(coverage_references(" None — design preparation").is_empty());
    }
}
