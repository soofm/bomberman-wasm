use super::Input;
use crate::geometry::Direction;
use crate::models::{Actions, Player, Tiles, World};

#[derive(Default)]
pub struct AIInput {
  pub next_eval: i32,
  pub direction: Option<Direction>,
}

impl Input for AIInput {
  fn eval(&mut self, player: &Player, world: &World) -> Actions {
    if self.next_eval > 0 {
      self.next_eval -= 1;
    } else {
    let (col, row) = (player.position.0.round() as i32, player.position.1.round() as i32);
    let open_tiles = get_open_tiles((col, row), &world.tiles);
    if self.direction != Some(Direction::Left) { self.direction = Some(Direction::Left); }
    else { self.direction = Some(Direction::Right); }
    self.next_eval = 30;
  }

    Actions {
      player_id: player.id,
      direction: self.direction,
      place_bomb: false,
    }
  }
}

fn get_farthest_tile(position: (i32, i32), tiles: Vec<(i32, i32)>) {

}

fn get_open_tiles(position: (i32, i32), tiles: &Tiles) -> Vec<(i32, i32)> {    
  let mut visited = Vec::with_capacity((tiles.width * tiles.height) as usize);
  let mut queue: Vec<(i32, i32)> = Vec::with_capacity(10);
  let mut result: Vec<(i32, i32)> = Vec::with_capacity(10);

  for row in 0..tiles.height {
    for col in 0..tiles.width {
      if tiles.is_blocked(col, row) { visited[(row * tiles.width + col) as usize] = true; }
    }
  }

  queue.push(position);
  while !queue.is_empty() {
    let (col, row) = queue.remove(0);
    result.push((col, row));

    if col > 0 && !visited[(row * tiles.width + col - 1) as usize] {
      queue.push((col - 1, row));
    }
    if col < tiles.width - 1 && !visited[(row * tiles.width + col + 1) as usize] {
      queue.push((col + 1, row));
    }
    if row > 0 && !visited[((row - 1) * tiles.width + col) as usize] {
      queue.push((col, row - 1));
    }
    if row < tiles.height - 1 && !visited[((row + 1) * tiles.width + col) as usize] {
      queue.push((col, row + 1));
    }
  }
  result
}