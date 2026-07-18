---
aliases: [llm-wiki-root, load-skill-contract]
tags: [meta/schema, domain/wiki]
created: 2026-07-07
md-load-skill: '[[LLM_WIKI#^load-skill]]'
md-required: '[[LLM_WIKI#^required]]'
md-env: '[[LLM_WIKI#^env]]'
md-runtime: '[[LLM_WIKI#^runtime]]'
md-companions: '[[LLM_WIKI#^companions]]'
md-structure: '[[LLM_WIKI#^structure]]'
md-blurbs: '[[LLM_WIKI#^blurbs]]'
description: 'Root contract + agent context — load-skill output is injected into every llm-wiki skill load; this wiki decides what goes there (spec: llm-wiki skill setup/wiki-root-contract.md)'
---

# LLM_WIKI — root contract

Claimed by the llm-wiki skill: at every load, it runs `md run '{"file":"LLM_WIKI.md","name":"load-skill"}'` and inlines the output. The shared contract (path-is-the-interface, identity handshake, lean output, fix lines) ships with the skill — `setup/wiki-root-contract.md`; only this wiki's injection lives here.

home-wiki injects: the [[#^required|^required]] gate first (hard invariants — silent when clean, any output is an error-level violation; the same block is lefthook pre-commit guarded, so no commit lands until it resolves), the identity/version line, then the ~100-word [[SCHEMA#Agent kernel]] hot-loaded straight from `SCHEMA.md` (the operational law every load needs — flow, frontmatter-is-truth, wikilink discipline, domain-home invariant, read-the-page-type-first; the rest of SCHEMA stays on demand), then the four **key blocks** below, each a `#`-headed section and all **silent when healthy** so a clean load carries law, not status noise — [[#^env|^env]] (`# Environment` — env-variable anchors, pure env checks, no `md`; the sessions root and the repos root are the two most important concepts of llm-wiki, so this section explains them, not just prints them; always emitted), [[#^runtime|^runtime]] (`# Runtime` — toolchain doctor via [[runtime]]; prints only `MISSING` (+ fix, exit 1) or `DRIFT` (+ fix, exit 0) findings and nothing at all — no header — when every tool is present and on-pin), [[#^companions|^companions]] (`# Companions` — paths the agent must KNOW immediately: the `companion_entry` case table mirrors `^structure` — it IS the claim map, its slugs the roster; one line per repo — present as `- [slug](realpath) — purpose`, absent as `- slug — absent … — purpose`; a companion's FULL contract loads on first touch, its own `LLM_WIKI.md#load-skill`, never inlined here — that is the bulk of the old injection cost), [[#^structure|^structure]] (`# Structure` — the folder map: the case table in the block IS the predefined folder → display-name + description list, read on demand from this page; the check is live and silent — only a dir missing from the map prints `UNLISTED` with the fix, which [[#^required|^required]] greps to block the commit). A failing block prints an error for the agent to fix; the remaining blocks still run. The injection is composed of named blocks only — everything below [[#^load-skill|^load-skill]] (the [[#Agent context]] section) is page prose, read on demand, never injected. CLAUDE.md/AGENTS.md auto-loading is unreliable, so the context agents actually need rides the skill load instead of the harness.

```bash
# required: hard invariants — silent on success, detail + exit 1 on violation.
# Error level: lefthook pre-commit job llm-wiki-required runs this block and
# blocks the commit until it resolves. Currently gated: ^structure map drift,
# ^blurbs root-folder blurb presence (both hard gates as of C1).
out="$(md run '{"file":"LLM_WIKI.md","name":"structure"}' 2>&1)" || {
  echo "REQUIRED: ^structure block itself failed:"
  echo "$out"
  exit 1
}
drift="$(printf '%s\n' "$out" | grep -F 'UNLISTED' || true)"
if [ -n "$drift" ]; then
  echo "REQUIRED: top-level folders missing from the ^structure map:"
  echo "$drift"
  echo "fix: add each folder to the ^structure map in LLM_WIKI.md, or if it violates the wiki convention, move it where it belongs"
  exit 1
fi
blurbs="$(md run '{"file":"LLM_WIKI.md","name":"blurbs"}' 2>&1)" || {
  echo "REQUIRED: root folders missing their blurb (<folder>/<FOLDER>.md carrying a ^check block):"
  echo "$blurbs"
  echo "fix: create each named blurb with identity + a readable paragraph + a ^check block (mirrors health/blurb-presence.md; assets is exempt as a gitignored mirror mount)"
  exit 1
}
```

^required

```bash
# env: env-variable anchors — pure env checks, no md. The sessions root and the
# repos root are the two most important concepts of llm-wiki: explain, not just print.
echo
echo "# Environment"
echo
echo "- CCC_LLM_WIKI_PATH → $(realpath "${CCC_LLM_WIKI_PATH:?unset — hard error}") — the wiki root: every layer (inbox/ sources/ domains/ synthesis/ effects/) resolves under it"
if [ -n "${CCC_SESSION_PATH:-}" ]; then
  sr="$(realpath "$CCC_SESSION_PATH")"; src='from $CCC_SESSION_PATH'
else
  sr="$(ccc-cli session path 2>/dev/null || echo "$CCC_LLM_WIKI_PATH/sessions")"; src='ccc-cli derive → home-wiki-sessions companion'
fi
echo "- sessions root → $sr ($src) — where multi-agent work lives: every ccc session tree (agents/ tasks/ rosters/ results/) is created here, hive-partitioned year=/month=; work lands here first, ccc-compound graduates it into the wiki"
echo "- CCC_LLM_WIKI_REPOS_ROOT → ${CCC_LLM_WIKI_REPOS_ROOT:-<unset>} — the address layer: every repo this wiki knows resolves at \$CCC_LLM_WIKI_REPOS_ROOT/<slug> (a real clone or a symlink both satisfy it), and new repos clone directly there by default; the root itself is never a git repo"
```

^env

```bash
# runtime: toolchain doctor — silent when healthy. The check (health/runtime.md
# ^check) emits nothing when every tool is present and on-pin; it prints only
# MISSING (+ fix, exit 1) or DRIFT (+ fix, exit 0) findings. Emit the # Runtime
# header only when there is something to report, so a healthy load adds zero lines.
out="$(md run '{"file":"health/runtime.md","name":"check"}' 2>&1)" && rc=0 || rc=$?
if [ -n "$out" ]; then
  echo
  echo "# Runtime"
  echo
  printf '%s\n' "$out"
fi
exit $rc
```

^runtime

```bash
# companions: paths the agent must KNOW immediately — companion repos at the
# address layer. companion_entry mirrors ^structure: the case table IS home-wiki's
# claim map (slug → what the companion is claimed for), its slugs the loop roster.
# One line per companion — path + one-line purpose. A companion's FULL contract
# loads on first touch (that repo's own LLM_WIKI.md#load-skill), never inlined
# here — that is the 66%-of-injection saving this block used to spend.
companion_entry() {
  case "$1" in
    home-wiki-repos)     echo 'staging backend — external repos the wiki studies nest here first; graduation moves one to a standalone checkout' ;;
    home-wiki-codespace) echo 'agent code workspace — code agents write belongs here until it graduates to its own repo' ;;
    home-wiki-secrets)   echo 'service access — live credentials belong here, sops-encrypted' ;;
    home-wiki-assets)    echo 'asset store — every binary/non-md file backing a wiki source belongs here (LFS), never in the wiki tree' ;;
    home-wiki-archive)   echo 'cold archive — retired credentials and starter bundles belong here, SOPS-encrypted, append-only, never read in normal operation' ;;
  esac
}
echo
echo "# Companions"
echo
echo "> path + purpose only — a companion's full contract loads on first touch (its own LLM_WIKI.md#load-skill)"
echo
for c in home-wiki-repos home-wiki-codespace home-wiki-secrets home-wiki-assets home-wiki-archive; do
  p="${CCC_LLM_WIKI_REPOS_ROOT:?unset — hard error}/$c"
  if [ -e "$p" ]; then
    echo "- [$c]($(realpath "$p")) — $(companion_entry "$c")"
  else
    echo "- $c — absent (claimed, created on first need) — $(companion_entry "$c")"
  fi
done
```

^companions

```bash
# structure: folder-map drift check — silent when healthy. The case table IS the
# predefined list (folder → display name + description, read on demand from this
# page); the check is live over the actual top-level dirs. Only a dir MISSING
# from the map prints its raw name + UNLISTED — the drift marker ^required greps
# for. A fully-mapped tree contributes zero lines (no header).
structure_entry() {
  case "$1" in
    inbox)     echo 'Inbox — raw inputs, immutable after drop (never edit in place — ingest into domains); `_unstaged/` drop zone + tool partitions + `filed/<cluster>/<lane>/`' ;;
    sources)   echo 'Sources — immutable interpretations of raw inputs, one page per ingested source; bucket masters in `sources/SOURCES.md`' ;;
    domains)   echo 'Domains — our reasoning, evolving: `domains/<cluster>/<domain>/`, one grouping level, clusters are pure containers' ;;
    synthesis) echo 'Synthesis — guideline-producing layer: analyses + recipe tombstones, 6 sub-buckets' ;;
    effects)   echo 'Effects — descriptor tier: one pin-verified page per effect; point or own, never copy (`effects/EFFECTS.md`)' ;;
    decisions) echo 'Decisions — lazy-decision queue: `DECISIONS.md` hub + cluster dirs; `ambig/` = machine-stub queue' ;;
    bases)     echo 'Bases — computed catalogs (`.base` views); the catalog is computed, never maintained by hand' ;;
    logs)      echo 'Logs — operation log, one file per operation (ingest / compound / incorporate)' ;;
    health)    echo 'Health — aggregate invariants run via `md run`; `HEALTH.md` is the enforcement map' ;;
    templates) echo 'Templates — reusable templates (daily notes)' ;;
    foreign)   echo 'Foreign — gitignored runtime mount point for foreign wiki mirrors' ;;
    assets)    echo 'Assets — gitignored symlink mount of the home-wiki-assets companion (exact wiki-relative mirror tree); payloads live there behind type/asset-pointer pages here, embeds render through the mount' ;;
    tmp)       echo 'Tmp — gitignored ephemeral workspace (agent renders, one-off extracts); never a source of truth; non-md payloads do not belong here long-term (normalize to assets companion)' ;;
  esac
}
for d in */; do
  d="${d%/}"
  if [ -z "$(structure_entry "$d")" ]; then
    echo "- \`$d/\` — UNLISTED: not in the ^structure map — add it to LLM_WIKI.md ^structure, or if it violates the wiki convention, move it where it belongs"
  fi
done
```

^structure

```bash
# blurbs: root-contract blurb rule — every root-level folder presents exactly one
# blurb (`<folder>/<FOLDER>.md` carrying identity + a readable paragraph + a ^check
# block). The root contract enforces EXISTENCE + interface only; what a folder does
# internally is the blurb's own contract, out of root scope (layered contracts, each
# one level down). Iterates the top-level dirs, skips hidden + runtime mounts
# (foreign and assets are gitignored mirror mounts, not wiki layers). Silent when
# clean; prints NOBLURB (folder has no blurb) / NOCHECK (blurb has no ^check) per gap.
#
# ENFORCEMENT ACTIVE (C1): wired into ^required (and thus run transitively by
# ^load-skill) — a NOBLURB / NOCHECK finding blocks the commit via the lefthook
# llm-wiki-required job. The U-C1 blurb infrastructure landed first (bases/domains/
# templates blurbs + the ^check additions); assets is exempted above as a gitignored
# mirror mount (aligning with health/blurb-presence.md), so the gate is clean.
rc=0
for d in */; do
  d="${d%/}"
  case "$d" in .*|foreign|assets|tmp) continue ;; esac
  u="$(printf '%s' "$d" | tr '[:lower:]' '[:upper:]')"
  blurb="$d/$u.md"
  if [ ! -f "$blurb" ]; then
    echo "NOBLURB: $d — fix: create $blurb with a ^check block"
    rc=1
    continue
  fi
  grep -q '\^check' "$blurb" || { echo "NOCHECK: $blurb has no ^check block"; rc=1; }
done
exit $rc
```

^blurbs

```bash
# load-skill: required gate (silent when clean) + identity line + SCHEMA kernel hot-load + the key blocks (a failing block reports, the rest still run).
md run '{"file":"LLM_WIKI.md","name":"required"}' || echo "ERROR: ^required block failed — hard invariants violated (detail above); commits are blocked until fixed"
sed -n 's/^contract-version:[[:space:]]*/llm-wiki: home-wiki · contract v/p' SCHEMA.md | head -1
md read '{"target":"[[SCHEMA#Agent kernel]]","expect-unique":true}' </dev/null || echo "ERROR: SCHEMA.md#Agent kernel failed — fix the section"
md run '{"file":"LLM_WIKI.md","name":"env"}' || echo "ERROR: ^env block failed — fix the env vars it names"
md run '{"file":"LLM_WIKI.md","name":"runtime"}' || echo "ERROR: ^runtime block failed — run the health/runtime.md check and fix what it reports"
md run '{"file":"LLM_WIKI.md","name":"companions"}' || echo "ERROR: ^companions block failed — companion paths unknown; fix and re-run"
md run '{"file":"LLM_WIKI.md","name":"structure"}' || echo "ERROR: ^structure block failed — folder map unknown; fix and re-run"
```

^load-skill

## Agent context

Personal knowledge wiki — the home layer of an llm-wiki system: layered knowledge (inbox → sources → domains → synthesis → effects) operated by the `/llm-wiki` skill. Entry: `SCHEMA.md`. Repo root = `wiki/` after extraction; mounted back into locus at `locus/wiki` the prefix returns transparently. Standalone operation works without locus.

### Companion repos

Sibling repos beside this wiki, named by grammar `<wiki-slug>-<purpose>` (genesis contract §10, C43-companion-grammar / C44-claimed-trio): claimed by name alone and created on first need, so an absent companion is normal. Resolve at `$CCC_LLM_WIKI_REPOS_ROOT/<name>`. The claims live in the [[#^companions|^companions]] `companion_entry` map; `home-wiki-sessions` is claimed outside the map — it is the sessions root explained in `# Environment`. The skill load injects only each companion's path + one-line purpose; a companion's full `LLM_WIKI.md` contract loads on first touch, not every load.

### Conventions

- `SCHEMA.md` governs all wiki pages; frontmatter is for filtering, body is for reading

- Wikilinks in shortest-unambiguous canonical form

- The domain catalog is computed, not maintained — query live:

  ```bash
  obsidian base:query path="bases/DOMAINS.base" view="By cluster" format=md    # content catalog (prefix wiki/ when vault root is locus)
  obsidian base:query path="bases/DECISIONS.base" view="Open questions" format=md # standing decision surface (reading digest: root DIGEST.md)
  ```

  Headless fallback (Obsidian not running): `rg '^description:' domains` — the catalog data is `description:` frontmatter on `type/domain-index` pages.

### Obsidian

Vault root = repo root. Excluded Files: `foreign`. (`sessions` is now the separate `home-wiki-sessions` companion vault; the fleet bases — `FLEET.base`, `TASKS.base`, `DECISIONS.base` — moved there with the agent notes they join.)
