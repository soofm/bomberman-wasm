#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    HardBlock,
    SoftBlock,
    Bomb { timer: u32, player_id: u32 }
}
