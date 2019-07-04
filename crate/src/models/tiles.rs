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

  pub fn iter(&self) -> std::slice::Iter<Tile> {
    self.tiles.iter()
  }

  pub fn get_tiles_ptr(&self) -> *const Tile {
    self.tiles.as_ptr()
  }

  pub fn adjacent_soft_block_count(&self, col: i32, row: i32) -> i32 {
    let mut count = 0;
    if col > 0 && self.get(col - 1, row) == Tile::SoftBlock { count += 1; }
    if col < self.width - 1 && self.get(col + 1, row) == Tile::SoftBlock { count += 1; }
    if row > 0 && self.get(col, row - 1) == Tile::SoftBlock { count += 1; }
    if row < self.height - 1 && self.get(col, row + 1) == Tile::SoftBlock { count += 1; }

    count
  }

  pub fn is_blocked(&self, col: i32, row: i32) -> bool {
    if col < 0 || row < 0 || col >= self.width || row >= self.height { return true; }
    match self.get(col, row) {
      Tile::SoftBlock | Tile::HardBlock => true,
      _ => false
    }
  }

  pub fn set(&mut self, col: i32, row: i32, value: Tile) {
    self.tiles[(row * self.width + col) as usize] = value;
  }
}