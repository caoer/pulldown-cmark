---
aliases: [obsidian-style, md-conventions]
tags: [meta/style-guide, domain/obsidian]
created: 2026-04-11
lint-ignore: [backticked-wikilink]
---

# Obsidian Markdown Style Guide

> [!abstract] Purpose
> Convention reference for agents writing/updating `.md` files in this project. Assumes deep Obsidian knowledge — this is a lookup table, not a tutorial.

## Linking

**Always use wikilinks. Never use standard markdown links for internal references.**

| Do | Don't |
|---|---|
| `[[style-guide]]` | `[style guide](./style-guide.md)` |
| `[[style-guide\|our conventions]]` | `[our conventions](../obsidian/style-guide.md)` |
| `[[note#Heading]]` | `[section](note.md#heading)` |
| `[[note#^block-id]]` | _(no standard equivalent)_ |

Standard `[text](url)` links are **only** for external URLs.

### Wikilink resolution

- Obsidian resolves by filename, not path — `[[plan]]` finds `plan.md` anywhere in vault
- Disambiguate duplicates with partial path: `[[projects/plan]]`
- Renames auto-update all references vault-wide

## Embeds

Prefix a wikilink with `!` to transclude content inline.

```md
![[note]]                  %% entire note %%
![[note#Section]]          %% heading section only %%
![[note#^block-id]]        %% single block %%
![[diagram.png]]           %% image, default size %%
![[diagram.png|500]]       %% image, 500px wide %%
![[demo.mp4]]              %% video %%
![[spec.pdf]]              %% inline PDF %%
```

> [!tip] Image storage
> Configure vault attachment folder to `assets/` or co-locate with the note. Never scatter images in vault root.

## Callouts

Use callouts instead of bold/italic for emphasis blocks. They degrade to blockquotes in non-Obsidian renderers.

```md
> [!type] Optional Title
> Body text.
```

**Core types:** `note`, `info`, `tip`, `warning`, `danger`, `bug`, `example`, `question`, `abstract`, `success`, `failure`, `quote`

**Foldable:** append `+` (default open) or `-` (default collapsed) to the type:

```md
> [!faq]- Collapsed by default
> Click to expand.

> [!details]+ Open by default
> Click to collapse.
```

**Nesting:**

```md
> [!note] Outer
> Content
>> [!warning] Inner
>> Nested callout
```

### When to use which type

| Context | Type |
|---|---|
| Caveats, gotchas | `warning` |
| Prerequisite, dependency | `info` |
| Best practice, recommendation | `tip` |
| Known issue, defect | `bug` |
| Decision rationale, trade-off | `abstract` |
| Code snippet, usage pattern | `example` |
| Open question, needs answer | `question` |

## Frontmatter

Every file **must** have YAML frontmatter. Minimum:

```yaml
---
tags: [topic/subtopic]
---
```

Full template for engineering docs:

```yaml
---
aliases: [short-name]
tags: [domain/area, type/spec, status/active]
created: 2026-04-11
updated: 2026-04-11
---
```

- Tags in frontmatter omit the `#` prefix
- Use nested tags: `domain/area` not flat `domain-area`
- `aliases` enable linking by alternate names

## Tags

### Placement

- **Frontmatter** (preferred): structured, doesn't clutter body text
- **Inline**: `#tag` in body text — use sparingly for contextual markers

### Conventions for this project

| Prefix | Purpose | Example |
|---|---|---|
| `type/` | Document type | `type/spec`, `type/adr`, `type/guide` |
| `domain/` | Knowledge area | `domain/infra`, `domain/frontend` |
| `status/` | Lifecycle state | `status/active`, `status/archived` |
| `project/` | Project scope | `project/locus`, `project/telegram-bot` |
| `meta/` | About the vault itself | `meta/style-guide`, `meta/template` |

## Comments

```md
%% Agent-only notes, hidden in reading mode %%

%%
Multi-line block.
Use for: TODOs, WIP context, template instructions.
%%
```

Use comments for:
- Template placeholders: `%% Replace with project name %%`
- Agent instructions that shouldn't render
- Hiding draft sections without deleting

## Highlights

```md
==highlighted text==
```

Use for drawing attention to key terms on first definition. Don't overuse — callouts are better for blocks.

## Math

```md
Inline: $O(n \log n)$
Block:
$$
\sum_{i=1}^{n} x_i
$$
```

## Diagrams

**Mermaid** — natively supported:

````md
```mermaid
graph TD
    A --> B
```
````

**D2** — requires `obsidian-d2` plugin + local `d2` binary. Not portable. Prefer Mermaid for vault-native diagrams.

## Dataview

> [!info] Plugin dependency
> Dataview is a community plugin. Queries render as raw code blocks without it.

Common patterns:

````md
%% List all notes linking here %%
```dataview
LIST FROM [[]]
```

%% Task rollup %%
```dataview
TASK FROM #project/locus WHERE !completed
```

%% Table dashboard %%
```dataview
TABLE status, updated FROM #type/spec SORT updated DESC
```
````

## Anti-Patterns

> [!danger] Don't do these

| Anti-pattern | Correct approach |
|---|---|
| `[link text](relative/path.md)` for internal links | `[[page-name\|link text]]` |
| `![alt](path/img.png)` for vault images | `![[img.png]]` or `![[img.png\|400]]` |
| Raw HTML (`<div>`, `<details>`) | Callouts, embeds, or comments |
| Flat tags (`#my-project-infra`) | Nested tags (`#project/infra`) |
| No frontmatter | Always include `---` block with at least `tags` |
| Hardcoded relative paths in links | Let wikilink resolution handle it |

## Engineering Doc Recommendations

1. **One concept per file.** Link between files with wikilinks rather than cramming everything into one doc.
2. **Use embeds for shared content.** Don't copy-paste — transclude with `![[shared-section]]`.
3. **Callouts for decision records.** `> [!abstract]` for decisions, `> [!question]` for open items.
4. **Dataview for dashboards.** Auto-generate index pages from frontmatter rather than manually maintaining lists.
5. **Block references for precise linking.** Append `^block-id` to important paragraphs so other notes can reference them directly.
6. **Comments for agent context.** Use `%%` comments for metadata and instructions that agents need but humans shouldn't see in reading mode.
