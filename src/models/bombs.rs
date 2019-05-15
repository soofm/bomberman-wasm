use super::{Bomb, BombState, Players, Tiles};
use rand::RngCore;

pub struct Bombs {
    bombs: Vec<Bomb>,
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

    pub fn update<R: RngCore>(&mut self, players: &mut Players, tiles: &mut Tiles, rng: &mut R) {
        for bomb in self.bombs.iter_mut() {
            bomb.update(players, tiles, rng);
        }
        
        while self.bombs.len() > 0 && self.bombs[0].state == BombState::Dead {
            self.bombs.remove(0);
        }
    }
}
