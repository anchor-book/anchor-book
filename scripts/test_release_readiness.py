import copy
import unittest

from release_readiness import REPOSITORY, WORKFLOWS, ready

SHA = "a" * 40


class Readiness(unittest.TestCase):
    def setUp(self):
        self.pr = {"merged_at": "2026-11-25", "merge_commit_sha": SHA,
                   "base": {"ref": "main", "repo": {"full_name": REPOSITORY}},
                   "head": {"ref": "release-plz-2026-11-25", "sha": "b" * 40,
                            "repo": {"full_name": REPOSITORY}}}
        self.parents = [{"sha": "c" * 40}]
        self.comparison = "diverged"
        run = {"head_sha": SHA, "head_branch": "main", "event": "push",
               "head_repository": {"full_name": REPOSITORY}, "run_number": 1,
               "run_attempt": 1, "status": "completed", "conclusion": "success"}
        self.runs = {w: [copy.deepcopy(run)] for w in WORKFLOWS}

    def api(self, path):
        if "/pulls?" in path:
            return [self.pr]
        if "/compare/" in path:
            return {"status": self.comparison}
        if "/workflows/" in path:
            workflow = path.split("/workflows/")[1].split("/")[0]
            return {"workflow_runs": self.runs[workflow]}
        return {"parents": self.parents}

    def test_checked_release_and_retry_are_eligible(self):
        self.assertTrue(ready(SHA, self.api)[0])
        self.assertTrue(ready(SHA, self.api)[0])

    def test_every_workflow_must_pass(self):
        for w in WORKFLOWS:
            for result in ("failure", "cancelled", "skipped", "neutral", None):
                with self.subTest(workflow=w, conclusion=result):
                    self.setUp()
                    self.runs[w][0]["conclusion"] = result
                    self.assertFalse(ready(SHA, self.api)[0])
            self.setUp()
            self.runs[w] = []
            self.assertFalse(ready(SHA, self.api)[0])

    def test_pending_or_newer_failed_run_overrides_old_success(self):
        for status, conclusion in (("in_progress", "success"), ("completed", "failure")):
            self.setUp()
            newer = dict(self.runs["book.yml"][0], run_attempt=2,
                         status=status, conclusion=conclusion)
            self.runs["book.yml"].append(newer)
            self.assertFalse(ready(SHA, self.api)[0])

    def test_other_commit_branch_event_or_repository_is_not_evidence(self):
        for field, value in (("head_sha", "d" * 40), ("head_branch", "other"),
                             ("event", "pull_request"),
                             ("head_repository", {"full_name": "someone/fork"})):
            self.setUp()
            self.runs["package.yml"][0][field] = value
            self.assertFalse(ready(SHA, self.api)[0])

    def test_nonrelease_unmerged_or_fork_pr_does_not_publish(self):
        for case in ("ordinary", "unmerged", "fork", "other-merge"):
            self.setUp()
            if case == "ordinary":
                self.pr["head"]["ref"] = "feature"
            elif case == "unmerged":
                self.pr["merged_at"] = None
            elif case == "fork":
                self.pr["head"]["repo"]["full_name"] = "someone/fork"
            else:
                self.pr["merge_commit_sha"] = "d" * 40
            self.assertFalse(ready(SHA, self.api)[0])

    def test_merge_or_rebase_cannot_change_the_published_commit(self):
        self.parents.append({"sha": "e" * 40})
        with self.assertRaises(ValueError):
            ready(SHA, self.api)
        for comparison in ("ahead", "identical", "behind"):
            self.setUp()
            self.comparison = comparison
            with self.assertRaises(ValueError):
                ready(SHA, self.api)

    def test_api_errors_and_invalid_sha_stop_evaluation(self):
        with self.assertRaises(ValueError):
            ready("main", self.api)
        def failed_api(path):
            raise RuntimeError("GitHub unavailable")
        with self.assertRaises(RuntimeError):
            ready(SHA, failed_api)


if __name__ == "__main__":
    unittest.main()
