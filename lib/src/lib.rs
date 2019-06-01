pub mod controllers;
pub mod geometry;
pub mod models;
mod utils;

use std::sync::Mutex;
use controllers::{bomb_controller, player_controller, render};
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
}

#[wasm_bindgen]
impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            rng: rand::thread_rng(),
        }
    }    
    
    pub fn tick(&mut self, input: &[i32]) -> i32 {
        let mut safe_input: [i32; 4] = Default::default();
        safe_input.copy_from_slice(input);

        let world: &mut World = &mut WORLD.lock().unwrap();
        let mut actions: [Actions; 4] = Default::default();
        for (index, (player, &input)) in world.players.iter().zip(input.iter()).enumerate() {
            actions[index] = if input >= 0 {
                player_controller::eval_input(player, input)
            } else {
                player_controller::eval_cpu_actions(player, world)
            };
        }

        // handle player movement and collisions
        player_controller::update(&mut world.bombs, &mut world.players, &mut world.tiles, actions);

        // handle bombs
        bomb_controller::update(&mut world.bombs, &mut world.players, &mut world.tiles, &mut self.rng);

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
    
    pub fn reset(&self) {
        let world: &mut World = &mut WORLD.lock().unwrap();
        *world = World::new();
    }
}
