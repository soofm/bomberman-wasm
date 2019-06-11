pub const EXPLOSION_TIMER: i32 = 30;

#[derive(Debug)]
pub struct Explosion {
  pub player_id: usize,
  pub x: f64,
  pub y: f64,
  pub left: i32,
  pub right: i32,
  pub up: i32,
  pub down: i32,
  pub timer: i32,
}

impl Explosion {
  pub fn new(player_id: usize, x: f64, y: f64, left: i32, right: i32, up: i32, down: i32) -> Self {
    Explosion {
      player_id: player_id,
      x: x,
      y: y,
      left: left,
      right: right,
      up: up,
      down: down,
      timer: EXPLOSION_TIMER,
    }
  }
}