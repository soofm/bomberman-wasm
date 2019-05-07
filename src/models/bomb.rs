use super::{Tile, Tiles};

const EXPLOSION_TS: i32 = 180;
const DEAD_TS: i32 = 210;

#[derive(PartialEq, Eq)]
pub enum BombState {
    Armed,
    Exploding { left: i32, right: i32, up: i32, down: i32 },
    Dead
}

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

    pub fn update(&mut self, tiles: &mut Tiles) {
        self.timer += 1;
        match self.timer {
            0 => { self.state = BombState::Armed; },
            EXPLOSION_TS => {
                let e = self.explode(tiles);
                self.state = BombState::Exploding{ left: e.0, right: e.1, up: e.2, down: e.3 };
            },
            DEAD_TS => { self.state = BombState::Dead },
            _ => {}
        };
    }

    fn explode(&mut self, tiles: &mut Tiles) -> (i32, i32, i32, i32) {
        let ocol = self.position.0.round() as usize;
        let orow = self.position.1.round() as usize;
        let max_width = tiles.width - 1;
        let max_height = tiles.height - 1;

        let left = self.explode_ray(tiles, ocol, orow, |col, _row| { col > 0 }, |col, row| { (col - 1, row) });
        let right = self.explode_ray(tiles, ocol, orow, |col, _row| { col < max_width }, |col, row| { (col + 1, row) });
        let up = self.explode_ray(tiles, ocol, orow, |_col, row| { row > 0 }, |col, row| { (col, row - 1) });
        let down = self.explode_ray(tiles, ocol, orow, |_col, row| { row < max_height }, |col, row| { (col, row + 1) });
        (left, right, up, down)
    }

    fn explode_ray<F, G>(&mut self, tiles: &mut Tiles, ocol: usize, orow: usize, cond: F, next: G) -> i32
        where F: Fn(usize, usize) -> bool, G: Fn(usize, usize) -> (usize, usize) {
        let mut res = 0;
        let mut col = ocol;
        let mut row = orow;
        let mut found_tile: Option<Tile> = None;
        while cond(col, row) && res < self.power && found_tile == None {
            res += 1;
            let next_pos = next(col, row);
            col = next_pos.0;
            row = next_pos.1;
            let tile = tiles.tile(col, row);
            if tile != Tile::Empty { found_tile = Some(tile); }
        }
        match found_tile {
            Some(Tile::SoftBlock) => {
                tiles.set_tile(col, row, Tile::Empty);
                res
            },
            Some(Tile::HardBlock) => { res - 1 },
            _ => { res },
        }
    }
}

pub struct Bombs {
    pub bombs: Vec<Bomb>,
}

impl Bombs {
    pub fn new(max_size: usize) -> Self {
        Bombs {
            bombs: Vec::with_capacity(max_size),
        }
    }

    pub fn get(&self) -> &Vec<Bomb> {
        &self.bombs
    }

    pub fn add(&mut self, bomb: Bomb) {
        &self.bombs.push(bomb);
    }

    pub fn update(&mut self, tiles: &mut Tiles) {
        for bomb in self.bombs.iter_mut() {
            bomb.update(tiles);
        }
        
        while self.bombs.len() > 0 && self.bombs[0].state == BombState::Dead {
            self.bombs.remove(0);
        }
    }
}