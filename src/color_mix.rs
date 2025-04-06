use std::fmt::Display;

use crate::{
  bank::Bank,
  color::{Color, PrimaryColor},
  error::ColorMixResult,
  player::Player,
  pot::Pot,
};

#[derive(Clone, Copy, Debug)]
pub enum PlayCube {
  Red,
  Yellow { color: PrimaryColor },
  Blue,
  Orange { double_color: Color },
  Purple { steal_color: Color },
  Green,
}

impl PlayCube {
  pub fn color(&self) -> Color {
    match self {
      Self::Red => Color::Red,
      Self::Yellow { .. } => Color::Yellow,
      Self::Blue => Color::Blue,
      Self::Orange { .. } => Color::Orange,
      Self::Purple { .. } => Color::Purple,
      Self::Green => Color::Green,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
  PlayCube(PlayCube),
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

  fn other_player_mut(&mut self) -> &mut Player {
    if self.p1_turn {
      &mut self.players.1
    } else {
      &mut self.players.0
    }
  }

  fn trigger_action(&mut self, play_cube: PlayCube) -> ColorMixResult {
    match play_cube {
      PlayCube::Red => {
        let other_player = self.other_player_mut();
        other_player.damage();
      }
      PlayCube::Yellow { color } => {
        self.pot.rotate_color(color);
      }
      PlayCube::Blue => {
        let player = self.cur_player_mut();
        player.heal();
      }
      PlayCube::Orange { double_color } => {
        todo!();
      }
      PlayCube::Purple { steal_color } => {
        let other_player = self.other_player_mut();
        other_player.remove_cube(steal_color)?;
        let player = self.cur_player_mut();
        player.add_cube(steal_color);
      }
      PlayCube::Green => {
        self.pot.swap_inverted_state();
      }
    }
    Ok(())
  }

  fn play_cube(&mut self, play_cube: PlayCube) -> ColorMixResult {
    let player = self.cur_player_mut();
    let color = play_cube.color();
    player.remove_cube(color)?;

    self.trigger_action(play_cube)?;

    self.bank.deposit(color);
    Ok(())
  }

  fn draw_from_pot(&mut self) -> ColorMixResult {
    let pot_color = self.pot.color();

    if self.bank.maybe_withdraw(pot_color) {
      let player = self.cur_player_mut();
      player.add_cube(pot_color);
    }

    Ok(())
  }

  fn finish_turn(&mut self) -> ColorMixResult {
    self.p1_turn = !self.p1_turn;
    self.draw_from_pot()
  }

  pub fn do_action(&mut self, action: Action) -> ColorMixResult {
    match action {
      Action::PlayCube(play_cube) => self.play_cube(play_cube),
      Action::FinishTurn => self.finish_turn(),
    }
  }

  pub fn finished(&self) -> bool {
    self.players.0.is_dead() || self.players.1.is_dead()
  }

  pub fn p1_wins(&self) -> bool {
    debug_assert!(self.finished());
    self.players.1.is_dead()
  }
}
