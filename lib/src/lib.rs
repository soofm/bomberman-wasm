pub mod controllers;
pub mod geometry;
pub mod models;
mod utils;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use controllers::{Engine, render};
use geometry::Direction;
use lazy_static::lazy_static;
use models::World;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use self::controllers::Input;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new());
}

#[wasm_bindgen]
pub struct GameData {
    world: World,
    input: Input,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        GameData {
            world: World::new(),
            input: Input::default(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let mut engine = Engine::new();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let ctx = Box::new(ctx);

    render::render_bg(&canvas, &DATA.lock().unwrap().world);

    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let input = &mut DATA.lock().unwrap().input;
            match event.key().as_ref() {
                "w" => { input.pressed_up = true; input.last_direction = Some(Direction::Up); },
                "a" => { input.pressed_left = true; input.last_direction = Some(Direction::Left); },
                "s" => { input.pressed_down = true; input.last_direction = Some(Direction::Down); },
                "d" => { input.pressed_right = true; input.last_direction = Some(Direction::Right); },
                _ => {},
            };
        }) as Box<dyn FnMut(_)>);
        window().add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let input = &mut DATA.lock().unwrap().input;
            match event.key().as_ref() {
                "w" => input.pressed_up = false,
                "a" => input.pressed_left = false,
                "s" => input.pressed_down = false,
                "d" => input.pressed_right = false,
                "e" => input.place_bomb = true,
                _ => {},
            };
        }) as Box<dyn FnMut(_)>);
        window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let data: &mut GameData = &mut DATA.lock().unwrap();
        engine.update(&data.input, &mut data.world);
        data.input.place_bomb = false;
        render::render_frame(&ctx, &data.world);
        
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
