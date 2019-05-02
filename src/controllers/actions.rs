use crate::geometry::Direction;

#[derive(Default)]
pub struct Actions {
    pub direction: Direction,
    pub place_bomb: bool,
}
