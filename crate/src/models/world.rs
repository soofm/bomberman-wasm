use crate::geometry::Entity;
use super::{Bomb, Explosion, Player, Tiles};

pub struct World {
  pub bombs: Vec<Bomb>,
  pub explosions: Vec<Explosion>,
  pub players: [Player; 4],
  pub tiles: Tiles,
}

impl World {
  pub fn new(tiles: Tiles, players: [Player; 4]) -> Self {
    World {
      tiles: tiles,
      players: players,
      bombs: Vec::new(),
      explosions: Vec::new(),
    }
  }

  pub fn entity_positions(&self) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = self.players.iter()
      .filter(|player| player.is_alive)
      .map(|player| player.current_tile()).collect();
    for position in self.bombs.iter().map(|bomb| bomb.current_tile()) {
      result.push(position);
    }

    result
  }
}
