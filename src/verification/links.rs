//! Evidence links: `/// anchorbook: <anchor-id>#<criterion-id>` doc-comment lines
//! on the functions they link.
//!
//! A line scanner finds every comment line that starts `anchorbook:`; the Rust
//! parser attaches doc-comment links to their functions. Any link line the
//! parser does not attach to a supported function is a located finding, so a
//! misplaced or mistyped link is never silently ignored.
use crate::records::{RecordData, RecordInventory, VerificationMethod};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    ops::RangeInclusive,
    path::{Path, PathBuf},
};
use syn::{Attribute, Expr, Item, Lit, Meta};

/// The crate target that contains a linked function.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Target {
    /// The package's library.
    Lib,
    /// A binary, by name.
    Bin(String),
    /// An integration-test target, by name.
    Test(String),
}
impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lib => write!(f, "lib"),
            Self::Bin(name) => write!(f, "bin {name}"),
            Self::Test(name) => write!(f, "test {name}"),
        }
    }
}

/// A linked function's identity: package, crate target, module path, and name.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId {
    /// Cargo package name.
    pub package: String,
    /// Crate target within the package.
    pub target: Target,
    /// Module path within the target, outermost first.
    pub module: Vec<String>,
    /// Function name.
    pub name: String,
}
impl FunctionId {
    /// The module path and name, as `module::name`.
    pub fn path(&self) -> String {
        let mut parts = self.module.clone();
        parts.push(self.name.clone());
        parts.join("::")
    }
}
impl fmt::Display for FunctionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} {})", self.path(), self.package, self.target)
    }
}

/// What kind of function carries a link, which determines the method it links.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// A function marked with a test attribute, such as `#[test]` or `#[tokio::test]`.
    Test,
    /// An executable function with a Verus contract, such as `#[verus_spec]`.
    Verified,
}

/// A valid link from a function to an anchor criterion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    /// Linked anchor.
    pub anchor: String,
    /// Linked criterion of that anchor.
    pub criterion: String,
    /// The method the link supplies evidence for.
    pub method: VerificationMethod,
    /// The linked function.
    pub function: FunctionId,
    /// The function's kind.
    pub mode: Mode,
    /// For a vacuity witness, the linked function it calls.
    pub witness: Option<String>,
    /// Source file, relative to the configuration directory.
    pub path: PathBuf,
    /// One-based line of the link.
    pub line: usize,
}

/// A located problem with a link or with a source file that may hold links.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkFinding {
    /// Source file, relative to the configuration directory.
    pub path: PathBuf,
    /// One-based line.
    pub line: usize,
    /// Stable code, such as `verify.link-misplaced`.
    pub code: &'static str,
    /// What is wrong and how to correct it.
    pub message: String,
}
impl fmt::Display for LinkFinding {
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

/// A link line found by the scanner: its line, and whether it is a doc comment.
struct Candidate {
    line: usize,
    doc: bool,
}

/// Comments whose text starts `anchorbook:`. A line's comment starts at its
/// first comment marker outside a string literal; markers inside the comment
/// text, such as a quoted `///`, do not start another.
fn candidates(text: &str, literals: &[Span]) -> Vec<Candidate> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let inside_string = |at: usize| {
            let position = (index + 1, line[..at].chars().count());
            literals
                .iter()
                .any(|(start, end)| *start <= position && position < *end)
        };
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        // A block comment's continuation line: ` * anchorbook: ...`.
        if let Some(after) = trimmed.strip_prefix('*')
            && after.trim_start().starts_with("anchorbook:")
            && !inside_string(indent)
        {
            found.push(Candidate {
                line: index + 1,
                doc: true,
            });
            continue;
        }
        let Some(at) = line.match_indices('/').map(|(at, _)| at).find(|&at| {
            (line[at..].starts_with("//") || line[at..].starts_with("/*")) && !inside_string(at)
        }) else {
            continue;
        };
        let rest = &line[at..];
        let doc = rest.starts_with("///") || rest.starts_with("//!") || rest.starts_with("/**");
        let text = &rest[if doc { 3 } else { 2 }..];
        if text.trim_start().starts_with("anchorbook:") {
            found.push(Candidate {
                line: index + 1,
                doc,
            });
        }
    }
    found
}

