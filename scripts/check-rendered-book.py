#!/usr/bin/env python3
"""Check the current Book's rendered records and local links (Python 3.11+).

Usage: python3 scripts/check-rendered-book.py target/anchorbook
Run mdbook build first. This is a development acceptance check, not an engine API.
"""
from collections import Counter
from html.parser import HTMLParser
from pathlib import Path
import re
import sys
import tomllib
from urllib.parse import unquote, urlsplit

PROJECT = Path(__file__).resolve().parent.parent
OUTPUT = Path(sys.argv[1] if len(sys.argv) > 1 else PROJECT / "target/anchorbook").resolve()


def normalize(text):
    return " ".join(text.split())


def definitions(path):
    fence = None
    body = []
    for line in path.read_text().splitlines():
        match = re.match(r"^(`{3,}|~{3,})(.*)$", line)
        if fence is None:
            if match:
                fence = (match[1], match[2])
                body = []
        elif match and match[1][0] == fence[0][0] and len(match[1]) >= len(fence[0]) and not match[2].strip():
            if fence[1] in ("anchorbook-anchor", "anchorbook-requirement", "anchorbook-decision"):
                yield tomllib.loads("\n".join(body))
            fence = None
        else:
            body.append(line)


class Page(HTMLParser):
    def __init__(self, path):
        super().__init__(convert_charrefs=True)
        self.ids = Counter()
        self.links = []
        self.record_links = []
        self.records = {}
        self.active = None
        self.feed(path.read_text())

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if "id" in attrs:
            self.ids[attrs["id"]] += 1
        if tag == "section" and "ab-record" in attrs.get("class", "").split():
            self.active = attrs["id"]
            assert self.active not in self.records, f"Duplicate record {self.active}"
            self.records[self.active] = ""
        if tag == "a" and "href" in attrs:
            self.links.append(attrs["href"])
            if "ab-record-link" in attrs.get("class", "").split():
                self.record_links.append(attrs["href"])

    def handle_endtag(self, tag):
        if tag == "section":
            self.active = None

    def handle_data(self, data):
        if self.active:
            self.records[self.active] += data + " "


pages = {p.resolve(): Page(p) for p in OUTPUT.rglob("*.html")}
assert pages, "No rendered HTML found"
expected = {}
for source in (PROJECT / "book").rglob("*.md"):
    records = list(definitions(source))
    if not records:
        continue
    page = pages[(OUTPUT / source.relative_to(PROJECT / "book")).with_suffix(".html").resolve()]
    assert set(page.records) == {r["id"] for r in records}, source
    for record in records:
        assert record["id"] not in expected
        expected[record["id"]] = record
        assert page.ids[record["id"]] == 1
        text = normalize(page.records[record["id"]])
        for key, value in record.items():
            if key in ("schema-version", "status"):
                continue
            values = value if isinstance(value, list) else [value]
            for field in values:
                if isinstance(field, dict):
                    values_to_check = [v for k, v in field.items() if k != "kind"]
                else:
                    values_to_check = [field]
                for value_to_check in values_to_check:
                    items = value_to_check if isinstance(value_to_check, list) else [value_to_check]
                    for item in items:
                        assert normalize(item) in text, (record["id"], key, item)
        for criterion in record.get("criteria", []):
            methods = ", ".join(criterion["verification"])
            assert f"Required verification: {methods}" in text, (record["id"], criterion["id"])
        assert record["status"].title() in text

print_page = pages[OUTPUT / "print.html"]
assert set(print_page.records) == set(expected)
for identity in expected:
    assert print_page.ids[identity] == 1, identity
for href in print_page.record_links:
    assert href.startswith("#"), f"Print link leaves combined view: {href}"
    assert href[1:] in expected, href

links = 0
for path, page in pages.items():
    for href in page.links:
        url = urlsplit(href)
        if url.scheme or url.netloc or href.startswith("/"):
            continue
        target = (path.parent / unquote(url.path)).resolve() if url.path else path
        assert target.exists(), (path.name, href)
        if target in pages and url.fragment:
            assert unquote(url.fragment) in pages[target].ids, (path.name, href)
        links += 1
print(f"Rendered acceptance passed: {len(expected)} complete records, unique print targets, {links} local links.")
