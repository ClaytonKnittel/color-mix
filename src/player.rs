use std::fmt::Display;

use crate::{color::Color, cube_set::CubeSet, error::ColorMixResult};

pub struct Player {
  cubes: CubeSet,
  hp: u32,
}

impl Player {
  pub const STARTING_HP: u32 = 10;

  pub fn add_cube(&mut self, color: Color) {
    self.cubes.insert(color);
  }

  pub fn remove_cube(&mut self, color: Color) -> ColorMixResult {
    self.cubes.remove(color)
  }

  pub fn damage(&mut self) {
    self.hp -= 1;
  }

  pub fn heal(&mut self) {
    self.hp += 1;
  }

  pub fn is_dead(&self) -> bool {
    self.hp == 0
  }
}

impl Default for Player {
  fn default() -> Self {
    Self {
      cubes: CubeSet::default(),
      hp: Self::STARTING_HP,
    }
  }
}

impl Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Hp: {} - cubes: {}", self.hp, self.cubes)
  }
}
