#!/usr/bin/env bash
# Prove the engine can build and test without relying on a valid/rendered Book.
set -euo pipefail
project_dir=$(cd "$(dirname "$0")/.." && pwd)
fixture_dir=$(mktemp -d)
trap 'rm -rf "$fixture_dir"' EXIT
cp "$project_dir/Cargo.toml" "$project_dir/Cargo.lock" "$fixture_dir/"
cp -R "$project_dir/src" "$project_dir/tests" "$fixture_dir/"
# Keep compilation artifacts outside the disposable source and its Book.
export CARGO_TARGET_DIR="$project_dir/target/independence"
cargo test --manifest-path "$fixture_dir/Cargo.toml" --locked --offline
cp -R "$project_dir/book" "$fixture_dir/book"
cat >> "$fixture_dir/book/introduction.md" <<'RECORD'

```anchorbook-requirement
schema-version = 999
id = "deliberately-invalid"
title = "Deliberate fixture"
status = "draft"
statement = "This definition must be rejected."
source = "Independent negative acceptance experiment."
```
RECORD
# Independent tests must still pass with this invalid Book present.
cargo test --manifest-path "$fixture_dir/Cargo.toml" --locked --offline
if cargo test --manifest-path "$fixture_dir/Cargo.toml" --locked --offline \
    --test current_book -- --ignored > "$fixture_dir/acceptance.log" 2>&1; then
    echo "Invalid Book unexpectedly passed acceptance" >&2
    exit 1
fi
if ! grep -q 'record.invalid-definition' "$fixture_dir/acceptance.log"; then
    cat "$fixture_dir/acceptance.log" >&2
    echo "Acceptance failed for an unexpected reason" >&2
    exit 1
fi
echo "Engine independence passed: absent Book, invalid Book, and expected acceptance failure."
