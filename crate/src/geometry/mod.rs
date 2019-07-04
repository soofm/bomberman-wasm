mod direction;
mod entity;
mod in_range;

pub use self::direction::Direction;
pub use self::entity::Entity;
pub use self::in_range::InRange;

pub const CORNER: f64 = 1.0 / 3.0;
