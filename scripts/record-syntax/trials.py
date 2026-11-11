#!/usr/bin/env python3
"""Agent trials for Record Line Grammar milestone 1.

  trials.py run [AGENT-VARIANT ...]   start fresh non-interactive sessions
  trials.py score                     score the recorded sessions

Each session gets its own copy of one variant's book/ directory under
/tmp/rlg-trials/<agent>-<variant>, outside any repository, so it cannot see the
record index or this script. The prompt is the task set in tasks.md with
identifiers in the variant's form; no engine tools are available. Transcripts and
scores are written to target/record-syntax/trials/.

Requires variants.py to have run, Python 3.11 or tomli, and the `claude` and
`codex` commands on PATH.
"""
import json, pathlib, re, shutil, subprocess, sys, time
sys.dont_write_bytecode = True
from concurrent.futures import ThreadPoolExecutor

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import variants  # noqa: E402

OUT = variants.OUT
TRIALS = OUT / "trials"
WORK = pathlib.Path("/tmp/rlg-trials")
AGENTS = ["claude", "codex"]
TIMEOUT = 1200
IDS = {
    "toml": ("requirement-observation-only", "check-observation-only", "requirement-truthful-results"),
    "fence": ("requirement-observation-only", "check-observation-only", "requirement-truthful-results"),
    "lines": ("R-observation-only", "A-check-observation-only", "R-truthful-results"),
}

PROMPT = """This directory contains a project's Book: Markdown sources under book/. The
Book defines records: anchors, requirements, and decisions. Answer the questions
below by searching and reading these files. Do not run builds, tests, or project
tools. Then make the two edits in the Book sources.

1. Where is the requirement `{r}` defined? Give the file and line.
2. Which records cite `{r}`? Give their identifiers.
3. Which anchors have status draft? Give their identifiers.
4. Which criteria require proof? For each, give its anchor's identifier and the
   criterion's identifier.
5. Edit: make criterion `no-execution` of anchor `{a}` require both `test` and
   `review`.
6. Edit: make anchor `{a}` also cite the requirement `{r2}`.

Keep each edit in the Book's existing record format, and change nothing else.
End your reply with your answers as a JSON object in a ```json block:
{{"definition": "<file>:<line>", "citing": ["<id>"], "draft_anchors": ["<id>"],
"proof_criteria": [["<anchor id>", "<criterion id>"]]}}
"""


def command(agent, workdir):
    if agent == "claude":
        return ["claude", "-p", "--output-format", "stream-json", "--verbose",
                "--no-session-persistence", "--permission-mode", "acceptEdits",
                "--allowedTools", "Read", "Grep", "Glob", "Edit",
                "--disallowedTools", "Bash", "Write", "WebFetch", "WebSearch", "Task", "NotebookEdit"]
    return ["codex", "exec", "--json", "--ephemeral", "--skip-git-repo-check",
            "-s", "workspace-write", "-C", str(workdir), "-"]


def run_one(name):
    agent, variant = name.split("-")
    workdir = WORK / name
    shutil.rmtree(workdir, ignore_errors=True)
    workdir.mkdir(parents=True)
    shutil.copytree(OUT / variant / "book", workdir / "book")
    r, a, r2 = IDS[variant]
    prompt = PROMPT.format(r=r, a=a, r2=r2)
    TRIALS.mkdir(parents=True, exist_ok=True)
    start = time.time()
    with open(TRIALS / f"{name}.jsonl", "w") as log, open(TRIALS / f"{name}.err", "w") as err:
        try:
            p = subprocess.run(command(agent, workdir), input=prompt, cwd=workdir, stdout=log,
                               stderr=err, text=True, timeout=TIMEOUT)
            status = p.returncode
        except subprocess.TimeoutExpired:
            status = "timeout"
    meta = {"agent": agent, "variant": variant, "exit": status, "seconds": round(time.time() - start),
            "prompt": prompt}
    (TRIALS / f"{name}.meta.json").write_text(json.dumps(meta, indent=1))
    return name, status, meta["seconds"]


