use crate::color::Color;

pub struct Pot {
  colors: [Color; Self::POT_SIZE],
}

impl Pot {
  pub const POT_SIZE: usize = 2;
}

impl Default for Pot {
  fn default() -> Self {
    Self { colors: [Color::Yellow, Color::Yellow] }
  }
}
