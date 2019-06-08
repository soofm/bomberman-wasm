pub mod controllers;
pub mod geometry;
pub mod models;
mod utils;

use std::sync::Mutex;
use controllers::{ai, input, render, state};
use models::{Actions, World};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct GameEngine {
    rng: rand::rngs::ThreadRng,
    input_state: [input::InputState; 4]
}

#[wasm_bindgen]
pub enum InputType {
    Left,
    Right,
    Up,
    Down,
    Bomb,
}

#[wasm_bindgen]
impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            rng: rand::thread_rng(),
            input_state: Default::default(),
        }
    }    
    
    pub fn tick(&mut self) -> i32 {
        let world: &mut World = &mut WORLD.lock().unwrap();
        let mut actions: [Actions; 4] = Default::default();
        for (index, (player, input_state)) in world.players.iter().zip(self.input_state.iter_mut()).enumerate() {
            if player.is_human {
                actions[index] = input::eval(player, input_state);
                input_state.bomb = false;
            } else {
                actions[index] = ai::eval(player, world);
            }
        }
        
        state::update(&mut world.bombs, &mut world.players, &mut world.tiles, actions, &mut self.rng);

        let mut winner_id: i32 = -1;
        for player in world.players.iter() {
            if player.is_alive {
                if winner_id > 0 {
                    winner_id = 0;
                    break;
                }
                winner_id = player.id;
            }
        }
        winner_id
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let world: &World = &WORLD.lock().unwrap();
        render::render_frame(ctx, world);
    }
    
    pub fn set_human_player(&self, player_id: usize) {
        if player_id < 4 {
            let world: &mut World = &mut WORLD.lock().unwrap();
            world.players[player_id].is_human = true;
        }
    }

    pub fn send_input(&mut self, player_id: usize, input_type: InputType, on: bool) {
        if player_id < 4 {
            let input_state = &mut self.input_state[player_id];
            match input_type {
                InputType::Left => {
                    input_state.left = on;
                    if on { input_state.h = true; }
                },
                InputType::Right => {
                    input_state.right = on;
                    if on { input_state.h = true; }
                },
                InputType::Up => {
                    input_state.up = on;
                    if on { input_state.h = true; }
                },
                InputType::Down => {
                    input_state.down = on;
                    if on { input_state.h = true; }
                },
                InputType::Bomb => {
                    if !on { input_state.bomb = true; }
                },
            }
        }
    }

    pub fn reset(&self) {
        let world: &mut World = &mut WORLD.lock().unwrap();
        *world = World::new();
    }
}
