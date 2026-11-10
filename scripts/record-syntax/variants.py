#!/usr/bin/env python3
"""Record syntax variants for Record Line Grammar milestone 1.

Writes three disposable copies of this Book's sources under
target/record-syntax/<variant>/book, with every record and literal record example
in one syntax:

  toml   the current TOML form (records unchanged; examples converted to TOML)
  fence  the same TOML with kind, identifier, and status moved to the fence line
  lines  the goal line grammar, with kind-coded identifiers throughout

Each copy is read back and compared with the records of this Book, so a variant
that loses an identity, field, reference, criterion, or method fails. The record
positions in each copy are written to target/record-syntax/<variant>/index.json
for searches.py. The repository itself is never modified.

Usage: scripts/record-syntax/variants.py
Requires Python 3.11 (tomllib) or the tomli package.
"""
import json, pathlib, re, shutil, sys
sys.dont_write_bytecode = True
try:
    import tomllib
except ImportError:
    import tomli as tomllib

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parent.parent
OUT = REPO / "target" / "record-syntax"
VARIANTS = ["toml", "fence", "lines"]
KINDS = ["anchor", "requirement", "decision"]
PROOF_ITEMS = ["Domain", "Boundary", "Assumptions", "Exclusions"]
RECORD_FENCE = re.compile(r"^```anchorbook-(anchor|requirement|decision)(?: (\S+) (draft|accepted))?$")


def paragraphs(text):
    return [" ".join(p.split()) for p in re.split(r"\n\s*\n", text.strip()) if p.strip()]


# --- Model -----------------------------------------------------------------

def from_toml(kind, data):
    """Normalized record: every text field as a list of paragraphs."""
    r = {"kind": kind, "id": data["id"], "title": data["title"], "status": data["status"]}
    if kind == "anchor":
        r["property"] = paragraphs(data["property"])
        r["authorities"] = [(a["kind"], a["id"]) for a in data["authorities"]]
        r["criteria"] = [{"id": c["id"], "methods": list(c["verification"]),
                          "statement": paragraphs(c["statement"])} for c in data["criteria"]]
    elif kind == "requirement":
        r["statement"] = paragraphs(data["statement"])
        r["source"] = paragraphs(data["source"])
    else:
        for f in ["choice", "rationale", "reconsider-when"]:
            r[f] = paragraphs(data[f])
        r["requirements"] = list(data["requirements"])
        r["alternatives"] = [" ".join(a.split()) for a in data["alternatives"]]
    return r


def code(kind, ident):
    if kind == "anchor":
        return "A-" + ident
    prefix = kind + "-"
    assert ident.startswith(prefix), ident
    return ("R-" if kind == "requirement" else "D-") + ident[len(prefix):]


def uncode(ident):
    k, name = ident[0], ident[2:]
    return {"A": ("anchor", name), "R": ("requirement", "requirement-" + name),
            "D": ("decision", "decision-" + name)}[k]


# --- TOML emitters ---------------------------------------------------------

def toml_text(paras):
    if len(paras) == 1 and len(paras[0]) < 70 and '"' not in paras[0]:
        return f'"{paras[0]}"'
    return "'''\n" + "\n\n".join(paras) + "\n'''"


def toml_str(s):
    return json.dumps(s, ensure_ascii=False)


def toml_body(r, with_identity):
    out = ["schema-version = 1"]
    if with_identity:
        out.append(f"id = {toml_str(r['id'])}")
    out.append(f"title = {toml_str(r['title'])}")
    if with_identity:
        out.append(f"status = {toml_str(r['status'])}")
    if r["kind"] == "anchor":
        out.append(f"property = {toml_text(r['property'])}")
        out.append("authorities = [")
        out += [f'  {{ kind = "{k}", id = "{i}" }},' for k, i in r["authorities"]]
        out.append("]")
        for c in r["criteria"]:
            out += ["", "[[criteria]]", f"id = {toml_str(c['id'])}",
                    "verification = [" + ", ".join(toml_str(m) for m in c["methods"]) + "]",
                    f"statement = {toml_text(c['statement'])}"]
    elif r["kind"] == "requirement":
        out.append(f"statement = {toml_text(r['statement'])}")
        out.append(f"source = {toml_text(r['source'])}")
    else:
        out.append(f"choice = {toml_text(r['choice'])}")
        out.append("requirements = [" + ", ".join(toml_str(x) for x in r["requirements"]) + "]")
        out.append(f"rationale = {toml_text(r['rationale'])}")
        out.append("alternatives = [" + ", ".join(toml_str(a) for a in r["alternatives"]) + "]")
        out.append(f"reconsider-when = {toml_text(r['reconsider-when'])}")
    return out