def events(name):
    for line in (TRIALS / f"{name}.jsonl").read_text().splitlines():
        try:
            yield json.loads(line)
        except json.JSONDecodeError:
            continue


def transcript_stats(name, agent):
    """Tool calls, lines returned by tools, final text, and model/usage facts."""
    calls, lines, final, facts = [], 0, "", {}
    for e in events(name):
        if agent == "claude":
            if e.get("type") == "system" and e.get("subtype") == "init":
                facts["model"] = e.get("model")
                facts["version"] = e.get("claude_code_version")
            for block in (e.get("message") or {}).get("content", []) if isinstance((e.get("message") or {}).get("content"), list) else []:
                if block.get("type") == "tool_use":
                    calls.append({"tool": block["name"], "input": block["input"]})
                elif block.get("type") == "tool_result":
                    c = block.get("content")
                    text = c if isinstance(c, str) else "\n".join(x.get("text", "") for x in c or [] if isinstance(x, dict))
                    lines += text.count("\n") + (1 if text else 0)
            if e.get("type") == "result":
                final = e.get("result", "")
                facts.update(turns=e.get("num_turns"), cost_usd=e.get("total_cost_usd"),
                             duration_ms=e.get("duration_ms"), usage=e.get("usage"))
        else:
            item = e.get("item") or {}
            if e.get("type") == "item.completed":
                if item.get("type") == "command_execution":
                    out = item.get("aggregated_output", "")
                    calls.append({"tool": "shell", "input": item.get("command")})
                    lines += out.count("\n") + (1 if out and not out.endswith("\n") else 0)
                elif item.get("type") in ("file_change", "mcp_tool_call", "web_search"):
                    calls.append({"tool": item["type"], "input": item.get("changes") or item.get("tool")})
                elif item.get("type") == "agent_message":
                    final = item.get("text", "")
            if e.get("type") == "turn.completed":
                facts["usage"] = e.get("usage")
            if e.get("type") == "thread.started":
                facts["thread"] = e.get("thread_id")
    return calls, lines, final, facts


def answer_json(text):
    blocks = re.findall(r"```json\s*(\{.*?\})\s*```", text, re.S)
    try:
        return json.loads(blocks[-1]) if blocks else None
    except json.JSONDecodeError:
        return None


def norm(ident, variant):
    ident = ident.strip().strip("`")
    if variant == "lines" and re.match(r"^[ARD]-", ident):
        return variants.uncode(ident)[1]
    return ident


def expected(variant):
    facts = json.loads((OUT / "records.json").read_text())
    index = json.loads((OUT / variant / "index.json").read_text())
    target = next(r for r in index["records"] if r["id"] == "requirement-observation-only")
    return {
        "definition": (target["file"], set(range(target["start"], target["lines"]["identity"] + 1))),
        "citing": {f["id"] for f in facts if "requirement-observation-only" in f["cites"]},
        "draft_anchors": {f["id"] for f in facts if f["kind"] == "anchor" and f["status"] == "draft"},
        "proof_criteria": {(f["id"], c) for f in facts for c in f["proof"]},
    }


def edited_records():
    """This Book's records with the two requested edits applied."""
    chapters = variants.read_book(variants.REPO / "book")
    records = [v for _, items in chapters.values() for k, _, _, v in items if k == "record"]
    for r in records:
        if r["id"] == "check-observation-only":
            r["authorities"] = r["authorities"] + [("requirement", "requirement-truthful-results")]
            for c in r["criteria"]:
                if c["id"] == "no-execution":
                    c["methods"] = ["test", "review"]
    return records


