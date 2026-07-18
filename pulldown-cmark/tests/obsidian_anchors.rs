//! Obsidian block anchors (`^id` at a line tail), gated by
//! `Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS`.
//!
//! Every positive test asserts both the event payload (id without the caret)
//! and the byte span: slicing the source by the reported span must reproduce
//! the anchor text (`^id`, caret included) exactly.

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};

/// All `BlockAnchor` events with their byte spans, anchors flag on.
fn anchors(src: &str) -> Vec<(String, core::ops::Range<usize>)> {
    Parser::new_ext(src, Options::ENABLE_OBSIDIAN_BLOCK_ANCHORS)
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
fn anchor_at_list_item_tail() {
    assert_single_anchor("- item ^li\n", "li");
}

#[test]
fn anchor_at_blockquote_line_tail() {
    assert_single_anchor("> quoted ^bq\n", "bq");
}

#[test]
fn anchor_on_middle_line_of_paragraph() {
    // line tail, not paragraph tail
    assert_single_anchor("first ^mid\nsecond\n", "mid");
}

#[test]
fn crlf_anchor() {
    assert_single_anchor("line one ^crlf-anchor\r\nplain\r\n", "crlf-anchor");
}

#[test]
fn anchor_at_eof_without_newline() {
    assert_single_anchor("tail ^eof", "eof");
}

#[test]
fn anchor_with_trailing_whitespace() {
    assert_single_anchor("text ^ws \n", "ws");
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
