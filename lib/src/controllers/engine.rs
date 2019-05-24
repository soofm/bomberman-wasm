use super::{bomb_controller, player_controller};
use super::Input;
use crate::models::{Actions, Player, World};
use crate::geometry::Direction;

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
        for player in world.players.iter() {
            let actions = match player.is_human {
                true => eval_input(player, input),
                false => eval_cpu_actions(player, world)
            };
            player_actions.push(actions);
        }

        // handle player movement and collisions
        player_controller::update(player_actions, &mut world.players, &mut world.bombs, &mut world.tiles);

        // handle bombs
        bomb_controller::update(&mut world.bombs, &mut world.players, &mut world.tiles, &mut self.rng);
    }
}

fn eval_input(player: &Player, input: &Input) -> Actions {
    let mut dir: Option<Direction> = None;
    if input.pressed_left && !input.pressed_right {
        dir = Some(Direction::Left)
    } else if input.pressed_right && !input.pressed_left {
        dir = Some(Direction::Right)
    };

    if input.pressed_up && !input.pressed_down {
        dir = if dir != None { input.last_direction } else { Some(Direction::Up) };
    } else if input.pressed_down && !input.pressed_up {
        dir = if dir != None { input.last_direction } else { Some(Direction::Down) };
    }
    
    Actions {
        player_id: player.id,
        direction: dir,
        place_bomb: input.place_bomb,
    }
}

fn eval_cpu_actions(player: &Player, world: &World) -> Actions {
    // todo CPU AI
    Actions {
        player_id: player.id,
        direction: None,
        place_bomb: false,
    }
}
