//! Event + byte-span tests for Obsidian callouts
//! (`Options::ENABLE_OBSIDIAN_CALLOUTS`).
//!
//! Reference semantics: probe-confirmed Obsidian 1.12.7 recognition
//! (session `18-02-meridian-rs`, `results/obsidian-dialect-conformance.md`
//! §2; probes `probes/zzprobe-callouts.md` K01-K20 and
//! `probes/zzprobe-oq.md` OQ5-OQ7). The first line's content after the
//! blockquote marker must match `/^\[!([^\]]+)\]([+\-]?)(?:\s|$)/`;
//! the bracket content splits at its first `|` into raw type +
//! verbatim metadata.

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

/// Text spans of all `Event::Text` events, resolved against the source.
fn texts<'a>(src: &'a str, opts: Options) -> Vec<&'a str> {
    events(src, opts)
        .into_iter()
        .filter_map(|(ev, r)| match ev {
            Event::Text(_) => Some(&src[r]),
            _ => None,
        })
        .collect()
}

fn callout(kind: &str, fold: Option<CalloutFold>) -> Option<Callout<'_>> {
    Some(Callout {
        kind: kind.into(),
        metadata: None,
        fold,
    })
}

fn callout_meta<'a>(
    kind: &'a str,
    metadata: &'a str,
    fold: Option<CalloutFold>,
) -> Option<Callout<'a>> {
    Some(Callout {
        kind: kind.into(),
        metadata: Some(metadata.into()),
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
                        metadata: None,
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
                        metadata: None,
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
    // probe K13: >[!tip] with no space after > is a callout
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
    // no closing bracket before end-of-line => not a callout;
    // the text must be preserved
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
    // probe K05: [!] needs at least one char of bracket content
    let src = "> [!] nope\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn glued_text_after_tag_is_plain() {
    // probe K10: [!note]junk — no whitespace/EOL after ] => plain quote
    let src = "> [!note]junk after\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
    // multi-byte glued content: same rule, and the scanner must stay
    // on char boundaries while rejecting; head text is preserved
    // (inline parsing splits the unmatched brackets into text pieces)
    let src = "> [!note]中文标题\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
    assert_eq!(texts(src, OBSIDIAN).concat(), "[!note]中文标题");
}

#[test]
fn glued_fold_title_is_plain() {
    // probe OQ5: [!note]-Glued title — text glued to the fold marker
    // kills the whole match (lane ruling "fold -, rest is title" was
    // wrong; dialect truth outranks lane rulings)
    let src = "> [!note]-Glued title\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
    assert_eq!(texts(src, OBSIDIAN).concat(), "[!note]-Glued title");
}

#[test]
fn both_fold_markers_is_plain() {
    // probe K09: [!faq]+- — fold path fails on the trailing `-`,
    // no-fold path fails on the `+` => plain quote (regex backtracking)
    let src = "> [!faq]+- Both markers\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn unicode_type_is_a_callout() {
    // probe OQ7: type is one+ of ANY char except ] — [!注意] IS a callout
    // (lane charset [A-Za-z0-9_-]+ was wrong)
    let src = "> [!注意] Unicode type\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("注意", None), 0..src.len())]);
    assert_eq!(texts(src, OBSIDIAN), vec!["Unicode type"]);
}

#[test]
fn spaces_in_type_are_a_callout() {
    // probe K03: [!my custom type] is a callout with the raw multi-word
    // type; canonical_kind gives Obsidian's normalized form
    let src = "> [!my custom type] Multiword\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("my custom type", None), 0..src.len())]);
    let c = q[0].1.as_ref().unwrap();
    assert_eq!(c.canonical_kind().as_ref(), "my-custom-type");
    // title span is byte-exact after the tag prefix
    let evs = events(src, OBSIDIAN);
    let title = evs
        .iter()
        .find(|(ev, _)| matches!(ev, Event::Text(_)))
        .unwrap();
    assert_eq!(&src[title.1.clone()], "Multiword");
}

#[test]
fn space_after_bang_is_a_callout() {
    // probe K04: [! note] is a callout; the raw kind keeps the space,
    // normalization trims it to "note"
    let src = "> [! note] Space after bang\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout(" note", None), 0..src.len())]);
    let c = q[0].1.as_ref().unwrap();
    assert_eq!(c.canonical_kind().as_ref(), "note");
}

#[test]
fn metadata_split_events_and_spans() {
    // probe K11: first | inside the brackets splits type from a
    // verbatim metadata string; kind stays the raw pre-| slice
    let src = "> [!note|some-meta] Meta title\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(
        q,
        vec![(None, callout_meta("note", "some-meta", None), 0..src.len())],
    );
    let evs = events(src, OBSIDIAN);
    let title = evs
        .iter()
        .find(|(ev, _)| matches!(ev, Event::Text(_)))
        .unwrap();
    assert_eq!(title.1, 20..30);
    assert_eq!(&src[title.1.clone()], "Meta title");

    // probe K12: color metadata is carried verbatim
    let src = "> [!quote|#ff0000] Color meta\n";
    assert_eq!(
        quotes(src, OBSIDIAN),
        vec![(None, callout_meta("quote", "#ff0000", None), 0..src.len())],
    );

    // only the FIRST | splits; the rest is metadata verbatim
    let src = "> [!a|b|c]\n";
    assert_eq!(
        quotes(src, OBSIDIAN),
        vec![(None, callout_meta("a", "b|c", None), 0..src.len())],
    );

    // metadata combines with a fold marker
    let src = "> [!note|meta]- T\n";
    assert_eq!(
        quotes(src, OBSIDIAN),
        vec![(
            None,
            callout_meta("note", "meta", Some(CalloutFold::Folded)),
            0..src.len(),
        )],
    );

    // regex-derived (unprobed): [!|meta] has one+ bracket chars, so it
    // matches with an empty raw type
    let src = "> [!|meta] x\n";
    assert_eq!(
        quotes(src, OBSIDIAN),
        vec![(None, callout_meta("", "meta", None), 0..src.len())],
    );
}

