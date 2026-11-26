"""Choose required campaigns for PRs, main pushes, and manual runs."""

import os
from pathlib import Path
import re
import sys

from mutation_scope import changed_paths, git, select_scope


PROSE_FILES = {"README.md", "CHANGELOG.md", "CONTRIBUTING.md", "AGENTS.md", "LICENSE"}


def proof_required(repo, base, head):
    if select_scope(repo, base, head) != "none":
        return True
    for path in changed_paths(repo, base, head):
        if Path(path).name == "Cargo.toml" or path in PROSE_FILES:
            continue  # Manifest semantics were checked above.
        if path.startswith("book/") and path.endswith(".md") and path != "book/SUMMARY.md":
            # Deliberately overapproximate record-bearing chapters instead of
            # duplicating the engine's Markdown/record parser. Read both sides:
            # removing a record or hiding it in a fence must still require proof.
            for revision in (base, head):
                entry = git(repo, "ls-tree", revision, "--", path)
                if entry:
                    if not entry.startswith((b"100644 blob ", b"100755 blob ")):
                        return True  # A symlink is not the chapter's contents.
                    raw = git(repo, "show", f"{revision}:{path}").decode("utf-8")
                    if any(line.lstrip().startswith(">") for line in raw.splitlines()):
                        return True
            continue
        # Config, navigation, verification machinery, and unclassified inputs
        # remain significant. Only known prose and descriptive metadata skip.
        return True
    return False


def campaign_scope(repo, kind, event, before, head="HEAD"):
    if kind not in {"mutation", "proof"}:
        raise ValueError(f"unknown campaign: {kind}")
    if event == "pull_request":
        base = f"{head}^1"  # GitHub's tested merge commit, compared to target.
    elif event == "push":
        if not before or before == "0" * 40:
            return "full"  # Initial push has no comparable baseline.
        if not re.fullmatch(r"[0-9a-fA-F]{40}", before):
            raise ValueError("push baseline must be a Git commit SHA")
        base = before  # Includes every commit in a multi-commit push.
    else:
        return "full"  # Manual dispatch or an unknown caller.
    if kind == "proof":
        return "full" if proof_required(repo, base, head) else "none"
    selected = select_scope(repo, base, head)
    return "full" if event == "push" and selected != "none" else selected


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: ci_scope.py mutation|proof")
    scope = campaign_scope(Path.cwd(), sys.argv[1], os.environ.get("GITHUB_EVENT_NAME", ""),
                           os.environ.get("PUSH_BEFORE", ""))
    print(f"code={'false' if scope == 'none' else 'true'}")
    print(f"scope={scope}")
    print(f"::notice::{sys.argv[1].capitalize()} scope: {scope}; "
          f"{'campaign skipped (no new verification evidence)' if scope == 'none' else 'campaign required'}.",
          file=sys.stderr)
