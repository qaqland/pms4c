use indoc::indoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

fn parse(input: &'_ str) -> Vec<Event<'_>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);
    parser.collect()
}

#[test]
fn simple_paragraph() {
    let input = indoc! {"
        Hello World
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("Hello World".into()),
        Event::End(TagEnd::Paragraph),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn simple_softbreak() {
    let input = indoc! {"
        Hello
        World
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("Hello".into()),
        Event::SoftBreak,
        Event::Text("World".into()),
        Event::End(TagEnd::Paragraph),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn simple_hardbreak() {
    let input = indoc! {"
        Hello  
        World
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("Hello".into()),
        Event::HardBreak,
        Event::Text("World".into()),
        Event::End(TagEnd::Paragraph),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn block_quote_1() {
    let input = indoc! {"
        > Hello
        World
    "};

    let events = vec![
        Event::Start(Tag::BlockQuote(None)),
        Event::Start(Tag::Paragraph),
        Event::Text("Hello".into()),
        Event::SoftBreak,
        Event::Text("World".into()),
        Event::End(TagEnd::Paragraph),
        Event::End(TagEnd::BlockQuote(None)),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn block_quote_2() {
    let input = indoc! {"
        > Hello  
        World
    "};

    let events = vec![
        Event::Start(Tag::BlockQuote(None)),
        Event::Start(Tag::Paragraph),
        Event::Text("Hello".into()),
        Event::HardBreak,
        Event::Text("World".into()),
        Event::End(TagEnd::Paragraph),
        Event::End(TagEnd::BlockQuote(None)),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn block_quote_3() {
    let input = indoc! {"
        > Hello

        World
    "};

    let events = vec![
        Event::Start(Tag::BlockQuote(None)),
        Event::Start(Tag::Paragraph),
        Event::Text("Hello".into()),
        Event::End(TagEnd::Paragraph),
        Event::End(TagEnd::BlockQuote(None)),
        Event::Start(Tag::Paragraph),
        Event::Text("World".into()),
        Event::End(TagEnd::Paragraph),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn code_block() {
    let input = indoc! {"
        ```bash
        Hello
        World
        ```
    "};

    let events = vec![
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced("bash".into()))),
        Event::Text("Hello\nWorld\n".into()),
        Event::End(TagEnd::CodeBlock),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn simple_inline_end() {
    let input = indoc! {"
        **Hello**
        World
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Start(Tag::Strong),
        Event::Text("Hello".into()),
        Event::End(TagEnd::Strong),
        Event::SoftBreak,
        Event::Text("World".into()),
        Event::End(TagEnd::Paragraph),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}

#[test]
fn simple_inline_start() {
    let input = indoc! {"
        Hello
        *World*
    "};

    let events = vec![
        Event::Start(Tag::Paragraph),
        Event::Text("Hello".into()),
        Event::SoftBreak,
        Event::Start(Tag::Emphasis),
        Event::Text("World".into()),
        Event::End(TagEnd::Emphasis),
        Event::End(TagEnd::Paragraph),
    ];

    let output = parse(input);

    assert_eq!(output, events);
}
