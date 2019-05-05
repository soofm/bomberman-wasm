use crate::geometry::Position;

pub struct Bomb {
    pub position: Position,
    pub timer: u32,
    pub owner_id: u32
}

impl Bomb {
    pub fn new(position: Position, owner_id: u32) -> Bomb {
        Bomb {
            position: position,
            timer: 3,
            owner_id: owner_id
        }
    }
}