def emit_toml(r):
    return [f"```anchorbook-{r['kind']}"] + toml_body(r, True) + ["```"]


def emit_fence(r):
    return [f"```anchorbook-{r['kind']} {r['id']} {r['status']}"] + toml_body(r, False) + ["```"]


# --- Line grammar ----------------------------------------------------------

def emit_lines(r):
    out = [f"**`{code(r['kind'], r['id'])}`** ({r['status']}): **{r['title']}**"]

    def field(label, paras):
        out.append(f"{label}: {paras[0]}")
        out.extend(paras[1:])

    if r["kind"] == "anchor":
        field("Property", r["property"])
        out.append("Authority: " + "; ".join(f"`{code(k, i)}`" for k, i in r["authorities"]))
        for c in r["criteria"]:
            out += criterion_lines(c)
    elif r["kind"] == "requirement":
        field("Statement", r["statement"])
        field("Source", r["source"])
    else:
        field("Choice", r["choice"])
        reqs = "; ".join(f"`{code('requirement', x)}`" for x in r["requirements"]) or "none"
        out.append(f"Requirements: {reqs}")
        field("Rationale", r["rationale"])
        out.append("Alternatives:")
        out += [f"- {a}" for a in r["alternatives"]]
        field("Reconsider when", r["reconsider-when"])
    return ["> " + line for line in out]


def criterion_lines(c):
    head = f"- Criterion `{c['id']}` [{', '.join(c['methods'])}]: "
    if "proof" not in c["methods"]:
        assert len(c["statement"]) == 1, c["id"]
        return [head + c["statement"][0]]
    claim, items = split_proof(c["statement"])
    return [head + claim] + [f"  - {k}: {v}" for k, v in items]


def split_proof(paras):
    labels = ["Claim"] + PROOF_ITEMS
    found = []
    for p in paras:
        label, _, rest = p.partition(": ")
        if label in labels:
            found.append([label, rest])
        else:
            assert found, p
            found[-1][1] += " " + p
    assert [k for k, _ in found] == labels, [k for k, _ in found]
    return found[0][1], [tuple(x) for x in found[1:]]


def join_proof(claim, items):
    return ["Claim: " + claim] + [f"{k}: {v}" for k, v in items]


def parse_lines(lines):
    """Parse one blockquote record (lines without the '> ' prefix)."""
    m = re.fullmatch(r"\*\*`([ARD]-[a-z][a-z0-9-]*)`\*\* \((draft|accepted)\): \*\*(.+)\*\*", lines[0])
    assert m, lines[0]
    kind, ident = uncode(m.group(1))
    r = {"kind": kind, "id": ident, "title": m.group(3), "status": m.group(2)}
    labels = {"anchor": ["Property", "Authority"], "requirement": ["Statement", "Source"],
              "decision": ["Choice", "Requirements", "Rationale", "Alternatives", "Reconsider when"]}[kind]
    current = None
    for line in lines[1:]:
        label, sep, rest = line.partition(": ")
        if line == "Alternatives:":
            current = "alternatives"; r[current] = []
        elif sep and label in labels:
            current = label.lower().replace(" ", "-")
            if current == "authority":
                r["authorities"] = [uncode(x) for x in re.findall(r"`([ARD]-[a-z0-9-]+)`", rest)]
            elif current == "requirements":
                r["requirements"] = [] if rest == "none" else [uncode(x)[1] for x in re.findall(r"`(R-[a-z0-9-]+)`", rest)]
            else:
                r[current] = [rest]
        elif line.startswith("- Criterion "):
            cm = re.fullmatch(r"- Criterion `([a-z0-9-]+)` \[([a-z, ]+)\]: (.+)", line)
            assert cm, line
            r.setdefault("criteria", []).append({"id": cm.group(1), "methods": cm.group(2).split(", "),
                                                 "statement": [cm.group(3)], "items": []})
            current = "criterion"
        elif line.startswith("  - ") and current == "criterion":
            k, _, v = line[4:].partition(": ")
            r["criteria"][-1]["items"].append((k, v))
        elif line.startswith("- ") and current == "alternatives":
            r["alternatives"].append(line[2:])
        else:
            assert current and isinstance(r.get(current), list), line
            r[current].append(line)
    for c in r.get("criteria", []):
        items = c.pop("items")
        if items:
            c["statement"] = join_proof(c["statement"][0], items)
    return r


