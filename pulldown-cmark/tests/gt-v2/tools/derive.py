#!/usr/bin/env python3
"""GT-v2 expected-output deriver. Provenance record, not a test harness.

Derivation method (per fixture, documented in README):
  1. Candidate node stream: parser-bench rust-pulldown lane binary `extract`
     (full mode, info payloads kept). Lane = pulldown-cmark 0.13.4 + Obsidian
     post-passes; the de-facto schema the frozen wire-contract v1 adopted.
  2. v2 schema extension: every `anchor` node gains info.id (the id without
     the caret) — asserts the fork's Event::BlockAnchor(CowStr) payload.
  3. v2 dialect deltas (DELTAS below): nodes the lane law excludes but the
     v2 dialect includes — nested callouts. Spans are NOT invented: byte
     ranges come from a pulldown offset_iter probe run against the fork at
     names-contract HEAD (eea0453), nested BlockQuote ranges, clipped per
     the block-span law (final line terminator excluded). Probe log in the
     derivation notes.
  4. text_prefix_16b recomputed from raw bytes; lane disagreement = abort.
  5. Ordering law enforced: span.start asc, span.end desc.

Re-running requires the lane binary; the OUTPUT (ground-truth/*.json) is the
authority once frozen — this script exists so a reviewer can reproduce it.
"""
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LANE_BIN = Path(
    "/Users/Shared/projects/caoer/home-wiki-codespace/parser-bench/"
    "lanes/rust-pulldown/target/release/rust-pulldown-bench"
)

NODE_KEYS = ("kind", "hpath", "span", "text_prefix_16b", "unterminated", "info")

# spec-h:anchors@feed-1 (2b24e94a, probe-confirmed vs Obsidian 1.12.7):
#   (a) anchors fire only at end of a paragraph's inline text — interior
#       line-tails of multi-line paragraphs do NOT register (lane over-fires)
#   (b) trailing whitespace after ^id invalidates (lane over-fires)
#   (c) table-cell tail anchors DO register (lane never fires there)
# ANCHOR_REMOVES drops lane over-fires by info.id; anchor entries in DELTAS
# using "find" add rule-(c) nodes (span located by unique byte search).
ANCHOR_REMOVES = {
    "adversarial/anchors-edge-positions.md": {"spaced"},          # rule (b)
    "adversarial/zzprobe-anchors-a.md": {"a02", "a07"},           # rules (b),(a)
}

# spec-h feed-2 (2b24e94a batch #2, probe-confirmed):
#   callouts — glued fold-title (`[!x]-Title`) is NOT a callout at all;
#   wikilinks — `\![[x]]` escape defeats embed binding (wikilink, not embed).
# Lane over-fires both; removed by exact span (audited above in this file's
# derivation notes; spans printed by the lane run being patched).
REMOVES_BY_SPAN = {
    "adversarial/callouts-fold-title.md": [[216, 252]],   # [!note]-Title-glued
    "adversarial/batch2-pipe-escape.md": [[287, 301]],    # \![[not-bound]] embed
}

