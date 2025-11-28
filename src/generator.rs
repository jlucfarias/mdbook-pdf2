use anyhow::Error;
use serde::Deserialize;
use mdbook_renderer::{RenderContext, book::BookItem};
use printpdf::*;

use crate::{config::Config, document::Document};

#[derive(Deserialize, Debug)]
pub struct Generator {
  pub context: RenderContext,
  pub options: Config,
  pub document: Document,
}

impl Generator {
  pub fn new(ctx: RenderContext, options: Config) -> Self {
    let title = ctx.config.book.title.clone().unwrap_or(String::new());
    let document = Document::new(title);

    Self {
      context: ctx,
      options: options,
      document: document,
    }
  }

  pub fn build(mut self) -> Result<(), Error> {
    for item in self.context.book.items {
      if let BookItem::Chapter(ref ch) = item {
        println!("Chapter {} {}", ch.name, ch.content);
      }
    }

    println!("Options {:?}", self.options);

    let page1_contents = vec![Op::Marker { id: "debugging-marker".to_string() }];
    let page_size = self.options.page.size.size(self.options.page.landscape);
    let page1 = PdfPage::new(Mm(page_size.0), Mm(page_size.1), page1_contents);
    self.document.add_page(page1);

    let bytes = self.document.export();
    std::fs::write( format!("{}.pdf", self.document.title), &bytes).unwrap();

    Ok(())
  }
}
