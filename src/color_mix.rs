use crate::{bank::Bank, player::Player, pot::Pot};

#[derive(Default)]
pub struct ColorMix {
  players: (Player, Player),
  pot: Pot,
  bank: Bank,
}

impl ColorMix {
  pub fn new() -> Self {
    Self::default()
  }
}
