#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    HardBlock,
    SoftBlock,
}

pub struct Tiles {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Tiles {
    pub fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Tiles {
            tiles: tiles,
            width: width,
            height: height,
        }
    }

    pub fn tile(&self, col: usize, row: usize) -> Tile {
        self.tiles[row * self.width + col]
    }

    pub fn is_blocked(&self, col: i32, row: i32) -> bool {
        if col < 0 || row < 0 { return true; }
        let col = col as usize;
        let row = row as usize;
        col >= self.width || row >= self.height || self.tiles[row * self.width + col] != Tile::Empty
    }

    pub fn set_tile(&mut self, col: usize, row: usize, value: Tile) {
        self.tiles[row * self.width + col] = value;
    }
}