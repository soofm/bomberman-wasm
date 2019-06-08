extern crate bomberman_wasm;

#[cfg(test)]
mod test {
  use rand::rngs::mock::StepRng;
  use bomberman_wasm::controllers::game_state;
  use bomberman_wasm::models::{Bomb, Player, Tile, Tiles};

  const BOMB_LIFE: i32 = 210;  

  #[test]
  fn bomb_can_destroy_soft_blocks() {
    let mut players = [ Player::new(1, (0.0, 0.0)), Player::new(2, (0.0, 0.0)), Player::new(3, (0.0, 0.0)), Player::new(4, (0.0, 0.0)) ];
    let mut rng = StepRng::new(0, 0);
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::SoftBlock } else { Tile::Empty } 
    }).collect();
    let mut tiles = Tiles::new(tiles, 3, 3);
    let mut bombs = vec![Bomb::new(1, 1, (1.0, 1.0))];
    for _i in 0..BOMB_LIFE { game_state::update_bombs(&mut bombs, &mut players, &mut tiles, &mut rng); }
    
    assert_eq!(tiles.get(1, 0), Tile::Empty);
    assert_eq!(tiles.get(0, 1), Tile::Empty);
    assert_eq!(tiles.get(2, 1), Tile::Empty);
    assert_eq!(tiles.get(1, 2), Tile::Empty);
  }

  #[test]
  fn bomb_cannot_destroy_hard_blocks() {
    let mut players = [ Player::new(1, (0.0, 0.0)), Player::new(2, (0.0, 0.0)), Player::new(3, (0.0, 0.0)), Player::new(4, (0.0, 0.0)) ];
    let mut rng = StepRng::new(0, 0);
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::HardBlock } else { Tile::Empty } 
    }).collect();
    let mut tiles = Tiles::new(tiles, 3, 3);
    let mut bombs = vec![Bomb::new(1, 1, (1.0, 1.0))];
    for _i in 0..BOMB_LIFE { game_state::update_bombs(&mut bombs, &mut players, &mut tiles, &mut rng); }
    
    assert_eq!(tiles.get(1, 0), Tile::HardBlock);
    assert_eq!(tiles.get(0, 1), Tile::HardBlock);
    assert_eq!(tiles.get(2, 1), Tile::HardBlock);
    assert_eq!(tiles.get(1, 2), Tile::HardBlock);
  }

  #[test]
  fn bomb_works_at_corners() {
    let mut players = [ Player::new(1, (0.0, 0.0)), Player::new(2, (0.0, 0.0)), Player::new(3, (0.0, 0.0)), Player::new(4, (0.0, 0.0)) ];
    let mut rng = StepRng::new(0, 0);
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::SoftBlock } else { Tile::Empty } 
    }).collect();
    let mut tiles = Tiles::new(tiles, 3, 3);
    let mut bombs = vec![Bomb::new(1, 1, (2.0, 2.0))];
    for _i in 0..BOMB_LIFE { game_state::update_bombs(&mut bombs, &mut players, &mut tiles, &mut rng); }
    
    assert_eq!(tiles.get(1, 0), Tile::SoftBlock);
    assert_eq!(tiles.get(0, 1), Tile::SoftBlock);
    assert_eq!(tiles.get(2, 1), Tile::Empty);
    assert_eq!(tiles.get(1, 2), Tile::Empty);
  }

  #[test]
  fn bomb_does_not_destroy_multiple_blocks() {
    let mut players = [ Player::new(1, (0.0, 0.0)), Player::new(2, (0.0, 0.0)), Player::new(3, (0.0, 0.0)), Player::new(4, (0.0, 0.0)) ];
    let mut rng = StepRng::new(0, 0);
    let tiles = (0..9).map(|i| {
      if i == 1 || i == 2 { Tile::SoftBlock } else { Tile::Empty } 
    }).collect();
    let mut tiles = Tiles::new(tiles, 3, 3);
    let mut bombs = vec![Bomb::new(1, 3, (0.0, 0.0))];

    for _i in 0..BOMB_LIFE { game_state::update_bombs(&mut bombs, &mut players, &mut tiles, &mut rng); }
    
    assert_eq!(tiles.get(1, 0), Tile::Empty);
    assert_eq!(tiles.get(2, 0), Tile::SoftBlock);
  }

  #[test]
  fn bomb_range_is_equal_to_power() {
    let mut players = [ Player::new(1, (0.0, 0.0)), Player::new(2, (0.0, 0.0)), Player::new(3, (0.0, 0.0)), Player::new(4, (0.0, 0.0)) ];
    let mut rng = StepRng::new(0, 0);
    let tiles = (0..9).map(|i| {
      if i == 2 { Tile::SoftBlock } else { Tile::Empty }
    }).collect();
    let mut tiles = Tiles::new(tiles, 3, 3);
    let mut bombs = vec![Bomb::new(1, 1, (0.0, 0.0))];

    for _i in 0..BOMB_LIFE { game_state::update_bombs(&mut bombs, &mut players, &mut tiles, &mut rng); }
    
    assert_eq!(tiles.get(2, 0), Tile::SoftBlock);
  }
}
