//! Obsidian block anchors (`^id` at a line tail), gated by
//! `Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS`.
//!
//! Semantics follow probe-confirmed Obsidian 1.12.7 behavior (live
//! metadataCache probes, session file `probes/zzprobe-anchors-a.md`; CRLF law
//! from `probes/zzprobe-crlf-fm.md`):
//! - anchors bind to the end of a PARAGRAPH's inline text, not to interior
//!   line tails (probe ^a07), including the lazy `text\n^id` form;
//! - trailing whitespace between the id and the line ending invalidates a
//!   paragraph anchor — the blockid regex `$` sits right after the id
//!   (probe ^a02); a CR that is part of a CRLF line ending is NOT trailing
//!   whitespace;
//! - table-cell tail anchors fire — the inline tokenizer runs per cell on
//!   trimmed cell text (probe ^a15).
//!
//! Every positive test asserts both the event payload (id without the caret)
//! and the byte span: slicing the source by the reported span must reproduce
//! the anchor text (`^id`, caret included) exactly.

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};

/// All `BlockAnchor` events with their byte spans, anchors flag on.
fn anchors(src: &str) -> Vec<(String, core::ops::Range<usize>)> {
    anchors_opts(src, Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS)
}

/// All `BlockAnchor` events with their byte spans, explicit options.
fn anchors_opts(src: &str, opts: Options) -> Vec<(String, core::ops::Range<usize>)> {
    Parser::new_ext(src, opts)
        .into_offset_iter()
        .filter_map(|(ev, range)| match ev {
            Event::BlockAnchor(id) => Some((id.to_string(), range)),
            _ => None,
        })
        .collect()
}

/// Asserts exactly one anchor with the given id whose span slices back to `^id`.
fn assert_single_anchor(src: &str, id: &str) {
    let found = anchors(src);
    assert_eq!(found.len(), 1, "expected one anchor in {:?}, got {:?}", src, found);
    let (got_id, span) = &found[0];
    assert_eq!(got_id, id, "wrong id in {:?}", src);
    assert_eq!(
        &src[span.clone()],
        format!("^{}", id),
        "span {:?} does not slice back to the anchor in {:?}",
        span,
        src
    );
}

#[test]
fn plain_line_tail_anchor() {
    // anchor as the entire line
    assert_single_anchor("^abc\n", "abc");
}

#[test]
fn anchor_after_text() {
    assert_single_anchor("some text ^ref-1\n", "ref-1");
}

#[test]
fn anchor_event_sequence_and_span() {
    let src = "para ^anchor1\n";
    let events: Vec<_> = Parser::new_ext(src, Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS)
        .into_offset_iter()
        .collect();
    let expected = [
        (Event::Start(Tag::Paragraph), 0..14),
        (Event::Text(CowStr::Borrowed("para ")), 0..5),
        (Event::BlockAnchor(CowStr::Borrowed("anchor1")), 5..13),
        (Event::End(TagEnd::Paragraph), 0..14),
    ];
    assert_eq!(&events, &expected);
    assert_eq!(&src[5..13], "^anchor1");
}

#[test]
fn no_anchor_inside_fenced_code() {
    assert_eq!(anchors("```\ncode ^nope\n```\n"), vec![]);
}

#[test]
fn no_anchor_inside_indented_code() {
    assert_eq!(anchors("    code ^nope\n"), vec![]);
}

#[test]
fn no_anchor_inside_inline_code() {
    // single-line code span: the tail of the line is the closing backtick
    assert_eq!(anchors("a `x ^nope` b\n"), vec![]);
    // multi-line code span: `^nope` IS at a line tail, but inside the span —
    // the anchor item must be detached with the code-span interior
    assert_eq!(anchors("a `x ^nope\ny` b\n"), vec![]);
}

#[test]
fn no_anchor_with_flag_off() {
    let src = "para ^anchor1\n";
    let events: Vec<_> = Parser::new(src).collect();
    assert!(
        !events.iter().any(|e| matches!(e, Event::BlockAnchor(_))),
        "anchor fired with flag off"
    );
    // upstream behavior byte-identical: the marker stays plain text
    assert!(events.contains(&Event::Text(CowStr::Borrowed("para ^anchor1"))));
}

#[test]
fn anchor_at_heading_tail() {
    let src = "# Title ^h1\n";
    assert_single_anchor(src, "h1");
    // and it is inside the heading, not a stray paragraph
    let events: Vec<_> = Parser::new_ext(src, Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS).collect();
    let anchor_pos = events
        .iter()
        .position(|e| matches!(e, Event::BlockAnchor(_)))
        .unwrap();
    assert!(matches!(events[anchor_pos - 2], Event::Start(Tag::Heading { .. })));
}

#[test]
fn anchor_at_heading_tail_with_trailing_whitespace() {
    // NOT probe-covered: heading text is trimmed by ATX parsing before the
    // tail check, so the paragraph trailing-whitespace law (probe ^a02) does
    // not apply here. Locks current behavior; flagged for a GT-v2 probe.
    assert_single_anchor("# Title ^h2 \n", "h2");
}

#[test]
fn anchor_at_list_item_tail() {
    assert_single_anchor("- item ^li\n", "li");
}

#[test]
fn anchor_at_blockquote_line_tail() {
    assert_single_anchor("> quoted ^bq\n", "bq");
}

// --- paragraph-tail law (probe ^a07): interior line tails never fire -------

#[test]
fn no_anchor_on_interior_paragraph_line() {
    // probe ^a07: "P07 line one\nline two ^a07\nline three" registers NO
    // block — the blockid regex anchors at the end of the paragraph's inline
    // text, not per-line
    assert_eq!(anchors("P07 line one\nline two ^a07\nline three\n"), vec![]);
    // two-line form: anchor on the first of two lines is interior
    assert_eq!(anchors("first ^mid\nsecond\n"), vec![]);
}

