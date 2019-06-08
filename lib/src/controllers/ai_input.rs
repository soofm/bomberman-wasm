use super::ai;
use crate::geometry::Direction;
use crate::models::{Actions, Player, World};
use rand::{Rng, RngCore};

#[derive(Default)]
pub struct AIInput {
  next_eval: i32,
  path: Vec<(i32, i32)>,
  current: (f64, f64),
}

impl AIInput {
  pub fn eval<R: RngCore>(&mut self, player: &Player, world: &World, rng: &mut R) -> Actions {
    let (ox, oy) = player.position;
    let ocol = ox.round() as i32;
    let orow = oy.round() as i32;

    if self.next_eval > 0 {
      self.next_eval -= 1;
    } else {      
      let available_paths = ai::find_available_paths((ocol, orow), &world.tiles);
      let len = available_paths.keys().len();
      let n = rng.gen_range(0, len);
      let &target = available_paths.keys().skip(n).next().unwrap();
      self.path = ai::rebuild_path(target, available_paths);
      if let Some(next) = self.path.pop() {
        self.current = (next.0 as f64, next.1 as f64);
      }

      self.next_eval = 30;
    }

    if (ox - self.current.0).abs() < 0.1 && (oy - self.current.1).abs() < 0.1 {
      if let Some(next) = self.path.pop() {
        self.current = (next.0 as f64, next.1 as f64);
      }
    }

    let mut direction: Option<Direction> = None;
    let (x, y) = self.current;
    if x < ox {
      direction = Some(Direction::Left)
    } else if x > ox {
      direction = Some(Direction::Right)
    } else if y < oy {
      direction = Some(Direction::Up)
    } else if y > oy {
      direction = Some(Direction::Down)
    }

    Actions {
      player_id: player.id,
      direction: direction,
      place_bomb: false,
    }
  }
}
