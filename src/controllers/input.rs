use crate::geometry::Direction;

#[derive(Default)]
pub struct Input {
    pub pressed_up: bool,
    pub pressed_right: bool,
    pub pressed_down: bool,
    pub pressed_left: bool,
    pub place_bomb: bool,
    pub last_direction: Option<Direction>,
}
