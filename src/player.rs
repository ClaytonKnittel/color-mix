use crate::{color::Color, cube_set::CubeSet, error::ColorMixResult};

pub struct Player {
  cubes: CubeSet,
  hp: u32,
}

impl Player {
  pub const STARTING_HP: u32 = 10;

  pub fn add_cube(&mut self, color: Color) -> ColorMixResult {
    self.cubes.insert(color);
    Ok(())
  }

  pub fn remove_cube(&mut self, color: Color) -> ColorMixResult {
    self.cubes.remove(color)
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
