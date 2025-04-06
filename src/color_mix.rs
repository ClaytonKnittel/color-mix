use crate::{bank::Bank, color::Color, error::ColorMixResult, player::Player, pot::Pot};

#[derive(Clone, Copy)]
pub enum Action {
  PlayCube(Color),
  FinishTurn,
}

pub struct ColorMix {
  players: (Player, Player),
  pot: Pot,
  bank: Bank,
  p1_turn: bool,
}

impl ColorMix {
  pub fn new(initial_count: u32) -> Self {
    Self {
      players: (Player::default(), Player::default()),
      pot: Pot::default(),
      bank: Bank::new(initial_count),
      p1_turn: true,
    }
  }

  fn cur_player_mut(&mut self) -> &mut Player {
    if self.p1_turn {
      &mut self.players.0
    } else {
      &mut self.players.1
    }
  }

  fn trigger_action(&mut self, color: Color) -> ColorMixResult {
    todo!();
  }

  fn play_cube(&mut self, color: Color) -> ColorMixResult {
    let player = self.cur_player_mut();
    player.remove_cube(color)?;

    self.trigger_action(color)?;

    self.bank.deposit(color);
    Ok(())
  }

  fn draw_from_pot(&mut self) -> ColorMixResult {
    let pot_color = self.pot.color();

    if self.bank.maybe_withdraw(pot_color) {
      let player = self.cur_player_mut();
      player.add_cube(pot_color)
    } else {
      Ok(())
    }
  }

  fn finish_turn(&mut self) -> ColorMixResult {
    self.p1_turn = !self.p1_turn;
    self.draw_from_pot()
  }

  pub fn do_action(&mut self, action: Action) -> ColorMixResult {
    match action {
      Action::PlayCube(color) => self.play_cube(color),
      Action::FinishTurn => self.finish_turn(),
    }
  }
}
