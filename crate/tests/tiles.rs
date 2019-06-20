extern crate bomberman_wasm;

#[cfg(test)]
mod test {
  use bomberman_wasm::models::{Tile, Tiles};
  
  #[test]
  fn soft_block_tiles_are_considered_blocked() {
    let tiles = vec![Tile::SoftBlock];
    let tiles = Tiles::new(tiles, 1, 1);

    assert_eq!(tiles.is_blocked(0, 0), true);
  }

  #[test]
  fn hard_block_tiles_are_considered_blocked() {
    let tiles = vec![Tile::HardBlock];
    let tiles = Tiles::new(tiles, 1, 1);

    assert_eq!(tiles.is_blocked(0, 0), true);
  }

  #[test]
  fn powerup_tiles_are_not_considered_blocked() {
    let tiles = vec![
      Tile::PowerupBombNumber,
      Tile::PowerupBombPower,
      Tile::PowerupSpeed,
      Tile::PowerupBoots
    ];
    let tiles = Tiles::new(tiles, 2, 2);

    assert_eq!(tiles.is_blocked(0, 0), false);
    assert_eq!(tiles.is_blocked(0, 1), false);
    assert_eq!(tiles.is_blocked(1, 0), false);
    assert_eq!(tiles.is_blocked(1, 1), false);
  }

  #[test]
  fn empty_tiles_are_not_considered_blocked() {
    let tiles = vec![Tile::Empty];
    let tiles = Tiles::new(tiles, 1, 1);

    assert_eq!(tiles.is_blocked(0, 0), false);
  }

  #[test]
  fn adjacent_soft_block_count_counts_all_soft_blocks() {
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::SoftBlock } else { Tile::Empty } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);

    assert_eq!(tiles.adjacent_soft_block_count(1, 1), 4);
  }

  #[test]
  fn adjacent_soft_block_count_does_not_count_hard_blocks() {
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::HardBlock } else { Tile::Empty } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);

    assert_eq!(tiles.adjacent_soft_block_count(1, 1), 0);
  }

  #[test]
  fn adjacent_soft_block_count_works_in_corners() {
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::SoftBlock } else { Tile::Empty } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);

    assert_eq!(tiles.adjacent_soft_block_count(0, 0), 2);
    assert_eq!(tiles.adjacent_soft_block_count(0, 2), 2);
    assert_eq!(tiles.adjacent_soft_block_count(2, 0), 2);
    assert_eq!(tiles.adjacent_soft_block_count(2, 2), 2);
  }
}