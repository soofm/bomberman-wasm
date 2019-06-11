use crate::geometry::Direction;

#[derive(Default)]
pub struct Actions {
  pub direction: Option<Direction>,
  pub place_bomb: bool,
}