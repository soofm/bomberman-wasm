use super::Input;
use crate::models::{Bomb, Player, Tile, World};
use crate::geometry::{LinearDirection, Position};

#[derive(Default)]
struct Actions {
    pub player_id: i32,
    pub direction_x: LinearDirection,
    pub direction_y: LinearDirection,
    pub place_bomb: bool,
}

pub fn update(input: &Input, world: &mut World) {
    // get actions
    let mut player_actions: Vec<Actions> = vec![];
    for player in &world.players {
        let actions = match player.is_human {
            true => eval_input(player, input),
            false => eval_cpu_actions(player, world)
        };
        player_actions.push(actions);
    }

    // handle bombs
    &world.bombs.update(&mut world.tiles);

    // handle player movement and collisions
    for (index, player) in world.players.iter_mut().enumerate() {
        let actions = &player_actions[index];
        let speed = player.speed as f64 / 60.0;
        
        let col = player.position.0.round();
        let row = player.position.1.round();

        if actions.place_bomb && world.tiles.tile(col as usize, row as usize) == Tile::Empty {
            world.bombs.add(Bomb::new(player.id, player.bomb_power, (col, row)));
        }

        // calculate new position
        let x = player.position.0 + speed * &actions.direction_x.as_f64();
        let y = player.position.1 + speed * &actions.direction_y.as_f64();
        
        player.move_to(x, y, &world.tiles);
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
