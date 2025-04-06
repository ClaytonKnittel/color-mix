use crate::color::{Color, PrimaryColor};

pub struct Pot {
  colors: [PrimaryColor; Self::POT_SIZE],
}

impl Pot {
  pub const POT_SIZE: usize = 2;

  pub fn color(&self) -> Color {
    self.colors[0].mix(self.colors[1])
  }
}

impl Default for Pot {
  fn default() -> Self {
    Self {
      colors: [PrimaryColor::Yellow, PrimaryColor::Yellow],
    }
  }
}