def edit_result(name, variant):
    """Whether the edited copy reads back as exactly the requested records."""
    try:
        got = variants.read_variant(variant, WORK / name / "book")
    except Exception as exc:  # an edit the reader cannot parse
        return {"valid": False, "reason": f"unreadable: {exc!r}"[:200]}
    key = lambda r: (r["kind"], r["id"])
    canon = lambda r: {**r, **({"authorities": sorted(r["authorities"]),
                                "criteria": [{**c, "methods": sorted(c["methods"])} for c in r["criteria"]]}
                               if r["kind"] == "anchor" else {})}
    want = {key(r): canon(r) for r in edited_records()}
    have = {key(r): canon(r) for r in got}
    wrong = sorted(k[1] for k in want.keys() | have.keys() if want.get(k) != have.get(k))
    target = have.get(("anchor", "check-observation-only"))
    done = {
        "methods": bool(target) and any(c["id"] == "no-execution" and c["methods"] == ["review", "test"]
                                        for c in target["criteria"]),
        "authority": bool(target) and ("requirement", "requirement-truthful-results") in target["authorities"],
    }
    return {"valid": not wrong, "requested_edits": done, "records_differing": wrong}


def score():
    rows = []
    for meta_path in sorted(TRIALS.glob("*.meta.json")):
        name = meta_path.name[: -len(".meta.json")]
        meta = json.loads(meta_path.read_text())
        agent, variant = meta["agent"], meta["variant"]
        calls, lines, final, facts = transcript_stats(name, agent)
        ans = answer_json(final) or {}
        exp = expected(variant)
        d = str(ans.get("definition", ""))
        dfile, _, dline = d.rpartition(":")
        definition = dfile.endswith(exp["definition"][0]) and dline.isdigit() and int(dline) in exp["definition"][1]
        sets = {
            "citing": {norm(x, variant) for x in ans.get("citing", [])},
            "draft_anchors": {norm(x, variant) for x in ans.get("draft_anchors", [])},
            "proof_criteria": {(norm(a, variant), c) for a, c in ans.get("proof_criteria", [])},
        }
        correct = {"definition": definition}
        for k, v in sets.items():
            correct[k] = {"correct": v == exp[k], "missing": len(exp[k] - v), "extra": len(v - exp[k])}
        rows.append({"trial": name, "agent": agent, "variant": variant, "exit": meta["exit"],
                     "seconds": meta["seconds"], "tool_calls": len(calls), "tool_output_lines": lines,
                     "answers_found": bool(ans), "correct": correct, "edits": edit_result(name, variant),
                     "facts": facts, "calls": calls})
    (TRIALS / "scores.json").write_text(json.dumps(rows, indent=1, default=str))
    head = "| Trial | Exit | Seconds | Tool calls | Tool output lines | Definition | Citing | Draft anchors | Proof criteria | Edits valid | Requested edits |"
    out = [head, "|" + " --- |" * 11]
    for r in rows:
        c = r["correct"]
        cell = lambda k: "yes" if c[k]["correct"] else f"no (−{c[k]['missing']} +{c[k]['extra']})"
        e = r["edits"]
        req = ", ".join(k for k, v in e.get("requested_edits", {}).items() if v) or "none"
        out.append(f"| {r['trial']} | {r['exit']} | {r['seconds']} | {r['tool_calls']} | {r['tool_output_lines']} "
                   f"| {'yes' if c['definition'] else 'no'} | {cell('citing')} | {cell('draft_anchors')} "
                   f"| {cell('proof_criteria')} | {'yes' if e['valid'] else 'no: ' + (e.get('reason') or ', '.join(e['records_differing']))} | {req} |")
    (TRIALS / "scores.md").write_text("\n".join(out) + "\n")
    print("\n".join(out))


def main():
    if sys.argv[1:2] == ["run"]:
        names = sys.argv[2:] or [f"{a}-{v}" for a in AGENTS for v in variants.VARIANTS]
        with ThreadPoolExecutor(len(names)) as pool:
            for name, status, seconds in pool.map(run_one, names):
                print(f"{name}: exit {status}, {seconds}s", flush=True)
    elif sys.argv[1:2] == ["score"]:
        score()
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
