# GT-v2 open questions — ledger against the Stream H dialect spec

The Stream H dialect-conformance spec is COMPLETE
(`year=2026/month=07/18-02-meridian-rs/results/obsidian-dialect-conformance.md`,
sections 1–6, every claim `[doc]`/`[probe]`/`[source]`-tagged). All fixture
expectations in this pack now derive from it wherever it rules; MANIFEST
basis columns cite the spec section per fixture. This file records what
remains open and how each formerly-open ruling was settled.

## Still open

| # | Concern | Status |
|---|---------|--------|
| spec §6 | Stream H's own remaining unknowns (7 items: `getFirstLinkpathDest` tie-break, fold `-` initial DOM state, etc.) | resolution/render-level; none affect node emission in this pack — future GT-v3 candidates |

## Resolved fork-side (ruled by fork leader 5c8052b8, pre-freeze)

- **OQ-13 — span convention holds by construction.** Callout is a field ON
  the existing BlockQuote tag; spans are vanilla pulldown nested-BQ ranges,
  unchanged from `eea0453`. Pinned as an explicit invariant on the
  callouts-dialect-fix worker so recognition changes cannot drift it.
- **OQ-18 — normalized `info.type` ruled compatible.** Contract amendment
  in flight (callouts-dialect-fix): fork `Callout` gains additive
  `metadata: Option<CowStr>` (verbatim); `kind` stays the RAW pre-pipe
  slice (span-faithful); normalization lives in a public helper
  `canonical_kind()` = trim + lowercase + ws-runs→dash. GT-v2's
  `info.type` asserts the helper's output; `info.metadata` the verbatim
  string. Parser never mutates source-derived slices.

## Resolved (all rulings per spec section, probe-confirmed)

| # | Construct | Final ruling encoded |
|---|-----------|---------------------|
| OQ-1 | CRLF frontmatter | **flipped** — parses (LAW-0: line-ending normalization precedes all parsing); node added, span raw-byte with CRLF clipped |
| OQ-2 | refs/anchors inside `%%…%%` | **split** (§4.4/§1.3) — links+embeds ARE extracted (live for cache); anchors are NOT; negative `^in-comment-tail` fixture added |
| OQ-3 | trailing whitespace after `^id` | **flipped** — invalidates (§1.2, probes a02/c04 incl. tab) |
| OQ-4 | `^-` lone-hyphen id | **confirmed** — valid (§1.1) |
| OQ-5 | `[!note]-Title-glued` | **flipped** — not a callout at all (§2.1: after fold char must be `\s` or EOL); also kills `[!note]junk`, `[!faq]+-` — negatives added |
| OQ-6 | `[!note] - spaced dash` | **confirmed** — callout, no fold, title `- …` (§2.1) |
| OQ-7 | `[!注意]` unicode type | **flipped** — callout; type charset is anything-except-`]`, then normalized (§2.1–2.2) |
| OQ-8 | `![[]]` | **confirmed** — no node (§3.1) |
| OQ-9 | `[[P#Head#^blk]]` | **ruled** (resolution side) — resolves NULL, block refs only as sole segment (§5.3); parser raw-fragment convention unaffected |
| OQ-10 | `[[P#A#B]]` raw fragment | **confirmed** — cache stores raw string; split-at-first-`#` with raw remainder matches (§4.1) |
| OQ-11 | degenerates | **split** — `[[]]` no node (confirmed); `[[|alias-only]]` tokenizes with link = raw `"|alias-only"`, empty path disables alias split (§4.2) |
| OQ-12 | `[[#]]`, `[[#^]]` | **confirmed** — tokenize with raw link text, resolve NULL (§4.1) |
| OQ-14 | `^id` on callout head line | **ruled** — head-line tail id is title text, NOT an anchor (§1.3); body-paragraph tail DOES register (attaches to callout section). Head-line anchor nodes removed |
| OQ-15 | `[!]` empty type, `[!unclosed` | **confirmed** — both plain quotes (§2.1: type needs ≥1 char; recognition needs `]`) |
| OQ-16 | pipe metadata shape | **resolved** — metadata string verbatim (§2.2); encoded as additive `info.metadata`, present only when a pipe was written |
| OQ-17 | `[[a\|b]]` backslash | **resolved** — escaped `\|` also splits; link `a`, display `b`, backslash consumed (§4.2) |

## Laws carried but not (yet) fixture-locked

Cache/resolution-level rulings that do not change node emission, recorded
for rung-2 consumers: duplicate `^id` → LAST wins (§1.4); blocks-map keys
lowercased, `section.id` typed case (§1.1); own-line id overrides same-block
inline id (§1.2); embed recursion = depth cap ≈5, not cycle detection —
treat as small finite cap (§3.3); heading cache stores raw markdown, both
raw and stripped forms resolve (§5.1); footnote subpaths `#[^id]` exist,
case-sensitive (§5.4); resolve normalization strip-set verbatim in §5.0.
