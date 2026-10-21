//! Readable HTML presentation of validated Book records.
mod html;
mod links;

use crate::records::{Chapter, RecordErrors, read_records};
use std::{collections::BTreeMap, error::Error, fmt, path::Path};

/// Chapter source and its renderer-selected output route.
#[derive(Clone, Copy, Debug)]
pub struct RenderChapter<'a> {
    /// Source identity and Markdown text.
    pub source: Chapter<'a>,
    /// Relative chapter path supplied by the renderer, before the HTML extension.
    pub output_path: &'a Path,
}

/// A presentation failure, separate from structural checking.
#[derive(Debug)]
pub enum RenderError {
    /// An unusable source or output route.
    InvalidInput(String),
    /// The record inventory did not validate.
    Records(RecordErrors),
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
        }
    }
}
impl Error for RenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Records(error) => Some(error),
            _ => None,
        }
    }
}

/// Validate the complete inventory and replace only definition ranges with HTML.
///
/// Results follow input chapter order. No files are read or written. The rest of
/// each chapter is preserved byte-for-byte, including examples and diagrams.
///
/// # Errors
/// Returns an error for invalid records, missing/duplicate source identities,
/// or unusable/colliding output routes. Never returns partial rendered chapters.
pub fn render_chapters(chapters: &[RenderChapter<'_>]) -> Result<Vec<String>, RenderError> {
    let sources: Vec<_> = chapters.iter().map(|c| c.source).collect();
    let inventory = read_records(&sources).map_err(RenderError::Records)?;
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
        let mut content = String::new();
        let mut cursor = 0;
        for record in records {
            let range = record.location().range();
            content.push_str(&chapter.source.content[cursor..range.start]);
            content.push_str(&html::record(record, &inventory, &routes));
            cursor = range.end;
        }
        content.push_str(&chapter.source.content[cursor..]);
        rendered.push(content);
    }
    Ok(rendered)
}
