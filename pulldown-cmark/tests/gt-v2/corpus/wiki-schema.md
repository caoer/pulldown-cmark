---
aliases: [wiki-schema, wiki-conventions]
tags: [meta/schema, domain/wiki]
contract-version: 3
created: 2026-04-11
updated: 2026-07-11
lint-ignore: [backticked-wikilink]
---

# Wiki Schema — home-wiki

> [!ABSTRACT] What this file is
> Local overlay for the home-wiki — pins the contract version and declares the wiki-specific extensions. Current law only; history lives in decision pages and git.

## Contract

This wiki is bound by **contract v3** (pinned `contract-version: 3` above). The contract is the machine-checkable base — layout, frontmatter minima, addressing, decision queue, log/synthesis/check contracts, the effects/descriptor tier, the leaf-repo constraint — materialized in the llm-wiki skill (`references/contract-v3.md`) and resolved by `md schema`. This overlay shallow-merges on top; local wins.

### Canonical-clean requirement

This wiki must pass `md check --rules wikilink-canonicalize` clean — zero non-canonical links (contract §3a shortest-unambiguous addressing). Enforcement state lives on the rule page: [[wikilink-canonicalize]].

## Architecture

Five layers, each with an immutability boundary:

| Layer | Directory | Owner | Purpose | Immutability |
| -- | -- | -- | -- | -- |
| Raw inputs | `inbox/` | Human | Provenance-keyed capture lanes ([[INBOX]] is the door) | Never modified after drop |
| Sources | `sources/` | LLM | Honest interpretation of raw inputs | Immutable once written |
| Domains | `domains/<cluster>/<domain>/` | LLM | Our reasoning — why things matter to us | Evolving |
| Synthesis | `synthesis/` | LLM | Producing guidelines — how to make outputs | Evolving |
| Effects | `effects/` | LLM | Descriptor pages — one pin-verified page per effect | Descriptor; content homed in its repo or colocated |
| Schema | `SCHEMA.md` | Co-evolved | This file — conventions and workflows | — |

Information flows downstream: raw inputs → sources → domains/synthesis → effects. An effect page **points** to content homed in a repo (`repo`+`commit` / `location`+`checksum`) or **owns** content colocated — point or own, never copy. The effects vocabulary is four precise verbs — **install**, **verify**, **apply**, **publish**; [[EFFECTS]] owns the pin contract and entrypoints.

### Storage normalization

The wiki tree is text. Drops of **any** file type are welcome in any lane — convenience is never blocked at write time — but a non-md payload doesn't *live* here: it normalizes to the [[home-wiki-assets]] companion at the **exact wiki-relative mirror path** (`inbox/a/b/c.ext` ↔ `home-wiki-assets/inbox/a/b/c.ext`, LFS), leaving an **asset-pointer page** at the payload's path + `.md`. A git repo dropped into the tree relocates the same way in concept — to the `home-wiki-repos` staging companion — with a pointer page in its place. Rules of force:

- **Pointer frontmatter is the truth** — `asset:` names the mirror path; the pointer's *filename* is convention, not derivation. `md run '{"file":"<pointer>","name":"fetch"}'` prints the payload's absolute path (loud when the clone or LFS smudge is missing), so a working agent is never surprised by a dangling reference.
- **Whitelist** (Obsidian-native, must stay in-vault): `.base`, `.canvas`, `.gitkeep`.
- **Gitignored payloads are exempt** — a gitignore entry records a local-only ruling (e.g. the 2026-06-10 das-ego media); the sweep keys off *tracked/staged* files and never overrides those rulings.
- **Embeds render via the mount** — `assets/` at the wiki root is a gitignored symlink to the companion (the `foreign/` precedent), so `![[img.png]]` resolves while git stays clean.
- **Enforcement**: [[asset-normalization]] — pre-commit gate on staged files; `^sweep` performs move + pointer generation.

## Page Types

Contract §1–§11 defines the base page types; this overlay's per-type minima and owner pages:

