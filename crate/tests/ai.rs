extern crate bomberman_wasm;

#[cfg(test)]
mod test {
  use bomberman_wasm::controllers::ai;
  use bomberman_wasm::models::{Tile, Tiles};

  #[test]
  fn find_available_paths_works() {
    let tiles = (0..9).map(|i| {
      if i % 3 == 0 || i / 3 == 2 { Tile::Empty } else { Tile::HardBlock } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);

    let available_tiles = ai::find_available_paths((0, 2), &tiles);
    assert_eq!(available_tiles.get(&(2, 2)), Some(&(1, 2)));
    assert_eq!(available_tiles.get(&(1, 2)), Some(&(0, 2)));
    assert_eq!(available_tiles.get(&(0, 2)), None);
    assert_eq!(available_tiles.get(&(0, 1)), Some(&(0, 2)));
    assert_eq!(available_tiles.get(&(0, 0)), Some(&(0, 1)));
  }

  #[test]
  fn rebuild_paths_works() {
    let tiles = (0..9).map(|i| {
      if i % 3 == 0 || i / 3 == 2 { Tile::Empty } else { Tile::HardBlock } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);

    let available_tiles = ai::find_available_paths((0, 2), &tiles);
    let path = ai::rebuild_path((2, 2), available_tiles);
    assert_eq!(path, [(2, 2), (1, 2), (0, 2)]);
  }
}