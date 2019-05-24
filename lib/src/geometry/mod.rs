mod direction;
mod in_range;
mod position;

pub use self::direction::Direction;
pub use self::in_range::InRange;
pub use self::position::Position;

pub const CORNER: f64 = 1.0 / 3.0;
