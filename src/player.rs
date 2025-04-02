use crate::cube_set::CubeSet;

pub struct Player {
  cubes: CubeSet,
  hp: u32,
}

impl Player {
  pub const STARTING_HP: u32 = 10;
}

impl Default for Player {
  fn default() -> Self {
    Self {
      cubes: CubeSet::default(),
      hp: Self::STARTING_HP,
    }
  }
}
