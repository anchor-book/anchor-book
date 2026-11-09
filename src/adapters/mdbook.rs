//! mdBook 0.5.4 JSON protocol adapter. Does not read files or invoke mdBook.
use crate::{
    records::Chapter,
    rendering::{RenderChapter, RenderError, render_chapters},
};
use serde_json::Value;
use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

/// Failure to decode, validate, or transform a preprocessor request.
#[derive(Debug)]
pub enum PreprocessError {
    /// Invalid JSON, retaining the underlying parsing error.
    Json(serde_json::Error),
    /// Unsupported or malformed protocol data.
    Input(String),
    /// Invalid records or output routes.
    Render(RenderError),
}
impl fmt::Display for PreprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(f, "render.invalid-input: {error}"),
            Self::Input(message) => write!(f, "render.invalid-input: {message}"),
            Self::Render(error) => write!(f, "{error}"),
        }
    }
}
impl Error for PreprocessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(e) => Some(e),
            Self::Render(e) => Some(e),
            _ => None,
        }
    }
}
fn input(message: impl Into<String>) -> PreprocessError {
    PreprocessError::Input(message.into())
}
struct Source {
    path: PathBuf,
    output: PathBuf,
    content: String,
    roadmap: bool,
}
fn collect(
    items: &Value,
    roadmaps: Option<&Path>,
    within: bool,
    sources: &mut Vec<Source>,
) -> Result<(), PreprocessError> {
    for item in items
        .as_array()
        .ok_or_else(|| input("Book items and chapter sub_items must be arrays"))?
    {
        if item.as_str() == Some("Separator") {
            continue;
        }
        let object = item.as_object().ok_or_else(|| input("invalid Book item"))?;
        if object.len() != 1 {
            return Err(input("Book items must have exactly one variant"));
        }
        if let Some(title) = object.get("PartTitle") {
            if !title.is_string() {
                return Err(input("PartTitle must be a string"));
            }
            continue;
        }
        let chapter = object
            .get("Chapter")
            .and_then(Value::as_object)
            .ok_or_else(|| input("expected Chapter, PartTitle, or Separator"))?;
        let content = chapter
            .get("content")
            .and_then(Value::as_str)
            .ok_or_else(|| input("chapter content must be a string"))?;
        let source = chapter
            .get("source_path")
            .and_then(Value::as_str)
            .ok_or_else(|| input("chapter needs a source_path"))?;
        let output = chapter
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| input(format!("{source}: chapter needs an output path")))?;
        let roadmap = within || roadmaps.is_some_and(|r| Path::new(source) == r);
        sources.push(Source {
            path: source.into(),
            output: output.into(),
            content: content.into(),
            roadmap,
        });
        collect(
            chapter
                .get("sub_items")
                .ok_or_else(|| input(format!("{source}: missing sub_items")))?,
            roadmaps,
            roadmap,
            sources,
        )?;
    }
    Ok(())
}
fn replace(items: &mut Value, rendered: &mut impl Iterator<Item = String>) {
    for item in items.as_array_mut().expect("validated items") {
        if let Some(chapter) = item.get_mut("Chapter") {
            chapter["content"] = Value::String(rendered.next().expect("one result per chapter"));
            replace(&mut chapter["sub_items"], rendered);
        }
    }
}
/// Locate the Book configuration named by `[preprocessor.anchorbook] config`.
///
/// The value is resolved against the Book root that mdBook reports, so callers
/// can run the structural check before [`preprocess`] renders anything.
///
/// # Errors
/// Returns `render.invalid-input` for malformed JSON, unsupported mdBook
/// versions/renderers, or a missing root or `config` setting.
pub fn configuration_path(request: &str) -> Result<PathBuf, PreprocessError> {
    let (context, _): (Value, Value) =
        serde_json::from_str(request).map_err(PreprocessError::Json)?;
    supported(&context)?;
    let root = context
        .get("root")
        .and_then(Value::as_str)
        .ok_or_else(|| input("context must contain the Book root"))?;
    let config = context
        .pointer("/config/preprocessor/anchorbook/config")
        .and_then(Value::as_str)
        .filter(|path| !path.is_empty())
        .ok_or_else(|| {
            input("[preprocessor.anchorbook] config must name the Book configuration")
        })?;
    Ok(Path::new(root).join(config))
}
fn supported(context: &Value) -> Result<(), PreprocessError> {
    if context.get("renderer").and_then(Value::as_str) != Some("html") {
        return Err(input("only the HTML renderer is supported"));
    }
    if context.get("mdbook_version").and_then(Value::as_str) != Some("0.5.4") {
        return Err(input("this prototype requires mdBook 0.5.4"));
    }
    Ok(())
}
/// Transform `[context, book]` JSON into complete Book JSON for the HTML renderer.
///
/// Preserves unknown metadata and navigation fields. Validates all chapter input
/// before replacing any content. No source files are read or changed. No
/// chapters are treated as roadmaps; see [`preprocess_with_roadmaps`].
///
/// # Errors
/// Returns located record errors or `render.invalid-input` for malformed JSON,
/// unsupported mdBook versions/renderers, invalid chapter shapes or routes.
pub fn preprocess(request: &str) -> Result<String, PreprocessError> {
    preprocess_with_roadmaps(request, None)
}

/// Like [`preprocess`], treating the chapter whose source path is `roadmaps`,
/// relative to the Book source, and every chapter nested under it as roadmaps.
///
/// # Errors
/// As [`preprocess`], plus located roadmap errors.
pub fn preprocess_with_roadmaps(
    request: &str,
    roadmaps: Option<&Path>,
) -> Result<String, PreprocessError> {
    let (context, mut book): (Value, Value) =
        serde_json::from_str(request).map_err(PreprocessError::Json)?;
    supported(&context)?;
    let items = book
        .get("items")
        .ok_or_else(|| input("Book must contain items"))?;
    let mut sources = Vec::new();
    collect(items, roadmaps, false, &mut sources)?;
    let chapters: Vec<_> = sources
        .iter()
        .map(|s| RenderChapter {
            source: Chapter::new(Path::new(&s.path), &s.content),
            output_path: &s.output,
            roadmap: s.roadmap,
        })
        .collect();
    let rendered = render_chapters(&chapters).map_err(PreprocessError::Render)?;
    replace(&mut book["items"], &mut rendered.into_iter());
    serde_json::to_string(&book).map_err(PreprocessError::Json)
}
