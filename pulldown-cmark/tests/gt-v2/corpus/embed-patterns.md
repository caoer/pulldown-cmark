---
tags: [domain/obsidian, type/example]
created: 2026-04-11
---

# Embed Patterns

Practical transclusion patterns for engineering docs.

## Embed Entire Note

```md
![[shared-glossary]]
```


Use for: glossaries, team agreements, shared definitions that multiple docs reference.

## Embed a Section

```md
![[api-spec#Authentication]]
```

Use for: pulling one section from a larger spec into a focused doc.

## Embed a Block

First, tag the block in the source note:

```md
The retry policy uses exponential backoff with jitter,
capped at 30 seconds. ^retry-policy
```

Then embed it elsewhere:

```md
![[resilience-patterns#^retry-policy]]
```

Use for: precise references — definitions, decisions, specific rules.
## Embed Images

```md
![[architecture-diagram.png]]          %% full size %%
![[architecture-diagram.png|600]]      %% 600px wide %%
![[screenshot.png|300x200]]            %% exact dimensions %%
```

## Embed PDF

```md
![[rfc-9110.pdf]]                      %% full document %%
![[rfc-9110.pdf#page=42]]              %% specific page %%
```

## Composition Pattern

Build a summary doc entirely from embeds:

```md
# Q2 Engineering Summary

## Architecture
![[q2-architecture#Overview]]

## Key Decisions
![[adr-001#^decision]]
![[adr-002#^decision]]
![[adr-003#^decision]]

## Metrics
![[q2-metrics#Performance]]

## Open Items
![[q2-retro#Action Items]]
```

> [!tip] Single source of truth
> Each piece of content lives in one canonical note. The summary note contains only structure and embeds. Updates to source notes propagate everywhere automatically.

> [!warning] Don't embed recursively
> If note A embeds note B which embeds note A, Obsidian shows a warning and stops. Keep embed chains acyclic.
