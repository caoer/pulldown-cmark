//! Event + byte-span tests for Obsidian callouts
//! (`Options::ENABLE_OBSIDIAN_CALLOUTS`).
//!
//! Reference semantics: the parser-bench callout post-pass scored 145/145
//! against frozen ground truth with the head pattern
//! `^[ \t]{0,3}>[ \t]*\[!([A-Za-z0-9_-]+)\]([+-]?)` — arbitrary type,
//! optional fold marker, no-space `>[!x]` accepted, title text allowed
//! after the tag.

use pulldown_cmark::{
    BlockQuoteKind, Callout, CalloutFold, Event, Options, Parser, Tag, TagEnd,
};
use std::ops::Range;

fn events(src: &str, opts: Options) -> Vec<(Event<'_>, Range<usize>)> {
    Parser::new_ext(src, opts).into_offset_iter().collect()
}

/// All `Start(BlockQuote)` events as (kind, callout, span).
fn quotes<'a>(
    src: &'a str,
    opts: Options,
) -> Vec<(Option<BlockQuoteKind>, Option<Callout<'a>>, Range<usize>)> {
    events(src, opts)
        .into_iter()
        .filter_map(|(ev, range)| match ev {
            Event::Start(Tag::BlockQuote { kind, callout }) => Some((kind, callout, range)),
            _ => None,
        })
        .collect()
}

fn callout(kind: &str, fold: Option<CalloutFold>) -> Option<Callout<'_>> {
    Some(Callout {
        kind: kind.into(),
        fold,
    })
}

const OBSIDIAN: Options = Options::ENABLE_OBSIDIAN_CALLOUTS;

#[test]
fn arbitrary_type_with_title_events_and_spans() {
    let src = "> [!custom-type] Unknown type\n> body\n";
    assert_eq!(
        events(src, OBSIDIAN),
        vec![
            (
                Event::Start(Tag::BlockQuote {
                    kind: None,
                    callout: Callout {
                        kind: "custom-type".into(),
                        fold: None,
                    }
                    .into(),
                }),
                0..37,
            ),
            (Event::Start(Tag::Paragraph), 17..37),
            (Event::Text("Unknown type".into()), 17..29),
            (Event::SoftBreak, 29..30),
            (Event::Text("body".into()), 32..36),
            (Event::End(TagEnd::Paragraph), 17..37),
            (Event::End(TagEnd::BlockQuote(None)), 0..37),
        ],
    );
    // the blockquote span is byte-exact over the whole callout
    assert_eq!(&src[0..37], src);
}

#[test]
fn bare_head_consumes_line_like_gfm_tag() {
    let src = "> [!note]\n> body\n";
    assert_eq!(
        events(src, OBSIDIAN),
        vec![
            (
                Event::Start(Tag::BlockQuote {
                    kind: None,
                    callout: Callout {
                        kind: "note".into(),
                        fold: None,
                    }
                    .into(),
                }),
                0..17,
            ),
            (Event::Start(Tag::Paragraph), 12..17),
            (Event::Text("body".into()), 12..16),
            (Event::End(TagEnd::Paragraph), 12..17),
            (Event::End(TagEnd::BlockQuote(None)), 0..17),
        ],
    );
}

#[test]
fn fold_open_marker() {
    let src = "> [!note]+\n> body\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(
        q,
        vec![(None, callout("note", Some(CalloutFold::Open)), 0..18)],
    );
}

#[test]
fn fold_folded_marker_with_title() {
    let src = "> [!faq]- Collapsed (foldable) callout\n> Folded body.\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(
        q,
        vec![(
            None,
            callout("faq", Some(CalloutFold::Folded)),
            0..src.len(),
        )],
    );
    // title stays inline content with byte-exact spans
    let evs = events(src, OBSIDIAN);
    let title = evs
        .iter()
        .find(|(ev, _)| matches!(ev, Event::Text(_)))
        .unwrap();
    assert_eq!(&src[title.1.clone()], "Collapsed (foldable) callout");
}

#[test]
fn no_space_variant() {
    let src = ">[!tip] no space after > (Obsidian still accepts)\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("tip", None), 0..src.len())]);
    let evs = events(src, OBSIDIAN);
    let first_text = evs
        .iter()
        .find(|(ev, _)| matches!(ev, Event::Text(_)))
        .unwrap();
    assert_eq!(first_text.1.start, 8);
    assert_eq!(&src[first_text.1.start..13], "no sp");
}

#[test]
fn no_space_variant_with_fold() {
    let src = ">[!custom-type]- folded\n> x\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(
        q,
        vec![(
            None,
            callout("custom-type", Some(CalloutFold::Folded)),
            0..src.len(),
        )],
    );
}

