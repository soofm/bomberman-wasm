use super::{Tile, Tiles, Bombs, Player, Powerup};

const MAX_BOMBS: usize = 36;

pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: Tiles,
    pub bombs: Bombs,
    pub players: Vec<Player>,
    pub powerups: Vec<Powerup>,
}

impl World {
    pub fn new() -> World {
        // world setup. todo: get this from level
        let width: usize = 15;
        let height: usize = 11;
        let tiles = (0..width * height)
            .map(|i| {
                let x = i % width;
                let y = i / width;
                if y % 2 == 1 && x % 2 == 1 {
                    Tile::HardBlock
                } else if (x == 0 || x == width - 1) && (y < 3 || height - y <= 3) {
                    Tile::Empty
                } else if (y == 0 || y == height - 1) && (x < 3 || width - x <= 3) {
                    Tile::Empty
                } else {
                    Tile::SoftBlock
                }
            })
            .collect();
        let players = vec![
            Player::new(1, (0.0, 0.0), true),
            Player::new(2, ((width - 1) as f64, 0.0), false),
            Player::new(3, (0.0, (height - 1) as f64), false),
            Player::new(4, ((width - 1) as f64, (height - 1) as f64), false)
        ];
        
        World {
            width: width,
            height: height,
            tiles: Tiles::new(tiles, width, height),
            bombs: Bombs::new(MAX_BOMBS),
            players: players,
            powerups: vec![]
        }
    }
}
