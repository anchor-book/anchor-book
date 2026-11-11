#!/usr/bin/env python3
"""Minimal-search measurements for Record Line Grammar milestone 1.

For each search task in tasks.md and each variant written by variants.py, runs
the one search an agent would use (the equivalent `grep -rnE` command is
recorded), classifies every matched line from the variant's record index, and
counts the further lines an agent must read above a match to answer: the
lines back to the record's kind, identifier, or criterion when the matched line
does not state them.

Usage: scripts/record-syntax/variants.py && scripts/record-syntax/searches.py
Writes target/record-syntax/searches.json and searches.md.
"""
import json, pathlib, re, sys
sys.dont_write_bytecode = True

OUT = pathlib.Path(__file__).resolve().parent.parent.parent / "target" / "record-syntax"
TARGET = "requirement-observation-only"

# task -> variant -> (ERE pattern, facts the answer needs from the record)
SEARCHES = {
    "definition": {
        "toml": ('^id = "requirement-observation-only"', ["identity"]),
        "fence": ('^```anchorbook-requirement requirement-observation-only ', ["identity"]),
        "lines": ('^> \\*\\*`R-observation-only`', ["identity"]),
    },
    "references": {
        "toml": ('"requirement-observation-only"', ["identity"]),
        "fence": ('"requirement-observation-only"', ["identity"]),
        "lines": ('`R-observation-only`', ["identity"]),
    },
    "draft-anchors": {
        "toml": ('status = "draft"', ["kind", "identity"]),
        "fence": ('^```anchorbook-anchor [^ ]+ draft$', ["kind", "identity"]),
        "lines": ('^> \\*\\*`A-[^`]*`\\*\\* \\(draft\\)', ["kind", "identity"]),
    },
    "proof-criteria": {
        "toml": ('^verification = \\[.*"proof"', ["identity", "criterion"]),
        "fence": ('^verification = \\[.*"proof"', ["identity", "criterion"]),
        "lines": ('Criterion `[^`]*` \\[[^]]*proof', ["identity", "criterion"]),
    },
}


def answers(records):
    by_id = {r["id"]: r for r in records}
    return {
        "definition": {(TARGET,)},
        "references": {(r["id"],) for r in records if r["kind"] != "requirement"
                       and TARGET in r.get("cites", [])},
        "draft-anchors": {(r["id"],) for r in records if r["kind"] == "anchor" and r["status"] == "draft"},
        "proof-criteria": {(r["id"], c) for r in records for c in r.get("proof", [])},
    }


def measure(variant, task, pattern, needs, index, cites, proof):
    root = OUT / variant / "book"
    regex = re.compile(pattern)  # these patterns mean the same in ERE and Python
    spans = {}
    for r in index["records"]:
        spans.setdefault(r["file"], []).append(("record", r))
    for e in index["examples"]:
        spans.setdefault(e["file"], []).append(("example", e))
    found, rows = set(), {"answer": 0, "other record lines": 0, "examples": 0, "prose": 0}
    further = 0
    for path in sorted(root.rglob("*.md")):
        rel = str(path.relative_to(root))
        for n, line in enumerate(path.read_text().split("\n"), 1):
            if not regex.search(line):
                continue
            where = next(((k, s) for k, s in spans.get(rel, []) if s["start"] <= n <= s["end"]), None)
            if where is None:
                rows["prose"] += 1
                continue
            kind, span = where
            if kind == "example":
                rows["examples"] += 1
                continue
            loc = span["lines"]
            crit = next((c for c, v in loc["criteria"].items() if v["methods"] == n or v["id"] == n), None)
            needed = [loc[f] for f in needs if f in ("kind", "identity")]
            if "criterion" in needs and crit:
                needed.append(loc["criteria"][crit]["id"])
            further += max(0, n - min(needed))
            key = {"definition": (span["id"],) if n == loc["identity"] else None,
                   "references": (span["id"],) if n in loc["references"] and span["id"] in cites else None,
                   "draft-anchors": (span["id"],) if span["kind"] == "anchor" and span["status"] == "draft" else None,
                   "proof-criteria": (span["id"], crit) if crit and crit in proof.get(span["id"], []) else None,
                   }[task]
            if key and key not in found:
                found.add(key)
                rows["answer"] += 1
            else:
                rows["other record lines"] += 1
    return found, rows, further


def main():
    records = json.loads((OUT / "records.json").read_text())
    expected = answers(records)
    cites = {r["id"] for r in records if TARGET in r.get("cites", [])}
    proof = {r["id"]: r["proof"] for r in records if r.get("proof")}
    results = []
    for task, variants in SEARCHES.items():
        for variant, (pattern, needs) in variants.items():
            index = json.loads((OUT / variant / "index.json").read_text())
            found, rows, further = measure(variant, task, pattern, needs, index, cites, proof)
            matched = sum(rows.values())
            results.append({"task": task, "variant": variant, "command": f"grep -rnE '{pattern}' book",
                            "matched": matched, **rows, "wrong": matched - rows["answer"],
                            "further_lines": further, "complete": found == expected[task],
                            "answers": len(expected[task])})
    (OUT / "searches.json").write_text(json.dumps(results, indent=1))
    head = "| Task | Variant | Search | Matched | Answer | Other record lines | Examples | Prose | Further lines read | Complete |"
    lines = [head, "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |"]
    for r in results:
        cmd = r["command"].replace("|", "\\|")
        lines.append(f"| {r['task']} | {r['variant']} | `{cmd}` | {r['matched']} | {r['answer']} of {r['answers']} "
                     f"| {r['other record lines']} | {r['examples']} | {r['prose']} | {r['further_lines']} | {r['complete']} |")
    (OUT / "searches.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
