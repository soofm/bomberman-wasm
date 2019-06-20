pub mod controllers;
pub mod geometry;
pub mod models;
mod utils;

use controllers::{game_state, level, AIInput, HumanInput};
use models::{Actions, Tile, World};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
  utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct GameEngine {
  world: World,
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
  pub fn new() -> Self {
    let (tiles, players) = level::build_level();
    GameEngine {
      world: World::new(tiles, players),
      rng: rand::thread_rng(),
      human_inputs: Default::default(),
      ai_inputs: Default::default(),
    }
  }
  
  pub fn tick(&mut self) -> i32 {
    let world = &mut self.world;
    let mut actions: [Actions; 4] = Default::default();
    for (index, ((player, human_input), ai_input)) in world.players.iter()
      .zip(self.human_inputs.iter_mut())
      .zip(self.ai_inputs.iter_mut())
      .enumerate() {
      actions[index] = if player.is_human {
        human_input.eval()
      } else {
        ai_input.eval(player, index, world, &mut self.rng)
      }
    }
    
    game_state::update(world, actions, &mut self.rng);

    let mut winner_id: i32 = -1;
    for (index, player) in world.players.iter().enumerate() {
      if player.is_alive {
        if winner_id > 0 {
          winner_id = 0;
          break;
        }
        winner_id = (index as i32) + 1;
      }
    }
    winner_id
  }

  pub fn get_tiles_ptr(&self) -> *const Tile {
    self.world.tiles.get_tiles_ptr()
  }

  pub fn render_players(&self, render_player: &js_sys::Function) {
    let this = JsValue::NULL;
    for (index, player) in self.world.players.iter().enumerate() {
      let player_data = (if player.is_alive { 1 } else { 0 } << 8) + index as i32;
      render_player.call3(&this, &JsValue::from(player_data), &JsValue::from(player.x), &JsValue::from(player.y)).unwrap();
    }
  }

  pub fn render_bombs(&mut self, add_bomb: &js_sys::Function, move_bomb: &js_sys::Function, remove_bomb: &js_sys::Function) {
    let this = JsValue::NULL;
    let mut i = 0;
    for bomb in self.world.bombs.iter() {
      match bomb.timer {
        0 => { remove_bomb.call0(&JsValue::NULL).unwrap(); },
        models::BOMB_TIMER => { add_bomb.call2(&this, &JsValue::from(bomb.x), &JsValue::from(bomb.y)).unwrap(); },
        _ => {
          move_bomb.call3(&this, &JsValue::from(i), &JsValue::from(bomb.x), &JsValue::from(bomb.y)).unwrap();
          i += 1;
        },
      };
    }
  }

  pub fn render_explosions(&mut self, add_explosion: &js_sys::Function, remove_explosion: &js_sys::Function) {
    let this = JsValue::NULL;
    for explosion in self.world.explosions.iter() {
      match explosion.timer {
        0 => { remove_explosion.call0(&JsValue::NULL).unwrap(); },
        models::EXPLOSION_TIMER => {
          let explosion_size = (explosion.left << 24) + (explosion.right << 16) + (explosion.up << 8) + (explosion.down);
          add_explosion.call3(&this, &JsValue::from(explosion_size), &JsValue::from(explosion.x), &JsValue::from(explosion.y)).unwrap();
        },
        _ => {},
      };
    }
  }

  pub fn set_human_player(&mut self, player_id: usize) {
    self.world.players[player_id].is_human = true;
  }

  pub fn send_input(&mut self, player_id: usize, input_type: InputType, on: bool) {
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