/// Where a string literal starts and ends, as one-based lines and zero-based
/// character columns.
type Span = ((usize, usize), (usize, usize));

/// Spans of the string literals in parsed source, so comment-like text inside a
/// string, such as fixture source in a test, is never read as a link. Doc
/// comments reach the token stream as string literals too; they are told apart
/// by the source text at their start, which is a comment marker.
fn string_literals(text: &str) -> Vec<Span> {
    fn walk(tokens: proc_macro2::TokenStream, lines: &[&str], out: &mut Vec<Span>) {
        for token in tokens {
            match token {
                proc_macro2::TokenTree::Group(group) => walk(group.stream(), lines, out),
                proc_macro2::TokenTree::Literal(literal) => {
                    let start = literal.span().start();
                    let end = literal.span().end();
                    let source: String = lines
                        .get(start.line.saturating_sub(1))
                        .map(|l| l.chars().skip(start.column).collect())
                        .unwrap_or_default();
                    let string = literal.to_string();
                    let is_string = string.ends_with('"') || string.ends_with('#');
                    if is_string && !source.starts_with("//") && !source.starts_with("/*") {
                        out.push(((start.line, start.column), (end.line, end.column)));
                    }
                }
                _ => {}
            }
        }
    }
    let lines: Vec<&str> = text.lines().collect();
    let mut spans = Vec::new();
    if let Ok(tokens) = text.parse::<proc_macro2::TokenStream>() {
        walk(tokens, &lines, &mut spans);
    }
    spans
}

/// The package and crate target a source file belongs to, and its module path.
fn context(file: &Path, base: &Path) -> Option<(String, Target, Vec<String>)> {
    let mut directory = file.parent();
    while let Some(dir) = directory {
        if let Some(name) = package_name(&dir.join("Cargo.toml")) {
            return target_of(file.strip_prefix(dir).ok()?, dir, name);
        }
        if dir == base {
            break;
        }
        directory = dir.parent();
    }
    None
}

fn package_name(manifest: &Path) -> Option<String> {
    let text = fs::read_to_string(manifest).ok()?;
    let document = toml_edit::Document::parse(text.as_str()).ok()?;
    document
        .get("package")?
        .get("name")?
        .as_str()
        .map(str::to_owned)
}

fn module_of(parts: &[String]) -> Vec<String> {
    let mut module: Vec<String> = parts.to_vec();
    if let Some(last) = module.pop() {
        let stem = last.strip_suffix(".rs").unwrap_or(&last);
        if !matches!(stem, "mod" | "lib" | "main") {
            module.push(stem.to_owned());
        }
    }
    module
}

fn target_of(
    relative: &Path,
    package_dir: &Path,
    package: String,
) -> Option<(String, Target, Vec<String>)> {
    let parts: Vec<String> = relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    let names: Vec<&str> = parts.iter().map(String::as_str).collect();
    let stem = |name: &str| name.strip_suffix(".rs").unwrap_or(name).to_owned();
    let (target, module) = match names.as_slice() {
        ["src", "main.rs"] => (Target::Bin(package.clone()), Vec::new()),
        ["src", "bin", file] => (Target::Bin(stem(file)), Vec::new()),
        ["src", "bin", dir, ..] => (Target::Bin((*dir).to_owned()), module_of(&parts[3..])),
        ["src", ..] => (Target::Lib, module_of(&parts[1..])),
        ["tests", file] => (Target::Test(stem(file)), Vec::new()),
        ["tests", dir, ..]
            if package_dir
                .join("tests")
                .join(dir)
                .join("main.rs")
                .is_file() =>
        {
            (Target::Test((*dir).to_owned()), module_of(&parts[2..]))
        }
        _ => return None,
    };
    Some((package, target, module))
}

fn mode_of(attrs: &[Attribute]) -> Option<Mode> {
    let last = |attr: &Attribute| {
        attr.path()
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default()
    };
    if attrs.iter().any(|a| last(a) == "test") {
        return Some(Mode::Test);
    }
    let verified = attrs.iter().any(|attr| match last(attr).as_str() {
        "verus_spec" | "verus_verify" => true,
        "cfg_attr" => matches!(&attr.meta, Meta::List(list)
            if list.tokens.to_string().contains("verus_spec")),
        _ => false,
    });
    verified.then_some(Mode::Verified)
}

