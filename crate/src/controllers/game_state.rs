use crate::geometry::{InRange, Position};
use crate::models::{Actions, Bomb, Player, Tile, Tiles, World};
use rand::{Rng, RngCore};

pub fn update<R: RngCore>(world: &mut World, actions: [Actions; 4], rng: &mut R) {
  // run ticks before any new items are added
  while world.bombs.len() > 0 && world.bombs[0].timer == 0 {
    world.bombs.remove(0);
  }
  for bomb in world.bombs.iter_mut() {
    bomb.timer -= 1;
  }

  while world.explosions.len() > 0 && world.explosions[0].timer == 0 {
    world.explosions.remove(0);
  }
  for explosion in world.explosions.iter_mut() {
    explosion.timer -= 1;
  }
  
  update_players(world, actions);
  update_bombs(world, rng);
}

fn update_players(world: &mut World, actions: [Actions; 4]) {
  for (index, (player, action)) in world.players.iter_mut().zip(actions.iter()).enumerate() {
    if !player.is_alive { continue; }

    let dist = player.speed as f64 / 60.0;
    
    if let Some(dir) = action.direction {
      player.move_in_direction(dir, dist, &world.tiles);
    }

    // detect bomb collisions
    let col = player.x.round() as i32;
    let row = player.y.round() as i32;
    for explosion in world.explosions.iter() {
      if explosion.y == row && (col >= explosion.x - explosion.left && col <= explosion.x + explosion.right) ||
        explosion.x == col && (row >= explosion.y - explosion.up && row <= explosion.y + explosion.down) {
          player.is_alive = false;
      }
    }

    if player.bomb_number > 0 && action.place_bomb && !world.tiles.is_blocked(col, row) {
      player.bomb_number -= 1;
      add_bomb(&mut world.bombs, &mut world.tiles, player, index);
    }

    let tile = world.tiles.get(col, row);
    if player.apply_powerup(tile) {
      world.tiles.set(col, row, Tile::Empty);
    }
  }
}

fn add_bomb(bombs: &mut Vec<Bomb>, tiles: &mut Tiles, player: &Player, id: usize) {
  let x = player.x.round();
  let y = player.y.round();
  bombs.push(Bomb::new(id, player.bomb_power, x, y));
  tiles.set(x as i32, y as i32, Tile::Bomb);
}

fn update_bombs<R: RngCore>(world: &mut World, rng: &mut R) {
  for bomb in world.bombs.iter_mut() {
    let col = bomb.x.round() as i32;
    let row = bomb.y.round() as i32;

    if bomb.timer == 0 {
      world.tiles.set(col, row, Tile::Empty);

      let explosion = bomb.calc_explosion(&world.tiles);
      if explosion.left > 0 && world.tiles.get(col - explosion.left, row) == Tile::SoftBlock {
        world.tiles.set(col - explosion.left, row, gen_tile_replacement(rng));
      }
      if explosion.right > 0 && world.tiles.get(col + explosion.right, row) == Tile::SoftBlock {
        world.tiles.set(col + explosion.right, row, gen_tile_replacement(rng));
      }
      if explosion.up > 0 && world.tiles.get(col, row - explosion.up) == Tile::SoftBlock {
        world.tiles.set(col, row - explosion.up, gen_tile_replacement(rng));
      }
      if explosion.down > 0 && world.tiles.get(col, row + explosion.down) == Tile::SoftBlock {
        world.tiles.set(col, row + explosion.down, gen_tile_replacement(rng));
      }
      world.explosions.push(explosion);

      world.players[bomb.player_id].bomb_number += 1;
    } else {
      break;
    }
  }
}

fn gen_tile_replacement<R: RngCore>(rng: &mut R) -> Tile {
    let n = rng.gen::<f64>();
    match n {
      x if x.in_range(0.0, 0.5) => Tile::Empty,
      x if x.in_range(0.5, 0.65) => Tile::PowerupBombNumber,
      x if x.in_range(0.65, 0.8) => Tile::PowerupBombPower,
      x if x.in_range(0.8, 0.95) => Tile::PowerupSpeed,
      x if x.in_range(0.95, 1.0) => Tile::PowerupBoots,
      _ => Tile::Empty,
    }
}
