#!/usr/bin/env python3
"""GT-v2.2 amendment deriver — provenance record for the one corrected file.

Reuses the frozen tools/derive.py verbatim (imported, never edited) with
exactly ONE rule change, per the spec §1.2 ruling (2026-07-18):

    kitchen-sink `^nested-task` [447,459] is a TRUE NO-FIRE. The anchor
    sits on the FIRST line of a task item whose next line (`> > inner
    tail line ^in-inner`) is a lazy continuation — the §1.2 interior
    rule (v3-followup V03) applies within the item, and the v3d/v3e
    nested-callout replica confirms the pattern holds inside a `> >`
    callout body. The overwrite explanation (in-inner stealing a cache
    slot, which would NOT remove the node — a08/a09 precedent) is
    EXCLUDED by the z00 discriminator probe: same construct minus the
    tail-line anchor still does not fire. Probes: zzprobe-v3d-followup
    y00, zzprobe-v3e-followup z00. This is the exact GT-v3 probe
    candidate the v2.1 ledger flagged ("Multi-line list-item check",
    FLAG-not-amended); the probe has now been run.

Rule change: add the id to derive.ANCHOR_REMOVES —
  adversarial/kitchen-sink.md : "nested-task"  (y00/z00 pattern:
      first-line anchor of a multi-line item, lazy continuation inside
      a nested callout — parse-level no-fire, not a resolution override)

The v2.1 overlay does not touch kitchen-sink (its ERRATUM_UNREMOVES
covers other files only), so the frozen deriver's rule state IS the
v2.1-view rule state for this fixture; the single change above yields
the v2.2 view. Everything else — lane run, prefix law, ordering law,
normalization — is the frozen deriver's own code. As a consistency
proof, this script also re-derives WITHOUT the rule change and asserts
the result is byte-identical to the frozen ground-truth file
(lane-drift guard).

Output: amendments/v2.2/ground-truth/kitchen-sink.expected.json
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

RULING_REMOVES = {
    "adversarial/kitchen-sink.md": {"nested-task"},
}


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for rel, ids in RULING_REMOVES.items():
        stem = Path(rel).stem

        # lane-drift guard: frozen rules must reproduce the frozen bytes
        frozen = (GT2 / "ground-truth" / f"{stem}.expected.json").read_text()
        check = json.dumps(derive.derive(rel), indent=1, sort_keys=True) + "\n"
        if check != frozen:
            sys.exit(f"{rel}: frozen-rule re-derivation drifted from the "
                     f"frozen ground-truth file — lane binary changed? abort")

        # the one rule change
        derive.ANCHOR_REMOVES[rel] = derive.ANCHOR_REMOVES.get(rel, set()) | ids
        doc = json.dumps(derive.derive(rel), indent=1, sort_keys=True) + "\n"
        (OUT / f"{stem}.expected.json").write_text(doc)

        removed = [n for n in json.loads(frozen)["nodes"]
                   if n["kind"] == "anchor" and n["info"]["id"] in ids]
        print(f"{rel}: -{len(removed)} anchor node(s) "
              f"{[(n['info']['id'], n['span']) for n in removed]}")


if __name__ == "__main__":
    main()