/// Link text and its line from doc attributes.
fn doc_links(attrs: &[Attribute]) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        let Meta::NameValue(pair) = &attr.meta else {
            continue;
        };
        let Expr::Lit(literal) = &pair.value else {
            continue;
        };
        let Lit::Str(text) = &literal.lit else {
            continue;
        };
        let start = attr.pound_token.span.start().line;
        for (offset, line) in text.value().lines().enumerate() {
            let line_text = line.trim().trim_start_matches('*').trim_start();
            if let Some(rest) = line_text.strip_prefix("anchorbook:") {
                found.push((start + offset, rest.trim().to_owned()));
            }
        }
    }
    found
}

/// Parsed link text: anchor, criterion, and an optional witnessed function.
fn parse_link(text: &str) -> Result<(String, String, Option<String>), String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let identifier = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    };
    let usage = "expected `anchorbook: <anchor-id>#<criterion-id>`, optionally followed by `witness <function>`";
    let (reference, witness) = match words.as_slice() {
        [reference] => (*reference, None),
        [reference, "witness", function] if identifier(function) => {
            (*reference, Some((*function).to_owned()))
        }
        _ => return Err(usage.to_owned()),
    };
    match reference.split_once('#') {
        Some((anchor, criterion)) if identifier(anchor) && identifier(criterion) => {
            Ok((anchor.to_owned(), criterion.to_owned(), witness))
        }
        _ => Err(usage.to_owned()),
    }
}

struct FileReader<'a> {
    path: PathBuf,
    context: Option<(String, Target, Vec<String>)>,
    inventory: &'a RecordInventory,
    attached: BTreeSet<usize>,
    macros: Vec<RangeInclusive<usize>>,
    links: Vec<(Link, usize)>,
    findings: Vec<LinkFinding>,
}
impl FileReader<'_> {
    fn finding(&mut self, line: usize, code: &'static str, message: impl Into<String>) {
        self.findings.push(LinkFinding {
            path: self.path.clone(),
            line,
            code,
            message: message.into(),
        });
    }

    fn items(&mut self, items: &[Item], module: &[String]) {
        for item in items {
            match item {
                Item::Fn(function) => {
                    let name = function.sig.ident.to_string();
                    let definition = function.sig.ident.span().start().line;
                    self.function(&function.attrs, &name, module, definition);
                }
                Item::Mod(inner) => {
                    if let Some((_, items)) = &inner.content {
                        let mut path = module.to_vec();
                        path.push(inner.ident.to_string());
                        self.items(items, &path);
                    }
                }
                Item::Impl(block) => {
                    for item in &block.items {
                        if let syn::ImplItem::Fn(method) = item {
                            for (line, _) in doc_links(&method.attrs) {
                                self.attached.insert(line);
                                self.finding(
                                    line,
                                    "verify.link-unsupported",
                                    format!(
                                        "links on methods such as `{}` are not supported yet; link a module-level function",
                                        method.sig.ident
                                    ),
                                );
                            }
                        }
                    }
                }
                Item::Macro(invocation) => {
                    let start = invocation
                        .mac
                        .path
                        .segments
                        .first()
                        .map_or(0, |s| s.ident.span().start().line);
                    let end = invocation.mac.delimiter.span().close().end().line;
                    self.macros.push(start..=end);
                }
                _ => {}
            }
        }
    }

    fn function(&mut self, attrs: &[Attribute], name: &str, module: &[String], definition: usize) {
        let links = doc_links(attrs);
        if links.is_empty() {
            return;
        }
        let mode = mode_of(attrs);
        let mut seen = BTreeSet::new();
        for (line, text) in links {
            self.attached.insert(line);
            let Some((package, target, file_module)) = self.context.clone() else {
                self.finding(
                    line,
                    "verify.link-unsupported",
                    "the file is not part of a crate target the engine can identify (src/, src/bin/, or tests/ of a Cargo package)",
                );
                continue;
            };
            let Some(mode) = mode else {
                self.finding(
                    line,
                    "verify.link-unsupported",
                    format!(
                        "`{name}` is neither a test (`#[test]`) nor a verified function (`#[verus_spec]`), so the link supplies no evidence"
                    ),
                );
                continue;
            };
            let (anchor, criterion, witness) = match parse_link(&text) {
                Ok(parsed) => parsed,
                Err(message) => {
                    self.finding(line, "verify.link-malformed", message);
                    continue;
                }
            };
            if !seen.insert((anchor.clone(), criterion.clone())) {
                self.finding(
                    line,
                    "verify.link-duplicate",
                    format!("`{name}` already links `{anchor}#{criterion}`"),
                );
                continue;
            }
            let method = match mode {
                Mode::Test => VerificationMethod::Test,
                Mode::Verified => VerificationMethod::Proof,
            };
            if witness.is_some() && mode != Mode::Verified {
                self.finding(
                    line,
                    "verify.link-malformed",
                    "a witness must be a verified function",
                );
                continue;
            }
            if let Err(message) = self.resolve(&anchor, &criterion, method) {
                let code = if message.contains("requires") {
                    "verify.link-method-mismatch"
                } else {
                    "verify.link-dangling"
                };
                self.finding(line, code, message);
                continue;
            }
            let mut module_path = file_module.clone();
            module_path.extend(module.iter().cloned());
            self.links.push((
                Link {
                    anchor,
                    criterion,
                    method,
                    function: FunctionId {
                        package,
                        target,
                        module: module_path,
                        name: name.to_owned(),
                    },
                    mode,
                    witness,
                    path: self.path.clone(),
                    line,
                },
                definition,
            ));
        }
    }

    fn resolve(
        &self,
        anchor: &str,
        criterion: &str,
        method: VerificationMethod,
    ) -> Result<(), String> {
        let record = self
            .inventory
            .get(anchor)
            .ok_or_else(|| format!("`{anchor}` is not a record in this Book"))?;
        let RecordData::Anchor(data) = record.data() else {
            return Err(format!("`{anchor}` is not an anchor"));
        };
        let found = data
            .criteria
            .iter()
            .find(|c| c.id == criterion)
            .ok_or_else(|| format!("anchor `{anchor}` has no criterion `{criterion}`"))?;
        if !found.verification.contains(&method) {
            let required: Vec<&str> = found.verification.iter().map(|m| m.as_str()).collect();
            return Err(format!(
                "the link supplies {} evidence, but `{anchor}#{criterion}` requires {}",
                method.as_str(),
                required.join(", ")
            ));
        }
        Ok(())
    }
}

