use super::bomb_controller;
use crate::models::{Actions, Bomb, BombState, Player, Tile, Tiles, World};
use crate::geometry::{Direction, Position};
 
pub fn update(bombs: &mut Vec<Bomb>, players: &mut [Player; 4], tiles: &mut Tiles, actions: [Actions; 4]) {
    for (player, action) in players.iter_mut().zip(actions.iter()) {
        if !player.is_alive { continue; }

        let dist = player.speed as f64 / 60.0;
        
        if let Some(dir) = action.direction {
            player.move_in_direction(dir, dist, tiles);
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

        if player.bomb_number > 0 && action.place_bomb && !tiles.is_blocked(col, row) {
            player.bomb_number -= 1;
            bomb_controller::add_bomb(bombs, player, tiles);
        }

        if let Tile::Powerup(powerup) = tiles.get(col, row) {
            player.apply_powerup(powerup);
            tiles.set(col, row, Tile::Empty);
        }
    }
}

pub fn recycle_bomb(players: &mut [Player], player_id: i32) {
    let player = players.iter_mut().find(|player| player.id == player_id);
    if let Some(mut player) = player {
        player.bomb_number += 1;
    }
}

// input: i32
// i000 0000 00ab lrud
pub fn eval_input(player: &Player, input: i32) -> Actions {
    let bomb = input & 16 != 0;
    let left = input & 8 != 0;
    let right = input & 4 != 0;
    let up = input & 2 != 0;
    let down = input & 1 != 0;

    let dir_x: Option<Direction> = if left && !right {
        Some(Direction::Left)
    } else if right && !left {
        Some(Direction::Right)
    } else {
        None
    };

    let dir_y: Option<Direction> = if up && !down {
        Some(Direction::Up)
    } else if down && !up {
        Some(Direction::Down)
    } else {
        None
    };

    let dir: Option<Direction> = if dir_x != None && dir_y != None {
        if input & 32 != 0 {
            dir_x
        } else {
            dir_y
        }
    } else if dir_x != None {
        dir_x
    } else {
        dir_y
    };

    Actions {
        player_id: player.id,
        direction: dir,
        place_bomb: bomb,
    }
}

pub fn eval_cpu_actions(player: &Player, world: &World) -> Actions {
    // todo CPU AI
    Actions {
        player_id: player.id,
        direction: None,
        place_bomb: false,
    }
}
