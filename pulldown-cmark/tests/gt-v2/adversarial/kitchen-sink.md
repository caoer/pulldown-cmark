---
title: kitchen sink integration
tags: [gt-v2, adversarial]
status: synthesized
---

# Kitchen sink ^ks-h1

Opening paragraph with [[Plain]] and [[Deep#Sub#Path|display]] and ![[Embedded#^blk]].

## Constructs interleaved ^ks-h2

> [!outer]- Folded outer with ![[embed-in-title]] and [[link#in-title]]
> body wikilink [[body-link#Head]] and anchor at tail ^in-outer
> > [!inner]+ nested open
> > - [ ] nested task with embed ![[task-embed|al]] ^nested-task
> > inner tail line ^in-inner

A table hosting inline constructs:

| construct | sample |
|-----------|--------|
| wikilink | [[T#F]] |
| embed | ![[T#^b]] |
| code | `[[not-a-link]]` |

```text
# not a heading
> [!not-a-callout]
![[not-an-embed]] ^not-an-anchor
```

%% comment wrapping ![[embed-in-comment]] and [[link#in-comment]] and tail ^in-comment %%

Inline `` `![[double-backtick-embed]]` `` stays code.

### Deep section ^ks-h3

1. ordered item [[ordered#link]]
2. ordered task follows
   - [x] checked nested with fragment link [[X#^y|z]] ^ordered-anchor

Final lonely anchor on its own line:

^ks-final
