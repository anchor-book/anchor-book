"""Exercise scope decisions through real Git commits, including the CLI output."""

from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from mutation_scope import select_scope


SCRIPT = Path(__file__).with_name("mutation_scope.py").resolve()
MANIFEST = '[package]\nname = "fixture"\nversion = "0.1.0"\ndescription = "old"\n[dependencies]\nserde = "1"\n'


class MutationScopeTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.repo = Path(self.directory.name)
        self.git("init", "-q")
        self.git("config", "user.name", "Fixture")
        self.git("config", "user.email", "fixture@example.invalid")

    def git(self, *args):
        return subprocess.check_output(["git", *args], cwd=self.repo, stderr=subprocess.PIPE)

    def commit(self, files):
        for name, contents in files.items():
            path = self.repo / name
            if contents is None:
                path.unlink(missing_ok=True)
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(contents)
        self.git("add", "-A")
        self.git("commit", "-qm", "fixture", "--allow-empty")
        return self.git("rev-parse", "HEAD").decode().strip()

    def compare(self, before, after, expected):
        base = self.commit(before)
        head = self.commit(after)
        self.assertEqual(select_scope(self.repo, base, head), expected)

    def test_descriptive_fields_and_formatting(self):
        fields = {
            "description": '"new"', "documentation": '"https://example.invalid/docs"',
            "homepage": '"https://example.invalid"', "repository": '"https://example.invalid/git"',
            "license": '"MIT"', "license-file": '"LICENSE"', "readme": 'false',
            "authors": '["Author"]', "keywords": '["design"]', "categories": '["development-tools"]',
        }
        for key, value in fields.items():
            with self.subTest(key=key):
                changed = MANIFEST.replace('description = "old"', f"{key} = {value}")
                self.compare({"Cargo.toml": MANIFEST}, {"Cargo.toml": changed}, "none")
                self.compare({"Cargo.toml": changed}, {"Cargo.toml": MANIFEST}, "none")
        self.compare({"Cargo.toml": MANIFEST}, {"Cargo.toml": "# comment\n" + MANIFEST.replace(" = ", "=")}, "none")

    def test_build_affecting_manifest_changes_require_full_scope(self):
        changes = [
            MANIFEST.replace('version = "0.1.0"', 'version = "0.2.0"'),
            MANIFEST.replace('name = "fixture"', 'name = "renamed"'),
            MANIFEST.replace('serde = "1"', 'serde = "2"'),
            MANIFEST.replace('[dependencies]', 'build = "setup.rs"\n[dependencies]'),
            MANIFEST.replace('[dependencies]', 'edition = "2024"\n[dependencies]'),
            MANIFEST.replace('[dependencies]', 'rust-version = "1.88"\n[dependencies]'),
            MANIFEST.replace('[dependencies]', 'exclude = ["src/other.rs"]\n[dependencies]'),
            MANIFEST + '\n[features]\ndefault = ["feature"]\nfeature = []\n',
            MANIFEST + '\n[package.metadata]\nconfiguration = "changed"\n',
            MANIFEST + '\n[profile.release]\nopt-level = 2\n',
            MANIFEST + '\n[lib]\npath = "other.rs"\n',
            MANIFEST + '\n[lints.rust]\nunsafe_code = "forbid"\n',
            MANIFEST.replace('description = "old"', 'description = { workspace = true }'),
            MANIFEST + '\n[target."cfg(unix)".dependencies]\nlibc = "0.2"\n',
        ]
        for changed in changes:
            with self.subTest(changed=changed):
                self.compare({"Cargo.toml": MANIFEST}, {"Cargo.toml": changed}, "full")

    def test_metadata_does_not_hide_source_contract_or_test_changes(self):
        for path in ["src/lib.rs", "tests/acceptance.rs", "contract/writing.md"]:
            with self.subTest(path=path):
                self.compare({"Cargo.toml": MANIFEST, path: "before"},
                             {"Cargo.toml": MANIFEST.replace('"old"', '"new"'), path: "after"}, "diff")

    def test_settings_and_nested_manifests(self):
        for path in ["Cargo.lock", "verification/Cargo.lock", "build.rs", "verification/build.rs",
                     "rust-toolchain.toml", ".cargo/mutants.toml", ".github/workflows/mutation.yml",
                     ".github/scripts/mutation_scope.py", ".github/scripts/test_mutation_scope.py"]:
            with self.subTest(path=path):
                self.compare({path: "before"}, {path: "after"}, "full")
        self.compare({"verification/Cargo.toml": MANIFEST},
                     {"verification/Cargo.toml": MANIFEST.replace('serde = "1"', 'serde = "2"')}, "full")

    def test_build_values_are_compared_without_coercing_types(self):
        for before_value, after_value in [("1", "true"), ("1", "1.0")]:
            with self.subTest(after_value=after_value):
                before = MANIFEST + f"\n[package.metadata]\nsetting = {before_value}\n"
                after = MANIFEST + f"\n[package.metadata]\nsetting = {after_value}\n"
                self.compare({"Cargo.toml": before}, {"Cargo.toml": after}, "full")

    def test_metadata_names_in_other_tables_are_not_ignored(self):
        for suffix in ['[dependencies]\ndescription = "1"\n', '[workspace.package]\ndescription = "1"\n']:
            with self.subTest(suffix=suffix):
                self.compare({"Cargo.toml": suffix}, {"Cargo.toml": suffix.replace('"1"', '"2"')}, "full")

    def test_multiline_description_cannot_hide_dependency_changes(self):
        before = MANIFEST.replace('"old"', '\'\'\'Example:\n[dependencies]\nserde = "9"\n\'\'\'')
        self.compare({"Cargo.toml": before}, {"Cargo.toml": before.replace('serde = "9"', 'serde = "8"')}, "none")
        self.compare({"Cargo.toml": before}, {"Cargo.toml": before.replace('serde = "1"', 'serde = "2"')}, "full")

    def test_missing_and_unparseable_manifests_require_full_scope(self):
        for contents in ["[package", MANIFEST + '[package]\nname = "duplicate"\n', None]:
            with self.subTest(contents=contents):
                self.compare({"Cargo.toml": MANIFEST}, {"Cargo.toml": contents}, "full")
                self.compare({"Cargo.toml": contents, "README.md": "fixture"}, {"Cargo.toml": MANIFEST}, "full")

    def test_cli_reports_skipped_campaign_and_invalid_comparison_fails(self):
        base = self.commit({"Cargo.toml": MANIFEST, "README.md": "before"})
        head = self.commit({"Cargo.toml": MANIFEST.replace('"old"', '"new"'), "README.md": "after"})
        result = subprocess.run([sys.executable, "-B", str(SCRIPT), base, head], cwd=self.repo, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "code=false\nscope=none\n")
        invalid = subprocess.run([sys.executable, "-B", str(SCRIPT), "missing-revision", head], cwd=self.repo, capture_output=True, text=True)
        self.assertNotEqual(invalid.returncode, 0)
        self.assertEqual(invalid.stdout, "")


if __name__ == "__main__":
    unittest.main()