# --- Reading the Book --------------------------------------------------------

def read_book(book):
    """Records in fence order per chapter, and the line-grammar literal examples."""
    chapters = {}
    for path in sorted(book.rglob("*.md")):
        rel = str(path.relative_to(book))
        lines = path.read_text().split("\n")
        items, i = [], 0
        while i < len(lines):
            m = RECORD_FENCE.match(lines[i])
            if m and m.group(2) is None:
                end = lines.index("```", i + 1)
                data = tomllib.loads("\n".join(lines[i + 1:end]))
                items.append(("record", i, end, from_toml(m.group(1), data)))
                i = end + 1
                continue
            if lines[i].startswith("```markdown"):
                end = lines.index("```", i + 1)
                quote = [l[2:] if l.startswith("> ") else l[1:] for l in lines[i + 1:end] if l.startswith(">")]
                if quote and len(quote) == end - i - 1:
                    items.append(("example", i, end, quote))
                i = end + 1
                continue
            i += 1
        chapters[rel] = (lines, items)
    return chapters


def example_record(quote):
    """A literal example as a record, or ('criterion', c) for a criterion fragment."""
    if quote[0].startswith("**`"):
        return parse_lines(quote)
    fake = ["**`A-example`** (draft): **Example**", "Property: x", "Authority: `R-x`"] + quote
    return ("criterion", parse_lines(fake)["criteria"][0])


# --- Writing variants ----------------------------------------------------------

def rename_prose(line, ids):
    if " becoming `" in line:  # prose that states the rename itself
        return line
    def span(m):
        base, _, crit = m.group(1).partition("#")
        if base in ids:
            return "`" + ids[base] + ("#" + crit if crit else "") + "`"
        return m.group(0)
    line = re.sub(r"`([a-z][a-z0-9-]*(?:#[a-z0-9-]+)?)`", span, line)
    return re.sub(r"#([a-z][a-z0-9-]*)\)", lambda m: "#" + ids.get(m.group(1), m.group(1)) + ")", line)


def emit(variant, r):
    return {"toml": emit_toml, "fence": emit_fence, "lines": emit_lines}[variant](r)


def emit_example(variant, ex):
    if isinstance(ex, tuple):
        c = ex[1]
        if variant == "lines":
            return ["```markdown"] + ["> " + l for l in criterion_lines(c)] + ["```"]
        body = ["[[criteria]]", f"id = {toml_str(c['id'])}",
                "verification = [" + ", ".join(toml_str(m) for m in c["methods"]) + "]",
                f"statement = {toml_text(c['statement'])}"]
        return ["```toml"] + body + ["```"]
    if variant == "lines":
        return ["```markdown"] + emit_lines(ex) + ["```"]
    return ["````markdown"] + emit(variant, ex) + ["````"]


def write_variant(variant, chapters, ids):
    root = OUT / variant / "book"
    index = {"records": [], "examples": []}
    for rel, (lines, items) in chapters.items():
        out, pos = [], 0
        for kind, start, end, value in items:
            out += [rename_prose(l, ids) if variant == "lines" else l for l in lines[pos:start]]
            if kind == "record":
                block = lines[start:end + 1]
                if variant == "fence":
                    head = block.index("[[criteria]]") if "[[criteria]]" in block else len(block)
                    block = [f"```anchorbook-{value['kind']} {value['id']} {value['status']}"] + [
                        l for i, l in enumerate(block) if 0 < i and not (i < head and l.startswith(("id = ", "status = ")))]
                elif variant == "lines":
                    block = emit_lines(value)
            else:
                block = emit_example(variant, example_record(value))
            block = "\n".join(block).split("\n")  # emitted strings may span lines
            entry ={"file": rel, "start": len(out) + 1, "end": len(out) + len(block)}
            if kind == "record":
                entry.update(kind=value["kind"], id=value["id"], status=value["status"],
                             lines=locate(variant, value, block, entry["start"]))
                index["records"].append(entry)
            else:
                index["examples"].append(entry)
            out += block
            pos = end + 1
        out += [rename_prose(l, ids) if variant == "lines" else l for l in lines[pos:]]
        dest = root / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_text("\n".join(out))
    for f in ["book.toml", "anchorbook.toml"]:
        shutil.copy(REPO / f, OUT / variant / f)
    shutil.copytree(REPO / "theme", OUT / variant / "theme")
    (OUT / variant / "index.json").write_text(json.dumps(index, indent=1))
    return root


