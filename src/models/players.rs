use super::{Actions, Bomb, Bombs, Player, Tile, Tiles};
use crate::geometry::Position;

pub struct Players {
    players: Vec<Player>,
}

impl Players {
    pub fn new(players: Vec<Player>) -> Self {
        Players {
            players: players,
        }
    }

    pub fn get(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn update(&mut self, actions: Vec<Actions>, bombs: &mut Bombs, tiles: &mut Tiles) {
        for (index, player) in self.players.iter_mut().enumerate() {
            let actions = &actions[index];
            let speed = player.speed as f64 / 60.0;
            
            let col = player.position.0.round();
            let row = player.position.1.round();

            if player.bomb_number > 0 && actions.place_bomb && !tiles.is_blocked(col as i32, row as i32) {
                player.bomb_number -= 1;
                bombs.add(Bomb::new(player.id, player.bomb_power, (col, row)));
            }

            // calculate new position
            let x = player.position.0 + speed * &actions.direction_x.as_f64();
            let y = player.position.1 + speed * &actions.direction_y.as_f64();
            
            player.move_to(x, y, &tiles);

            let col = player.position.0.round() as i32;
            let row = player.position.1.round() as i32;

            if let Tile::Powerup(powerup) = tiles.get(col, row) {
                player.apply_powerup(powerup);
                tiles.set(col, row, Tile::Empty);
            }
        }
    }

    pub fn recycle_bomb(&mut self, player_id: i32) {
        let player = self.players.iter_mut().find(|player| player.id == player_id);
        if let Some(mut player) = player {
            player.bomb_number += 1;
        }
    }
}
