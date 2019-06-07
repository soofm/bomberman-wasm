use crate::geometry::{InRange, Position};
use crate::models::{Actions, Bomb, BombState, Player, PowerupType, Tile, Tiles};
use rand::{Rng, RngCore};

const EXPLOSION_TS: i32 = 180;
const DEAD_TS: i32 = 210;

pub fn update<R: RngCore>(bombs: &mut Vec<Bomb>, players: &mut [Player; 4], tiles: &mut Tiles, actions: [Actions; 4], rng: &mut R) {
    update_players(bombs, players, tiles, actions);
    update_bombs(bombs, players, tiles, rng);
}

pub fn update_players(bombs: &mut Vec<Bomb>, players: &mut [Player; 4], tiles: &mut Tiles, actions: [Actions; 4]) {
    for (player, action) in players.iter_mut().zip(actions.iter()) {
        if !player.is_alive { continue; }

        let dist = player.speed as f64 / 60.0;
        
        if let Some(dir) = action.direction {
            player.move_in_direction(dir, dist, tiles);
        }

        // detect bomb collisions
        let col = player.position.0.round() as i32;
        let row = player.position.1.round() as i32;
        for bomb in bombs.iter() {
            let bx = bomb.position.0.round() as i32;
            let by = bomb.position.1.round() as i32;
            if bx == col || by == row {
                match bomb.state {
                    BombState::Exploding{ left, right, up, down } => {
                        if by == row && (col >= bx - left && col <= bx + right) ||
                            bx == col && (row >= by - up && row <= by + down) {
                                player.is_alive = false;
                            }
                    },
                    _ => {},
                };
            }
        }

        let col = player.position.0.round() as i32;
        let row = player.position.1.round() as i32;

        if player.bomb_number > 0 && action.place_bomb && !tiles.is_blocked(col, row) {
            player.bomb_number -= 1;
            add_bomb(bombs, player, tiles);
        }

        if let Tile::Powerup(powerup) = tiles.get(col, row) {
            player.apply_powerup(powerup);
            tiles.set(col, row, Tile::Empty);
        }
    }
}

fn recycle_bomb(players: &mut [Player], player_id: i32) {
    let player = players.iter_mut().find(|player| player.id == player_id);
    if let Some(mut player) = player {
        player.bomb_number += 1;
    }
}

fn add_bomb(bombs: &mut Vec<Bomb>, player: &Player, tiles: &mut Tiles) {
    let position = (player.position.0.round(), player.position.1.round());
    bombs.push(Bomb::new(player.id, player.bomb_power, position));
    tiles.set(position.0 as i32, position.1 as i32, Tile::Bomb);
}

pub fn update_bombs<R: RngCore>(bombs: &mut Vec<Bomb>, players: &mut [Player; 4], tiles: &mut Tiles, rng: &mut R) {
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
            recycle_bomb(players, bomb.player_id);
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

    let left = explode_ray(bomb, tiles, ocol, orow, -1, 0, rng);
    let right = explode_ray(bomb, tiles, ocol, orow, 1, 0, rng);
    let up = explode_ray(bomb, tiles, ocol, orow, 0, -1, rng);
    let down = explode_ray(bomb, tiles, ocol, orow, 0, 1, rng);
    (left, right, up, down)
}

fn explode_ray<R: RngCore>(bomb: &mut Bomb, tiles: &mut Tiles, ocol: i32, orow: i32, x: i32, y: i32, rng: &mut R) -> i32 {
    let mut res = 0;
    let mut col = ocol;
    let mut row = orow;
    let mut found_tile: Option<Tile> = None;
    while res < bomb.power && found_tile == None {
        res += 1;
        col += x;
        row += y;
        if col < 0 || col >= tiles.width || row < 0 || row >= tiles.height { break; }
        else if tiles.is_blocked(col, row) { found_tile = Some(tiles.get(col, row)); }
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
            tiles.set(col, row, tile);
            res
        },
        Some(Tile::HardBlock) => { res - 1 },
        _ => { res },
    }
}