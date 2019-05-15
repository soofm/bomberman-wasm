use crate::geometry::LinearDirection;

#[derive(Default)]
pub struct Actions {
    pub player_id: i32,
    pub direction_x: LinearDirection,
    pub direction_y: LinearDirection,
    pub place_bomb: bool,
}