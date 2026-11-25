#!/usr/bin/env python3
"""Select a checked, squash-merged release commit; never publish anything here."""
import json
import os
import re
import subprocess
import sys

REPOSITORY = "anchor-book/anchor-book"
WORKFLOWS = ("book.yml", "proof.yml", "mutation.yml", "package.yml")


def ready(sha, api):
    if not re.fullmatch(r"[0-9a-f]{40}", sha):
        raise ValueError("Expected a full commit SHA")
    prefix = "repos/" + REPOSITORY
    prs = api(f"{prefix}/commits/{sha}/pulls?per_page=100")
    releases = [p for p in prs if p["merged_at"]
                and p["merge_commit_sha"] == sha
                and p["base"]["ref"] == "main"
                and p["base"]["repo"]["full_name"] == REPOSITORY
                and p["head"]["repo"]
                and p["head"]["repo"]["full_name"] == REPOSITORY
                and p["head"]["ref"].startswith("release-plz-")]
    if not releases:
        return False, "No merged release PR for this commit"
    if len(releases) != 1:
        raise ValueError("Expected exactly one release PR")
    commit = api(f"{prefix}/commits/{sha}")
    if len(commit["parents"]) != 1:
        raise ValueError("Release PRs must use Squash and merge; publication stopped")
    # release-plz can check out the PR head when it is an ancestor. A squash
    # must leave that head diverged so the checked merge commit is published.
    head = releases[0]["head"]["sha"]
    comparison = api(f"{prefix}/compare/{head}...{sha}")
    if comparison["status"] != "diverged":
        raise ValueError("Release PR head is not a distinct squash; publication stopped")
    for workflow in WORKFLOWS:
        runs = api(f"{prefix}/actions/workflows/{workflow}/runs"
                   f"?head_sha={sha}&branch=main&event=push&per_page=100")["workflow_runs"]
        runs = [r for r in runs if r["head_sha"] == sha
                and r["head_branch"] == "main" and r["event"] == "push"
                and r["head_repository"]["full_name"] == REPOSITORY]
        if not runs:
            return False, f"Missing main push checks: {workflow}"
        latest = max(runs, key=lambda r: (r["run_number"], r["run_attempt"]))
        if latest["status"] != "completed" or latest["conclusion"] != "success":
            return False, f"Checks have not passed: {workflow}"
    return True, f"All release checks passed for {sha}"


def github_api(path):
    return json.loads(subprocess.check_output(["gh", "api", path], text=True))


if __name__ == "__main__":
    is_ready, reason = ready(sys.argv[1], github_api)
    print(reason)
    with open(os.environ["GITHUB_OUTPUT"], "a") as output:
        output.write(f"ready={str(is_ready).lower()}\n")
