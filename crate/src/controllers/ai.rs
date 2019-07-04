use std::cmp;
use std::collections::HashMap;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use crate::geometry::Entity;
use crate::models::{Bomb, Explosion, Player, Tile, Tiles, World, EXPLOSION_TIMER};

const SAFE_TILE: i32 = 999;

pub fn get_accessible_tiles(x: i32, y: i32, tiles: &Tiles) -> HashMap<(i32, i32), (i32, i32)> {
  let mut visited: Vec<bool> = tiles.iter().map(|tile| {
    match tile {
      Tile::SoftBlock | Tile::HardBlock => true,
      _ => false
    }
  }).collect();
  let mut queue = Vec::new();
  let mut result: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
  let row_idx = tiles.width as usize;

  queue.push((x, y));
  while !queue.is_empty() {
    let (x, y) = queue.remove(0);

    let index = (y * tiles.width + x) as usize;
    visited[index] = true;
    if x > 0 && !visited[index - 1] {
      let next = (x - 1, y);
      queue.push(next);
      result.insert(next, (x, y));
    }
    if x < tiles.width - 1 && !visited[index + 1] {
      let next = (x + 1, y);
      queue.push(next);
      result.insert(next, (x, y));
    }
    if y > 0 && !visited[index - row_idx] {
      let next = (x, y - 1);
      queue.push(next);
      result.insert(next, (x, y));
    }
    if y < tiles.height - 1 && !visited[index + row_idx] {
      let next = (x, y + 1);
      queue.push(next);
      result.insert(next, (x, y));
    }
  }

  result
}

pub fn eval_bomb_placement<'a, I: Iterator<Item = &'a (i32, i32)>, R: RngCore>(
  options: I, player: &Player, player_id: usize, world: &World, rng: &mut R) -> bool {
  if player.bomb_number == 0 { return false; }

  // When using player bomb power, the AI tends to drop bombs at intersections and trap itself.
  // Only drop bombs directly next to soft blocks.
  let (col, row) = player.current_tile();
  let asb_count = world.tiles.adjacent_soft_block_count(col, row);
  
  // The AI can be more aggressive when targeting players.
  let bomb = Bomb::new(255, player.bomb_power, player.x, player.y);
  let explosion = bomb.calc_explosion(&world.tiles);
  let mut enemy_in_range = false;
  for (id, enemy) in world.players.iter().enumerate() {
    if id == player_id || !enemy.is_alive { continue; }
    let (col, row) = enemy.current_tile();
    if (explosion.y == row && (col >= explosion.x - explosion.left && col <= explosion.x + explosion.right)) ||
       (explosion.x == col && (row >= explosion.y - explosion.up && row <= explosion.y + explosion.down)) {
      enemy_in_range = true;
      break;
    }
  }

  if asb_count > 0 || enemy_in_range {
    for option in options {
      let (col, row) = *option;
      if !(explosion.y == row && (col >= explosion.x - explosion.left && col <= explosion.x + explosion.right)) &&
         !(explosion.x == col && (row >= explosion.y - explosion.up && row <= explosion.y + explosion.down)) {
        return rng.gen_range(0, 2) == 0;
      }    
    }
  }
  false
}

pub fn eval_tile_safety(world: &World, new_bomb: Option<Bomb>) -> Vec<i32> {
  let mut tile_safety: Vec<i32> = world.tiles.iter().map(|tile| {
    match tile {
      Tile::SoftBlock | Tile::HardBlock => 0,
      _ => SAFE_TILE,
    }
  }).collect();

  for bomb in world.bombs.iter() {
    let explosion = bomb.calc_explosion(&world.tiles);
    mark_explosion_radius_unsafe(&explosion, &mut tile_safety, EXPLOSION_TIMER + bomb.timer, world.tiles.width);
  }

  for explosion in world.explosions.iter() {
    mark_explosion_radius_unsafe(explosion, &mut tile_safety, explosion.timer, world.tiles.width);
  }

  if let Some(bomb) = new_bomb {
    let explosion = bomb.calc_explosion(&world.tiles);
    mark_explosion_radius_unsafe(&explosion, &mut tile_safety, EXPLOSION_TIMER + bomb.timer, world.tiles.width);
  }

  tile_safety
}

