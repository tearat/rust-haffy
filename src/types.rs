#[derive(Debug)]
#[derive(PartialEq)]
pub struct Char {
  pub title: String,
  pub count: u32,
  pub code: String
}

pub type Alphabet = Vec<Char>;