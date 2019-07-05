use rand::{Rng, RngCore};
use crate::geometry::{Entity, InRange};
use crate::models::{Actions, Bomb, Player, Tile, World};

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
  let entity_positions = world.entity_positions();

  for (index, (player, action)) in world.players.iter_mut().zip(actions.iter()).enumerate() {
    if !player.is_alive { continue; }

    let dist = player.speed as f64 / 60.0;
    
    if let Some(dir) = action.direction {
      player.move_in_direction(dir, dist, &entity_positions, &world.tiles, &mut world.bombs);
    }

    // detect bomb collisions
    let (col, row) = player.current_tile();
    for explosion in world.explosions.iter() {
      if explosion.y == row && (col >= explosion.x - explosion.left && col <= explosion.x + explosion.right) ||
        explosion.x == col && (row >= explosion.y - explosion.up && row <= explosion.y + explosion.down) {
          player.is_alive = false;
      }
    }

    if player.bomb_number > 0 && action.place_bomb && !world.tiles.is_blocked(col, row)
      && !world.bombs.iter()
        .map(|bomb| bomb.current_tile())
        .any(|bomb| bomb.0 == col && bomb.1 == row) {
      player.bomb_number -= 1;
      add_bomb(&mut world.bombs, player, index);
    }

    let tile = world.tiles.get(col, row);
    if player.apply_powerup(tile) {
      world.tiles.set(col, row, Tile::Empty);
    }
  }
}

fn add_bomb(bombs: &mut Vec<Bomb>, player: &Player, id: usize) {
  let x = player.x.round();
  let y = player.y.round();
  bombs.push(Bomb::new(id, player.bomb_power, x, y));
}

fn update_bombs<R: RngCore>(world: &mut World, rng: &mut R) {
  let entity_positions = world.entity_positions();

  for bomb in world.bombs.iter_mut() {
    let (col, row) = bomb.current_tile();

    if let Some(dir) = bomb.direction {
      bomb.move_in_direction(dir, &entity_positions, &mut world.tiles);
    }

    if bomb.timer == 0 {
      if world.tiles.is_blocked(col, row) { world.tiles.set(col, row, Tile::Empty); }

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
    }
  }
}

fn gen_tile_replacement<R: RngCore>(rng: &mut R) -> Tile {
    let n = rng.gen::<f64>();
    match n {
      x if x.in_range(0.0, 0.6) => Tile::Empty,
      x if x.in_range(0.6, 0.72) => Tile::PowerupBombNumber,
      x if x.in_range(0.72, 0.84) => Tile::PowerupBombPower,
      x if x.in_range(0.84, 0.96) => Tile::PowerupSpeed,
      x if x.in_range(0.96, 1.0) => Tile::PowerupBoots,
      _ => Tile::Empty,
    }
}
