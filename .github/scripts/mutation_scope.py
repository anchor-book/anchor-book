"""Select PR mutation scope from Git changes; requires Python 3.11+."""

from pathlib import Path
import subprocess
import sys
import tomllib


TEXT_METADATA = {
    "description", "documentation", "homepage", "repository", "license", "license-file",
}
LIST_METADATA = {"authors", "keywords", "categories"}
SCOPE_FILES = {
    ".github/workflows/mutation.yml",
    ".github/scripts/mutation_scope.py",
    ".github/scripts/test_mutation_scope.py",
}


def git(repo, *args):
    return subprocess.check_output(["git", *args], cwd=repo, stderr=subprocess.PIPE)


def typed_value(value):
    # Python considers True == 1 == 1.0; Cargo/build metadata can distinguish them.
    if isinstance(value, dict):
        return ("table", tuple(sorted((key, typed_value(item)) for key, item in value.items())))
    if isinstance(value, list):
        return ("array", tuple(typed_value(item) for item in value))
    return (type(value).__name__, repr(value))


def build_manifest(raw):
    manifest = tomllib.loads(raw.decode("utf-8"))
    package = manifest.get("package", {})
    if isinstance(package, dict):
        for key, value in list(package.items()):
            descriptive = (
                key in TEXT_METADATA and isinstance(value, str)
                or key in LIST_METADATA and isinstance(value, list)
                and all(isinstance(item, str) for item in value)
                or key == "readme" and isinstance(value, (str, bool))
            )
            if descriptive:
                del package[key]
    return typed_value(manifest)


def select_scope(repo, base, head):
    # NUL separation preserves filenames; no rename inference hides a deleted manifest.
    changed = git(repo, "diff", "--name-only", "--no-renames", "-z", base, head)
    paths = [path.decode("utf-8") for path in changed.split(b"\0") if path]
    for path in paths:
        name = Path(path).name
        if (
            path in SCOPE_FILES
            or path.startswith(".cargo/")
            or name in {"Cargo.lock", "build.rs", "rust-toolchain", "rust-toolchain.toml"}
        ):
            return "full"
        if name == "Cargo.toml":
            try:
                before = build_manifest(git(repo, "show", f"{base}:{path}"))
                after = build_manifest(git(repo, "show", f"{head}:{path}"))
            except (subprocess.CalledProcessError, UnicodeDecodeError, tomllib.TOMLDecodeError):
                return "full"
            if before != after:
                return "full"
    if any(path.startswith(("src/", "contract/", "tests/")) for path in paths):
        return "diff"
    return "none"


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit("usage: mutation_scope.py BASE HEAD")
    scope = select_scope(Path.cwd(), *sys.argv[1:])
    print(f"code={'false' if scope == 'none' else 'true'}")
    print(f"scope={scope}")
    print(f"::notice::Mutation scope: {scope}; {'campaign skipped' if scope == 'none' else 'campaign required'}.", file=sys.stderr)
