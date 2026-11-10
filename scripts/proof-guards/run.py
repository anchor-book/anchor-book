#!/usr/bin/env python3
"""Hand-run proof guard experiment for formal-authority (Verification Evidence milestone 7).

Applies each change in code_changes.py or spec_changes.py, one at a time, to a
disposable copy of this repository, verifies the complete crate with the pinned
Verus command, and for surviving code changes runs the record-API tests. The
repository itself is never modified.

Usage: scripts/proof-guards/run.py code|spec [--workers N]
Requires `cargo verus` from the pinned release on PATH; see CONTRIBUTING.
"""
import argparse, json, os, shutil, subprocess, sys, tempfile, time
sys.dont_write_bytecode = True
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
TARGET = "src/records/authority.rs"
VERIFY = ["cargo", "verus", "verify", "--fwd-verus-args-to", "roots", "--locked",
          "--target-dir", "target/proof-guards", "--lib", "--", "--no-cheating", "--output-json"]
TESTS = ["records", "authority_criteria", "rendering"]
KINDS = ["postcondition not satisfied", "invariant not satisfied", "precondition not satisfied",
         "possible arithmetic underflow/overflow", "decreases not satisfied", "assertion failed"]

def load(mode):
    sys.path.insert(0, HERE)
    return __import__("code_changes" if mode == "code" else "spec_changes").CHANGES

def copy_repo(dest):
    shutil.copytree(REPO, dest, ignore=shutil.ignore_patterns("target", ".git"))

def verify(work, env):
    start = time.time()
    p = subprocess.run(VERIFY, cwd=work, env=env, capture_output=True, text=True)
    kinds = sorted({k for k in KINDS if k in p.stderr})
    outcome = "survivor" if p.returncode == 0 else ("rejected" if kinds else "unviable")
    first = next((l for l in p.stderr.splitlines() if l.startswith("error")), "")
    return dict(outcome=outcome, failed=kinds, first_error=first, seconds=round(time.time() - start, 1))

def failing_tests(work, env):
    failed = []
    for test in TESTS:
        p = subprocess.run(["cargo", "test", "--locked", "--test", test], cwd=work, env=env,
                           capture_output=True, text=True)
        names = [l.split()[1] for l in p.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        failed += names or ([f"{test}: exit {p.returncode}"] if p.returncode else [])
    return failed

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["code", "spec"])
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    changes = load(args.mode)
    original = open(os.path.join(REPO, TARGET)).read()
    for cid, _, old, _ in changes:
        if original.count(old) != 1:
            sys.exit(f"{cid}: original text does not occur exactly once")
    root = tempfile.mkdtemp(prefix="proof-guards-")
    env = dict(os.environ, CARGO_TARGET_DIR="")
    env.pop("CARGO_TARGET_DIR")
    def worker(index):
        work = os.path.join(root, f"work-{index}")
        copy_repo(work)
        path = os.path.join(work, TARGET)
        results = []
        for cid, item, old, new in changes[index::args.workers]:
            open(path, "w").write(original.replace(old, new))
            result = dict(id=cid, item=item, **verify(work, env))
            if args.mode == "code" and result["outcome"] == "survivor":
                result["failing_tests"] = failing_tests(work, env)
            open(path, "w").write(original)
            results.append(result)
        return results
    with ThreadPoolExecutor(args.workers) as pool:
        results = [r for group in pool.map(worker, range(args.workers)) for r in group]
    order = [c[0] for c in changes]
    results.sort(key=lambda r: order.index(r["id"]))
    shutil.rmtree(root)
    json.dump(results, sys.stdout, indent=1)
    print()

if __name__ == "__main__":
    main()
