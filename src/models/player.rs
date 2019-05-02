use crate::geometry::Position;

pub struct Player {
    pub id: u32,
    pub is_alive: bool,
    pub position: Position,
}

impl Player {
    pub fn new(id: u32, position: Position) -> Player {
        Player {
            id: id,
            is_alive: true,
            position: position
        }
    }
}
