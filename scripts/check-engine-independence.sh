#!/usr/bin/env bash
# Prove the engine builds and tests without a valid Book, and that Book
# acceptance rejects seeded defects. The render gate is tested in build_gate.rs.
set -euo pipefail
project_dir=$(cd "$(dirname "$0")/.." && pwd)
experiment_dir=$(mktemp -d)
trap 'rm -rf "$experiment_dir"' EXIT
fixture_dir="$experiment_dir/project"
mkdir -p "$fixture_dir"
cp "$project_dir/Cargo.toml" "$project_dir/Cargo.lock" "$fixture_dir/"
cp -R "$project_dir/src" "$project_dir/tests" "$fixture_dir/"
# Keep compilation artifacts outside the disposable source and its Book.
export CARGO_TARGET_DIR="$project_dir/target/independence"
export CARGO_NET_OFFLINE=true
unset ANCHORBOOK_TEST_BOOK ANCHORBOOK_TEST_OUTPUT

independent_tests() {
    if ! cargo test --manifest-path "$fixture_dir/Cargo.toml" --locked --offline \
        > "$experiment_dir/engine.log" 2>&1; then
        cat "$experiment_dir/engine.log" >&2
        exit 1
    fi
    echo "$1: independent engine tests passed."
}

restore_book() {
    rm -rf "$fixture_dir/book"
    cp -R "$project_dir/book" "$fixture_dir/book"
    cp "$project_dir/anchorbook.toml" "$project_dir/book.toml" "$fixture_dir/"
}

acceptance_failure() {
    local label=$1 test_name=$2 diagnostic=$3 status=0
    cargo test --manifest-path "$fixture_dir/Cargo.toml" --locked --offline \
        --test current_book "$test_name" -- --ignored --exact \
        > "$experiment_dir/acceptance.log" 2>&1 || status=$?
    if [ "$status" -ne 101 ] ||
        ! grep -Fq "$diagnostic" "$experiment_dir/acceptance.log"; then
        cat "$experiment_dir/acceptance.log" >&2
        echo "$label: acceptance did not reject the seeded defect: $diagnostic." >&2
        exit 1
    fi
    echo "$label: acceptance rejected the seeded defect ($diagnostic)."
}

independent_tests "Absent Book"
test ! -e "$fixture_dir/book"

restore_book
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
independent_tests "Invalid record"
acceptance_failure "Invalid record" current_book_records "record.invalid-definition"

restore_book
printf '\n- [Missing chapter](deliberately-missing.md)\n' >> "$fixture_dir/book/SUMMARY.md"
test ! -e "$fixture_dir/book/deliberately-missing.md"
independent_tests "Broken navigation"
acceptance_failure "Broken navigation" current_book_structure "structure.chapter-missing"

restore_book
printf '\nUnsupported prose outside navigation.\n' >> "$fixture_dir/book/SUMMARY.md"
independent_tests "Incomplete navigation"
acceptance_failure "Incomplete navigation" current_book_structure "input.unsupported"

echo "Engine independence and Book acceptance experiments passed."
