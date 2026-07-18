# GT-v2.2 — amendment 2: §1.2 list-item rulings (nested-task no-fire + law 3)

Additive overlay on the FROZEN GT-v2 pack (pin `3d92925`) and the v2.1
overlay (pin `1f16846`). No v2 file and no v2.1 file is edited; consumers
apply the overlay rule below. Authored by the gt-v2-2-amendment worker
(ffa18e3b), session `18-02-meridian-rs`.

## Rulings being encoded

Spec `results/obsidian-dialect-conformance.md` §1.2 position table, two
rows updated 2026-07-18 from the v3d/v3e follow-up probes (probe inputs:
session `probes/zzprobe-v3d-followup.md`, `probes/zzprobe-v3e-followup.md`):

1. **Nested-callout replica of the V03 no-fire is a TRUE NO-FIRE** —
   a multi-line list item with the anchor on its FIRST line (lazy
   continuation follows) does not register, including inside a `> >`
   callout body (probe y00: exact kitchen-sink construct, y-prefixed
   ids). The competing explanation — last-writer overwrite by the
   continuation-line anchor, which would NOT remove the node (a08/a09
   precedent) — is **excluded by the z00 discriminator**: the same
   construct minus the tail-line anchor still does not fire.

2. **Law 3: list items inside blockquotes/callouts never take
   `listItems[].id`** — item-level anchor binding exists only for
   TOP-LEVEL lists; in quote/callout contexts the id registers to the
   enclosing top-level section and `listItems[].id` stays null
   (probes y01, y02, z01).

This discharges the v2.1 ledger's FLAG ("Multi-line list-item check" —
`^nested-task` left un-amended pending exactly the callout-nested probe,
named there as the GT-v3 probe candidate). The probe has now been run.

## Amended — corrected by this overlay

| # | Surface | Defect | Correction | Evidence |
|---|---------|--------|------------|----------|
| 1 | `ground-truth/kitchen-sink.expected.json` | anchor `^nested-task` [447,459] wrongly present — first-line anchor of a multi-line task item (next fixture line is a lazy continuation) inside the nested `[!inner]` callout; §1.2 interior rule applies within the item | overlay file removes the node (full replacement, 34 → 33 nodes) | y00 (true no-fire), z00 (overwrite excluded); spec §1.2 multi-line-item row |

## Superseded prose (frozen files, not edited)

| # | Surface | Statement | Superseded reading | Evidence |
|---|---------|-----------|--------------------|----------|
| 2 | `amendments/v2.1/AMENDMENT.md` § "Multi-line list-item check" | "continuation-tail anchors bind at ITEM level" (stated from the top-level probes V01–V06) | item-level binding is TOP-LEVEL-LIST-ONLY; inside quote/callout contexts the enclosing top-level section takes the id, `listItems[].id` stays null (law 3). The V01–V06 observations stand for top-level lists | y01, y02, z01; spec §1.2 in-quote-item row |

## Law-3 targeted sweep (every v2.1-overlay-view fixture)

The v2.1 audit swept the OLD rule set; this sweep applies law 3 to all 19
expected files in the v2.1 view (base + v2.1 overlay).

Method: (a) info-key inventory across all 19 files — the gt-v2 schema
carries NO binding-capable field (anchor info = `id` only; task info =
`checked`/`depth` only, where `depth` is list nesting, not binding), so a
violation can only be a wrongly-present/absent node or a derivation note
implying item binding; (b) mechanical enumeration of every anchor node
whose line sits inside a quote (9 total), classifying the ones on
list-item lines (2); (c) grep of every prose/derivation surface
(`MANIFEST.tsv` notes, `tools/derive.py`, `tools/verify.py`, `README.md`,
`OPEN-QUESTIONS.md`, v2.1 ledger/manifest/rederive) for
`listItems`/item-level/item-tail binding claims.

Findings:

- **Violation (amended, row 1):** `kitchen-sink` `^nested-task` — the
  only in-quote item anchor wrongly present (emission-level, ruling 1).
- **Audited, left standing:** `callouts-fold-title` `^task-in-callout`
  [545,561] — the only other in-quote item-line anchor. Single-line item
  (the next fixture line `> - [x] done task inside callout` opens a NEW
  item, not a continuation), so the §1.2 item-tail position fires and the
  id REGISTERS (probes y01, z01 — solo nested tasks register). Under law
  3 it binds to the enclosing top-level section rather than the item, but
  this pack asserts parse-level node emission, not cache binding, and the
  expected file carries no item-binding field. Node presence correct.
- **Audited, left standing:** the 7 remaining in-quote anchors
  (`anchors-edge-positions` ^in-quote/^on-callout-head(v2.1)/
  ^in-callout-body, `callouts-fold-title` ^title-anchor(v2.1),
  `kitchen-sink` ^in-outer/^in-inner, `zzprobe-anchors-a` ^a14) are on
  quote/callout line-tails, not list-item lines — outside law-3 scope,
  governed by §1.2/§1.3 rules already audited at v2/v2.1.
  `kitchen-sink` `^in-inner` [480,489] specifically remains correct: it
  is the continuation/paragraph tail (fires either way — v2.1 audit), and
  y00 confirms the tail-line anchor registers in the replica.
- **Prose surfaces:** zero item-binding claims outside the v2.1 ledger
  sentence superseded in row 2. Corpus fixtures contain no list items
  inside quotes/callouts with anchors (mechanical sweep, 0 hits outside
  the two above).

## Overlay rule for consumers

For each fixture, expected output = the LAST present of:
`ground-truth/<stem>.expected.json` (base) ←
`amendments/v2.1/ground-truth/<stem>.expected.json` ←
`amendments/v2.2/ground-truth/<stem>.expected.json`.
Amended files are FULL replacements. Totals under the v2.2 view:
19 files, 663 expected nodes (662 base + 2 v2.1 − 1 v2.2).

## Reproduction and verification

- `python3 amendments/v2.2/tools/rederive.py` — regenerates the overlay
  file with the frozen deriver + the one corrected rule; aborts if the
  frozen rules no longer reproduce the frozen bytes (lane drift). The
  v2.1 rule change (ERRATUM_UNREMOVES) does not touch kitchen-sink, so
  the frozen deriver's rule state is the v2.1-view state for this
  fixture.
- `python3 amendments/v2.2/tools/verify-overlay.py` — materializes the
  three-layer consumer view and runs the FROZEN `tools/verify.py` on it
  unmodified. Green at amendment time: 19 files, 663 nodes, 0 violations.
- `python3 amendments/v2.1/tools/verify-overlay.py` — v2.1 view, still
  green (19 files, 664 nodes, 0 violations).
- `python3 tools/verify.py` — base pack invariants, still green.

v2.2 immutability: files under `amendments/v2.2/` are frozen on commit;
further corrections go to `amendments/v2.3/` or GT-v3, never in place.
