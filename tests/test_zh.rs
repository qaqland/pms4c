use indoc::indoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use pms4c::pms4c;

#[test]
fn simple_paragraph() {
    let input = indoc! {"
        大家
        早上好
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("大家早上好".into()),
        Event::End(TagEnd::Paragraph),
    ];

    assert_eq!(events, pms4c(input));
}

#[test]
fn simple_softbreak() {
    let input = indoc! {"
        大**家**
        早上好
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("大".into()),
        Event::Start(Tag::Strong),
        Event::Text("家".into()),
        Event::End(TagEnd::Strong),
        Event::Text("早上好".into()),
        Event::End(TagEnd::Paragraph),
    ];

    assert_eq!(events, pms4c(input));
}

#[test]
fn simple_hardbreak() {
    let input = indoc! {"
        大家
        *早上*好
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("大家".into()),
        Event::Start(Tag::Emphasis),
        Event::Text("早上".into()),
        Event::End(TagEnd::Emphasis),
        Event::Text("好".into()),
        Event::End(TagEnd::Paragraph),
    ];

    assert_eq!(events, pms4c(input));
}

#[test]
fn block_quote_1() {
    let input = indoc! {"
        > 大家
        > 早上好
    "};
    assert_eq!(input, ">\n>大家早上好");
}

#[test]
fn block_quote_2() {
    let input = indoc! {"
        ```
        大家
        早上好
        ```
    "};
    assert_eq!(input, "```\n大家\n早上好\n```\n");
}

#[test]
fn code_block() {
    let input = indoc! {"
        Hello
        大家早上好
    "};
}

#[test]
fn simple_inline_end() {
    let input = indoc! {"
        **Hello**
        大家早上好
    "};
}
