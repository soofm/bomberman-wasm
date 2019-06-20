extern crate bomberman_wasm;

#[cfg(test)]
mod test {
  use bomberman_wasm::controllers::ai;
  use bomberman_wasm::models::{Bomb, Player, Tile, Tiles, World};
  use bomberman_wasm::models::{BOMB_TIMER, EXPLOSION_TIMER};

  const SAFE_TILE: i32 = 999;

  #[test]
  fn get_accessible_tiles_works() {
    let tiles = (0..9).map(|i| {
      if i % 3 == 0 || i / 3 == 2 { Tile::Empty } else { Tile::HardBlock } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);

    let available_tiles = ai::get_accessible_tiles(0, 2, &tiles);
    assert_eq!(available_tiles.get(&(2, 2)), Some(&(1, 2)));
    assert_eq!(available_tiles.get(&(1, 2)), Some(&(0, 2)));
    assert_eq!(available_tiles.get(&(0, 2)), None);
    assert_eq!(available_tiles.get(&(0, 1)), Some(&(0, 2)));
    assert_eq!(available_tiles.get(&(0, 0)), Some(&(0, 1)));
  }

  #[test]
  fn eval_tile_safety_works() {
    let tiles = (0..9).map(|i| {
      if i == 0 { Tile::Bomb }
      else if i % 3 == 0 || i / 3 == 2 { Tile::Empty }
      else { Tile::HardBlock } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);
    let bombs = vec![Bomb::new(1, 1, 0.0, 0.0)];
    let players = [ Player::new(0.0, 0.0), Player::new(0.0, 0.0), Player::new(0.0, 0.0), Player::new(0.0, 0.0) ];
    let mut world = World::new(tiles, players);
    world.bombs = bombs;

    let tile_safety = ai::eval_tile_safety(&world, None);

    assert_eq!(tile_safety[0], EXPLOSION_TIMER + BOMB_TIMER);
    assert_eq!(tile_safety[1], 0);
    assert_eq!(tile_safety[3], EXPLOSION_TIMER + BOMB_TIMER);
    assert_eq!(tile_safety[6], SAFE_TILE);
  }
}