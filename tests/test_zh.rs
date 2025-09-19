use indoc::indoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use pms4c::pms4c;

#[test]
fn common_common() {
    let input = indoc! {"
        大家
        早上好
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("大家".into()),
        Event::Text("早上好".into()),
        Event::End(TagEnd::Paragraph),
    ];

    assert_eq!(events, pms4c(input));
}

#[test]
fn strong_common() {
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
fn common_emphasis() {
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
fn blockquote_1() {
    let input = indoc! {"
        > 大家
        > 早上好
    "};

    let events = vec![
        Event::Start(Tag::BlockQuote(None)),
        Event::Start(Tag::Paragraph),
        Event::Text("大家".into()),
        Event::Text("早上好".into()),
        Event::End(TagEnd::Paragraph),
        Event::End(TagEnd::BlockQuote(None)),
    ];

    assert_eq!(events, pms4c(input));
}

#[test]
fn codeblock_1() {
    let input = indoc! {"
        ```
        大家
        早上好
        ```
    "};
    assert_eq!(input, "```\n大家\n早上好\n```\n");
}

#[test]
fn en_common_common() {
    let input = indoc! {"
        Hello
        大家早上好
    "};
}

#[test]
fn en_strong_common() {
    let input = indoc! {"
        **Hello**
        大家早上好
    "};
}
