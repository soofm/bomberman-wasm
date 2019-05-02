use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x: x, y: y }
    }
}