#[test]
fn interior_anchor_demotes_to_literal_text() {
    // the would-be anchor survives as literal paragraph text
    let src = "first ^mid\nsecond\n";
    let events: Vec<_> = Parser::new_ext(src, Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS)
        .into_offset_iter()
        .collect();
    let expected = [
        (Event::Start(Tag::Paragraph), 0..18),
        (Event::Text(CowStr::Borrowed("first ")), 0..6),
        (Event::Text(CowStr::Borrowed("^mid")), 6..10),
        (Event::SoftBreak, 10..11),
        (Event::Text(CowStr::Borrowed("second")), 11..17),
        (Event::End(TagEnd::Paragraph), 0..18),
    ];
    assert_eq!(&events, &expected);
}

#[test]
fn anchor_at_paragraph_last_line() {
    // the paragraph's LAST line tail fires
    assert_single_anchor("line one\nline two ^last\n", "last");
    assert_single_anchor("line one\nline two ^last2\n\nnext para\n", "last2");
}

#[test]
fn lazy_anchor_line_fires() {
    // probe ^a07 lazy form: "text\n^id" with no blank line — the anchor line
    // is a lazy continuation of the paragraph and the anchor fires
    let src = "text\n^a09\n";
    assert_single_anchor(src, "a09");
    let events: Vec<_> = Parser::new_ext(src, Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS)
        .into_offset_iter()
        .collect();
    let expected = [
        (Event::Start(Tag::Paragraph), 0..10),
        (Event::Text(CowStr::Borrowed("text")), 0..4),
        (Event::SoftBreak, 4..5),
        (Event::BlockAnchor(CowStr::Borrowed("a09")), 5..9),
        (Event::End(TagEnd::Paragraph), 0..10),
    ];
    assert_eq!(&events, &expected);
}

// --- trailing-whitespace law (probe ^a02): `$` sits right after the id -----

#[test]
fn no_anchor_with_trailing_whitespace() {
    // probe ^a02: "^a02 " (space before newline) registers NO block
    assert_eq!(anchors("^a02 \n"), vec![]);
    assert_eq!(anchors("text ^ws \n"), vec![]);
    // tab counts as trailing whitespace too
    assert_eq!(anchors("text ^ws\t\n"), vec![]);
    // same law at EOF without a newline
    assert_eq!(anchors("tail ^eof "), vec![]);
    // hard-break trailing spaces are still trailing whitespace
    assert_eq!(anchors("text ^ws  \nmore\n"), vec![]);
}

// --- CRLF law (probe file zzprobe-crlf-fm.md) ------------------------------

#[test]
fn crlf_anchor_at_paragraph_tail() {
    // CR of a CRLF line ending is part of the LINE ENDING, not trailing
    // whitespace: "^id\r\n" fires
    assert_single_anchor("tail ^crlf1\r\n", "crlf1");
    // multi-line CRLF paragraph, anchor on the last line
    assert_single_anchor("one\r\ntwo ^crlf2\r\n", "crlf2");
}

#[test]
fn no_crlf_anchor_with_trailing_space_or_interior_line() {
    // "^id \r\n" (space before CRLF) does not fire
    assert_eq!(anchors("tail ^crlf3 \r\n"), vec![]);
    // interior line tail with CRLF does not fire
    assert_eq!(anchors("one ^crlf4\r\ntwo\r\n"), vec![]);
}

// --- table-cell law (probe ^a15): cell tails fire on trimmed cell text -----

#[test]
fn anchor_at_table_cell_tail() {
    // probe ^a15: "| v1 ^a15 | v2 |" registers block a15 — the inline
    // tokenizer runs per cell; whitespace before the closing pipe is cell
    // padding, trimmed before the tail check
    let src = "| h1 | h2 |\n| --- | --- |\n| v1 ^a15 | v2 |\n";
    let opts = Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS | Options::ENABLE_TABLES;
    let found = anchors_opts(src, opts);
    assert_eq!(found.len(), 1, "expected one anchor, got {:?}", found);
    let (id, span) = &found[0];
    assert_eq!(id, "a15");
    assert_eq!(&src[span.clone()], "^a15");
}

#[test]
fn anchor_at_last_table_cell_without_closing_pipe() {
    let src = "| h1 | h2 |\n| --- | --- |\n| v1 | v2 ^cell-x\n";
    let opts = Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS | Options::ENABLE_TABLES;
    let found = anchors_opts(src, opts);
    assert_eq!(found.len(), 1, "expected one anchor, got {:?}", found);
    let (id, span) = &found[0];
    assert_eq!(id, "cell-x");
    assert_eq!(&src[span.clone()], "^cell-x");
}

// ---------------------------------------------------------------------------

#[test]
fn anchor_at_eof_without_newline() {
    assert_single_anchor("tail ^eof", "eof");
}

#[test]
fn no_anchor_mid_line() {
    assert_eq!(anchors("mid ^not anchor\n"), vec![]);
}

#[test]
fn no_anchor_non_ascii_id() {
    assert_eq!(anchors("uni ^ünïcode\n"), vec![]);
}

#[test]
fn no_anchor_without_separator() {
    // caret glued to preceding text
    assert_eq!(anchors("text^nope\n"), vec![]);
}

#[test]
fn no_anchor_for_escaped_caret() {
    assert_eq!(anchors("text \\^nope\n"), vec![]);
}

#[test]
fn no_anchor_for_bare_caret() {
    // caret with no id
    assert_eq!(anchors("text ^\n"), vec![]);
}
