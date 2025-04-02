use std::ops::Index;

use crate::color::Color;

pub struct CubeSet {
  count: [u32; Color::NUM_COLORS],
}

impl Default for CubeSet {
  fn default() -> Self {
    Self { count: [0; Color::NUM_COLORS] }
  }
}

impl Index<Color> for CubeSet {
  type Output = u32;

  fn index(&self, index: Color) -> &u32 {
    &self.count[index.to_idx()]
  }
}
