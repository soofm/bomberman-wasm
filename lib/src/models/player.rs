use super::PowerupType;
use crate::geometry::Position;

const MAX_BOMBS: i32 = 9;
const MAX_POWER: i32 = 15;
const MAX_SPEED: i32 = 15;

pub struct Player {
  pub id: i32,
  pub position: (f64, f64),
  pub is_alive: bool,
  pub speed: i32,
  pub bomb_number: i32,
  pub bomb_power: i32,
  pub has_boots: bool,
  pub is_human: bool,
}

impl Player {
  pub fn new(id: i32, position: (f64, f64)) -> Self {
    Player {
      id: id,
      position: position,
      is_alive: true,
      speed: 5,
      bomb_number: 1,
      bomb_power: 1,
      has_boots: false,
      is_human: false,
    }
  }

  pub fn apply_powerup(&mut self, powerup: PowerupType) {
    match powerup {
      PowerupType::BombNumber => if self.bomb_number < MAX_BOMBS { self.bomb_number += 1; },
      PowerupType::BombPower => if self.bomb_power < MAX_POWER { self.bomb_power += 1; },
      PowerupType::Speed => if self.speed < MAX_SPEED { self.speed += 1; },
      PowerupType::Boots => { self.has_boots = true; },
    }
  }
}

impl Position for Player {
  fn position(&self) -> (f64, f64) {
    self.position
  }
  fn set_position(&mut self, x: f64, y: f64) {
    self.position = (x, y);
  }
}