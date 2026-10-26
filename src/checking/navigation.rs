use mdbook_summary::{SummaryItem, parse_summary};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub(super) struct Entry {
    pub target: Option<PathBuf>,
    pub depth: usize,
    pub offset: usize,
}
#[derive(Debug)]
pub(super) struct Failure {
    pub code: &'static str,
    pub message: String,
    pub offset: Option<usize>,
    pub cause: Option<Box<dyn Error + Send + Sync>>,
}
impl Failure {
    fn unsupported(offset: usize, message: &str) -> Self {
        Self {
            code: "input.unsupported",
            message: message.into(),
            offset: Some(offset),
            cause: None,
        }
    }
}

pub(super) fn parse(text: &str) -> Result<Vec<Entry>, Failure> {
    let origins = origins(text)?;
    let summary = parse_summary(text).map_err(|error| Failure {
        code: "input.parse-failed",
        message: format!("{error:#}"),
        offset: None,
        cause: Some(error.into_boxed_dyn_error()),
    })?;
    fn collect(
        items: &[SummaryItem],
        depth: usize,
        out: &mut Vec<(Option<PathBuf>, usize)>,
    ) -> Result<(), Failure> {
        for item in items {
            match item {
                SummaryItem::Link(link) => {
                    out.push((link.location.clone(), depth));
                    collect(&link.nested_items, depth + 1, out)?;
                }
                SummaryItem::Separator | SummaryItem::PartTitle(_) => {}
                _ => {
                    return Err(Failure {
                        code: "input.unsupported",
                        message: "unknown mdBook summary item".into(),
                        offset: None,
                        cause: None,
                    });
                }
            }
        }
        Ok(())
    }
    let mut entries = Vec::new();
    collect(&summary.prefix_chapters, 0, &mut entries)?;
    collect(&summary.numbered_chapters, 0, &mut entries)?;
    collect(&summary.suffix_chapters, 0, &mut entries)?;
    if entries.len() != origins.len()
        || entries
            .iter()
            .zip(&origins)
            .any(|((target, _), (original, _))| target != original)
    {
        return Err(Failure {
            code: "input.unsupported",
            message: "navigation links cannot be mapped reliably to mdBook's chapter hierarchy"
                .into(),
            offset: None,
            cause: None,
        });
    }
    Ok(entries
        .into_iter()
        .zip(origins)
        .map(|((target, depth), (_, offset))| Entry {
            target,
            depth,
            offset,
        })
        .collect())
}

// This validates the supported input and records offsets; mdBook, not this
// pass, determines chapter hierarchy, numbering, and affix semantics.
fn origins(text: &str) -> Result<Vec<(Option<PathBuf>, usize)>, Failure> {
    let mut links = Vec::new();
    let mut heading = false;
    let mut link_depth = 0usize;
    let mut list_depth = 0usize;
    for (event, range) in Parser::new(text).into_offset_iter() {
        let allowed = match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H1,
                ..
            }) if list_depth == 0 => {
                heading = true;
                true
            }
            Event::End(TagEnd::Heading(_)) => {
                heading = false;
                true
            }
            Event::Start(Tag::List(None)) => {
                list_depth += 1;
                if list_depth > 128 {
                    return Err(Failure::unsupported(
                        range.start,
                        "summary nesting exceeds the supported 128 list levels",
                    ));
                }
                true
            }
            Event::End(TagEnd::List(_)) => {
                list_depth = list_depth.saturating_sub(1);
                true
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                if !heading {
                    let dest = dest_url.replace("%20", " ");
                    links.push((
                        if dest.is_empty() {
                            None
                        } else {
                            Some(PathBuf::from(dest))
                        },
                        range.start,
                    ));
                }
                link_depth += 1;
                true
            }
            Event::End(TagEnd::Link) => {
                link_depth = link_depth.saturating_sub(1);
                true
            }
            Event::Start(Tag::Emphasis | Tag::Strong) | Event::Code(_) => heading || link_depth > 0,
            Event::Text(text) => heading || link_depth > 0 || text.trim().is_empty(),
            Event::Html(html) | Event::InlineHtml(html) => comments_only(&html),
            Event::Start(Tag::Paragraph | Tag::Item | Tag::HtmlBlock)
            | Event::End(
                TagEnd::Paragraph
                | TagEnd::Item
                | TagEnd::HtmlBlock
                | TagEnd::Emphasis
                | TagEnd::Strong,
            )
            | Event::Rule
            | Event::SoftBreak
            | Event::HardBreak => true,
            _ => false,
        };
        if !allowed {
            return Err(Failure::unsupported(
                range.start,
                "expected summary chapter links, H1 titles/parts, separators, or comments; this construct is unsupported",
            ));
        }
    }
    Ok(links)
}
fn comments_only(mut text: &str) -> bool {
    loop {
        text = text.trim();
        if text.is_empty() {
            return true;
        }
        let Some(rest) = text.strip_prefix("<!--") else {
            return false;
        };
        let Some(end) = rest.find("-->") else {
            return false;
        };
        text = &rest[end + 3..];
    }
}

