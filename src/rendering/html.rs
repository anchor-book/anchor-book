use super::links;
use crate::records::{Record, RecordData, RecordInventory, RecordKind, Status};
use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};
use std::{collections::BTreeMap, fmt::Write, path::Path};

/// What inline text needs to link identifiers: the record it belongs to, the
/// inventory, and the output route of every chapter.
#[derive(Clone, Copy)]
struct Context<'a> {
    from: &'a Record,
    inventory: &'a RecordInventory,
    routes: &'a BTreeMap<&'a Path, Vec<String>>,
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;")
}
fn paragraphs(text: &str, context: Context<'_>) -> String {
    // Each paragraph is rendered on its own; the encoded line breaks keep the
    // paragraphs apart without a blank line ending mdBook's HTML block.
    let text = text
        .trim()
        .split("\n\n")
        .map(|paragraph| inline(paragraph, context))
        .collect::<Vec<_>>()
        .join("&#10;&#10;");
    format!("<div class=\"ab-text\">{text}</div>")
}
fn field(label: &str, text: &str, context: Context<'_>) -> String {
    format!(
        "<div class=\"ab-field\"><h4>{label}</h4>{}</div>",
        paragraphs(text, context)
    )
}

/// A link destination that is safe to emit, with chapter links resolved as
/// mdBook resolves them in chapter text: `chapter.md#x` becomes `chapter.html#x`.
fn destination(dest: &str, kind: LinkType) -> Option<String> {
    if kind == LinkType::Email {
        return Some(format!("mailto:{dest}"));
    }
    if ["http://", "https://", "mailto:"]
        .iter()
        .any(|scheme| dest.starts_with(scheme))
    {
        return Some(dest.to_owned());
    }
    let path_end = dest.find(['?', '#']).unwrap_or(dest.len());
    if dest.starts_with('/') || dest[..path_end].contains(':') || dest.contains('\\') {
        return None;
    }
    let (path, rest) = dest.split_at(path_end);
    Some(match path.strip_suffix(".md") {
        Some(stem) => format!("{stem}.html{rest}"),
        None => dest.to_owned(),
    })
}

/// One paragraph of field text: code spans, emphasis, strong emphasis, and
/// links are formatted, and a code-spanned identifier of a record links to it;
/// any other Markdown, and raw HTML, is shown as its literal source text.
fn inline(text: &str, context: Context<'_>) -> String {
    let mut out = String::new();
    let mut literal_until = 0;
    for (event, range) in Parser::new_ext(text, Options::empty()).into_offset_iter() {
        if range.start < literal_until {
            continue;
        }
        match event {
            Event::Start(Tag::Paragraph) | Event::End(TagEnd::Paragraph) => {}
            Event::Text(t) => out.push_str(&escape(&t)),
            Event::Code(code) => out.push_str(&code_span(&code, context)),
            Event::Start(Tag::Emphasis) => out.push_str("<em>"),
            Event::End(TagEnd::Emphasis) => out.push_str("</em>"),
            Event::Start(Tag::Strong) => out.push_str("<strong>"),
            Event::End(TagEnd::Strong) => out.push_str("</strong>"),
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                ..
            }) => match destination(&dest_url, link_type) {
                Some(href) => {
                    write!(out, "<a href=\"{}\">", escape(&href)).expect("writing to String")
                }
                None => {
                    out.push_str(&escape(&text[range.clone()]));
                    literal_until = range.end;
                }
            },
            Event::End(TagEnd::Link) => out.push_str("</a>"),
            // Any other construct, raw HTML included, is shown as its source text.
            // A paragraph is one line, so no line breaks occur, and the end of a
            // construct shown literally falls inside its skipped range.
            Event::Start(_) => {
                out.push_str(&escape(&text[range.clone()]));
                literal_until = range.end;
            }
            _ => out.push_str(&escape(&text[range])),
        }
    }
    out
}

