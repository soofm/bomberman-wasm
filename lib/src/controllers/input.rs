use crate::models::{Actions, Player, World};

pub trait Input {
  fn eval(&mut self, player: &Player, world: &World) -> Actions;
}
