use super::{Explosion, Tile, Tiles};
use crate::geometry::{Direction, Position};

pub const BOMB_TIMER: i32 = 180;

#[derive(Debug)]
pub struct Bomb {
  pub player_id: usize,
  pub x: f64,
  pub y: f64,
  pub power: i32,
  pub direction: Option<Direction>,
  pub timer: i32,
}

impl Bomb {
  pub fn new(player_id: usize, power: i32, x: f64, y: f64) -> Self {
    Bomb {
      player_id: player_id,
      x: x,
      y: y,
      power: power,
      direction: None,
      timer: BOMB_TIMER,
    }
  }

  pub fn calc_explosion(&self, tiles: &Tiles) -> Explosion {
    let col = self.x.round() as i32;
    let row = self.y.round() as i32;

    let left = self.calc_explosion_ray(tiles, col, row, -1, 0);
    let right = self.calc_explosion_ray(tiles, col, row, 1, 0);
    let up = self.calc_explosion_ray(tiles, col, row, 0, -1);
    let down = self.calc_explosion_ray(tiles, col, row, 0, 1);
    Explosion::new(self.player_id, col, row, left, right, up, down)
  }

  fn calc_explosion_ray(&self, tiles: &Tiles, ocol: i32, orow: i32, x: i32, y: i32) -> i32 {
    let mut res = 0;
    let mut col = ocol;
    let mut row = orow;
    let mut found_tile: Option<Tile> = None;
    while res < self.power && found_tile != Some(Tile::HardBlock) && found_tile != Some(Tile::SoftBlock) {
      res += 1;
      col += x;
      row += y;
      if col < 0 || col >= tiles.width || row < 0 || row >= tiles.height {
        found_tile = None;
        break;
      }
      found_tile = Some(tiles.get(col, row));
    }
    match found_tile {
      Some(Tile::HardBlock) | None => res - 1,
      _ => res,
    }
  }
}

impl Position for Bomb {
  fn position(&self) -> (f64, f64) {
    (self.x, self.y)
  }
  fn set_position(&mut self, x: f64, y: f64) {
    self.x = x;
    self.y = y;
  }
}
