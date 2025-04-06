use std::fmt::Display;

use crate::color::{Color, PrimaryColor};

pub struct Pot {
  colors: [PrimaryColor; Self::POT_SIZE],
  inverted: bool,
}

impl Pot {
  pub const POT_SIZE: usize = 2;

  pub fn color(&self) -> Color {
    let color = self.colors[0].mix(self.colors[1]);
    if self.inverted {
      color.opposite()
    } else {
      color
    }
  }

  pub fn rotate_color(&mut self, color: PrimaryColor) {
    self.colors = [color, self.colors[0]]
  }

  pub fn swap_inverted_state(&mut self) {
    self.inverted = !self.inverted;
  }
}

impl Default for Pot {
  fn default() -> Self {
    Self {
      colors: [PrimaryColor::Yellow, PrimaryColor::Yellow],
      inverted: false,
    }
  }
}

impl Display for Pot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}: [{}, {}]{}",
      self.color(),
      self.colors[0],
      self.colors[1],
      if self.inverted { " (inv)" } else { "" }
    )
  }
}
