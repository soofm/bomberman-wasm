use crate::geometry::{Direction, Position};

pub const BOMB_TIMER: i32 = 180;

#[derive(Debug)]
pub struct Bomb {
  pub player_id: usize,
  pub x: f64,
  pub y: f64,
  pub power: i32,
  pub direction: Option<Direction>,
  pub timer: i32,
}

impl Bomb {
  pub fn new(player_id: usize, power: i32, x: f64, y: f64) -> Self {
    Bomb {
      player_id: player_id,
      x: x,
      y: y,
      power: power,
      direction: None,
      timer: BOMB_TIMER,
    }
  }
}

impl Position for Bomb {
  fn position(&self) -> (f64, f64) {
    (self.x, self.y)
  }
  fn set_position(&mut self, x: f64, y: f64) {
    self.x = x;
    self.y = y;
  }
}
