use super::{Tile, Bomb, Player, Powerup};
use crate::geometry::Position;

pub struct World {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
    pub bombs: Vec<Bomb>,
    pub players: Vec<Player>,
    pub powerups: Vec<Powerup>,
}

impl World {
    pub fn new() -> World {
        // world setup. todo: get this from level
        let width = 15;
        let height = 11;
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
            Player::new(Position::new(0.0, 0.0)),
            Player::new(Position::new((width - 1) as f64, 0.0)),
            Player::new(Position::new(0.0, (height - 1) as f64)),
            Player::new(Position::new((width - 1) as f64, (height - 1) as f64))
        ];
        
        World {
            width: width,
            height: height,
            tiles: tiles,
            players: players,
            bombs: vec![],
            powerups: vec![]
        }
    }
}
