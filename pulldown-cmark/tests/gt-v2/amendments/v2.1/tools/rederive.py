#!/usr/bin/env python3
"""GT-v2.1 amendment deriver — provenance record for the two corrected files.

Reuses the frozen tools/derive.py verbatim (imported, never edited) with
exactly ONE rule change, per the spec §1.3 erratum (2026-07-18):

    callout title-line tail anchors DO fire. The pre-freeze rule
    "callout head-line tail id is title text, not an anchor" was a
    miscoded generalization of the oq14a cache observation, which was a
    one-id-per-section last-writer overwrite (a RESOLUTION rule, not a
    parse rule). Probes: zzprobe-v3-followup v04 (title-only),
    zzprobe-v3b-followup w01 (title + plain body), w02a/b (order),
    w03 (folded).

Rule change: drop the two head-line ids from derive.ANCHOR_REMOVES —
  adversarial/anchors-edge-positions.md : "on-callout-head"  (w02 pattern:
      body anchor steals the CACHE slot, but node emission is parse-level;
      pack precedent a08/a09 keeps both nodes of a resolution override)
  adversarial/callouts-fold-title.md    : "title-anchor"     (v04 pattern:
      title-only callout, fires unconditionally)

Everything else — lane run, prefix law, ordering law, normalization —
is the frozen deriver's own code. As a consistency proof, this script
also re-derives WITHOUT the rule change and asserts the result is
byte-identical to the frozen ground-truth files (lane-drift guard).

Output: amendments/v2.1/ground-truth/<stem>.expected.json
"""
import importlib.util
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
GT2 = HERE.parent.parent.parent          # tests/gt-v2
OUT = HERE.parent / "ground-truth"

spec = importlib.util.spec_from_file_location("derive", GT2 / "tools" / "derive.py")
derive = importlib.util.module_from_spec(spec)
spec.loader.exec_module(derive)

ERRATUM_UNREMOVES = {
    "adversarial/anchors-edge-positions.md": {"on-callout-head"},
    "adversarial/callouts-fold-title.md": {"title-anchor"},
}


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for rel, ids in ERRATUM_UNREMOVES.items():
        stem = Path(rel).stem

        # lane-drift guard: frozen rules must reproduce the frozen bytes
        frozen = (GT2 / "ground-truth" / f"{stem}.expected.json").read_text()
        check = json.dumps(derive.derive(rel), indent=1, sort_keys=True) + "\n"
        if check != frozen:
            sys.exit(f"{rel}: frozen-rule re-derivation drifted from the "
                     f"frozen ground-truth file — lane binary changed? abort")

        # the one rule change
        derive.ANCHOR_REMOVES[rel] = derive.ANCHOR_REMOVES[rel] - ids
        doc = json.dumps(derive.derive(rel), indent=1, sort_keys=True) + "\n"
        (OUT / f"{stem}.expected.json").write_text(doc)

        added = [n for n in json.loads(doc)["nodes"]
                 if n["kind"] == "anchor" and n["info"]["id"] in ids]
        print(f"{rel}: +{len(added)} anchor node(s) "
              f"{[(n['info']['id'], n['span']) for n in added]}")


if __name__ == "__main__":
    main()
