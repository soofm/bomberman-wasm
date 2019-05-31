use super::player_controller;
use crate::geometry::InRange;
use crate::models::{Bomb, BombState, Player, PowerupType, Tile, Tiles};
use rand::{Rng, RngCore};

const EXPLOSION_TS: i32 = 180;
const DEAD_TS: i32 = 210;

pub fn add_bomb(bombs: &mut Vec<Bomb>, player: &Player, tiles: &mut Tiles) {
    let position = (player.position.0.round(), player.position.1.round());
    bombs.push(Bomb::new(player.id, player.bomb_power, position));
    tiles.set(position.0 as i32, position.1 as i32, Tile::Bomb);
}

pub fn update<R: RngCore>(bombs: &mut Vec<Bomb>, players: &mut [Player; 4], tiles: &mut Tiles, rng: &mut R) {
    for bomb in bombs.iter_mut() {
        update_bomb(bomb, players, tiles, rng);
    }
    
    while bombs.len() > 0 && bombs[0].state == BombState::Dead {
        bombs.remove(0);
    }
}

fn update_bomb<R: RngCore>(bomb: &mut Bomb, players: &mut [Player; 4], tiles: &mut Tiles, rng: &mut R) {
    bomb.timer += 1;
    match bomb.timer {
        0 => { bomb.state = BombState::Armed; },
        EXPLOSION_TS => {
            let e = explode(bomb, tiles, rng);
            player_controller::recycle_bomb(players, bomb.player_id);
            bomb.state = BombState::Exploding{ left: e.0, right: e.1, up: e.2, down: e.3 };
        },
        DEAD_TS => { bomb.state = BombState::Dead },
        _ => {}
    };
}

fn explode<R: RngCore>(bomb: &mut Bomb, tiles: &mut Tiles, rng: &mut R) -> (i32, i32, i32, i32) {
    let ocol = bomb.position.0.round() as i32;
    let orow = bomb.position.1.round() as i32;

    tiles.set(ocol, orow, Tile::Empty);

    let max_width = tiles.width - 1;
    let max_height = tiles.height - 1;

    let left = explode_ray(bomb, tiles, ocol, orow, |col, _row| { col > 0 }, |col, row| { (col - 1, row) }, rng);
    let right = explode_ray(bomb, tiles, ocol, orow, |col, _row| { col < max_width }, |col, row| { (col + 1, row) }, rng);
    let up = explode_ray(bomb, tiles, ocol, orow, |_col, row| { row > 0 }, |col, row| { (col, row - 1) }, rng);
    let down = explode_ray(bomb, tiles, ocol, orow, |_col, row| { row < max_height }, |col, row| { (col, row + 1) }, rng);
    (left, right, up, down)
}

fn explode_ray<F, G, R: RngCore>(bomb: &mut Bomb, tiles: &mut Tiles, ocol: i32, orow: i32, cond: F, next: G, rng: &mut R) -> i32
    where F: Fn(i32, i32) -> bool, G: Fn(i32, i32) -> (i32, i32) {
    let mut res = 0;
    let mut pos = (ocol, orow);
    let mut found_tile: Option<Tile> = None;
    while cond(pos.0, pos.1) && res < bomb.power && found_tile == None {
        res += 1;
        pos = next(pos.0, pos.1);
        if tiles.is_blocked(pos.0, pos.1) { found_tile = Some(tiles.get(pos.0, pos.1)); }
    }
    match found_tile {
        Some(Tile::SoftBlock) => {
            let n = rng.gen::<f64>();
            let tile = match n {
                x if x.in_range(0.0, 0.5) => Tile::Empty,
                x if x.in_range(0.5, 0.65) => Tile::Powerup(PowerupType::BombNumber),
                x if x.in_range(0.65, 0.8) => Tile::Powerup(PowerupType::BombPower),
                x if x.in_range(0.8, 0.95) => Tile::Powerup(PowerupType::Speed),
                x if x.in_range(0.95, 1.0) => Tile::Powerup(PowerupType::Boots),
                _ => Tile::Empty,
            };
            tiles.set(pos.0, pos.1, tile);
            res
        },
        Some(Tile::HardBlock) => { res - 1 },
        _ => { res },
    }
}