pub(super) fn is_external(target: &str) -> bool {
    if target.starts_with("//") {
        return true;
    }
    let Some((scheme, _)) = target.split_once(':') else {
        return false;
    };
    let mut chars = scheme.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keeps_hierarchy_drafts_and_reference_link_offsets() {
        let source = "# Summary\n\n[Intro](intro.md)\n\n# Part\n\n- [**Design**][design]\n  - [Child](child.md)\n- [Draft]()\n---\n[Appendix](appendix.md)\n\n[design]: design.md\n";
        let entries = parse(source).unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|e| (e.target.as_deref(), e.depth))
                .collect::<Vec<_>>(),
            [
                (Some(std::path::Path::new("intro.md")), 0),
                (Some(std::path::Path::new("design.md")), 0),
                (Some(std::path::Path::new("child.md")), 1),
                (None, 0),
                (Some(std::path::Path::new("appendix.md")), 0)
            ]
        );
        for (entry, label) in entries.iter().zip([
            "[Intro]",
            "[**Design**]",
            "[Child]",
            "[Draft]",
            "[Appendix]",
        ]) {
            assert!(source[entry.offset..].starts_with(label));
        }
    }
    #[test]
    fn unsupported_content_never_disappears_into_a_successful_summary() {
        for source in [
            "ordinary prose",
            "## Heading\n[Link](x.md)",
            "```md\n- [Hidden](x.md)\n```",
            "<script>bad</script>",
            "<!-- unclosed",
            "1. [Ordered](x.md)",
            "- [One](x.md) [Ignored](y.md)",
            "- [Outer](x.md)\n  # Nested part\n",
            "> [Quote](x.md)",
            "![Image](image.png)",
        ] {
            let error = parse(source).unwrap_err();
            assert_eq!(error.code, "input.unsupported", "{source}");
            if let Some(offset) = error.offset {
                assert!(source.is_char_boundary(offset));
            }
        }
        assert!(parse("").unwrap().is_empty());
        assert!(parse("# Summary\n").unwrap().is_empty());
        assert_eq!(
            parse("<!-- A -->\n\n- [One](x.md)\n<!-- B -->\n")
                .unwrap()
                .len(),
            1
        );
        assert!(comments_only("<!-- a -->\n<!-- b -->"));
        assert!(!comments_only("<!-- a --><script></script><!-- b -->"));
    }
    #[test]
    fn parser_errors_preserve_causes_and_destinations_keep_mdbook_semantics() {
        let error = parse("- [A](one.md)\n- [B](one.md)").unwrap_err();
        assert_eq!(error.code, "input.parse-failed");
        assert!(error.cause.is_some());
        assert!(error.offset.is_none());
        assert!(parse("- [A](a.md)\n\n[Suffix](s.md)\n\n- [B](b.md)").is_err());
        let entries = parse(
            "- [Space](a%20b.md)\n- [Fragment](a.md#head)\n- [External](https://example.invalid/a)",
        )
        .unwrap();
        assert_eq!(entries[0].target, Some("a b.md".into()));
        assert_eq!(entries[1].target, Some("a.md#head".into()));
        assert_eq!(entries[2].target, Some("https://example.invalid/a".into()));
        for target in [
            "https://example.invalid",
            "mailto:a@example.invalid",
            "//example.invalid/a",
            "custom+tag:abc",
            "a.b-c+1:thing",
        ] {
            assert!(is_external(target), "{target}");
        }
        for target in [
            "chapter.md",
            "dir/a.md",
            ":empty",
            "1bad:value",
            "bad scheme:value",
            "../chapter.md",
        ] {
            assert!(!is_external(target), "{target}");
        }
    }
    #[test]
    fn deeply_nested_input_is_incomplete_instead_of_recursing_without_a_bound() {
        let allowed = (0..128)
            .map(|n| format!("{}- [Level](level-{n}.md)\n", "  ".repeat(n)))
            .collect::<String>();
        let entries = parse(&allowed).unwrap();
        assert_eq!(entries.len(), 128);
        assert_eq!(entries.last().unwrap().depth, 127);
        let source = (0..129)
            .map(|n| format!("{}- [Level](level-{n}.md)\n", "  ".repeat(n)))
            .collect::<String>();
        let error = parse(&source).unwrap_err();
        assert!(error.message.contains("128"));
    }
}
