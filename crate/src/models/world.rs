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
}
