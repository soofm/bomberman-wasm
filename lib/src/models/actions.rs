use crate::geometry::Direction;

#[derive(Default)]
pub struct Actions {
    pub player_id: i32,
    pub direction: Option<Direction>,
    pub place_bomb: bool,
}