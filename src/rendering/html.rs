use super::links;
use crate::records::{Record, RecordData, RecordInventory, RecordKind, Status};
use std::{collections::BTreeMap, fmt::Write, path::Path};

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;")
}
fn paragraphs(text: &str) -> String {
    // Authored source wrapping is not a paragraph boundary. Keep blank-line
    // paragraphs while presenting prose naturally at any viewport width.
    let text = text
        .trim()
        .split("\n\n")
        .map(|paragraph| paragraph.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("<div class=\"ab-text\">{}</div>", escape(&text))
}
fn field(label: &str, text: &str) -> String {
    format!(
        "<div class=\"ab-field\"><h4>{label}</h4>{}</div>",
        paragraphs(text)
    )
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
        "\n<section class=\"ab-record ab-{}\" id=\"{}\" aria-label=\"{}\">\n<header class=\"ab-record-header\"><div class=\"ab-record-meta\"><span>{kind}</span><span class=\"ab-status\">{status}</span></div><h3>{}</h3><a class=\"ab-permalink\" href=\"#{}\" aria-label=\"Permalink: {}\">{}</a></header>\n",
        kind.to_lowercase(),
        record.id(),
        escape(record.title()),
        escape(record.title()),
        record.id(),
        escape(record.title()),
        record.id()
    );
    match record.data() {
        RecordData::Anchor(anchor) => {
            out.push_str(&field("Property", &anchor.property));
        }
        RecordData::Requirement(requirement) => {
            out.push_str(&field("Requirement", &requirement.statement));
            out.push_str(&field("Source", &requirement.source));
        }
        RecordData::Decision(decision) => {
            out.push_str(&field("Decision", &decision.choice));
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
                write!(
                    out,
                    "<li><code>{}</code>{}<p>Required verification: {methods}</p></li>",
                    escape(&criterion.id),
                    paragraphs(&criterion.statement)
                )
                .expect("writing to String");
            }
            out.push_str("</ol></div>");
        }
        RecordData::Decision(decision) => {
            out.push_str("<details class=\"ab-details\"><summary>Rationale, alternatives, and reconsideration</summary><div class=\"ab-details-body\">");
            out.push_str(&field("Rationale", &decision.rationale));
            out.push_str("<div class=\"ab-field\"><h4>Alternatives</h4><ul>");
            for alternative in &decision.alternatives {
                write!(out, "<li>{}</li>", paragraphs(alternative)).expect("writing to String");
            }
            out.push_str("</ul></div>");
            out.push_str(&field("Reconsider when", &decision.reconsider_when));
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
    out.push_str("</section>\n\n");
    out
}
