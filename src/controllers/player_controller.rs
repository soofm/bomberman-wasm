use super::bomb_controller;
use crate::models::{Actions, Bomb, BombState, Player, Tile, Tiles};
use crate::geometry::Position;
 
pub fn update(actions: Vec<Actions>, players: &mut Vec<Player>, bombs: &mut Vec<Bomb>, tiles: &mut Tiles) {
    for (index, player) in players.iter_mut().enumerate() {
        if !player.is_alive { continue; }

        let actions = &actions[index];
        let dist = player.speed as f64 / 60.0;
        
        if let Some(dir) = actions.direction {
            player.move_in_direction(dir, dist, &tiles);
        }

        // detect bomb collisions
        let col = player.position.0.round() as i32;
        let row = player.position.1.round() as i32;
        for bomb in bombs.iter() {
            let bx = bomb.position.0.round() as i32;
            let by = bomb.position.1.round() as i32;
            if bx == col || by == row {
                match bomb.state {
                    BombState::Exploding{ left, right, up, down } => {
                        if by == row && (col >= bx - left && col <= bx + right) ||
                            bx == col && (row >= by - up && row <= by + down) {
                                player.is_alive = false;
                            }
                    },
                    _ => {},
                };
            }
        }

        let col = player.position.0.round() as i32;
        let row = player.position.1.round() as i32;

        if player.bomb_number > 0 && actions.place_bomb && !tiles.is_blocked(col, row) {
            player.bomb_number -= 1;
            bomb_controller::add_bomb(bombs, player, tiles);
        }

        if let Tile::Powerup(powerup) = tiles.get(col, row) {
            player.apply_powerup(powerup);
            tiles.set(col, row, Tile::Empty);
        }
    }
}

pub fn recycle_bomb(players: &mut Vec<Player>, player_id: i32) {
    let player = players.iter_mut().find(|player| player.id == player_id);
    if let Some(mut player) = player {
        player.bomb_number += 1;
    }
}
