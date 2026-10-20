use std::ops::Range;

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

use super::{
    Chapter, Diagnostic, Problem, RecordKind, SourceLocation,
    schema::{self, Candidate},
};

fn label_kind(label: &str) -> Option<RecordKind> {
    match label {
        "anchorbook-anchor" => Some(RecordKind::Anchor),
        "anchorbook-requirement" => Some(RecordKind::Requirement),
        "anchorbook-decision" => Some(RecordKind::Decision),
        _ => None,
    }
}

pub(super) fn read(chapter: Chapter<'_>, diagnostics: &mut Vec<Diagnostic>) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let mut depth = 0usize;
    let mut in_code = false;
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    for (event, range) in Parser::new_ext(chapter.content, options).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let label = match &kind {
                    CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or(""),
                    CodeBlockKind::Indented => "",
                };
                if label.starts_with("anchorbook-") {
                    match boundaries(chapter.content, range.clone(), label, depth) {
                        Ok(body) => {
                            if let Some(kind) = label_kind(label) {
                                candidates.push(schema::parse(
                                    chapter,
                                    kind,
                                    range,
                                    body,
                                    diagnostics,
                                ));
                            } else {
                                reject(
                                    chapter,
                                    range,
                                    "unknown record label; expected anchorbook-anchor, anchorbook-requirement, or anchorbook-decision",
                                    &mut candidates,
                                    diagnostics,
                                );
                            }
                        }
                        Err(message) => {
                            reject(chapter, range, message, &mut candidates, diagnostics)
                        }
                    }
                } else if kind.is_indented()
                    && chapter.content[range.clone()]
                        .lines()
                        .any(|line| line.trim_start().starts_with("```anchorbook-"))
                {
                    reject(
                        chapter,
                        range,
                        "record fences must be top-level and unindented; put literal examples in an outer fenced block",
                        &mut candidates,
                        diagnostics,
                    );
                }
                in_code = true;
                depth += 1;
            }
            Event::Start(_) => depth += 1,
            Event::End(tag) => {
                depth = depth.saturating_sub(1);
                if tag == pulldown_cmark::TagEnd::CodeBlock {
                    in_code = false;
                }
            }
            Event::Text(text) if !in_code && text.contains("{anchor}") => {
                reject(
                    chapter,
                    range,
                    "legacy {anchor} markers are unsupported; use the canonical definition fence",
                    &mut candidates,
                    diagnostics,
                );
            }
            _ => {}
        }
    }
    candidates
}

fn reject(
    chapter: Chapter<'_>,
    range: Range<usize>,
    message: &str,
    candidates: &mut Vec<Candidate>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let location = SourceLocation::new(chapter.path, chapter.content, range);
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
}

fn boundaries(
    content: &str,
    range: Range<usize>,
    label: &str,
    depth: usize,
) -> Result<Range<usize>, &'static str> {
    if depth != 0 {
        return Err(
            "record definitions must not be nested in lists, quotes, or other Markdown containers",
        );
    }
    let line_start = content[..range.start].rfind('\n').map_or(0, |i| i + 1);
    let opening_end = content[range.start..]
        .find('\n')
        .map(|i| range.start + i)
        .ok_or("record fence is unclosed")?;
    let opening = content[line_start..opening_end]
        .strip_suffix('\r')
        .unwrap_or(&content[line_start..opening_end]);
    if opening != format!("```{label}") {
        return Err(
            "record opening fence must be exactly three backticks followed by its canonical label",
        );
    }
    let end = range.end;
    let without_lf = content[..end].strip_suffix('\n').unwrap_or(&content[..end]);
    let without_cr = without_lf.strip_suffix('\r').unwrap_or(without_lf);
    let closing_start = without_cr.rfind('\n').map_or(0, |i| i + 1);
    if closing_start <= opening_end || &without_cr[closing_start..] != "```" {
        return Err("record closing fence must be exactly three backticks on its own line");
    }
    Ok(opening_end + 1..closing_start)
}
