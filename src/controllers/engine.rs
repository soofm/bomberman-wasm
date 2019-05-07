use super::Input;
use crate::models::{Player, Tile, World};
use crate::geometry::LinearDirection;

const BOMB_TIMER: u32 = 300;

#[derive(Default)]
struct Actions {
    pub player_id: u32,
    pub direction_x: LinearDirection,
    pub direction_y: LinearDirection,
    pub place_bomb: bool,
}

pub fn update(input: &Input, world: &mut World) {
    let mut player_actions: Vec<Actions> = vec![];
    for player in &world.players {
        let actions = match player.is_human {
            true => eval_input(player, input),
            false => eval_cpu_actions(player, world)
        };
        player_actions.push(actions);
    }

    for (index, player) in world.players.iter_mut().enumerate() {
        let actions = &player_actions[index];
        let speed = player.speed as f64 / 60.0;
        
        let col = player.position.0.round() as usize;
        let row = player.position.1.round() as usize;
        let index = (row * world.width + col) as usize;

        if actions.place_bomb && world.tiles[index] == Tile::Empty {
            world.tiles[index] = Tile::Bomb{ timer: BOMB_TIMER, player_id: player.id };
        }

        // calculate new position
        let mut col = player.position.0 + speed * &actions.direction_x.as_f64();
        let mut row = player.position.1 + speed * &actions.direction_y.as_f64();
        
        if col < 0.0 { col = 0.0; }
        else if col > (world.width - 1) as f64 { col = (world.width - 1) as f64; }
        else if col.trunc() < player.position.0.trunc() {
            if world.tiles[index - 1] != Tile::Empty { col = player.position.0; }
        } else if col.trunc() > player.position.0.trunc() {
            if world.tiles[index + 1] != Tile::Empty { col = player.position.0; }
        }

        if row < 0.0 { row = 0.0; }
        else if row > (world.height - 1) as f64 { row = (world.height - 1) as f64; }
        else if row.trunc() < player.position.1.trunc() {
            if world.tiles[index - world.width] != Tile::Empty { row = player.position.1 }
        } else if row.trunc() > player.position.1.trunc() {
            if world.tiles[index + world.width] != Tile::Empty { row = player.position.1; }
        }

        player.position = (col, row);
    }
}

fn eval_input(player: &Player, input: &Input) -> Actions {
    let mut x: LinearDirection = LinearDirection::Zero;
    let mut y: LinearDirection = LinearDirection::Zero;
    if input.pressed_left && !input.pressed_right {
        x = LinearDirection::Negative;
    } else if input.pressed_right && !input.pressed_left {
        x = LinearDirection::Positive;
    }
    if input.pressed_up && !input.pressed_down {
        y = LinearDirection::Negative;
    } else if input.pressed_down && !input.pressed_up {
        y = LinearDirection::Positive;
    }
    let place_bomb = input.place_bomb;

    Actions {
        player_id: player.id,
        direction_x: x,
        direction_y: y,
        place_bomb,
    }
}

fn eval_cpu_actions(player: &Player, world: &World) -> Actions {
    // todo CPU AI
    Actions {
        player_id: player.id,
        direction_x: LinearDirection::Zero,
        direction_y: LinearDirection::Zero,
        place_bomb: false,
    }
}
