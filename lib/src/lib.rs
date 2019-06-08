pub mod controllers;
pub mod geometry;
pub mod models;
mod utils;

use std::sync::Mutex;
use controllers::{render, game_state, Input, AIInput, HumanInput};
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
  human_inputs: [HumanInput; 4],
  ai_inputs: [AIInput; 4],
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
  pub fn new(num_humans: i32) -> Self {
    GameEngine {
      rng: rand::thread_rng(),
      human_inputs: Default::default(),
      ai_inputs: Default::default(),
    }
  }
  
  pub fn tick(&mut self) -> i32 {
      let world: &mut World = &mut WORLD.lock().unwrap();
      let mut actions: [Actions; 4] = Default::default();
      for (index, ((player, human_input), ai_input)) in world.players.iter()
        .zip(self.human_inputs.iter_mut())
        .zip(self.ai_inputs.iter_mut())
        .enumerate() {
        actions[index] = if player.is_human {
          human_input.eval(player, world)
        } else {
          ai_input.eval(player, world)
        }
      }
      
      game_state::update(&mut world.bombs, &mut world.players, &mut world.tiles, actions, &mut self.rng);

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
      let input = &mut self.human_inputs[player_id];
      match input_type {
        InputType::Left => {
          input.left = on;
          if on { input.h = true; }
        },
        InputType::Right => {
          input.right = on;
          if on { input.h = true; }
        },
        InputType::Up => {
          input.up = on;
          if on { input.h = true; }
        },
        InputType::Down => {
          input.down = on;
          if on { input.h = true; }
        },
        InputType::Bomb => {
          if !on { input.bomb = true; }
        },
      }
    }
  }

  pub fn reset(&self) {
    let world: &mut World = &mut WORLD.lock().unwrap();
    *world = World::new();
  }
}
