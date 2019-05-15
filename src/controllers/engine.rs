use super::Input;
use crate::models::{Actions, Player, World};
use crate::geometry::LinearDirection;

pub struct Engine {
    rng: rand::rngs::ThreadRng,
}

impl Engine {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        Engine {
            rng,
        }
    }

    pub fn update(&mut self, input: &Input, world: &mut World) {
        // get actions
        let mut player_actions: Vec<Actions> = vec![];
        for player in world.players.get() {
            let actions = match player.is_human {
                true => eval_input(player, input),
                false => eval_cpu_actions(player, world)
            };
            player_actions.push(actions);
        }

        // handle player movement and collisions
        &world.players.update(player_actions, &mut world.bombs, &mut world.tiles);

        // handle bombs
        &world.bombs.update(&mut world.players, &mut world.tiles, &mut self.rng);
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