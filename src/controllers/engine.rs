use crate::GameData;
use crate::models::Bomb;

pub fn update(data: &mut GameData) {
    let player = &mut data.state.world.players[0];
    // web_sys::console::log_1(&format!("{:?}", player).into());
    if data.actions.pressed_up {
        player.position.y -= player.speed;
    } else if data.actions.pressed_right {
        player.position.x += player.speed;
    } else if data.actions.pressed_down {
        player.position.y += player.speed;
    } else if data.actions.pressed_left {
        player.position.x -= player.speed;
    }

    if data.actions.place_bomb {
        data.state.world.bombs.push(Bomb::new(player.position, 0));
        data.actions.place_bomb = false
    }
}