#[test]
fn plain_blockquote_stays_plain() {
    let src = "> plain blockquote, not a callout\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn malformed_head_falls_back_to_blockquote() {
    // no closing bracket => not a callout; the text must be preserved
    let src = "> [!note this is malformed\n> parser should fall back\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, None, 0..src.len())]);
    let evs = events(src, OBSIDIAN);
    // head text is intact inline content starting right after "> "
    let first_text = evs
        .iter()
        .find(|(ev, _)| matches!(ev, Event::Text(_)))
        .unwrap();
    assert_eq!(first_text.1.start, 2);
}

#[test]
fn empty_type_is_not_a_callout() {
    let src = "> [!] nope\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn flag_off_no_callout_semantics() {
    // With the flag off nothing changes: no callout, head text preserved.
    let src = "> [!note] Title\n> body\n";
    assert_eq!(quotes(src, Options::empty()), vec![(None, None, 0..23)]);
    let evs = events(src, Options::empty());
    assert!(evs
        .iter()
        .any(|(ev, r)| matches!(ev, Event::Text(_)) && &src[r.clone()] == "!note"));
}

#[test]
fn gfm_only_behavior_untouched() {
    let gfm = Options::ENABLE_GFM;
    // known tag + blank rest of line => alert, head line consumed
    assert_eq!(
        quotes("> [!NOTE]\n> body\n", gfm),
        vec![(Some(BlockQuoteKind::Note), None, 0..17)],
    );
    // arbitrary type is not a GFM alert
    assert_eq!(
        quotes("> [!custom-x]\n> body\n", gfm),
        vec![(None, None, 0..21)],
    );
    // title is not a GFM alert
    assert_eq!(
        quotes("> [!NOTE] Title\n", gfm),
        vec![(None, None, 0..16)],
    );
    // fold marker is not a GFM alert
    assert_eq!(
        quotes("> [!NOTE]-\n> body\n", gfm),
        vec![(None, None, 0..18)],
    );
}

#[test]
fn both_flags_populate_kind_and_callout() {
    let both = Options::ENABLE_GFM.union(OBSIDIAN);
    // GFM-shaped head: kind AND callout populated; type as written
    assert_eq!(
        quotes("> [!NOTE]\n> body\n", both),
        vec![(Some(BlockQuoteKind::Note), callout("NOTE", None), 0..17)],
    );
    // titled head: Obsidian callout but not a GFM alert
    assert_eq!(
        quotes("> [!note] Title\n", both),
        vec![(None, callout("note", None), 0..16)],
    );
    // folded head: Obsidian callout but not a GFM alert
    assert_eq!(
        quotes("> [!warning]-\n> w\n", both),
        vec![(
            None,
            callout("warning", Some(CalloutFold::Folded)),
            0..18,
        )],
    );
}

#[test]
fn nested_callout_inside_blockquote() {
    let src = "> outer\n> > [!info]\n> > nested callout inside\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q.len(), 2);
    assert_eq!(q[0], (None, None, 0..src.len()));
    assert_eq!(q[1].1, callout("info", None));
}

#[test]
fn callout_type_is_ascii_only() {
    // CJK type does not match the [A-Za-z0-9_-]+ charset => plain quote
    let src = "> [!注意]\n> body\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn cjk_title_and_body_spans_are_byte_exact() {
    let src = "> [!note] 标题一\n> 正文内容，第二行。\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("note", None), 0..src.len())]);
    let evs = events(src, OBSIDIAN);
    let texts: Vec<&str> = evs
        .iter()
        .filter_map(|(ev, r)| match ev {
            Event::Text(_) => Some(&src[r.clone()]),
            _ => None,
        })
        .collect();
    assert_eq!(texts, vec!["标题一", "正文内容，第二行。"]);
}

#[test]
fn cjk_immediately_after_tag() {
    // multi-byte char adjacent to the tag: no separating space to consume;
    // the scanner must stay on char boundaries
    let src = "> [!note]中文标题\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("note", None), 0..src.len())]);
    let evs = events(src, OBSIDIAN);
    let title = evs
        .iter()
        .find(|(ev, _)| matches!(ev, Event::Text(_)))
        .unwrap();
    assert_eq!(&src[title.1.clone()], "中文标题");
}

#[test]
fn second_line_tag_is_not_a_callout() {
    // the callout head must be the first line of the quote
    let src = "> intro\n> [!note] not a head\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn extra_whitespace_before_tag_is_accepted() {
    // lane pattern allows [ \t]* between > and [!
    let src = ">   [!note] spaced\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("note", None), 0..src.len())]);
}
