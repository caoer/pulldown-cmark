#!/usr/bin/env python3
"""GT-v2 mechanical verifier — no parser dependency, pure invariants.

Checks (splice law: slicing source bytes by span must reproduce the node):
  1. fixture <-> expected.json bijection (corpus/ + adversarial/)
  2. MANIFEST.tsv sha256 matches fixture bytes
  3. per node: span bounds sane; both endpoints on UTF-8 char boundaries;
     text_prefix_16b == raw[start:start+16] backslashreplace-decoded;
     anchor info.id == raw[start+1:end]
  4. node ordering: span.start asc, ties span.end desc
  5. kind in the v2 enum; node keys in the v2 schema

Exit 0 iff everything holds. Run from anywhere.
"""
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
KINDS = {"frontmatter", "heading", "fence", "inline-code", "anchor",
         "wikilink", "embed", "callout", "task", "table", "comment"}
NODE_KEYS = {"kind", "hpath", "span", "text_prefix_16b", "unterminated", "info"}

errors = 0


def err(msg: str) -> None:
    global errors
    errors += 1
    print(f"FAIL {msg}")


def utf8_boundary(raw: bytes, i: int) -> bool:
    return i == len(raw) or (raw[i] & 0xC0) != 0x80


def check_file(exp_path: Path) -> int:
    doc = json.loads(exp_path.read_text())
    src = ROOT / doc["file"]
    if not src.is_file():
        err(f"{exp_path.name}: fixture {doc['file']} missing")
        return 0
    raw = src.read_bytes()
    prev = None
    for i, n in enumerate(doc["nodes"]):
        where = f"{exp_path.name}[{i}]"
        if set(n) - NODE_KEYS:
            err(f"{where}: unknown keys {set(n) - NODE_KEYS}")
        if n["kind"] not in KINDS:
            err(f"{where}: unknown kind {n['kind']!r}")
        s, e = n["span"]
        if not (0 <= s <= e <= len(raw)):
            err(f"{where}: span {n['span']} out of bounds (len {len(raw)})")
            continue
        if not (utf8_boundary(raw, s) and utf8_boundary(raw, e)):
            err(f"{where}: span {n['span']} not on UTF-8 boundaries")
        want = raw[s:s + 16].decode("utf-8", "backslashreplace")
        if n["text_prefix_16b"] != want:
            err(f"{where}: prefix {n['text_prefix_16b']!r} != {want!r}")
        if n["kind"] == "anchor":
            aid = n.get("info", {}).get("id")
            if aid != raw[s + 1:e].decode("utf-8", "backslashreplace"):
                err(f"{where}: anchor id {aid!r} != span text")
        if prev is not None and (s, -e) < prev:
            err(f"{where}: ordering law violated at span {n['span']}")
        prev = (s, -e)
    return len(doc["nodes"])


def main() -> None:
    fixtures = {p.relative_to(ROOT).as_posix()
                for d in ("corpus", "adversarial") for p in (ROOT / d).glob("*.md")}
    expected = sorted((ROOT / "ground-truth").glob("*.expected.json"))
    covered = {json.loads(p.read_text())["file"] for p in expected}
    for f in sorted(fixtures - covered):
        err(f"fixture without expected output: {f}")
    for f in sorted(covered - fixtures):
        err(f"expected output without fixture: {f}")

    manifest = {}
    for line in (ROOT / "MANIFEST.tsv").read_text().splitlines()[1:]:
        cols = line.split("\t")
        manifest[cols[0]] = cols[1]
    for f in sorted(fixtures):
        digest = hashlib.sha256((ROOT / f).read_bytes()).hexdigest()
        if f not in manifest:
            err(f"MANIFEST missing {f}")
        elif manifest[f] != digest:
            err(f"MANIFEST sha mismatch for {f}")

    total = sum(check_file(p) for p in expected)
    print(f"{len(expected)} files, {total} nodes, {errors} violations")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
