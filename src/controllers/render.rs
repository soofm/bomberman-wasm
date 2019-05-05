use crate::models::{Tile, World};
use wasm_bindgen::JsValue;

const CELL_SIZE: u32 = 50;

pub fn render_bg(canvas: &web_sys::HtmlCanvasElement, ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let render_width = world.width * CELL_SIZE;
    let render_height = world.height * CELL_SIZE;

    canvas.set_width(render_width);
    canvas.set_height(render_height);
}

pub fn render_frame(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    render_tiles(ctx, world);
    render_players(ctx, world);
    render_bombs(ctx, world);
}

fn render_tiles(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let tiles = &world.tiles;

    ctx.begin_path();

    for row in 0..world.height {
        for col in 0..world.width {
            let idx = (row * world.width + col) as usize;

            let color = match tiles[idx] {
                Tile::Empty => "#FFF",
                Tile::SoftBlock => "#D2B48C",
                Tile::HardBlock => "#654321",
            };

            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                (col * CELL_SIZE) as f64,
                (row * CELL_SIZE) as f64,
                CELL_SIZE as f64,
                CELL_SIZE as f64
            );
        }
    }

    ctx.stroke();
}

fn render_players(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let players = &world.players;

    for (index, player) in players.iter().enumerate() {
        ctx.begin_path();

        let color = match index {
            0 => "#F00",
            1 => "#0F0",
            2 => "#00F",
            3 => "#FF0",
            _ => "#000"
        };

        ctx.set_fill_style(&color.into());
        ctx.arc(
            (player.position.x + 0.5) * CELL_SIZE as f64,
            (player.position.y + 0.5) * CELL_SIZE as f64,
            20.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).expect("Invalid radius");

        ctx.close_path();
        ctx.fill();
    }
}

fn render_bombs(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let bombs = &world.bombs;

    for (index, bomb) in bombs.iter().enumerate() {
        ctx.begin_path();

        let color = match bomb.owner_id {
            0 => "#A00",
            1 => "#0A0",
            2 => "#00A",
            3 => "#AA0",
            _ => "#000"
        };

        ctx.set_fill_style(&color.into());
        ctx.arc(
            (bomb.position.x + 0.5) * CELL_SIZE as f64,
            (bomb.position.y + 0.5) * CELL_SIZE as f64,
            15.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).expect("Invalid radius");

        ctx.close_path();
        ctx.fill();
    }
}
