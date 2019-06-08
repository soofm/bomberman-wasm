extern crate bomberman_wasm;

#[cfg(test)]
mod test {
  use bomberman_wasm::geometry::{Direction, Position};
  use bomberman_wasm::models::{Tile, Tiles};

  struct GameObject {
    position: (f64, f64),
  }

  impl GameObject {
    pub fn new(x: f64, y: f64) -> Self {
      GameObject { position: (x, y) }
    }
  }

  impl Position for GameObject {
    fn position(&self) -> (f64, f64) {
      self.position
    }
    fn set_position(&mut self, x: f64, y: f64) {
      self.position = (x, y);
    }
  }

  #[test]
  fn object_can_move_from_origin_to_empty_square() {
    let tiles = vec![Tile::Empty; 4];
    let tiles = Tiles::new(tiles, 2, 2);
    let mut obj = GameObject::new(0.0, 0.0);
    obj.move_in_direction(Direction::Right, 1.0, &tiles);
    assert_eq!(obj.position(), (1.0, 0.0));
  }

  #[test]
  fn object_can_move_over_empty_square_boundaries() {
    let tiles = vec![Tile::Empty; 6];
    let tiles = Tiles::new(tiles, 3, 2);
    let mut obj = GameObject::new(0.5, 0.0);
    obj.move_in_direction(Direction::Right, 1.0, &tiles);
    assert_eq!(obj.position(), (1.5, 0.0));
  }

  #[test]
  fn object_cannot_move_left_out_of_bounds() {
    let tiles = vec![Tile::Empty; 4];
    let tiles = Tiles::new(tiles, 2, 2);
    let mut obj = GameObject::new(0.0, 0.0);
    obj.move_in_direction(Direction::Left, 1.0, &tiles);
    assert_eq!(obj.position(), (0.0, 0.0));
  }

  #[test]
  fn object_cannot_move_up_out_of_bounds() {
    let tiles = vec![Tile::Empty; 4];
    let tiles = Tiles::new(tiles, 2, 2);
    let mut obj = GameObject::new(0.0, 0.0);
    obj.move_in_direction(Direction::Up, 1.0, &tiles);
    assert_eq!(obj.position(), (0.0, 0.0));
  }

  #[test]
  fn object_cannot_move_right_out_of_bounds() {
    let tiles = vec![Tile::Empty; 4];
    let tiles = Tiles::new(tiles, 2, 2);
    let mut obj = GameObject::new(1.0, 0.0);
    obj.move_in_direction(Direction::Right, 1.0, &tiles);
    assert_eq!(obj.position(), (1.0, 0.0));
  }

  #[test]
  fn object_cannot_move_down_out_of_bounds() {
    let tiles = vec![Tile::Empty; 4];
    let tiles = Tiles::new(tiles, 2, 2);
    let mut obj = GameObject::new(1.0, 1.0);
    obj.move_in_direction(Direction::Down, 1.0, &tiles);
    assert_eq!(obj.position(), (1.0, 1.0));
  }

  #[test]
  fn object_cannot_move_right_from_origin_to_occupied_square() {
    let tiles = vec![Tile::Empty, Tile::HardBlock, Tile::HardBlock, Tile::Empty];
    let tiles = Tiles::new(tiles, 2, 2);                                         
    let mut obj = GameObject::new(0.0, 0.0);
    obj.move_in_direction(Direction::Right, 1.0, &tiles);
    assert_eq!(obj.position(), (0.0, 0.0));
  }

  #[test]
  fn object_cannot_move_right_from_midpoint_when_either_square_is_occupied() {
    let tiles = vec![Tile::Empty, Tile::HardBlock, Tile::Empty, Tile::Empty];
    let tiles = Tiles::new(tiles, 2, 2);                                         
    let mut obj = GameObject::new(0.0, 0.5);
    obj.move_in_direction(Direction::Right, 0.5, &tiles);
    assert_eq!(obj.position(), (0.0, 0.5));
  }

  #[test]
  fn object_can_be_bumped_onto_grid_when_slightly_off_center() {
    let tiles = vec![Tile::Empty, Tile::HardBlock, Tile::Empty, Tile::Empty];
    let tiles = Tiles::new(tiles, 2, 2);                                         
    let mut obj = GameObject::new(0.0, 0.9);
    obj.move_in_direction(Direction::Right, 0.5, &tiles);
    assert_eq!(obj.position(), (0.5, 1.0));
  }

  #[test]
  fn object_cannot_move_down_from_origin_to_occupied_square() {
    let tiles = vec![Tile::Empty, Tile::HardBlock, Tile::HardBlock, Tile::Empty];
    let tiles = Tiles::new(tiles, 2, 2);                                         
    let mut obj = GameObject::new(0.0, 0.0);
    obj.move_in_direction(Direction::Down, 1.0, &tiles);
    assert_eq!(obj.position(), (0.0, 0.0));
  }

  #[test]
  fn object_cannot_move_down_from_midpoint_when_either_square_is_occupied() {
    let tiles = vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::HardBlock];
    let tiles = Tiles::new(tiles, 2, 2);                                         
    let mut obj = GameObject::new(0.5, 0.0);
    obj.move_in_direction(Direction::Down, 1.0, &tiles);
    assert_eq!(obj.position(), (0.5, 0.0));
  }
}
