#[repr(u8)]
pub enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self { Direction::None }
}