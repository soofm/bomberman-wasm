use crate::geometry::Direction;
use crate::models::{Actions, Player};

#[derive(Default)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub bomb: bool,
    pub h: bool,
}

pub fn eval(player: &Player, input_state: &InputState) -> Actions {
    let dir_x: Option<Direction> = if input_state.left && !input_state.right {
        Some(Direction::Left)
    } else if input_state.right && !input_state.left {
        Some(Direction::Right)
    } else {
        None
    };

    let dir_y: Option<Direction> = if input_state.up && !input_state.down {
        Some(Direction::Up)
    } else if input_state.down && !input_state.up {
        Some(Direction::Down)
    } else {
        None
    };

    let dir: Option<Direction> = if dir_x != None && dir_y != None {
        if input_state.h {
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
        place_bomb: input_state.bomb,
    }
}
