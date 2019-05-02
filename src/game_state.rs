use rand::prelude::*;

use super::models::World;

pub struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new() -> GameState {
        let mut rng = rand::thread_rng();
        GameState {
            world: World::new(&mut rng)
        }
    }

    //pub fn reset(&mut self) {
    //    
    //}
}