#[test]
fn spaced_dash_is_title_not_fold() {
    // probes OQ6/K16: [!note] - x is a callout with title "- x", no fold
    let src = "> [!note] - spaced dash title\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q.len(), 1);
    let c = q[0].1.as_ref().unwrap();
    assert_eq!(c.kind.as_ref(), "note");
    assert_eq!(c.fold, None);
    // the "- spaced dash title" renders as a list item inside the quote
    let evs = events(src, OBSIDIAN);
    assert!(evs
        .iter()
        .any(|(ev, _)| matches!(ev, Event::Start(Tag::List(None)))));
}

#[test]
fn tab_after_fold_marker() {
    // probe K20: [!note]-\tTitle — tab satisfies the required whitespace
    let src = "> [!note]-\tTab after minus\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(
        q,
        vec![(
            None,
            callout("note", Some(CalloutFold::Folded)),
            0..src.len(),
        )],
    );
    assert_eq!(texts(src, OBSIDIAN), vec!["Tab after minus"]);
}

#[test]
fn crlf_line_endings() {
    // \r is a line ending, never bracket content or title text
    let src = "> [!note] Title\r\n> body\r\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("note", None), 0..src.len())]);
    assert_eq!(texts(src, OBSIDIAN), vec!["Title", "body"]);

    // bare head with CRLF: the tag consumes through the CRLF
    let src = "> [!note]\r\n> body\r\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, callout("note", None), 0..src.len())]);

    // fold marker at CRLF end-of-line
    let src = "> [!info]-\r\n> body\r\n";
    assert_eq!(
        quotes(src, OBSIDIAN),
        vec![(
            None,
            callout("info", Some(CalloutFold::Folded)),
            0..src.len(),
        )],
    );

    // an unclosed bracket cannot span a CR line ending
    let src = "> [!note\r\n> ]\r\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn canonical_kind_normalization() {
    // probe K06: [!NOTE] → "note"; trim + lowercase + ws-runs→dash
    let c = |k: &'static str| Callout {
        kind: k.into(),
        metadata: None,
        fold: None,
    };
    assert_eq!(c("NOTE").canonical_kind().as_ref(), "note"); // K06
    assert_eq!(c("my custom type").canonical_kind().as_ref(), "my-custom-type"); // K03
    assert_eq!(c(" note").canonical_kind().as_ref(), "note"); // K04
    assert_eq!(c("My  Custom\tType").canonical_kind().as_ref(), "my-custom-type");
    assert_eq!(c("注意").canonical_kind().as_ref(), "注意"); // OQ7
    assert_eq!(c("custom-type").canonical_kind().as_ref(), "custom-type");
    assert_eq!(c("").canonical_kind().as_ref(), "");
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
    // metadata is not a GFM alert
    assert_eq!(
        quotes("> [!NOTE|meta]\n> body\n", gfm),
        vec![(None, None, 0..22)],
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
    // metadata head: Obsidian callout but not a GFM alert, even though
    // the raw pre-| type is a GFM tag name
    assert_eq!(
        quotes("> [!note|meta]\n> b\n", both),
        vec![(None, callout_meta("note", "meta", None), 0..19)],
    );
}

