use itertools::Itertools;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

mod cjk;

pub fn pms4c(input: &'_ str) -> Vec<Event<'_>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);

    let mut events = Vec::new();
    let mut iter = parser.multipeek();
    let mut ready = false;

    while let Some(event) = iter.next() {
        match &event {
            Event::Start(Tag::Paragraph) => ready = true,
            Event::End(TagEnd::Paragraph) => ready = true,
            Event::Text(text) => {
                ready = cjk::ends_with_cjk(&text);
            }
            Event::SoftBreak => {
                if ready {
                    continue;
                }
            }
            Event::End(TagEnd::Emphasis) | Event::End(TagEnd::Strong) => (),
            _ => ready = false,
        };

        events.push(event);
    }

    events
}
