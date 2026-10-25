use super::{Error, ErrorKind, Phase};
use serde::Deserialize;
use std::{collections::BTreeMap, ops::Range, path::Path};
use toml_edit::{Document, TableLike};

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct Raw {
    schema_version: i64,
    project: Project,
    book: Book,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Project {
    phase: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Book {
    source: String,
    output: String,
    sections: Sections,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct Sections {
    design: String,
    authority: String,
    implementation_roadmaps: String,
}

pub(super) struct Settings {
    pub phase: Phase,
    pub source: String,
    pub output: String,
    pub sections: [String; 3],
    spans: BTreeMap<String, Range<usize>>,
}
impl Settings {
    pub fn span(&self, field: &str) -> Option<Range<usize>> {
        self.spans.get(field).cloned()
    }
}

pub(super) fn parse(path: &Path, text: &str) -> Result<Settings, Error> {
    let document = Document::parse(text).map_err(|e| {
        Error::new(
            ErrorKind::Parse,
            path,
            None,
            e.span(),
            e.message(),
            Some(Box::new(e.clone())),
        )
    })?;
    check_fields(
        path,
        document.as_table(),
        "",
        &["schema-version", "project", "book"],
    )?;
    for (table_path, allowed) in [
        ("project", &["phase"][..]),
        ("book", &["source", "output", "sections"][..]),
        (
            "book.sections",
            &["design", "authority", "implementation-roadmaps"][..],
        ),
    ] {
        let mut keys = table_path.split('.');
        let mut item = document.get(keys.next().unwrap());
        for key in keys {
            item = item
                .and_then(|i| i.as_table_like())
                .and_then(|t| t.get(key));
        }
        if let Some(table) = item.and_then(|i| i.as_table_like()) {
            check_fields(path, table, table_path, allowed)?;
        }
    }
    let field_span = |field: &str| {
        let mut keys = field.split('.');
        let mut item = document.get(keys.next()?)?;
        for key in keys {
            item = item.as_table_like()?.get(key)?;
        }
        item.span()
    };
    let raw =
        Raw::deserialize(toml_edit::de::Deserializer::from(document.clone())).map_err(|e| {
            Error::new(
                ErrorKind::InvalidValue,
                path,
                None,
                e.span(),
                e.message(),
                Some(Box::new(e.clone())),
            )
        })?;
    if raw.schema_version != 1 {
        return Err(Error::new(
            ErrorKind::UnsupportedVersion,
            path,
            Some("schema-version"),
            field_span("schema-version"),
            "supported schema-version is 1",
            None,
        ));
    }
    let phase = match raw.project.phase.as_str() {
        "prototyping" => Phase::Prototyping,
        "building" => Phase::Building,
        "complete" => Phase::Complete,
        _ => {
            return Err(Error::invalid(
                path,
                Some("project.phase"),
                field_span("project.phase"),
                "phase must be prototyping, building, or complete",
            ));
        }
    };
    let spans = [
        "book.source",
        "book.output",
        "book.sections.design",
        "book.sections.authority",
        "book.sections.implementation-roadmaps",
    ]
    .into_iter()
    .filter_map(|field| field_span(field).map(|span| (field.to_owned(), span)))
    .collect();
    Ok(Settings {
        phase,
        source: raw.book.source,
        output: raw.book.output,
        sections: [
            raw.book.sections.design,
            raw.book.sections.authority,
            raw.book.sections.implementation_roadmaps,
        ],
        spans,
    })
}

fn check_fields(
    path: &Path,
    table: &dyn TableLike,
    prefix: &str,
    allowed: &[&str],
) -> Result<(), Error> {
    for (key, value) in table.iter() {
        if !allowed.contains(&key) {
            let field = if prefix.is_empty() {
                key.to_owned()
            } else {
                format!("{prefix}.{key}")
            };
            return Err(Error::new(
                ErrorKind::UnknownField,
                path,
                Some(&field),
                value.span(),
                format!("unknown field; expected only {}", allowed.join(", ")),
                None,
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../tests/fixtures/configuration.toml");

    #[test]
    fn phases_are_explicit_and_preserve_declared_paths() {
        for (name, phase) in [
            ("prototyping", Phase::Prototyping),
            ("building", Phase::Building),
            ("complete", Phase::Complete),
        ] {
            let source = INPUT.replace("prototyping", name);
            let value = parse(Path::new("anchorbook.toml"), &source).unwrap();
            assert_eq!(value.phase, phase);
            assert_eq!(value.source, "docs");
            assert_eq!(value.output, "generated/site");
            assert_eq!(value.sections, ["overview.md", "rules.md", "work.md"]);
            assert_eq!(
                &source[value.span("book.sections.design").unwrap()],
                "\"overview.md\""
            );
        }
    }

    #[test]
    fn required_fields_types_and_values_are_not_defaulted() {
        let path = Path::new("anchorbook.toml");
        for line in INPUT.lines().filter(|l| l.contains(" = ")) {
            let source = INPUT.replace(line, "");
            assert_eq!(
                parse(path, &source).err().unwrap().kind(),
                ErrorKind::InvalidValue,
                "{line}"
            );
        }
        for (old, new, kind) in [
            (
                "schema-version = 1",
                "schema-version = 2",
                ErrorKind::UnsupportedVersion,
            ),
            (
                "schema-version = 1",
                "schema-version = -1",
                ErrorKind::UnsupportedVersion,
            ),
            (
                "schema-version = 1",
                "schema-version = \"1\"",
                ErrorKind::InvalidValue,
            ),
            ("\"prototyping\"", "\"prototype\"", ErrorKind::InvalidValue),
            ("\"prototyping\"", "1", ErrorKind::InvalidValue),
            ("\"docs\"", "true", ErrorKind::InvalidValue),
            ("\"generated/site\"", "[]", ErrorKind::InvalidValue),
            ("\"overview.md\"", "[]", ErrorKind::InvalidValue),
            ("[book.sections]", "[other]", ErrorKind::UnknownField),
        ] {
            let source = INPUT.replace(old, new);
            let error = parse(path, &source).err().unwrap();
            assert_eq!(error.kind(), kind, "{new}: {error}");
            assert_eq!(error.path(), path);
            assert!(!error.message().is_empty());
        }
        let source = INPUT.replace("\"prototyping\"", "\"unknown\"");
        let error = parse(path, &source).err().unwrap();
        assert_eq!(error.field(), Some("project.phase"));
        assert_eq!(&source[error.span().unwrap()], "\"unknown\"");
        let error = parse(
            path,
            &INPUT.replace("schema-version = 1", "schema-version = 3"),
        )
        .err()
        .unwrap();
        assert_eq!(error.field(), Some("schema-version"));
    }

    #[test]
    fn syntax_and_unknown_fields_have_distinct_located_errors() {
        use std::error::Error as _;
        let path = Path::new("input.toml");
        for (source, field) in [
            (format!("extra = 1\n{INPUT}"), "extra"),
            (
                INPUT.replace("[project]", "[project]\nextra = 1"),
                "project.extra",
            ),
            (INPUT.replace("[book]", "[book]\nextra = 1"), "book.extra"),
            (
                INPUT.replace("[book.sections]", "[book.sections]\nextra = 1"),
                "book.sections.extra",
            ),
        ] {
            let error = parse(path, &source).err().unwrap();
            assert_eq!(error.kind(), ErrorKind::UnknownField);
            assert_eq!(error.field(), Some(field));
            assert_eq!(&source[error.span().unwrap()], "1");
            assert!(error.message().contains("expected only"));
        }
        for source in [
            INPUT.replace("schema-version = 1", "schema-version = ["),
            format!("schema-version = 1\n{INPUT}"),
        ] {
            let error = parse(path, &source).err().unwrap();
            assert_eq!(error.kind(), ErrorKind::Parse);
            assert!(error.span().is_some());
            assert!(error.source().is_some());
            assert!(error.to_string().contains("input.parse-failed: input.toml"));
        }
        let error = parse(path, &INPUT.replace("\"docs\"", "7")).err().unwrap();
        assert!(error.source().is_some());
    }
}
