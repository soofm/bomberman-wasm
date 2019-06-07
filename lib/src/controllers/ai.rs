use crate::models::{Actions, Player, World};

pub fn eval(player: &Player, world: &World) -> Actions {    
    Actions {
        player_id: player.id,
        direction: None,
        place_bomb: false,
    }
}
