use crate::geometry::Position;

#[derive(Copy, Clone)]
pub struct Player {
    pub position: Position,
    pub is_alive: bool,
    pub speed: f64,
    pub bomb_number: u32,
    pub bomb_power: u32,
}

impl Player {
    pub fn new(position: Position) -> Player {
        Player {
            position: position,
            is_alive: true,
            speed: 0.1,
            bomb_number: 1,
            bomb_power: 1,
        }
    }
}