| Page type | Dir | Frontmatter minima | Owner detail |
| -- | -- | -- | -- |
| domain home | `domains/<cluster>/<domain>/` | `tags: [domain/<name>, type/domain-index]` | this file (below) |
| domain leaf | same | `tags: [domain/<name>, type/reference]` | — |
| source | `sources/<bucket>/` | `tags: [type/source, domain/<x>]`, `source: [[raw]]`, `created` | [[SOURCES#Bucket masters]] |
| synthesis | `synthesis/<bucket>/` | `type: bucket`, `class: recipe\|analysis` | [[synthesis/SYNTHESIS]] |
| effect | `effects/<kind>/` | `type/effect` + one `effect/<kind>` + pin | [[EFFECTS]] |
| inbox | `inbox/` | per class (staging / archive / filed) | [[INBOX]], [[inbox-pipeline]] |
| asset pointer | payload path + `.md`, any layer | `tags: [type/asset-pointer]`, `asset`, `sha256`, `bytes`, `created`, `md-fetch` | [[asset-normalization]] |
| log | `logs/` | `tags: [meta/log, domain/wiki]`, `type: log`, `op`, `date` | — |

**Domain-home invariant** (full force): each domain has **EXACTLY ONE** `type/domain-index` home page — the `type/domain-index` tag, not the filename, is the authoritative home signal. Clusters are pure containers (no `.md` directly under `domains/<cluster>/`); leaf domain dir names stay globally unique, so a cluster re-cut changes zero canonical wikilinks. This aggregate (per-folder cardinality) invariant is enforced by [[domain-home-unique]] (pre-push job).

**Source provenance** (full force): every source page carries a `source:` `[[wikilink]]` to its raw `inbox/` file or external repo, and is **immutable once written**.

**Granularity**: a domain earns a dir iff it has a stable operating identity OR ≥3 coherent pages — otherwise it's a page in a parent domain.

## Effect pages — the receipt schema

Ratified by [[effect-attestation-refounding]] (approved 2026-07-13). The pin is refounded from a hand-maintained *vouch* into a **receipt written by the act of applying**; this section is the block-level law the migration realises onto each effect page. **Old rules still govern old shapes** — a legacy page (frontmatter `commit`, no `inputs:`) keeps the four-part pin contract in [[EFFECTS#The pin contract (four parts)]] until it migrates; the shapes coexist, discriminated by `inputs:` presence.

**Block-pointer convention.** Obsidian properties support scalars and quoted links, not objects — so structured machine data lives in **frontmatter-addressed body blocks** (the `md-env: '[[LLM_WIKI#^env]]'` precedent): the frontmatter field is a quoted same-page block-ref, the data is a fenced YAML block anchored `^<name>`, and the resolver reads it like any other node. Naming stays bare (`inputs`/`receipt`, never `md-inputs`): the `md-*` prefix is the *runnable* task namespace; these are *data* blocks, read by the resolver and `md attest`, never executed. **Omission never carries meaning** — a field whose absence would be ambiguous is present with an explicit value.

### Receipt tri-state

The `receipt` frontmatter key is an **explicitly nullable pointer**:

| Frontmatter | State |
| -- | -- |
| `receipt: null` | pointed, never applied — representable and queryable (omission cannot express null) |
| `receipt: '[[#^receipt]]'` | attested — the record lives in the `^receipt` block |
| key absent | **owned** effects only (`repo: home-wiki` is the discriminator; lint `owned-no-receipt`) |

A new pointed effect is **born invalid** (D1): `receipt: null` with chain items at `hash: null`, stated never omitted — the first realise is what makes it valid. The `null → '[[#^receipt]]'` flip at first attestation is the machinery's **only** frontmatter write; every later attestation rewrites the block only, so prose edits and machine writes stop colliding.

### The chain block — `^inputs`

Frontmatter carries the unconditional pointer `inputs: '[[#^inputs]]'` (never null — a chainless effect is illegal). The chain is a fenced YAML block anchored `^inputs`; each item is a dependency whose change invalidates the effect:

```yaml
# ^inputs
- ref: '[[some-domain-page#Section]]'
  claim: 'optional human annotation of what this dependency shapes'   # author, optional
  hash: 4c01d9e2                # machine-written at realise; born null
hash-algo: v1                   # spec version of every hash in this block (D2 = mdformat-canonical slice)
```

| Field | Writer | Semantics |
| -- | -- | -- |
| `ref` | author | `[[page]]`, `[[page#Section]]`, `[[page#^block]]`, or another effect page (composition) |
| `claim` | author, optional | human edge-annotation — what this input shapes |
| `hash` | **machine only** | content hash at last attestation, written only when the resolved content changed |
| `hash-algo` | machine | the data-contract version (`vN`); a mismatch is a **mechanical re-hash trigger**, never content invalidation |

- **Hash semantics.** Resolving a ref slices the section (heading anchor → next same-or-higher heading; `#^block` for finer grain). **Embeds expand, links don't**: `![[…]]` transclusions inside the slice resolve recursively (cycle-guarded); plain `[[…]]` references never expand (expanding them would make every hash cover the whole wiki). Chain hashes are **merkle-v1 composition over per-doc facts**, recomputed per run, never persisted. `inbox/scraped/**` is mdformat-excluded (faithful-capture lane), so a ref into scraped content hashes **raw bytes**, not the `hash-algo: v1` canonical slice (E11).
- **Two ref classes (challenge-C5).** An `inputs` edge to another effect hashes **by the dependency's kind**: an edge → a **pointed** effect hashes that dependency's **receipt `checksum`** (its receipt is the stable machine-written fact); an edge → an **owned** effect hashes **content slices** of its location (owned pages take no machine rewrites, so slices are safe and stable). Invalidation propagates through the DAG; execution stays per-effect (validity-dependence, not trigger fan-out).
- **Rootless is void** (`effect-rootless` = error): `inputs` MUST be non-empty. **Self-rooting is legal** — an ad-hoc effect lists its own spec section (`[[#Spec]]`); editing your own spec invalidates yourself. Effect→effect edges are **acyclic per snapshot** (`effect-graph-acyclic` = error).

### The receipt block — `^receipt`

Fenced YAML anchored `^receipt`, addressed by the explicit `receipt` key, written **only by the realise machinery, only when values change**:

```yaml
# ^receipt
commit: 6e1825c53deb40031ef3520d0cfbdc3d9c2e2de2
checksum: ab6b88fa60c16b7152094c68cc456aced6f43096
applied_at: 2026-07-09T14:02:11Z
procedure-hash: 91af7c20        # hash of the resolved ^check+^apply (post-inheritance)
hash-algo: v1
verdict: 'year=2026/month=07/09-16-ccc-skill-shrink/verdicts/<key>.reviewer.md@6e1825c5'
```

| Field | Meaning | Writer |
| -- | -- | -- |
| `commit` | full sha last applied-and-verified; on `origin/<branch>` (pointed) or ancestor of HEAD (owned) | machine |
| `location` | files/folders in the home (identity, with `repo`); scalar — **stays in frontmatter**, not the block | human (authoring) |
| `checksum` | `git -C <repo> rev-parse <commit>:<location>` — tree SHA for a dir, blob SHA for a file; reproducible from the receipt alone | machine |
| `applied_at` | UTC timestamp of the receipt write | machine |
| `procedure-hash` | hash of the **resolved** `^check`+`^apply` blocks (post-inheritance) at application — the cond-4 term (closes C8) | machine |
| `verdict` | `<path>@<commit-sha>` ref to the tier-2 **reviewer** verdict in home-wiki-sessions (§ verdict refs); absent on tier-1-only attests | machine |

- **`procedure-hash` (cond-4).** A procedure change (an `^apply`/`^check` edit) is an implicit chain input: a mismatch between recorded and currently-resolved procedure-hash is a receipt-moving invalidation with cause `procedure`, routing to a **tier-2 review of the procedure diff before execution**. Storms are reviewed once per unique `(old → new)` procedure-hash pair; the verdict is shared by every inheriting effect.
- **Verdict refs — `<path>@<commit-sha>`** (supersedes the draft's bare `obsidian://` URI). home-wiki-sessions is append-only as a load-bearing invariant; the sha survives file **moves** (object still reachable) but not history rewrites (a dangling ref degrades to a cache miss → recompute, never false-valid). Executor and reviewer verdicts are two files (`<key>.executor.md`, `<key>.reviewer.md`); the receipt's single `verdict` field points at the **reviewer** file — the binding gate — and is committed **before** `md attest` references it.
- **Owned effects** (`repo: home-wiki`) carry no `^receipt` block: colocated content is at wiki HEAD by definition. Their `^check` verifies gates and re-hashes the chain (input hashes only — no `applied_at`, P20); a hand-forged receipt fails reproduction the same way drift does. Enforcement is practical, not cryptographic.

### Release variant — un-stewarded third-party effects

For an effect the wiki recommends but homes in no repo it controls, `receipt-variant: release` (D4): `^apply` = **download → verify SHA-256 → install → receipt** recording the release **tag**, per-**asset SHA-256 hashes**, and the **installed path** (checksumming what you fetch, never "latest" or bare version prose). **Contract text only until a first instance exists** — zero release-pinned pages today (E10); tooling deferred.

### Well-known units and writer discipline

Machine regions sit contiguous at the page tail (suggested order: claim prose → `## Chain` (`^inputs`) → `## Receipt` (`^receipt`) → `## Changelog` → `## Notes`). History lands **owned** → a colocated `CHANGELOG.md`; **pointed** → a machine-appended `## Changelog` section — never a file inside `location` (that changes the attested tree and regresses its own receipt). `no-pin-values-in-body` forbids `commit`/`checksum`/resolved remote URLs in the **claim body** (page minus the Changelog unit). Repo-level facts (`remote`, `branch`, read-watermark `commit`) live only on the `sources/git/<slug>` catalog page; `repo:` on an effect is a **slug reference** that MUST resolve there (`repo-resolves-to-source-page`) or be `home-wiki`.

## Tag Taxonomy

The **prefix set is closed**; values within open prefixes are computed — the pages carrying a prefix ARE its live value set (`rg -o '\btopic/[a-z0-9-]+'` or the relevant `.base`), never enumerated here. Prefix semantics: `domain/` subject area (one per domain, no cluster component); `type/` page kind (open); `type/effect` + `effect/<kind>` effect pages (`effect/` kinds closed: skill, agent, prompt, site, document); `topic/`, `plugin/`, `source/`, `agent/`, `role/`, `session/`, `project/`, `convention/` open; `priority/`, `status/`, `use/`, `has/` closed. `do/` and `meta/` are open growing namespaces. The full prefix registry — every prefix, its semantics, and open/closed marking — is lint-owned: [[tag-taxonomy-enforcement#Prefix registry]].

## Linking Rules

- Always use `[[wikilinks]]`, never `[text](path.md)` for an internal page.
- **Canonical form**: shortest-unambiguous (contract §3a); `md fix --rules wikilink-canonicalize` normalizes. Use display text only when the page name isn't clear in context: `[[plugin-architecture|Plugin Architecture]]`.
- **Never wrap a live wikilink in inline-code backticks** (lint `backticked-wikilink`).
- Cross-link between domains freely; move a linked file with `obsidian move` so inbound links rewrite.

## Lint

The wiki must pass `md check` clean. Rules are literate pages under `health/rules/{contract,home-wiki}/`, one page per rule, wired in `meridian.yaml`. The enforcement map and opt-out mechanics (`lint-ignore:` frontmatter, `<!-- md:ignore -->` inline forms) live at [[HEALTH]].

## Agent kernel

- Flow: `_unstaged/` is movable staging; archive lanes and faithful `sources/` are immutable. `domains/` and `synthesis/` evolve. `effects/` point to pinned content or own colocated content—never copy. Non-md payloads normalize to the assets companion's exact mirror path, an asset-pointer page (`md run … fetch`) in their place.
- Frontmatter is query truth; body is for reading. Catalogs, rosters, and open tag values are computed, never hand-maintained.
- Use shortest-unambiguous `[[wikilinks]]`; never Markdown-link an internal page or code-wrap a live wikilink. Move linked files with `obsidian move`.
- Domain clusters are pure containers; leaf names are globally unique; each domain has exactly one `type/domain-index` home.
- Before specialized writes, read only the relevant SCHEMA page-type section.
