use crate::color::PrimaryColor;

pub struct Pot {
  colors: [PrimaryColor; Self::POT_SIZE],
}

impl Pot {
  pub const POT_SIZE: usize = 2;
}

impl Default for Pot {
  fn default() -> Self {
    Self {
      colors: [PrimaryColor::Yellow, PrimaryColor::Yellow],
    }
  }
}
