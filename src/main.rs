mod config;
mod generator;

use anyhow::Error;
use std::io;
use mdbook_core::utils;
use mdbook_renderer::RenderContext;

use crate::{generator::Generator, config::Config};

fn main() {
  let mut stdin = io::stdin();
  let ctx = RenderContext::from_json(&mut stdin).unwrap();

  if !ctx.config.contains_key("output.html") {
    let err = Error::msg("Could not find the HTML backend. Please make sure the HTML backend is enabled.");
    utils::log_backtrace(&err);

    return
  }

  let options = match ctx.config.get("output.pdf2") {
    Ok(t) => Config::read(t),
    Err(e) => {
      let err = Error::msg(format!("Could not parse config file: {}", e));
      utils::log_backtrace(&err);

      return
    }
  };

  if let Err(err) = Generator::new(ctx, options).build() {
    utils::log_backtrace(&err);
  };
}