/// A code span, linked when it names a record or one of an anchor's criteria.
fn code_span(code: &str, context: Context<'_>) -> String {
    let id = code.split_once('#').map_or(code, |(id, _)| id);
    match context.inventory.get(id) {
        Some(target) => {
            let href = links::relative(
                &context.routes[context.from.location().path()],
                &context.routes[target.location().path()],
                target.id(),
            );
            format!(
                "<a class=\"ab-record-link\" href=\"{}\" title=\"{}\"><code>{}</code></a>",
                escape(&href),
                escape(target.title()),
                escape(code)
            )
        }
        None => format!("<code>{}</code>", escape(code)),
    }
}
fn link(from: &Record, target: &Record, routes: &BTreeMap<&Path, Vec<String>>) -> String {
    let href = links::relative(
        &routes[from.location().path()],
        &routes[target.location().path()],
        target.id(),
    );
    format!(
        "<a class=\"ab-record-link\" href=\"{}\">{} <span class=\"ab-link-id\">{}</span></a>",
        escape(&href),
        escape(target.title()),
        escape(target.id())
    )
}
pub(super) fn record(
    record: &Record,
    inventory: &RecordInventory,
    routes: &BTreeMap<&Path, Vec<String>>,
) -> String {
    let kind = match record.kind() {
        RecordKind::Anchor => "Anchor",
        RecordKind::Requirement => "Requirement",
        RecordKind::Decision => "Decision",
    };
    let status = match record.status() {
        Status::Draft => "Draft",
        Status::Accepted => "Accepted",
    };
    let mut out = format!(
        "\n<section class=\"ab-record ab-{}\" id=\"{}\" aria-label=\"{}\">\n",
        kind.to_lowercase(),
        record.id(),
        escape(record.title())
    );
    let permalink = format!(
        "<a class=\"ab-permalink\" href=\"#{}\" aria-label=\"Permalink: {}\">{}</a>",
        record.id(),
        escape(record.title()),
        record.id()
    );
    if record.kind() == RecordKind::Anchor {
        writeln!(
            out,
            "<details class=\"ab-details ab-anchor-details\"><summary><h3><span class=\"ab-record-meta\"><span>{kind}</span><span class=\"ab-status\">{status}</span></span><span class=\"ab-anchor-title\">{}</span>{permalink}</h3></summary>",
            escape(record.title())
        )
        .expect("writing to String");
    } else {
        writeln!(
            out,
            "<header class=\"ab-record-header\"><div class=\"ab-record-meta\"><span>{kind}</span><span class=\"ab-status\">{status}</span></div><h3>{}</h3>{permalink}</header>",
            escape(record.title())
        )
        .expect("writing to String");
    }
    let context = Context {
        from: record,
        inventory,
        routes,
    };
    match record.data() {
        RecordData::Anchor(anchor) => {
            out.push_str(&field("Property", &anchor.property, context));
        }
        RecordData::Requirement(requirement) => {
            out.push_str(&field("Requirement", &requirement.statement, context));
            out.push_str(&field("Source", &requirement.source, context));
        }
        RecordData::Decision(decision) => {
            out.push_str(&field("Decision", &decision.choice, context));
        }
    }
    if record.kind() != RecordKind::Requirement {
        let label = if record.kind() == RecordKind::Anchor {
            "Authority"
        } else {
            "Requirements"
        };
        write!(out, "<div class=\"ab-field\"><h4>{label}</h4>").expect("writing to String");
        if record.references().is_empty() {
            out.push_str("<p>No recorded requirements.</p>");
        } else {
            out.push_str("<ul class=\"ab-links\">");
            for reference in record.references() {
                let target = inventory.get(&reference.id).expect("validated reference");
                write!(out, "<li>{}</li>", link(record, target, routes))
                    .expect("writing to String");
            }
            out.push_str("</ul>");
        }
        out.push_str("</div>");
    }
    match record.data() {
        RecordData::Anchor(anchor) => {
            out.push_str(
                "<div class=\"ab-field\"><h4>Verification criteria</h4><ol class=\"ab-criteria\">",
            );
            for criterion in &anchor.criteria {
                let methods = criterion
                    .verification
                    .iter()
                    .map(|method| method.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let scope = criterion.scope.as_ref().map_or(String::new(), |scope| {
                    let mut items = String::from("<dl class=\"ab-proof-scope\">");
                    for (label, text) in [
                        ("Domain", &scope.domain),
                        ("Boundary", &scope.boundary),
                        ("Assumptions", &scope.assumptions),
                        ("Exclusions", &scope.exclusions),
                    ] {
                        write!(items, "<dt>{label}</dt><dd>{}</dd>", inline(text, context))
                            .expect("writing to String");
                    }
                    items.push_str("</dl>");
                    items
                });
                write!(
                    out,
                    "<li><code class=\"ab-criterion-id\">{}</code>{}{scope}<p>Required verification: {methods}</p></li>",
                    escape(&criterion.id),
                    paragraphs(&criterion.statement, context)
                )
                .expect("writing to String");
            }
            out.push_str("</ol></div>");
        }
        RecordData::Decision(decision) => {
            out.push_str("<details class=\"ab-details\"><summary>Rationale, alternatives, and reconsideration</summary><div class=\"ab-details-body\">");
            out.push_str(&field("Rationale", &decision.rationale, context));
            out.push_str("<div class=\"ab-field\"><h4>Alternatives</h4><ul>");
            for alternative in &decision.alternatives {
                write!(out, "<li>{}</li>", paragraphs(alternative, context))
                    .expect("writing to String");
            }
            out.push_str("</ul></div>");
            out.push_str(&field(
                "Reconsider when",
                &decision.reconsider_when,
                context,
            ));
            out.push_str("</div></details>");
        }
        RecordData::Requirement(_) => {}
    }
    let backlinks = inventory.backlinks(record.id());
    if !backlinks.is_empty() {
        out.push_str(
            "<div class=\"ab-field ab-backlinks\"><h4>Referenced by</h4><ul class=\"ab-links\">",
        );
        for id in backlinks {
            write!(
                out,
                "<li>{}</li>",
                link(
                    record,
                    inventory.get(id).expect("validated backlink"),
                    routes
                )
            )
            .expect("writing to String");
        }
        out.push_str("</ul></div>");
    }
    if record.kind() == RecordKind::Anchor {
        out.push_str("</details>");
    }
    out.push_str("</section>\n\n");
    out
}

/// Inline text with backtick code spans; everything else is escaped.
fn inline_code(text: &str) -> String {
    let mut out = String::new();
    for (index, part) in text.split('`').enumerate() {
        if index % 2 == 1 {
            write!(out, "<code>{}</code>", escape(part)).expect("writing to String");
        } else {
            out.push_str(&escape(part));
        }
    }
    out
}
/// Opening of a roadmap criterion: a collapsible section, open by default, whose
/// heading keeps the criterion's fragment and shows its identifier.
pub(super) fn criterion_open(id: &str, title: &str) -> String {
    format!(
        "<details class=\"ab-details ab-roadmap-criterion\" open>\n<summary><h4 id=\"{id}\">{} <span class=\"ab-link-id\">{id}</span></h4></summary>\n",
        inline_code(title),
        id = escape(id),
    )
}
/// Opening of a milestone's evidence section, open by default.
pub(super) fn evidence_open(id: &str, title: &str) -> String {
    format!(
        "<details class=\"ab-details ab-milestone-evidence\" open>\n<summary><h3 id=\"{id}\">{}</h3></summary>\n",
        inline_code(title),
        id = escape(id),
    )
}
/// Opening of a milestone's task list, open by default, with its progress.
pub(super) fn tasks_open(done: usize, total: usize) -> String {
    format!(
        "<details class=\"ab-details ab-milestone-tasks\" open>\n<summary>Tasks <span class=\"ab-task-progress\">{done} of {total} done</span></summary>\n"
    )
}
/// Closing of a collapsible section, followed by a blank line so the
/// following Markdown is parsed normally.
pub(super) fn section_close() -> String {
    "\n</details>\n\n".to_owned()
}
/// Opening of a milestone card; the Markdown heading follows inside it.
pub(super) fn milestone_open() -> String {
    "<section class=\"ab-milestone\">\n\n".to_owned()
}
/// Closing of a milestone card.
pub(super) fn milestone_close() -> String {
    "\n</section>\n\n".to_owned()
}
/// Status and identifier shown under the milestone heading.
pub(super) fn milestone_meta(status: &str, id: &str) -> String {
    format!(
        "\n\n<div class=\"ab-milestone-meta\"><span class=\"ab-status\">{}</span><span class=\"ab-link-id\">{}</span></div>",
        escape(status),
        escape(id)
    )
}
/// A field as a labeled row; the value stays Markdown, separated by blank lines.
pub(super) fn milestone_field(label: &str, value: &str) -> String {
    format!(
        "<div class=\"ab-milestone-field\"><div class=\"ab-field-label\">{}</div><div class=\"ab-field-value\">\n\n{value}\n\n</div></div>",
        escape(label)
    )
}
