use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
pub const CONFIG: &str = "schema-version = 1\n[project]\nphase = \"prototyping\"\n[book]\nsource = \"docs\"\noutput = \"generated/site\"\n[book.sections]\ndesign = \"design.md\"\nauthority = \"authority.md\"\nimplementation-roadmaps = \"roadmap.md\"\n";
pub const SUMMARY: &str =
    "# Summary\n\n- [Design](design.md)\n- [Authority](authority.md)\n- [Work](roadmap.md)\n";
pub const RENDERER: &str =
    "[book]\nsrc = \"docs\"\n[build]\nbuild-dir = \"generated/site\"\ncreate-missing = true\n";
pub struct Book {
    pub root: PathBuf,
}
impl Book {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "anchorbook-check-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let book = Self {
            root: fs::canonicalize(root).unwrap(),
        };
        book.write("anchorbook.toml", CONFIG);
        book.write("book.toml", RENDERER);
        book.write("docs/SUMMARY.md", SUMMARY);
        for path in ["docs/design.md", "docs/authority.md", "docs/roadmap.md"] {
            book.write(
                path,
                "# Unrelated display title\n\nArbitrary authored content.\n",
            );
        }
        book
    }
    pub fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }
    pub fn config(&self) -> PathBuf {
        self.path("anchorbook.toml")
    }
    pub fn write(&self, path: &str, text: &str) {
        let path = self.path(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    pub fn snapshot(&self) -> BTreeMap<PathBuf, (bool, Vec<u8>, Option<PathBuf>)> {
        fn visit(
            root: &Path,
            path: &Path,
            out: &mut BTreeMap<PathBuf, (bool, Vec<u8>, Option<PathBuf>)>,
        ) {
            for entry in fs::read_dir(path).unwrap() {
                let path = entry.unwrap().path();
                let meta = fs::symlink_metadata(&path).unwrap();
                let relative = path.strip_prefix(root).unwrap().to_owned();
                if meta.is_symlink() {
                    out.insert(
                        relative,
                        (false, Vec::new(), Some(fs::read_link(&path).unwrap())),
                    );
                } else if meta.is_dir() {
                    out.insert(relative, (true, Vec::new(), None));
                    visit(root, &path, out);
                } else {
                    out.insert(relative, (false, fs::read(&path).unwrap(), None));
                }
            }
        }
        let mut result = BTreeMap::new();
        visit(&self.root, &self.root, &mut result);
        result
    }
}
impl Drop for Book {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
