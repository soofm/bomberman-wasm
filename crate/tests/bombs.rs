extern crate bomberman_wasm;

#[cfg(test)]
mod test {
  use bomberman_wasm::models::{Bomb, Tile, Tiles};
  
  #[test]
  fn explosion_includes_soft_blocks() {
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::SoftBlock } else { Tile::Empty } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);
    let bomb = Bomb::new(1, 1, 1.0, 1.0);

    let explosion = bomb.calc_explosion(&tiles);
    
    assert_eq!(explosion.x, 1);
    assert_eq!(explosion.y, 1);
    assert_eq!(explosion.left, 1);
    assert_eq!(explosion.right, 1);
    assert_eq!(explosion.up, 1);
    assert_eq!(explosion.down, 1);
  }

  #[test]
  fn explosion_does_not_include_hard_blocks() {
    let tiles = (0..9).map(|i| {
      if i % 2 == 1 { Tile::HardBlock } else { Tile::Empty } 
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);
    let bomb = Bomb::new(1, 1, 1.0, 1.0);

    let explosion = bomb.calc_explosion(&tiles);
    
    assert_eq!(explosion.x, 1);
    assert_eq!(explosion.y, 1);
    assert_eq!(explosion.left, 0);
    assert_eq!(explosion.right, 0);
    assert_eq!(explosion.up, 0);
    assert_eq!(explosion.down, 0);
  }
  
  #[test]
  fn explosion_includes_powerups() {
    let tiles = (0..9).map(|i| {
      match i {
        1 => Tile::PowerupBombNumber,
        3 => Tile::PowerupBombPower,
        5 => Tile::PowerupSpeed,
        7 => Tile::PowerupBoots,
        _ => Tile::Empty,
      }
    }).collect();
    let tiles = Tiles::new(tiles, 3, 3);
    let bomb = Bomb::new(1, 1, 1.0, 1.0);

    let explosion = bomb.calc_explosion(&tiles);
    
    assert_eq!(explosion.x, 1);
    assert_eq!(explosion.y, 1);
    assert_eq!(explosion.left, 1);
    assert_eq!(explosion.right, 1);
    assert_eq!(explosion.up, 1);
    assert_eq!(explosion.down, 1);
  }

  #[test]
  fn explosion_does_not_go_out_of_bounds_at_top_left() {
    let tiles = vec![Tile::Empty; 9];
    let tiles = Tiles::new(tiles, 3, 3);
    let bomb = Bomb::new(1, 1, 0.0, 0.0);

    let explosion = bomb.calc_explosion(&tiles);
    
    assert_eq!(explosion.x, 0);
    assert_eq!(explosion.y, 0);
    assert_eq!(explosion.left, 0);
    assert_eq!(explosion.right, 1);
    assert_eq!(explosion.up, 0);
    assert_eq!(explosion.down, 1);
  }

  #[test]
  fn explosion_does_not_go_out_of_bounds_at_bottom_right() {
    let tiles = vec![Tile::Empty; 9];
    let tiles = Tiles::new(tiles, 3, 3);
    let bomb = Bomb::new(1, 1, 2.0, 2.0);

    let explosion = bomb.calc_explosion(&tiles);
    
    assert_eq!(explosion.x, 2);
    assert_eq!(explosion.y, 2);
    assert_eq!(explosion.left, 1);
    assert_eq!(explosion.right, 0);
    assert_eq!(explosion.up, 1);
    assert_eq!(explosion.down, 0);
  }

  #[test]
  fn explosion_does_not_go_out_of_bounds_when_power_exceeds_bounds() {
    let tiles = vec![Tile::Empty; 9];
    let tiles = Tiles::new(tiles, 3, 3);
    let bomb = Bomb::new(1, 3, 0.0, 0.0);

    let explosion = bomb.calc_explosion(&tiles);
    
    assert_eq!(explosion.x, 0);
    assert_eq!(explosion.y, 0);
    assert_eq!(explosion.left, 0);
    assert_eq!(explosion.right, 2);
    assert_eq!(explosion.up, 0);
    assert_eq!(explosion.down, 2);
  }
}