def locate(variant, r, block, first):
    """1-based lines where each identifying fact of a record appears."""
    at = lambda pred: [first + i for i, l in enumerate(block) if pred(l)]
    if variant == "lines":
        loc = {"identity": first, "status": first, "kind": first}
        crit = [(first + i, re.match(r"> - Criterion `([^`]+)`", l).group(1))
                for i, l in enumerate(block) if l.startswith("> - Criterion ")]
        loc["criteria"] = {c: {"id": n, "methods": n} for n, c in crit}
        loc["references"] = at(lambda l: l.startswith(("> Authority:", "> Requirements:")))
        return loc
    loc = {"kind": first}
    if variant == "fence":
        loc["identity"] = loc["status"] = first
    else:
        loc["identity"] = at(lambda l: l.startswith("id = "))[0]
        loc["status"] = at(lambda l: l.startswith("status = "))[0]
    loc["criteria"] = {}
    starts = [i for i, l in enumerate(block) if l == "[[criteria]]"] + [len(block)]
    for s, e in zip(starts, starts[1:]):
        group = list(enumerate(block[s:e], s))
        n_id = next(i for i, l in group if l.startswith("id = "))
        n_methods = next(i for i, l in group if l.startswith("verification = "))
        loc["criteria"][json.loads(block[n_id][5:])] = {"id": first + n_id, "methods": first + n_methods}
    loc["references"] = at(lambda l: l.startswith(("  { kind =", "requirements = [")))
    return loc


def read_variant(variant, root):
    """Records as read back from a variant copy, for the faithfulness check."""
    records = []
    for path in sorted(root.rglob("*.md")):
        lines = path.read_text().split("\n")
        i, fence = 0, None
        while i < len(lines):
            l = lines[i]
            if fence:
                if l == fence:
                    fence = None
                i += 1
                continue
            if l.startswith("```") and not RECORD_FENCE.match(l):
                fence = "````" if l.startswith("````") else "```"
                i += 1
                continue
            m = RECORD_FENCE.match(l)
            if m:
                end = lines.index("```", i + 1)
                data = tomllib.loads("\n".join(lines[i + 1:end]))
                if variant == "fence":
                    assert m.group(2), l
                    data.update(id=m.group(2), status=m.group(3))
                records.append(from_toml(m.group(1), data))
                i = end + 1
            elif variant == "lines" and l.startswith("> **`"):
                j = i
                while j < len(lines) and lines[j].startswith(">"):
                    j += 1
                records.append(parse_lines([x[2:] for x in lines[i:j]]))
                i = j
            else:
                i += 1
    return records


def main():
    chapters = read_book(REPO / "book")
    records = [v for _, items in chapters.values() for k, _, _, v in items if k == "record"]
    examples = [v for _, items in chapters.values() for k, _, _, v in items if k == "example"]
    ids = {r["id"]: code(r["kind"], r["id"]) for r in records}
    shutil.rmtree(OUT, ignore_errors=True)
    key = lambda r: (r["kind"], r["id"])
    expected = sorted(records, key=key)
    summary = {}
    for v in VARIANTS:
        root = write_variant(v, chapters, ids)
        got = sorted(read_variant(v, root), key=key)
        if got != expected:
            diff = [(a["id"], b["id"]) for a, b in zip(expected, got) if a != b]
            sys.exit(f"{v}: records differ from the Book: {diff or (len(expected), len(got))}")
        size = sum(p.stat().st_size for p in root.rglob("*.md"))
        summary[v] = {"records": len(got), "examples": len(examples), "book_bytes": size}
        print(f"{v}: {len(got)} records and {len(examples)} examples match the Book; {size} bytes of Book source")
    counts = {k: sum(r["kind"] == k for r in records) for k in KINDS}
    summary["kinds"] = counts
    summary["criteria"] = sum(len(r.get("criteria", [])) for r in records)
    (OUT / "summary.json").write_text(json.dumps(summary, indent=1))
    facts = [{"id": r["id"], "kind": r["kind"], "status": r["status"],
              "cites": [i for _, i in r.get("authorities", [])] + r.get("requirements", []),
              "proof": [c["id"] for c in r.get("criteria", []) if "proof" in c["methods"]]}
             for r in records]
    (OUT / "records.json").write_text(json.dumps(facts, indent=1))
    print("kinds:", counts, "criteria:", summary["criteria"])


if __name__ == "__main__":
    main()
