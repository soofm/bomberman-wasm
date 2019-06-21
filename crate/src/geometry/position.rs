use crate::models::Tiles;
use super::{CORNER, Direction};

pub trait Position {
  fn position(&self) -> (f64, f64);
  fn set_position(&mut self, x: f64, y: f64);
  fn move_in_direction(&mut self, dir: Direction, dist: f64, can_move_bomb: bool, tiles: &Tiles) {
    let (x, y) = self.position();
    let (a, b, horiz, coef) = match dir {
      Direction::Left => (&x, &y, true, -1.0),
      Direction::Right => (&x, &y, true, 1.0),
      Direction::Up => (&y, &x, false, -1.0),
      Direction::Down => (&y, &x, false, 1.0),
    };
    let target_tile: i32;
    let adjusted: f64;
    let target = *a + coef * dist;
    if coef < 0.0 {
      target_tile = target.floor() as i32;
      adjusted = a.floor();
    } else {
      target_tile = target.ceil() as i32;
      adjusted = a.ceil();
    }

    let res = if target_tile != adjusted as i32 {
      let bi = b.floor() as i32;
      let bf = b.fract();
      let blocked = if horiz {
        (tiles.is_blocked(target_tile, bi), tiles.is_blocked(target_tile, bi + 1))
      } else {
        (tiles.is_blocked(bi, target_tile), tiles.is_blocked(bi + 1, target_tile))
      };
      if blocked.0 && blocked.1 {
        (adjusted, *b)
      } else if blocked.0 {
        if bf < 1.0 - CORNER { (adjusted, *b) } else { (target, b.ceil()) }
      } else if blocked.1 {
        if bf > CORNER { (adjusted, *b) } else { (target, b.floor()) }
      } else {
        (target, *b)
      }
    } else {
      (target, *b)
    };

    if horiz {
      self.set_position(res.0, res.1);
    } else {
      self.set_position(res.1, res.0);
    }
  }
}
