use crate::geometry::Direction;
use crate::models::{Actions, Player};

// input: i32
// i000 0000 00ab lrud
pub fn eval(player: &Player, input: i32) -> Actions {
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
