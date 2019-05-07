use crate::geometry::Position;

pub struct Player {
    pub id: i32,
    pub position: (f64, f64),
    pub is_alive: bool,
    pub is_human: bool,
    pub speed: i32,
    pub bomb_number: i32,
    pub bomb_power: i32,
}

impl Player {
    pub fn new(id: i32, position: (f64, f64), is_human: bool) -> Player {
        Player {
            id: id,
            position: position,
            is_alive: true,
            is_human: is_human,
            speed: 5,
            bomb_number: 1,
            bomb_power: 1,
        }
    }
}

impl Position for Player {
    fn position(&self) -> (f64, f64) {
        self.position
    }
    fn set_position(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }
}