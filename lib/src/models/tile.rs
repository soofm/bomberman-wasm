#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerupType {
  BombNumber,
  BombPower,
  Speed,
  Boots,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
  Empty,
  HardBlock,
  SoftBlock,
  Bomb,
  Powerup(PowerupType),
}
