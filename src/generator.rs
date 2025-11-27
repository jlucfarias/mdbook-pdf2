use anyhow::Error;
use serde::Deserialize;
use mdbook_renderer::{RenderContext, book::BookItem};

use crate::config::Config;

#[derive(Deserialize, Debug)]
pub struct Generator {
  pub context: RenderContext,
  pub options: Config,
  pub title: String,
}

impl Generator {
  pub fn new(ctx: RenderContext, options: Config) -> Self {
    let title = ctx.config.book.title.clone().unwrap_or(String::new());

    Self {
      context: ctx,
      options: options,
      title: title,
    }
  }

  pub fn build(self) -> Result<(), Error> {
    println!("Title {}", self.title);

    for item in self.context.book.items {
      if let BookItem::Chapter(ref ch) = item {
        println!("Chapter {}", ch.name);
      }
    }

    println!("Options {:?}", self.options);

    Ok(())
  }
}