fn rust_files(directory: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    let mut entries: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Read every link in `sources`, returning valid links and located findings.
pub(super) fn read(
    sources: &[PathBuf],
    base: &Path,
    inventory: &RecordInventory,
) -> (Vec<Link>, Vec<LinkFinding>) {
    let mut files = Vec::new();
    for source in sources {
        rust_files(source, &mut files);
    }
    files.sort();
    files.dedup();
    let mut links = Vec::new();
    let mut findings = Vec::new();
    for file in files {
        let display = file.strip_prefix(base).unwrap_or(&file).to_owned();
        let Ok(text) = fs::read_to_string(&file) else {
            findings.push(LinkFinding {
                path: display,
                line: 1,
                code: "verify.source-unreadable",
                message: "the file cannot be read, so links in it are not read".into(),
            });
            continue;
        };
        if !text.contains("anchorbook:") {
            continue;
        }
        let parsed = match syn::parse_file(&text) {
            Ok(parsed) => parsed,
            Err(error) => {
                findings.push(LinkFinding {
                    path: display,
                    line: error.span().start().line,
                    code: "verify.source-unparsable",
                    message: format!(
                        "the file does not parse as Rust ({error}), so its links are not read"
                    ),
                });
                continue;
            }
        };
        let candidates = candidates(&text, &string_literals(&text));
        let mut reader = FileReader {
            path: display,
            context: context(&file, base),
            inventory,
            attached: BTreeSet::new(),
            macros: Vec::new(),
            links: Vec::new(),
            findings: Vec::new(),
        };
        reader.items(&parsed.items, &[]);
        for candidate in &candidates {
            if reader.attached.contains(&candidate.line) {
                continue;
            }
            let message = if reader.macros.iter().any(|m| m.contains(&candidate.line)) {
                "links inside macro invocations such as `verus!` are not read; link a function outside the macro"
            } else if !candidate.doc {
                "this is an ordinary comment, which links nothing; write `/// anchorbook: <anchor-id>#<criterion-id>` on the function"
            } else {
                "this link is not on a module-level function, so it links nothing"
            };
            let code = if message.starts_with("links inside") {
                "verify.link-unsupported"
            } else {
                "verify.link-misplaced"
            };
            reader.finding(candidate.line, code, message);
        }
        links.append(&mut reader.links);
        findings.append(&mut reader.findings);
    }
    // One identity defined at two places, such as under different cfgs, is ambiguous.
    let mut definitions: BTreeMap<FunctionId, BTreeSet<(PathBuf, usize)>> = BTreeMap::new();
    for (link, definition) in &links {
        definitions
            .entry(link.function.clone())
            .or_default()
            .insert((link.path.clone(), *definition));
    }
    let mut valid = Vec::new();
    for (link, _) in links {
        if definitions[&link.function].len() > 1 {
            findings.push(LinkFinding {
                message: format!(
                    "`{}` is defined more than once, so its results cannot be told apart",
                    link.function
                ),
                path: link.path,
                line: link.line,
                code: "verify.link-ambiguous",
            });
        } else {
            valid.push(link);
        }
    }
    (valid, findings)
}

#[cfg(test)]
mod tests {
    use super::{candidates, parse_link};

    #[test]
    fn link_text_names_an_anchor_criterion_and_an_optional_witness() {
        assert_eq!(
            parse_link("A-x#y").unwrap(),
            ("A-x".to_owned(), "y".to_owned(), None)
        );
        assert_eq!(
            parse_link("x#y witness f").unwrap(),
            ("x".to_owned(), "y".to_owned(), Some("f".to_owned()))
        );
        assert_eq!(
            parse_link("A_b-1#c_d").unwrap(),
            ("A_b-1".to_owned(), "c_d".to_owned(), None)
        );
        for text in [
            "",
            "x",
            "#y",
            "x#",
            "x#y z",
            "x#y witness",
            "x#y witness a b",
            "x y#z",
            "x#y witness f!",
            "x.z#y",
            "x#y.z",
        ] {
            assert!(parse_link(text).is_err(), "{text}");
        }
    }

    #[test]
    fn the_scanner_finds_link_lines_in_every_comment_form() {
        let text = "/// anchorbook: a#b\n// anchorbook: a#b\n  //! anchorbook: a#b\n/* anchorbook: a#b */\n * anchorbook: a#b\nlet s = \"anchorbook: a#b\";\n/// see anchorbook: a#b\n";
        let found: Vec<(usize, bool)> = candidates(text, &[])
            .iter()
            .map(|c| (c.line, c.doc))
            .collect();
        assert_eq!(
            found,
            [(1, true), (2, false), (3, true), (4, false), (5, true)]
        );
    }

    #[test]
    fn a_comment_starts_at_its_first_marker_outside_strings() {
        let text = concat!(
            "let x = a * b / c; // anchorbook: a#b\n",
            "//! see `/// anchorbook: a#b`\n",
            "let u = \"//\"; // anchorbook: a#b\n",
            "f(\"x\"// anchorbook: a#b\n",
            "*p = 1; // anchorbook: a#b\n",
            "  * anchorbook: a#b\n",
        );
        let lines = |literals: &[super::Span]| -> Vec<(usize, bool)> {
            candidates(text, literals)
                .iter()
                .map(|c| (c.line, c.doc))
                .collect()
        };
        // A quoted `///` inside a comment's text does not start a link, and code
        // characters such as `*` and `/` before a comment do not hide it.
        // Without string spans, the `//` inside the string on line 3 starts its comment.
        assert_eq!(lines(&[]), [(1, false), (4, false), (5, false), (6, true)]);
        // With the strings' spans, comments after them are found, including one
        // that starts exactly where a string ends.
        let strings = [((3, 8), (3, 12)), ((4, 2), (4, 5))];
        assert_eq!(
            lines(&strings),
            [(1, false), (3, false), (4, false), (5, false), (6, true)]
        );
        // A continuation line inside a string is not a link.
        assert_eq!(
            lines(&[((6, 0), (6, 20))]),
            [(1, false), (4, false), (5, false)]
        );
    }
}
