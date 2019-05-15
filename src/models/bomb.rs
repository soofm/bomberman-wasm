use crate::geometry::{InRange, Position};
use super::{Players, PowerupType, Tile, Tiles};
use rand::Rng;

const EXPLOSION_TS: i32 = 180;
const DEAD_TS: i32 = 210;

#[derive(Debug, PartialEq, Eq)]
pub enum BombState {
    Armed,
    Exploding { left: i32, right: i32, up: i32, down: i32 },
    Dead
}

#[derive(Debug)]
pub struct Bomb {
    pub player_id: i32,
    pub position: (f64, f64),
    pub power: i32,
    pub state: BombState,
    timer: i32,
}

impl Bomb {
    pub fn new(player_id: i32, power: i32, position: (f64, f64)) -> Self {
        Bomb {
            player_id: player_id,
            position: position,
            power: power,
            state: BombState::Armed,
            timer: 0,
        }
    }

    pub fn update<R: Rng>(&mut self, players: &mut Players, tiles: &mut Tiles, rng: &mut R) {
        self.timer += 1;
        match self.timer {
            0 => { self.state = BombState::Armed; },
            EXPLOSION_TS => {
                let e = self.explode(tiles, rng);
                players.recycle_bomb(self.player_id);
                self.state = BombState::Exploding{ left: e.0, right: e.1, up: e.2, down: e.3 };
            },
            DEAD_TS => { self.state = BombState::Dead },
            _ => {}
        };
    }

    fn explode<R: Rng>(&mut self, tiles: &mut Tiles, rng: &mut R) -> (i32, i32, i32, i32) {
        let ocol = self.position.0.round() as i32;
        let orow = self.position.1.round() as i32;
        let max_width = tiles.width - 1;
        let max_height = tiles.height - 1;

        let left = self.explode_ray(tiles, ocol, orow, |col, _row| { col > 0 }, |col, row| { (col - 1, row) }, rng);
        let right = self.explode_ray(tiles, ocol, orow, |col, _row| { col < max_width }, |col, row| { (col + 1, row) }, rng);
        let up = self.explode_ray(tiles, ocol, orow, |_col, row| { row > 0 }, |col, row| { (col, row - 1) }, rng);
        let down = self.explode_ray(tiles, ocol, orow, |_col, row| { row < max_height }, |col, row| { (col, row + 1) }, rng);
        (left, right, up, down)
    }

    fn explode_ray<F, G, R: Rng>(&mut self, tiles: &mut Tiles, ocol: i32, orow: i32, cond: F, next: G, rng: &mut R) -> i32
        where F: Fn(i32, i32) -> bool, G: Fn(i32, i32) -> (i32, i32) {
        let mut res = 0;
        let mut pos = (ocol, orow);
        let mut found_tile: Option<Tile> = None;
        while cond(pos.0, pos.1) && res < self.power && found_tile == None {
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
}

impl Position for Bomb {
    fn position(&self) -> (f64, f64) {
        self.position
    }
    fn set_position(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }
}
