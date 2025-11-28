use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TomlConfig {
  pub landscape: bool,
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default = "FontSize::default")]
  pub font_size: FontSize,
  #[serde(default = "PageOptions::default")]
	pub page: PageOptions,
}

#[derive(Deserialize, Debug)]
pub struct FontSize {
	#[serde(default = "default_title")]
	pub title: u8,
	#[serde(default = "default_h1")]
	pub h1: u8,
	#[serde(default = "default_h2")]
	pub h2: u8,
	#[serde(default = "default_h3")]
	pub h3: u8,
	#[serde(default = "default_h4")]
	pub h4: u8,
	#[serde(default = "default_h5")]
	pub h5: u8,
	#[serde(default = "default_h6")]
	pub h6: u8,
	#[serde(default = "default_text")]
	pub text: u8,
}

#[derive(Deserialize, Debug)]
pub struct PageOptions {
	#[serde(default = "PageSize::default")]
	pub size: PageSize,
	#[serde(default = "default_landscape")]
	pub landscape: bool,
	#[serde(default = "default_margin")]
	pub margin: (f32, f32),
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PageSize {
	A4,
}

fn default_title() -> u8 { 25 }
fn default_h1() -> u8 { 22 }
fn default_h2() -> u8 { 20 }
fn default_h3() -> u8 { 17 }
fn default_h4() -> u8 { 14 }
fn default_h5() -> u8 { 12 }
fn default_h6() -> u8 { 12 }
fn default_text() -> u8 { 10 }
fn default_margin() -> (f32, f32) { (20.0, 20.0) }
fn default_landscape() -> bool { false }

impl Config {
  pub fn read(_ctx: Option<TomlConfig>) -> Self {
    Self::default()
  }
}

impl PageSize {
  pub fn size(&self, landscape: bool) -> (f32, f32) {
    let (x, y) = match self {
      PageSize::A4 => (210.0, 297.0),
    };

    if landscape {
      (y, x)
    } else {
      (x, y)
    }
  }
}

impl FontSize {
  pub fn get(&self, section: &str) -> u8 {
    match section {
      "h1" => self.h1,
      "h2" => self.h2,
      "h3" => self.h3,
      "h4" => self.h4,
      "h5" => self.h5,
      "h6" => self.h6,
      _ => self.text,
    }
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      font_size: Default::default(),
      page: Default::default(),
    }
  }
}

impl Default for FontSize {
  fn default() -> Self {
    Self {
      title: default_title(),
      h1: default_h1(),
      h2: default_h2(),
      h3: default_h3(),
      h4: default_h4(),
      h5: default_h5(),
      h6: default_h6(),
      text: default_text(),
    }
  }
}

impl Default for PageSize {
  fn default() -> Self { PageSize::A4 }
}

impl Default for PageOptions {
  fn default() -> Self {
    Self {
      size: Default::default(),
      landscape: default_landscape(),
      margin: default_margin(),
    }
  }
}
