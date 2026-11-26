"""Exercise the event boundary and conservative proof selection with Git history."""

import os
from pathlib import Path
import subprocess
import sys

from ci_scope import campaign_scope
from test_mutation_scope import GitFixture, MANIFEST


SCRIPT = Path(__file__).with_name("ci_scope.py").resolve()


class CampaignScopeTests(GitFixture):
    def scope(self, kind, event, before):
        return campaign_scope(self.repo, kind, event, before)

    def test_release_description_skips_before_and_after_merge(self):
        base = self.commit({"Cargo.toml": MANIFEST, "README.md": "before"})
        self.git("checkout", "-qb", "release")
        self.commit({"Cargo.toml": MANIFEST.replace('"old"', '"new"'), "README.md": "after"})
        self.git("checkout", "--detach", base)
        self.git("merge", "--no-ff", "-m", "merge release", "release")
        for kind in ("mutation", "proof"):
            for event in ("pull_request", "push"):
                with self.subTest(kind=kind, event=event):
                    self.assertEqual(self.scope(kind, event, base), "none")

    def test_push_includes_code_in_earlier_commits(self):
        base = self.commit({"src/lib.rs": "before"})
        self.commit({"src/lib.rs": "after"})
        self.commit({"CHANGELOG.md": "release notes"})
        for kind in ("mutation", "proof"):
            self.assertEqual(self.scope(kind, "push", base), "full")
        # Looking only at the final commit would silently miss the source edit.
        self.assertEqual(self.scope("mutation", "pull_request", base), "none")

    def test_pr_diff_and_main_full_for_source_changes(self):
        base = self.commit({"src/lib.rs": "before"})
        self.commit({"src/lib.rs": "after"})
        self.assertEqual(self.scope("mutation", "pull_request", base), "diff")
        self.assertEqual(self.scope("mutation", "push", base), "full")
        self.assertEqual(self.scope("proof", "pull_request", base), "full")

    def test_selected_release_head_compares_to_previous_main(self):
        base = self.commit({"src/lib.rs": "before"})
        self.git("checkout", "-qb", "release")
        release_head = self.commit({"CHANGELOG.md": "notes"})
        self.git("checkout", "--detach", base)
        previous_main = self.commit({"src/lib.rs": "after"})
        self.git("checkout", "--detach", release_head)
        # The caller can validate a release PR head distinct from the merge SHA.
        # Comparing that selected tree with previous main detects the difference.
        for kind in ("mutation", "proof"):
            self.assertEqual(self.scope(kind, "push", previous_main), "full")

    def test_manual_and_initial_push_require_full_campaigns(self):
        self.commit({"README.md": "only prose"})
        for kind in ("mutation", "proof"):
            for event, before in [("workflow_dispatch", ""), ("workflow_call", ""),
                                  ("push", ""), ("push", "0" * 40)]:
                with self.subTest(kind=kind, event=event, before=before):
                    self.assertEqual(self.scope(kind, event, before), "full")

    def test_prose_skips_but_record_chapters_still_require_proof(self):
        path = "book/design/example.md"
        base = self.commit({path: "# Explanation\n\nOrdinary prose.\n"})
        self.commit({path: "# Explanation\n\nRevised prose.\n"})
        self.assertEqual(self.scope("proof", "push", base), "none")
        for before, after in [
            ("# Example\n", "> **`A-example`** (draft): **Example**\n"),
            ("> **`A-example`** (draft): **Example**\n", "# Removed\n"),
            ("> Record\n", "```markdown\n> Record\n```\n"),
            ("> Record\n\nOld prose\n", "> Record\n\nNew prose\n"),
            ("# Old\n", "    > Indented quote\n"),
        ]:
            with self.subTest(before=before, after=after):
                base = self.commit({path: before})
                self.commit({path: after})
                self.assertEqual(self.scope("proof", "push", base), "full")
        base = self.commit({path: "> Record\n"})
        self.commit({path: None})
        self.assertEqual(self.scope("proof", "push", base), "full")

    def test_configuration_and_unclassified_inputs_require_proof(self):
        for path in ["anchorbook.toml", "book.toml", "book/SUMMARY.md",
                     "verification/helper.rs", "scripts/proof.sh", "new-input.txt",
                     ".github/workflows/proof.yml", ".github/workflows/release-plz.yml",
                     ".github/scripts/ci_scope.py", ".github/scripts/test_ci_scope.py",
                     "contract/writing.md", "tests/proof_runner.rs"]:
            with self.subTest(path=path):
                base = self.commit({path: "before"})
                self.commit({path: "after"})
                self.assertEqual(self.scope("proof", "push", base), "full")

    def test_replacing_prose_with_a_symlink_requires_proof(self):
        path = "book/chapter.md"
        base = self.commit({path: "Ordinary prose"})
        (self.repo / path).unlink()
        (self.repo / path).symlink_to("authority.md")
        self.commit({})
        self.assertEqual(self.scope("proof", "push", base), "full")

    def test_build_changes_require_both_campaigns_even_with_prose(self):
        base = self.commit({"Cargo.toml": MANIFEST})
        self.commit({"Cargo.toml": MANIFEST.replace('serde = "1"', 'serde = "2"'),
                     "README.md": "new docs"})
        for kind in ("mutation", "proof"):
            self.assertEqual(self.scope(kind, "pull_request", base), "full")

    def test_actual_workflow_entrypoint_and_failed_comparisons(self):
        base = self.commit({"README.md": "before"})
        self.commit({"README.md": "after"})
        for kind in ("mutation", "proof"):
            for before in (base, "f" * 40, "not-a-sha"):
                with self.subTest(kind=kind, before=before):
                    result = subprocess.run([sys.executable, "-B", str(SCRIPT), kind],
                        cwd=self.repo, capture_output=True, text=True,
                        env={**os.environ, "GITHUB_EVENT_NAME": "push", "PUSH_BEFORE": before})
                    if before == base:
                        self.assertEqual(result.returncode, 0, result.stderr)
                        self.assertEqual(result.stdout, "code=false\nscope=none\n")
                    else:
                        self.assertNotEqual(result.returncode, 0)
                        self.assertEqual(result.stdout, "")
