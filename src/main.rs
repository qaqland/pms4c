//! This is a demonstration of an mdBook preprocessor which parses markdown
//! and removes any instances of emphasis.

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use pulldown_cmark::{Options, Parser};
use std::io;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

struct RemoveSoftBreak;

impl Preprocessor for RemoveSoftBreak {
    fn name(&self) -> &str {
        "remove-softbreak"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        book.for_each_mut(|book_item| {
            if let BookItem::Chapter(chapter) = book_item {
                match remove_softbreak(chapter) {
                    Ok(content) => chapter.content = content,
                    Err(e) => {
                        eprintln!("Error processing chapter '{}': {}", chapter.name, e);
                    }
                }
            }
        });
        Ok(book)
    }
}

// ANCHOR: remove_softbreak
fn remove_softbreak(chapter: &mut Chapter) -> Result<String> {
    let mut buf = String::with_capacity(chapter.content.len());
    let options = Options::all();
    let parser = Parser::new_ext(&chapter.content, options);

    let events = pms4c::pms4c(parser);
    pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut buf)?;

    Ok(buf)
}
// ANCHOR_END: remove_softbreak

pub fn handle_preprocessing() -> Result<()> {
    let pre = RemoveSoftBreak;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
