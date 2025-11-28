use serde::Deserialize;
use printpdf::*;

#[derive(Deserialize, Debug)]
pub struct Document {
  pub title: String,
  _internal: PdfDocument,
  pages: Vec<PdfPage>,
}

fn default_title() -> String { "Title".to_string() }
fn default_pages() -> Vec<PdfPage> { vec![] }

impl Document {
  pub fn new(title: String) -> Self {
    let doc = PdfDocument::new(&title);

    Self {
      title: title,
      _internal: doc,
      pages: Default::default(),
    }
  }

  pub fn add_page(&mut self, page: PdfPage) {
    self.pages.push(page);
  }

  // let page = PdfPage::new(Mm(210.0), Mm(297.0), ops);
  /* let ops = vec![
      Op::SaveGraphicsState,
      Op::StartTextSection,
      Op::SetTextCursor { pos: Point::new(Mm(20.0), Mm(270.0)) },
      Op::SetFontSizeBuiltinFont { size: Pt(24.0), font: BuiltinFont::Helvetica },
      Op::SetLineHeight { lh: Pt(24.0) },
      Op::SetFillColor { col: Color::Rgb(Rgb {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        icc_profile: None,
      })},
      Op::AddLineBreak,
      Op::EndTextSection,
      Op::RestoreGraphicsState,
    ]; */

  pub fn export(&mut self) -> Vec<u8> {
    self._internal
      .clone()
      .with_pages(self.pages.clone())
      .save(&PdfSaveOptions::default(), &mut Vec::new())
  }
}

impl Default for Document {
  fn default() -> Self {
    Self {
      title: default_title(),
      _internal: PdfDocument::new(&default_title()),
      pages: default_pages(),
    }
  }
}
