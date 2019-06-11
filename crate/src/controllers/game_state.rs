use crate::geometry::{InRange, Position};
use crate::models::{Actions, Bomb, Explosion, Player, Tile, Tiles, World};
use rand::{Rng, RngCore};

pub fn update<R: RngCore>(world: &mut World, actions: [Actions; 4], rng: &mut R) {
  // run ticks before any new items are added
  for bomb in world.bombs.iter_mut() {
    bomb.timer -= 1;
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
      let ex = explosion.x.round() as i32;
      let ey = explosion.y.round() as i32;
      if ey == row && (col >= ex - explosion.left && col <= ex + explosion.right) ||
        ex == col && (row >= ey - explosion.up && row <= ey + explosion.down) {
          player.is_alive = false;
      }
    }

    let col = player.x.round() as i32;
    let row = player.y.round() as i32;

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
    if bomb.timer == 0 {
      let (left, right, up, down) = explode(&bomb, &mut world.tiles, rng);
      world.players[bomb.player_id].bomb_number += 1;
      world.explosions.push(Explosion::new(bomb.player_id, bomb.x, bomb.y, left, right, up, down));
    } else {
      break;
    }
  }
}

fn explode<R: RngCore>(bomb: &Bomb, tiles: &mut Tiles, rng: &mut R) -> (i32, i32, i32, i32) {
  let ocol = bomb.x.round() as i32;
  let orow = bomb.y.round() as i32;

  tiles.set(ocol, orow, Tile::Empty);

  let left = explode_ray(bomb, tiles, ocol, orow, -1, 0, rng);
  let right = explode_ray(bomb, tiles, ocol, orow, 1, 0, rng);
  let up = explode_ray(bomb, tiles, ocol, orow, 0, -1, rng);
  let down = explode_ray(bomb, tiles, ocol, orow, 0, 1, rng);
  (left, right, up, down)
}

fn explode_ray<R: RngCore>(bomb: &Bomb, tiles: &mut Tiles, ocol: i32, orow: i32, x: i32, y: i32, rng: &mut R) -> i32 {
  let mut res = 0;
  let mut col = ocol;
  let mut row = orow;
  let mut found_tile: Option<Tile> = None;
  while res < bomb.power && found_tile != Some(Tile::HardBlock) && found_tile != Some(Tile::SoftBlock) {
    res += 1;
    col += x;
    row += y;
    if col < 0 || col >= tiles.width || row < 0 || row >= tiles.height { break; }
    found_tile = Some(tiles.get(col, row));
  }
  match found_tile {
    Some(Tile::SoftBlock) => {
      let n = rng.gen::<f64>();
      let tile = match n {
        x if x.in_range(0.0, 0.5) => Tile::Empty,
        x if x.in_range(0.5, 0.65) => Tile::PowerupBombNumber,
        x if x.in_range(0.65, 0.8) => Tile::PowerupBombPower,
        x if x.in_range(0.8, 0.95) => Tile::PowerupSpeed,
        x if x.in_range(0.95, 1.0) => Tile::PowerupBoots,
        _ => Tile::Empty,
      };
      tiles.set(col, row, tile);
      res
    },
    Some(Tile::HardBlock) | None => { res - 1 },
    _ => { res },
  }
}