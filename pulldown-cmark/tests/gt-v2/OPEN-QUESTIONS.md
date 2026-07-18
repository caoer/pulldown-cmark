# GT-v2 open questions — dialect rulings needed before freeze

Each entry records the ruling the pack currently encodes (lane basis) and the
question queued on Stream H (obsidian-dialect-conformance). When Stream H's
spec answers one, the affected expected outputs are re-derived, the MANIFEST
basis column flips to `spec-h:<section>`, and the entry moves to Resolved.
This list doubles as Stream H's adversarial-candidate feed (task coupling).

| # | Fixture | Construct | Ruling encoded (lane) | Question for Stream H |
|---|---------|-----------|----------------------|----------------------|
| OQ-1 | crlf-mixed | `---\r\n` frontmatter | ~~not a node~~ → **RESOLVED, flipped** (feed-2) | — |
| OQ-2 | embeds-contexts, kitchen-sink | `![[x]]` / `[[x]]` inside `%% %%` | extracted (comments don't mask inline nodes) | Obsidian hides comment content — are refs inside comments live (backlinks/resolution)? |
| OQ-3 | anchors-edge-positions | `^spaced   ` trailing whitespace after id | ~~anchor~~ → **RESOLVED, flipped** | — |
| OQ-4 | anchors-edge-positions | `^-` lone-hyphen id | anchor (charset `[A-Za-z0-9-]+` admits it) | is `^-` a valid Obsidian block id? |
| OQ-5 | callouts-fold-title | `[!note]-Title-glued` | ~~callout, fold `-`~~ → **RESOLVED, flipped harder** (feed-2): not a callout at all | — |
| OQ-6 | callouts-fold-title | `[!note] - spaced dash` | no fold; `- …` is title text | confirm: fold marker must be glued to `]` (adjacent to the OQ-5 ruling, still wants explicit probe) |
| OQ-7 | callouts-type-charset | `[!注意]` unicode type | ~~not a callout~~ → **RESOLVED, flipped** (feed-2): callout, type charset is anything-but-`]` | — |
| OQ-8 | embeds-chains | `![[]]` empty target | no node | does Obsidian tokenize an empty embed? |
| OQ-9 | fragments-ambiguity | `[[Page#Head#^blk]]` | heading `"Head#^blk"` (split at first `#`; `#^` only checked at that split) | does a trailing `#^blk` inside a subpath act as a block ref? |
| OQ-10 | fragments-ambiguity, kitchen-sink | `[[Page#A#B]]` | heading `"A#B"` (parser keeps the raw fragment; names-contract: split at first `#`, GT-faithful) | confirm parser-level raw fragment is right; subpath resolution algebra is rung-2 (`resolve`) territory |
| OQ-11 | fragments-ambiguity | `[[]]`, `[[|alias-only]]` | `[[|alias-only]]` **RESOLVED** (feed-2): tokenized, target `""` + alias. Bare `[[]]` still encoded as no-node | does Obsidian tokenize bare `[[]]`? |
| OQ-12 | fragments-ambiguity | `[[#]]`, `[[#^]]` | node with empty target + empty heading/block | match Obsidian's tokenizer? |
| OQ-13 | callouts-nested, kitchen-sink | nested callout span start | at the innermost `>` of the head line (vanilla pulldown nested-BlockQuote ranges, probe @eea0453) | fork must confirm the same range convention once callout events land |
| OQ-14 | anchors-edge-positions, callouts-fold-title | `^id` at tail of a callout head line | anchor node emitted | what does the block ref address in Obsidian — the head line or the whole callout? (rung-2 resolve concern; node emission itself uncontested) |
| OQ-15 | callouts-type-charset | `[!]` empty type, `[!unclosed` | not callouts | feed-2's anything-but-`]` charset is silent on the empty type and the unclosed head — confirm both stay non-callouts |
| OQ-16 | batch2-pipe-escape | `[!note|meta]` pipe metadata | callout, `info.type` = pre-pipe part; metadata not captured | what shape captures the metadata — new `info` key by wire-contract amendment, or dropped at event level? |
| OQ-17 | batch2-pipe-escape | `[[Page\|esc]]` backslash disposition | split confirmed (feed-2); lane encodes target `"Page\"`, alias `"esc-alias"` — backslash left in target | does Obsidian's target strip the backslash (`Page`, escape consumed)? almost certainly yes — awaiting spec text to flip |

## Resolved

Per **spec-h:anchors@feed-1** (2b24e94a as-found message, probe-confirmed vs
Obsidian 1.12.7; probe inputs carried in as `zzprobe-anchors-a/-b.md`):

- **OQ-3 — flipped.** Trailing whitespace after `^id` invalidates the anchor.
  `^spaced` node removed from anchors-edge-positions; probe P02 is the
  canonical negative.
- **NEW LAW — paragraph-end only.** Anchors fire only at the end of a
  paragraph's inline text; interior line-tails of multi-line paragraphs do
  not register (probe P07 negative / P08 positive). Lane over-fires; corpus
  fixtures audited — zero corpus anchors affected.
- **NEW LAW — table-cell tails register** (id attaches to the whole table;
  attachment is rung-2 resolve territory, node emission is asserted here).
  Probe P15 + `^in-table-row` positives; mid-cell `^not-cell-tail` negative.
- Charset confirmed `[a-zA-Z0-9-]`; whitespace required before `^` (tab
  counts); escaped `\^` and doubled `^^` never fire; code contexts never
  fire; typed case preserved in the event id (blocks-map lowercasing is a
  resolution-layer concern, not an event concern).

Per **spec-h feed-2** (2b24e94a batch #2, probe-confirmed):

- **OQ-1 — flipped.** CRLF frontmatter parses (line-ending normalization
  precedes parse). crlf-mixed gains a frontmatter node; span stays raw-byte
  with the final `\r\n` clipped per span law.
- **OQ-5 — flipped harder.** `[!x]-Title` glued fold-title is not a callout
  at all (not merely fold-less); node removed from callouts-fold-title.
- **OQ-7 — flipped.** Callout type charset is anything-but-`]`: unicode
  types, spaces, leading-space types are all callouts (three nodes added).
- **OQ-11 — half-resolved.** `[[|alias-only]]` tokenizes (target `""`);
  bare `[[]]` remains open.
- **NEW — pipe metadata.** `[!type|metadata]` splits at the pipe; `info.type`
  carries the pre-pipe part (metadata shape → OQ-16).
- **NEW — escape defeats binding.** `\![[x]]` yields a wikilink, not an
  embed; `[[a\|b]]` still splits the alias (backslash disposition → OQ-17).
  Fixture: batch2-pipe-escape.md with positive+negative controls.

Spec skeleton:
`year=2026/month=07/18-02-meridian-rs/results/obsidian-dialect-conformance.md`
— remaining OQ rows re-derive as sections land.
