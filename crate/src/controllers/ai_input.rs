use super::ai;
use crate::geometry::Direction;
use crate::models::{Actions, Bomb, Player, World};
use rand::RngCore;

#[derive(Default)]
pub struct AIInput {
  direction: Option<Direction>,
  target: (i32, i32),
  skip: i32,
}

impl AIInput {
  pub fn eval<R: RngCore>(&mut self, player: &Player, player_id: usize, world: &World, rng: &mut R) -> Actions {
    let mut place_bomb = false;

    if self.skip > 0 {
      self.skip -= 1;
    }
    // Each time the AI runs its evaluation loop, it will decide to either stay put or move one tile in one direction.
    // If it decides to move, it will continue in that direction until it either reaches its destination or it is blocked.
    else if self.direction == None || world.tiles.is_blocked(self.target.0, self.target.1) ||
        (player.x - self.target.0 as f64).abs() <= 0.1 && (player.y - self.target.1 as f64).abs() <= 0.1 {
      // Evaluation loop:
      // * Find all accessible tiles
      // * Check if bomb should be placed at current location
      // * Get tile safety matrix with possible new bomb in mind
      // * Rank safe tiles and randomly choose from the best options
      // * Perform moves
      let col = player.x.round() as i32;
      let row = player.y.round() as i32;
      let accessible_tiles = ai::get_accessible_tiles(col, row, &world.tiles);
      place_bomb = ai::eval_bomb_placement(accessible_tiles.keys(), &player, player_id, world, rng);
      let new_bomb = if place_bomb {
        Some(Bomb::new(player_id, player.bomb_power, player.x, player.y))
      } else {
        None
      };
      let tile_safety = ai::eval_tile_safety(world, new_bomb);
      self.target = ai::choose_option(accessible_tiles, &player, player_id, world, &tile_safety, rng);

      self.direction = if self.target.0 < col {
        Some(Direction::Left)
      } else if self.target.0 > col {
        Some(Direction::Right)
      } else if self.target.1 < row {
        Some(Direction::Up)
      } else if self.target.1 > row {
        Some(Direction::Down)
      } else {
        self.skip = 5;
        None
      };
    }

    Actions {
      direction: self.direction,
      place_bomb: place_bomb,
    }
  }
}
