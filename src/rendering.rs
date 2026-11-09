//! Readable HTML presentation of validated Book records.
mod html;
mod links;

use crate::records::{Chapter, RecordErrors, read_records};
use crate::roadmaps::{Milestone, RoadmapErrors, read_roadmaps};
use std::{collections::BTreeMap, error::Error, fmt, path::Path};

/// Chapter source and its renderer-selected output route.
#[derive(Clone, Copy, Debug)]
pub struct RenderChapter<'a> {
    /// Source identity and Markdown text.
    pub source: Chapter<'a>,
    /// Relative chapter path supplied by the renderer, before the HTML extension.
    pub output_path: &'a Path,
    /// Whether the chapter belongs to the Book's implementation roadmaps.
    pub roadmap: bool,
}

/// A presentation failure, separate from structural checking.
#[derive(Debug)]
pub enum RenderError {
    /// An unusable source or output route.
    InvalidInput(String),
    /// The record inventory did not validate.
    Records(RecordErrors),
    /// Roadmap milestones did not validate.
    Roadmaps(RoadmapErrors),
}
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "render.invalid-input: {message}"),
            Self::Records(errors) => {
                for (i, diagnostic) in errors.diagnostics().iter().enumerate() {
                    if i != 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{diagnostic}")?;
                }
                Ok(())
            }
            Self::Roadmaps(errors) => write!(f, "{errors}"),
        }
    }
}
impl Error for RenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Records(error) => Some(error),
            Self::Roadmaps(error) => Some(error),
            _ => None,
        }
    }
}

/// Validate the complete inventory and roadmaps, and replace only definition
/// ranges and roadmap criterion headings with HTML.
///
/// Results follow input chapter order. No files are read or written. The rest of
/// each chapter is preserved byte-for-byte, including examples and diagrams.
///
/// # Errors
/// Returns an error for invalid records or roadmaps, missing/duplicate source
/// identities, or unusable/colliding output routes. Never returns partial
/// rendered chapters.
pub fn render_chapters(chapters: &[RenderChapter<'_>]) -> Result<Vec<String>, RenderError> {
    let sources: Vec<_> = chapters.iter().map(|c| c.source).collect();
    let inventory = read_records(&sources).map_err(RenderError::Records)?;
    let roadmap_sources: Vec<_> = chapters
        .iter()
        .filter(|c| c.roadmap)
        .map(|c| c.source)
        .collect();
    let milestones = read_roadmaps(&roadmap_sources, &inventory).map_err(RenderError::Roadmaps)?;
    let mut routes = BTreeMap::new();
    let mut destinations = std::collections::BTreeSet::new();
    for chapter in chapters {
        let route = links::route(chapter.output_path)?;
        if !destinations.insert(route.clone()) {
            return Err(RenderError::InvalidInput(format!(
                "duplicate output route: {}",
                chapter.output_path.display()
            )));
        }
        routes.insert(chapter.source.path, route);
    }
    let mut rendered = Vec::with_capacity(chapters.len());
    for chapter in chapters {
        let mut records: Vec<_> = inventory
            .records()
            .filter(|r| r.location().path() == chapter.source.path)
            .collect();
        records.sort_by_key(|r| r.location().range().start);
        // Edits sorted by position. At one offset, closing tags come innermost
        // first, then the next milestone opens, then replacements.
        let mut edits: Vec<(usize, usize, u8, String)> = records
            .iter()
            .map(|record| {
                let range = record.location().range();
                (
                    range.start,
                    range.end,
                    4,
                    html::record(record, &inventory, &routes),
                )
            })
            .collect();
        for milestone in milestones.iter().filter(|m| m.path == chapter.source.path) {
            milestone_edits(milestone, &mut edits);
        }
        edits.sort_by_key(|(start, end, order, _)| (*start, *order, *end));
        let mut content = String::new();
        let mut cursor = 0;
        for (start, end, _, replacement) in edits {
            content.push_str(&chapter.source.content[cursor..start]);
            content.push_str(&replacement);
            cursor = end;
        }
        content.push_str(&chapter.source.content[cursor..]);
        rendered.push(content);
    }
    Ok(rendered)
}

/// Wrap a milestone in a card: header metadata, a field grid, and open,
/// collapsible tasks, criteria, and evidence. Headings keep their fragments.
fn milestone_edits(milestone: &Milestone, edits: &mut Vec<(usize, usize, u8, String)>) {
    let at = |offset: usize, order: u8, text: String| (offset, offset, order, text);
    edits.push(at(milestone.heading.start, 3, html::milestone_open()));
    edits.push(at(
        milestone.heading.end,
        4,
        html::milestone_meta(&milestone.status, &milestone.id),
    ));
    edits.push((
        milestone.identifier.start,
        milestone.identifier.end,
        4,
        String::new(),
    ));
    for field in &milestone.fields {
        // A bare status repeats the badge; a status with an explanation stays.
        let text = if field.label == "Status" && field.value == milestone.status {
            String::new()
        } else {
            html::milestone_field(&field.label, &field.value)
        };
        edits.push((field.line.start, field.line.end, 4, text));
    }
    if let Some(tasks) = &milestone.tasks {
        edits.push((
            tasks.label.start,
            tasks.label.end,
            4,
            html::tasks_open(tasks.done, tasks.total),
        ));
        edits.push(at(tasks.end, 0, html::section_close()));
    }
    for criterion in &milestone.criteria {
        edits.push((
            criterion.heading.start,
            criterion.heading.end,
            4,
            html::criterion_open(&criterion.id, &criterion.title),
        ));
        edits.push(at(criterion.end, 0, html::section_close()));
    }
    if let Some(evidence) = &milestone.evidence {
        edits.push((
            evidence.heading.start,
            evidence.heading.end,
            4,
            html::evidence_open(&evidence.id, &evidence.title),
        ));
        edits.push(at(evidence.end, 1, html::section_close()));
    }
    edits.push(at(milestone.end, 2, html::milestone_close()));
}
