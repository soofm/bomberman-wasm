use crate::models::{Player, Tile, Tiles};

pub fn build_level() -> (Tiles, [Player; 4]) {
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
  let tiles = Tiles::new(tiles, width, height);
  let players = [
    Player::new(0.0, 0.0),
    Player::new((width - 1) as f64, 0.0),
    Player::new(0.0, (height - 1) as f64),
    Player::new((width - 1) as f64, (height - 1) as f64)
  ];
  (tiles, players)
}