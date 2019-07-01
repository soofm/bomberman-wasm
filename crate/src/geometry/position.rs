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
    let tari: i32;
    let a_adj: f64;
    let tar = *a + coef * dist;
    if coef < 0.0 {
      tari = tar.floor() as i32;
      a_adj = a.floor();
    } else {
      tari = tar.ceil() as i32;
      a_adj = a.ceil();
    }

    let mut res = (a_adj, *b);
    if tari != a_adj as i32 {
      let bf = b.fract();
      let b_adj = if bf <= CORNER {
        b.floor()
      } else if bf >= 1.0 - CORNER {
        b.ceil()
      } else {
        -1.0
      };

      if b_adj >= 0.0 {
        let tile = if horiz { (tari, b_adj as i32) } else { (b_adj as i32, tari) };
        if !tiles.is_blocked(tile.0, tile.1) {
          res = (tar, b_adj);
        }
      }
    } else {
      res = (tar, *b);
    }

    if horiz {
      self.set_position(res.0, res.1);
    } else {
      self.set_position(res.1, res.0);
    }
  }
}
