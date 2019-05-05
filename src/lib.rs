mod controllers;
mod geometry;
mod models;
mod game_state;
mod utils;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use controllers::{engine, render};
use game_state::GameState;
use lazy_static::lazy_static;
use geometry::Direction;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use self::controllers::Actions;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new());
}

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
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let ctx = Box::new(ctx);

    render::render_bg(&canvas, &ctx, &DATA.lock().unwrap().state.world);

    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let actions = &mut DATA.lock().unwrap().actions;
            match event.key().as_ref() {
                "w" => actions.pressed_up = true,
                "a" => actions.pressed_left = true,
                "s" => actions.pressed_down = true,
                "d" => actions.pressed_right = true,
                _ => {},
            };
        }) as Box<dyn FnMut(_)>);
        window().add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let actions = &mut DATA.lock().unwrap().actions;
            match event.key().as_ref() {
                "w" => actions.pressed_up = false,
                "a" => actions.pressed_left = false,
                "s" => actions.pressed_down = false,
                "d" => actions.pressed_right = false,
                "e" => actions.place_bomb = true,
                _ => {},
            };
        }) as Box<dyn FnMut(_)>);
        window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let data = &mut DATA.lock().unwrap();
        engine::update(data);
        render::render_frame(&ctx, &data.state.world);
        
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
