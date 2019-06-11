use super::Tile;
use crate::geometry::Position;

const MAX_BOMBS: i32 = 9;
const MAX_POWER: i32 = 15;
const MAX_SPEED: i32 = 12;

pub struct Player {
  pub x: f64,
  pub y: f64,
  pub is_alive: bool,
  pub speed: i32,
  pub bomb_number: i32,
  pub bomb_power: i32,
  pub has_boots: bool,
  pub is_human: bool,
}

impl Player {
  pub fn new(x: f64, y: f64) -> Self {
    Player {
      x: x,
      y: y,
      is_alive: true,
      speed: 3,
      bomb_number: 1,
      bomb_power: 1,
      has_boots: false,
      is_human: false,
    }
  }

  pub fn apply_powerup(&mut self, tile: Tile) -> bool {
    match tile {
      Tile::PowerupBombNumber => {
        if self.bomb_number < MAX_BOMBS { self.bomb_number += 1; }
        true
      },
      Tile::PowerupBombPower => {
        if self.bomb_power < MAX_POWER { self.bomb_power += 1; }
        true
      },
      Tile::PowerupSpeed => {
        if self.speed < MAX_SPEED { self.speed += 1; }
        true
      },
      Tile::PowerupBoots => {
        self.has_boots = true;
        true
      },
      _ => { false },
    }
  }
}

impl Position for Player {
  fn position(&self) -> (f64, f64) {
    (self.x, self.y)
  }
  fn set_position(&mut self, x: f64, y: f64) {
    self.x = x;
    self.y = y;
  }
}