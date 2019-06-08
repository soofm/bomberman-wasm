use super::Tile;

pub struct Tiles {
  tiles: Vec<Tile>,
  pub width: i32,
  pub height: i32,
}

impl Tiles {
  pub fn new(tiles: Vec<Tile>, width: i32, height: i32) -> Self {
    Tiles {
      tiles: tiles,
      width: width,
      height: height,
    }
  }

  pub fn get(&self, col: i32, row: i32) -> Tile {
    self.tiles[(row * self.width + col) as usize]
  }

  pub fn is_blocked(&self, col: i32, row: i32) -> bool {
    if col < 0 || row < 0 || col >= self.width || row >= self.height { return true; }
    match self.get(col, row) {
      Tile::SoftBlock | Tile::HardBlock | Tile::Bomb => true,
      _ => false
    }
  }

  pub fn map_blocked(&self) -> Vec<bool> {
    self.tiles.iter().map(|tile| {
      match tile {
        Tile::SoftBlock | Tile::HardBlock | Tile::Bomb => true,
        _ => false
      }
    }).collect()
  }

  pub fn set(&mut self, col: i32, row: i32, value: Tile) {
    self.tiles[(row * self.width + col) as usize] = value;
  }
}