#[test]
fn nested_callout_inside_blockquote() {
    // probe K14: the first-line rule applies per quote at any depth
    let src = "> outer\n> > [!info]\n> > nested callout inside\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q.len(), 2);
    assert_eq!(q[0], (None, None, 0..src.len()));
    assert_eq!(q[1].1, callout("info", None));
    // GT-v2 OQ-13 invariant: callout recognition must not alter
    // blockquote span semantics — the nested quote's span is the
    // vanilla nested-BQ range, starting at the innermost `>` of its
    // head line, identical to the same shape without the flag
    let plain = quotes(src, Options::empty());
    assert_eq!(q[1].2, plain[1].2);
    assert_eq!(q[1].2.start, src.find("> [!info]").unwrap());
    assert_eq!(q[1].2.end, src.len());
}

#[test]
fn cjk_title_and_body_spans_are_byte_exact() {
    let src = "> [!note] 标题一\n> 正文内容，第二行。\n";
    let q = quotes(src, OBSIDIAN);
    assert_eq!(q, vec![(None, callout("note", None), 0..src.len())]);
    assert_eq!(texts(src, OBSIDIAN), vec!["标题一", "正文内容，第二行。"]);
}

#[test]
fn second_line_tag_is_not_a_callout() {
    // probe K15: the callout head must be the first line of its quote
    let src = "> intro\n> [!note] not a head\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn leading_whitespace_before_tag_is_plain() {
    // conformance spec §2.1 [source]: the regex is anchored at the
    // first line's content after stripping `>` and at most one space,
    // so extra whitespace before [! kills the match (the lane pattern
    // `[ \t]*` was wrong)
    let src = ">   [!note] spaced\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn two_spaces_before_tag_is_plain() {
    // probe zzprobe-v3c-followup: the tokenizer strips `>` plus at most
    // ONE literal space, so a second space puts [! at offset 1 => plain
    let src = ">  [!note] two spaces\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
}

#[test]
fn tab_after_marker_is_plain() {
    // probe zzprobe-v3c-followup: the tokenizer never treats a tab as
    // the one stripped space, so `>\t[!note]` is a plain quote even
    // though CommonMark tab expansion lands the marker scan on `[`
    let src = ">\t[!note] tabbed\n";
    assert_eq!(quotes(src, OBSIDIAN), vec![(None, None, 0..src.len())]);
    // the head text survives as inline content, not a callout tag
    assert!(events(src, OBSIDIAN)
        .iter()
        .any(|(ev, r)| matches!(ev, Event::Text(_)) && &src[r.clone()] == "!note"));
}

#[test]
fn tab_after_marker_flag_off_unchanged() {
    // flags OFF must stay byte-identical: the obsidian flag makes no
    // difference on the tab edge, and plain-blockquote tab handling
    // (one column consumed as separator, rest is content indent) holds
    let src = ">\t[!note] tabbed\n";
    assert_eq!(events(src, Options::empty()), events(src, OBSIDIAN));
    assert_eq!(quotes(src, Options::empty()), vec![(None, None, 0..src.len())]);
}

#[test]
fn indented_marker_is_a_callout() {
    // probe zzprobe-v3c-followup: up to three spaces of indent before
    // `>` is fine — the strip rule applies to the quote content, not
    // the marker's own indentation
    let src = "   > [!note] indented\n";
    // the quote span starts at the `>` marker (upstream convention)
    assert_eq!(
        quotes(src, OBSIDIAN),
        vec![(None, callout("note", None), 3..src.len())],
    );
    assert_eq!(texts(src, OBSIDIAN), vec!["indented"]);
}
