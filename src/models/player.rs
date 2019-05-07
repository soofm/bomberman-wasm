pub struct Player {
    pub id: u32,
    pub position: (f64, f64),
    pub is_alive: bool,
    pub is_human: bool,
    pub speed: u32,
    pub bomb_number: u32,
    pub bomb_power: u32,
}

impl Player {
    pub fn new(id: u32, position: (f64, f64), is_human: bool) -> Player {
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
