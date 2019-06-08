use crate::geometry::{Direction, Position};

#[derive(Debug, PartialEq, Eq)]
pub enum BombState {
  Armed,
  Exploding { left: i32, right: i32, up: i32, down: i32 },
  Dead
}

#[derive(Debug)]
pub struct Bomb {
  pub player_id: i32,
  pub position: (f64, f64),
  pub power: i32,
  pub state: BombState,
  pub direction: Option<Direction>,
  pub timer: i32,
}

impl Bomb {
  pub fn new(player_id: i32, power: i32, position: (f64, f64)) -> Self {
    Bomb {
      player_id: player_id,
      position: position,
      power: power,
      state: BombState::Armed,
      direction: None,
      timer: 0,
    }
  }
}

impl Position for Bomb {
  fn position(&self) -> (f64, f64) {
    self.position
  }
  fn set_position(&mut self, x: f64, y: f64) {
    self.position = (x, y);
  }
}
