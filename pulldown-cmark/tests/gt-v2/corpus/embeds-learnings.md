---
tags: [domain/obsidian, type/learning]
created: 2026-04-11
lint-ignore: [backticked-wikilink]
---

# Embeds (Transclusion)

Prefix any wikilink with `!` to render the target's content inline. The source file stays in one place; it appears in many.

## Syntax

```md
![[note]]                   %% entire note %%
![[note#Section Heading]]   %% only that heading's section %%
![[note#^block-id]]         %% single paragraph/block %%
![[image.png]]              %% image %%
![[image.png|400]]          %% image at 400px width %%
![[image.png|400x300]]      %% image at exact dimensions %%
![[video.mp4]]              %% video player %%
![[audio.mp3]]              %% audio player %%
![[document.pdf]]           %% inline PDF viewer %%
![[document.pdf#page=3]]    %% PDF at specific page %%
```

## Image Storage

Configure in **Settings > Files & Links > Default location for new attachments**:

| Option | Path | When to use |
|---|---|---|
| Vault folder | `/` root | Small vaults |
| Same folder as note | `./` | Co-located assets |
| Specific folder | `assets/` | ==Recommended== — keeps vault clean |
| Subfolder under current | `./attachments/` | Per-directory isolation |

> [!tip] Convention for this project
> Use `assets/` at the vault level. Embed with `![[filename.png]]` — Obsidian finds it by name, no path needed.

## Block References

To make a block referenceable, append a `^block-id` after it:

```md
This is an important statement. ^key-insight
```

Other notes link to it with `[[note#^key-insight]]` or embed it with `![[note#^key-insight]]`.

Obsidian auto-generates block IDs when you type `[[note#^` and search — you can also manually assign them for stable references.

## Single Source of Truth

Embeds enable the **write once, embed everywhere** pattern:

- Define a term/concept in one canonical note
- Embed it into specs, guides, dashboards
- Update the source — all embeds update automatically

> [!warning] Portability
> `![[...]]` is Obsidian-only. In GitHub or other renderers it appears as a broken link. If portability matters, duplicate the content instead.
