---
embed: "![[fm-fake]] not a node in frontmatter"
---

## Heading with ![[embed-in-heading]] inside

> [!callout] title embed ![[in-callout-title]]
> body embed ![[in-callout-body#Frag]]

- list item with ![[in-list-item]]
- [ ] task with ![[in-task-item|alias]]

> plain quote with ![[in-plain-quote]]

| col |
|-----|
| cell ![[in-table-cell]] |

Broken across a newline is not a node: ![[bro
ken]]

```
![[in-fence-not-a-node]]
```

Inline code `![[in-inline-code-not-a-node]]` stays text.

%% ![[in-comment-still-a-node-question]] %%