pub fn choose_option<R: RngCore>(
  accessible_tiles: HashMap<(i32, i32), (i32, i32)>, player: &Player, player_id: usize, world: &World,
  tile_safety: &Vec<i32>, rng: &mut R) -> (i32, i32) {
  let (ocol, orow) = player.current_tile();

  if accessible_tiles.keys().len() == 0 { return (ocol, orow); }

  let index = (orow * world.tiles.width + ocol) as usize;

  let mut safe_tiles = Vec::new();
  let mut unsafe_tiles = Vec::new();
  for option in accessible_tiles.keys() {
    let powerup = match world.tiles.get(option.0, option.1) {
      Tile::PowerupBombNumber => 20 - player.bomb_number,
      Tile::PowerupBombPower => 20 - player.bomb_power,
      Tile::PowerupSpeed => 23 - player.speed,
      Tile::PowerupBoots => { if player.has_boots { 20 } else { 0 } },
      _ => 0,
    };
    let (path, path_safety) = rebuild_path(*option, player, &accessible_tiles, tile_safety, world.tiles.width);
    let path_start = match path.last() {
      Some(tile) => *tile,
      None => (-1, -1),
    };

    if path_safety == SAFE_TILE {
      let asb_count = world.tiles.adjacent_soft_block_count(option.0, option.1);
      let mut enemy_dist = 0;
      for (id, enemy) in world.players.iter().enumerate() {
        if id == player_id || !enemy.is_alive { continue; }
        let (col, row) = enemy.current_tile();
        enemy_dist += (option.0 - col).abs() + (option.1 - row).abs();
      }

      let tile_val = eval_tile(powerup, path.len() as i32, asb_count, enemy_dist, player.bomb_number);
      safe_tiles.push((path_start, tile_val));
    } else {
      unsafe_tiles.push((path_start, path_safety));
    }
  }

  let mut enemy_dist = 0;
  for (index, enemy) in world.players.iter().enumerate() {
    if index == player_id { continue; }
    let (col, row) = enemy.current_tile();
    enemy_dist += (ocol - col).abs() + (orow - row).abs();
  }
  
  if tile_safety[index] == SAFE_TILE {
    let asb_count = world.tiles.adjacent_soft_block_count(ocol, orow);
    safe_tiles.push(((ocol, orow), eval_tile(player.bomb_number, 0, asb_count, 0, enemy_dist)));
  } else {
    unsafe_tiles.push(((ocol, orow), tile_safety[index]));
  }
  let tile_vals = if safe_tiles.len() > 0 { safe_tiles } else { unsafe_tiles };
  let dist = WeightedIndex::new(tile_vals.iter().map(|item| item.1)).unwrap();
  tile_vals[dist.sample(rng)].0
}

fn mark_explosion_radius_unsafe(explosion: &Explosion, tile_safety: &mut Vec<i32>, timer: i32, width: i32) {
  let index = (explosion.y * width + explosion.x) as usize;
  if timer < tile_safety[index] { tile_safety[index] = timer; }
  for i in 0..explosion.left as usize {
    let index = index - (i + 1);
    if timer < tile_safety[index] { tile_safety[index] = timer; }
  }
  for i in 0..explosion.right as usize {
    let index = index + i + 1;
    if timer < tile_safety[index] { tile_safety[index] = timer; }
  }
  for i in 0..explosion.up as usize {
    let index = index - (i + 1) * width as usize;
    if timer < tile_safety[index] { tile_safety[index] = timer; }
  }
  for i in 0..explosion.down as usize {
    let index = index + (i + 1) * width as usize;
    if timer < tile_safety[index] { tile_safety[index] = timer; }
  }
}

fn eval_tile(powerup: i32, dist: i32, asb_count: i32, enemy_dist: i32, bomb_number: i32) -> i32 {
  let dist_val = if bomb_number > 0 { enemy_dist / 10 } else { cmp::max(0, 5 - enemy_dist / 10) };
  let x = powerup + 3 * asb_count + cmp::max(0, 10 - dist) + dist_val;

  x * x
}

fn rebuild_path(
  target: (i32, i32), player: &Player, accessible_tiles: &HashMap<(i32, i32), (i32, i32)>,
  tile_safety: &Vec<i32>, width: i32) -> (Vec<(i32, i32)>, i32) {
  let mut path = Vec::new();
  let mut node = target;
  while let Some(position) = accessible_tiles.get(&node) {
    path.push(node);
    node = *position;
  }

  let mut path_safety = tile_safety[(target.1 * width + target.0) as usize];
  if path_safety == SAFE_TILE {
    let ticks_per_tile = 60.0 / player.speed as f64;
    let mut ticks = ticks_per_tile / 2.0;
    for tile in path.iter().rev() {
      // When the player enters the tile, (ticks) ticks will have elapsed.
      // When the player leaves the tile, (ticks + ticks_per_tile) ticks will have elapsed.
      // The tile is unsafe between 0 and EXPLOSION_TIMER ticks.
      let timer = tile_safety[(tile.1 * width + tile.0) as usize];
      if timer >= ticks as i32 && timer - EXPLOSION_TIMER <= (ticks + ticks_per_tile) as i32 {
        if timer < path_safety { path_safety = timer; }
      }
      
      ticks += ticks_per_tile;
    }
  }

  (path, path_safety)
}
