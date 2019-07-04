use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
  Empty,
  HardBlock,
  SoftBlock,
  PowerupBombNumber,
  PowerupBombPower,
  PowerupSpeed,
  PowerupBoots,
}
