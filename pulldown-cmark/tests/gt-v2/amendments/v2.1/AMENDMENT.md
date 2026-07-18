# GT-v2.1 — amendment 1: spec §1.3 erratum (callout title-line tail anchors)

Additive overlay on the FROZEN GT-v2 pack (pin `3d92925`). No v2 file is
edited; consumers apply the overlay rule below. Authored by the
gt-v2-erratum-audit worker (7e02d38e), session `18-02-meridian-rs`.

## Erratum being encoded

Spec `results/obsidian-dialect-conformance.md` §1.3, corrected in place
2026-07-18 (durable record: session `inbox/erratum-oq14-for-gt-v3.md`):

> Callout title-line tail anchors **DO fire** and attach to the whole
> callout section; fold marker doesn't interfere. The earlier oq14a
> non-registration was the **one-id-per-section last-writer overwrite**
> (body anchor stole the slot) — a RESOLUTION rule, not a parse rule.

Probes (new files; the three SHA-frozen v2 probe fixtures untouched):
`probes/zzprobe-v3-followup.md` v04 (title-only, exact review construct),
`probes/zzprobe-v3b-followup.md` w01 (title anchor + plain body),
w02a/b (title + body anchor, order — body steals the cache slot),
w03 (folded title).

The pre-freeze rule "callout head-line tail id is title text, not an
anchor" was a miscoded generalization of that cache observation.

## Audit hit list (full sweep of every v2 surface)

Miscoded — corrected by this overlay:

| # | Surface | Defect | Correction | Evidence |
|---|---------|--------|------------|----------|
| 1 | `ground-truth/callouts-fold-title.expected.json` | anchor `^title-anchor` wrongly absent (title-only callout, fixture line 25) | overlay file adds node span [468,481] | v04, w01, w03 |
| 2 | `ground-truth/anchors-edge-positions.expected.json` | anchor `^on-callout-head` wrongly absent (title anchor + body anchor, fixture lines 32–33) | overlay file adds node span [627,643] | w02a/b; parse-level precedent below |
| 3 | `tools/derive.py` lines 43, 48–49 | `ANCHOR_REMOVES` drops `on-callout-head` + `title-anchor`; comment states the wrong §1.3 rule | corrected derivation = `tools/rederive.py` (frozen deriver, one rule change, lane-drift guard) | erratum |
| 4 | `README.md` line 50 (schema delta D5) | "…callout head-line tails and comment interiors never register" — head-line half is wrong | superseded reading: "comment interiors never register; callout head-line tails DO register (§1.3 erratum)". Comment half stands | erratum |
| 5 | `OPEN-QUESTIONS.md` line 46 (OQ-14 row) | "head-line tail id is title text, NOT an anchor … Head-line anchor nodes removed" | superseded: head-line tail IS an anchor; the removed nodes are restored by this overlay | erratum |
| 6 | `MANIFEST.tsv` rows 3, 5 (notes col) | "`^on-callout-head` (1.3) dropped" / "head-line `^title-anchor` dropped (1.3)" record the wrong derivation | superseded by `amendments/v2.1/MANIFEST.tsv` basis/notes for the two amended expected files | erratum |

Parse-level precedent for hit 2: the pack already keeps BOTH `a08` and
`a09` (`zzprobe-anchors-a`, spans [222,226]/[228,232]) although Obsidian's
cache drops a08 via the same last-writer override (§1.2 Form B). Node
emission in this pack is parse-level; resolution rules live in
OPEN-QUESTIONS "Laws carried but not fixture-locked" (§1.4 last-wins).
`^on-callout-head` losing its cache slot to `^in-callout-body` therefore
does not remove its node.

Byte-correct for their construct — audited, left standing:

- `embeds-contexts` `in-comment-tail` removal — §1.3 comment-interior
  half is untouched by the erratum (probe cm1).
- `zzprobe-anchors-a` removes {a02, a07}, `zzprobe-anchors-c` removes
  {c01, c04}, `anchors-edge-positions` removes {spaced} — all §1.2
  position rules (trailing-ws, interior line-tail, first-line), not the
  §1.3 generalization.
- The exact oq14a/b fixtures are not in this pack (only
  `zzprobe-anchors-a/b/c` were carried in); nothing to leave in place.
- Corpus fixtures: zero callout title-line anchor tails (swept).

## Multi-line list-item check (item-level ruling)

Ruling (same erratum memo): continuation-tail anchors bind at ITEM level;
an anchor on the item's FIRST line with a continuation following does NOT
fire (probes V01/V02/V03/V05/V06).

- No pack surface states a contrary generalization (README D5
  "paragraph-end only" and the c01 removal are consistent with V03).
- `kitchen-sink` `^in-inner` [480,489] is correct under the ruling
  (continuation/paragraph tail — fires either way).
- **FLAG, not amended:** `kitchen-sink` `^nested-task` [447,459] sits on
  the first line of a task item whose next line (`inner tail line
  ^in-inner`) is a lazy continuation under CommonMark and under Obsidian's
  probed top-level behavior (a10, b03, V01). If that lazy continuation
  holds inside a nested callout, this is the V03 no-fire pattern and the
  node is wrongly present. No probe covers the callout-nested variant, so
  per the pack's "nothing invented from memory" law this stays un-amended:
  GT-v3 probe candidate (construct: task item + lazy continuation line
  inside a `> >` callout body).

## Overlay rule for consumers

For each fixture, expected output =
`amendments/v2.1/ground-truth/<stem>.expected.json` if present, else
`ground-truth/<stem>.expected.json`. Amended files are FULL replacements.
Totals under the overlay: 19 files, 664 expected nodes (662 + 2).

## Reproduction and verification

- `python3 amendments/v2.1/tools/rederive.py` — regenerates the two
  overlay files with the frozen deriver + the one corrected rule; aborts
  if the frozen rules no longer reproduce the frozen bytes (lane drift).
- `python3 amendments/v2.1/tools/verify-overlay.py` — materializes the
  consumer view and runs the FROZEN `tools/verify.py` on it unmodified.
  Green at amendment time: 19 files, 664 nodes, 0 violations.
- `python3 tools/verify.py` — base pack invariants, still green.

v2.1 immutability: files under `amendments/v2.1/` are frozen on commit;
further corrections go to `amendments/v2.2/` or GT-v3, never in place.
