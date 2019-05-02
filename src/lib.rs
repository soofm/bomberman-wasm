mod controllers;
mod geometry;
mod models;
mod game_state;
mod utils;

use game_state::GameState;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

use models::Tile;
use geometry::Position;
use self::controllers::Actions;

#[wasm_bindgen]
pub struct GameData {
    state: GameState,
    actions: Actions,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        GameData {
            state: GameState::new(),
            actions: Actions::default(),
        }
    }

    pub fn width(&self) -> u32 {
        self.state.world.width
    }

    pub fn height(&self) -> u32 {
        self.state.world.height
    }

    pub fn tiles(&self) -> *const Tile {
        self.state.world.tiles.as_ptr()
    }

    pub fn player(&self, id: usize) -> Position {
        self.state.world.players[id].position
    }

    //pub fn tick(&mut self) {
    //    
    //}
}
