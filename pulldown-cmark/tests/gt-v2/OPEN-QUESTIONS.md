# GT-v2 open questions — dialect rulings needed before freeze

Each entry records the ruling the pack currently encodes (lane basis) and the
question queued on Stream H (obsidian-dialect-conformance). When Stream H's
spec answers one, the affected expected outputs are re-derived, the MANIFEST
basis column flips to `spec-h:<section>`, and the entry moves to Resolved.
This list doubles as Stream H's adversarial-candidate feed (task coupling).

| # | Fixture | Construct | Ruling encoded (lane) | Question for Stream H |
|---|---------|-----------|----------------------|----------------------|
| OQ-1 | crlf-mixed | `---\r\n` frontmatter | not a node (bytes 0..4 must be `---\n`) | does Obsidian parse CRLF frontmatter? (likely yes — would flip the law) |
| OQ-2 | embeds-contexts, kitchen-sink | `![[x]]` / `[[x]]` inside `%% %%` | extracted (comments don't mask inline nodes) | Obsidian hides comment content — are refs inside comments live (backlinks/resolution)? |
| OQ-3 | anchors-edge-positions | `^spaced   ` trailing whitespace after id | ~~anchor~~ → **RESOLVED, flipped** | — |
| OQ-4 | anchors-edge-positions | `^-` lone-hyphen id | anchor (charset `[A-Za-z0-9-]+` admits it) | is `^-` a valid Obsidian block id? |
| OQ-5 | callouts-fold-title | `[!note]-Title-glued` | fold `-`, rest is title | does Obsidian require a space/EOL after the fold marker? |
| OQ-6 | callouts-fold-title | `[!note] - spaced dash` | no fold; `- …` is title text | confirm: fold marker must be glued to `]` |
| OQ-7 | callouts-type-charset | `[!注意]` unicode type | not a callout (type charset `[A-Za-z0-9_-]+`) | does Obsidian accept unicode callout types? |
| OQ-8 | embeds-chains | `![[]]` empty target | no node | does Obsidian tokenize an empty embed? |
| OQ-9 | fragments-ambiguity | `[[Page#Head#^blk]]` | heading `"Head#^blk"` (split at first `#`; `#^` only checked at that split) | does a trailing `#^blk` inside a subpath act as a block ref? |
| OQ-10 | fragments-ambiguity, kitchen-sink | `[[Page#A#B]]` | heading `"A#B"` (parser keeps the raw fragment; names-contract: split at first `#`, GT-faithful) | confirm parser-level raw fragment is right; subpath resolution algebra is rung-2 (`resolve`) territory |
| OQ-11 | fragments-ambiguity | `[[]]`, `[[|alias-only]]` | no node | does Obsidian tokenize degenerate links? |
| OQ-12 | fragments-ambiguity | `[[#]]`, `[[#^]]` | node with empty target + empty heading/block | match Obsidian's tokenizer? |
| OQ-13 | callouts-nested, kitchen-sink | nested callout span start | at the innermost `>` of the head line (vanilla pulldown nested-BlockQuote ranges, probe @eea0453) | fork must confirm the same range convention once callout events land |
| OQ-14 | anchors-edge-positions, callouts-fold-title | `^id` at tail of a callout head line | anchor node emitted | what does the block ref address in Obsidian — the head line or the whole callout? (rung-2 resolve concern; node emission itself uncontested) |

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

Spec skeleton:
`year=2026/month=07/18-02-meridian-rs/results/obsidian-dialect-conformance.md`
— anchors section stabilizes first; remaining OQ rows re-derive as sections
land.