# v2 dialect deltas: nested callouts (lane law: top-level only; v2: every
# quote level whose first line is a callout head is a node).
# Spans from pulldown offset_iter nested-BlockQuote ranges (probe against
# names-contract eea0453, 2026-07-18), final line terminator clipped.
# fold/type parsed from the inner head line, same law as top-level.
# Anchor entries ("find"): spec-h:anchors@feed-1 rule (c) — table-cell tails.
DELTAS = {
    "adversarial/anchors-edge-positions.md": [
        {"kind": "anchor", "find": "^in-table-row"},
    ],
    "adversarial/zzprobe-anchors-a.md": [
        {"kind": "anchor", "find": "^a15"},
    ],
    # spec-h feed-2: CRLF frontmatter DOES parse (line-ending normalization
    # precedes parse). Span per raw-byte law: final `\r\n` clipped.
    "adversarial/crlf-mixed.md": [
        {"kind": "frontmatter", "span": [0, 27], "info": {"keys": ["title"]}},
    ],
    # spec-h feed-2: callout type charset is anything-but-`]`; unicode types
    # ARE callouts. Spans = pulldown BQ ranges, terminator clipped.
    "adversarial/callouts-type-charset.md": [
        {"kind": "callout", "span": [288, 333], "info": {"type": "two words", "fold": ""}},
        {"kind": "callout", "span": [335, 374], "info": {"type": " note", "fold": ""}},
        {"kind": "callout", "span": [376, 400], "info": {"type": "注意", "fold": ""}},
    ],
    # spec-h feed-2: `[!type|metadata]` splits at the pipe — type "note";
    # metadata capture shape pending spec text (OQ-16). BQ range 54..109.
    # `\![[x]]`: the escaped bang stays text; the wikilink node replaces the
    # lane's embed (removed above).
    "adversarial/batch2-pipe-escape.md": [
        {"kind": "callout", "span": [54, 108], "info": {"type": "note", "fold": ""}},
        {"kind": "wikilink", "find": "[[not-bound]]", "info": {"target": "not-bound"}},
    ],
    # spec-h feed-2: alias-only `[[|...]]` IS tokenized (bare `[[]]` still
    # open, OQ-11 residue).
    "adversarial/fragments-ambiguity.md": [
        {"kind": "wikilink", "find": "[[|alias-only]]", "info": {"target": "", "alias": "alias-only"}},
    ],
    "adversarial/callouts-nested.md": [
        {"kind": "callout", "span": [50, 80], "info": {"type": "inner", "fold": ""}},
        {"kind": "callout", "span": [102, 183], "info": {"type": "l2", "fold": "-"}},
        {"kind": "callout", "span": [144, 183], "info": {"type": "l3", "fold": "+"}},
        {"kind": "callout", "span": [212, 270], "info": {"type": "inside-plain", "fold": ""}},
        {"kind": "callout", "span": [382, 419], "info": {"type": "child-a", "fold": ""}},
        {"kind": "callout", "span": [424, 462], "info": {"type": "child-b", "fold": ""}},
        {"kind": "callout", "span": [477, 524], "info": {"type": "after-gap", "fold": ""}},
    ],
    "adversarial/kitchen-sink.md": [
        {"kind": "callout", "span": [371, 489], "info": {"type": "inner", "fold": "+"}},
    ],
}


def prefix16(raw: bytes, start: int) -> str:
    return raw[start : start + 16].decode("utf-8", "backslashreplace")


def derive(rel: str) -> dict:
    src = ROOT / rel
    raw = src.read_bytes()
    out = subprocess.run(
        [str(LANE_BIN), "extract", str(src)],
        capture_output=True, text=True, check=True,
    )
    lane_nodes = json.loads(out.stdout)[0]["nodes"]

    removes = ANCHOR_REMOVES.get(rel, set())
    span_removes = REMOVES_BY_SPAN.get(rel, [])
    nodes = []
    for n in lane_nodes:
        node = {k: n[k] for k in NODE_KEYS if k in n}
        s, e = node["span"]
        if node["span"] in span_removes:
            continue
        # v2 schema extension: anchor payload
        if node["kind"] == "anchor":
            node["info"] = {"id": raw[s + 1 : e].decode("utf-8")}
            if node["info"]["id"] in removes:
                continue
        # prefix law: recompute, never trust
        want = prefix16(raw, s)
        if node["text_prefix_16b"] != want:
            sys.exit(f"{rel}: lane prefix mismatch at {node['span']}: "
                     f"{node['text_prefix_16b']!r} != {want!r}")
        nodes.append(node)

    for d in DELTAS.get(rel, []):
        node = dict(d)
        if "find" in node:
            pat = node.pop("find").encode()
            if raw.count(pat) != 1:
                sys.exit(f"{rel}: find pattern {pat!r} not unique")
            i = raw.index(pat)
            node["span"] = [i, i + len(pat)]
            if node["kind"] == "anchor" and "info" not in node:
                node["info"] = {"id": pat[1:].decode()}
        node["text_prefix_16b"] = prefix16(raw, node["span"][0])
        nodes.append(node)

    nodes.sort(key=lambda n: (n["span"][0], -n["span"][1]))
    return {"file": rel, "schema": "gt-v2", "nodes": nodes}


def main() -> None:
    outdir = ROOT / "ground-truth"
    outdir.mkdir(exist_ok=True)
    fixtures = sorted(
        p.relative_to(ROOT).as_posix()
        for d in ("corpus", "adversarial")
        for p in (ROOT / d).glob("*.md")
    )
    for rel in fixtures:
        doc = derive(rel)
        stem = Path(rel).stem
        path = outdir / f"{stem}.expected.json"
        path.write_text(json.dumps(doc, indent=1, sort_keys=True) + "\n")
        print(f"{rel}: {len(doc['nodes'])} nodes -> {path.name}")


if __name__ == "__main__":
    main()
