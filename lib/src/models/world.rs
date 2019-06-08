use super::{Bomb, Player, Tile, Tiles};

const MAX_BOMBS: usize = 36;

pub struct World {
  pub bombs: Vec<Bomb>,
  pub players: [Player; 4],
  pub tiles: Tiles,
}

impl World {
  pub fn new() -> Self {
    // world setup. todo: get this from level
    let width: i32 = 15;
    let height: i32 = 11;
    let tiles = (0..width * height)
      .map(|i| {
        let x = i % width;
        let y = i / width;
        if y % 2 == 1 && x % 2 == 1 {
          Tile::HardBlock
        } else if (x == 0 || x == width - 1) && (y < 3 || height - y <= 3) ||
              (y == 0 || y == height - 1) && (x < 3 || width - x <= 3) {
          Tile::Empty
        } else {
          Tile::SoftBlock
        }
      })
      .collect();
    let players = [
      Player::new(1, (0.0, 0.0)),
      Player::new(2, ((width - 1) as f64, 0.0)),
      Player::new(3, (0.0, (height - 1) as f64)),
      Player::new(4, ((width - 1) as f64, (height - 1) as f64))
    ];
    
    World {
      tiles: Tiles::new(tiles, width, height),
      bombs: Vec::with_capacity(MAX_BOMBS),
      players: players,
    }
  }
}
