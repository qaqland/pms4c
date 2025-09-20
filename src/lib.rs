use itertools::{Itertools, MultiPeek};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

mod cjk;

pub fn pms4c(parser: Parser) -> Vec<Event> {
    let mut events = Vec::new();
    let mut iter = parser.multipeek();
    let mut ready = false;

    while let Some(event) = iter.next() {
        match &event {
            Event::End(TagEnd::Paragraph) => ready = false,
            Event::Text(text) => {
                ready = cjk::ends_with_cjk(&text);
            }
            Event::SoftBreak => {
                if ready && next_text_start_with_cjk(&mut iter) {
                    // skip this event to remove space
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

fn next_text_start_with_cjk(iter: &mut MultiPeek<Parser<'_>>) -> bool {
    while let Some(event) = iter.peek() {
        match event {
            Event::Text(text) => return cjk::starts_with_cjk(text),
            Event::Start(Tag::Strikethrough)
            | Event::End(TagEnd::Strikethrough)
            | Event::Start(Tag::Strong)
            | Event::End(TagEnd::Strong)
            | Event::Start(Tag::Emphasis)
            | Event::End(TagEnd::Emphasis) => {
                continue;
            }
            _ => return false,
        }
    }
    false
}
