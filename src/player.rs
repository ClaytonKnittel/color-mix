use std::fmt::Display;

use crate::{
  color::Color,
  cube_set::CubeSet,
  error::{ColorMixError, ColorMixResult},
};

pub struct Player {
  cubes: CubeSet,
  ephemeral_cubes: CubeSet,
  hp: u32,
}

impl Player {
  pub const STARTING_HP: u32 = 5;

  pub fn add_cube(&mut self, color: Color) {
    self.cubes.insert(color);
  }

  /// Returns true if the cube was real and should be removed from the bank.
  pub fn remove_cube(&mut self, color: Color) -> ColorMixResult<bool> {
    if self.ephemeral_cubes.capacity(color) > 0 {
      self.ephemeral_cubes.remove(color)?;
      Ok(false)
    } else {
      self.cubes.remove(color)?;
      Ok(true)
    }
  }

  pub fn double_cubes(&mut self, color: Color) {
    let total_cubes = self.cubes[color] + self.ephemeral_cubes[color];
    self.ephemeral_cubes.insert_n(color, total_cubes);
  }

  pub fn clear_ephemeral(&mut self) {
    self.ephemeral_cubes.clear();
  }

  pub fn damage(&mut self) {
    self.hp -= 1;
  }

  pub fn damage_n(&mut self, count: u32) {
    self.hp = self.hp.saturating_sub(count);
  }

  pub fn heal(&mut self) -> ColorMixResult {
    if self.hp < Self::STARTING_HP {
      self.hp += 1;
      Ok(())
    } else {
      Err(
        ColorMixError::InvalidAction(format!(
          "Cannot heal above max health: {}",
          Self::STARTING_HP
        ))
        .into(),
      )
    }
  }

  pub fn is_dead(&self) -> bool {
    self.hp == 0
  }
}

impl Default for Player {
  fn default() -> Self {
    Self {
      cubes: CubeSet::default(),
      ephemeral_cubes: CubeSet::default(),
      hp: Self::STARTING_HP,
    }
  }
}

impl Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Hp: {} - cubes: {}", self.hp, self.cubes)?;
    if !self.ephemeral_cubes.empty() {
      write!(f, " - ephemeral: {}", self.ephemeral_cubes)?;
    }
    Ok(())
  }
}
