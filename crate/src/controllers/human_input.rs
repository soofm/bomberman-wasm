use crate::geometry::Direction;
use crate::models::Actions;

#[derive(Default)]
pub struct HumanInput {
  pub left: bool,
  pub right: bool,
  pub up: bool,
  pub down: bool,
  pub bomb: bool,
  pub h: bool,
}

impl HumanInput {
  pub fn eval(&mut self) -> Actions {
    let dir_x: Option<Direction> = if self.left && !self.right {
      Some(Direction::Left)
    } else if self.right && !self.left {
      Some(Direction::Right)
    } else {
      None
    };

    let dir_y: Option<Direction> = if self.up && !self.down {
      Some(Direction::Up)
    } else if self.down && !self.up {
      Some(Direction::Down)
    } else {
      None
    };

    let dir: Option<Direction> = if dir_x != None && dir_y != None {
      if self.h {
        dir_x
      } else {
        dir_y
      }
    } else if dir_x != None {
      dir_x
    } else {
      dir_y
    };

    let bomb = self.bomb;
    self.bomb = false;

    Actions {
      direction: dir,
      place_bomb: bomb,
    }
  }
}