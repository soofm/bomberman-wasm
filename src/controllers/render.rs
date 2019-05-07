use crate::models::{Tile, World};

const CELL_SIZE: u32 = 50;

pub fn render_bg(canvas: &web_sys::HtmlCanvasElement, world: &World) {
    let render_width = world.width as u32 * CELL_SIZE;
    let render_height = world.height as u32 * CELL_SIZE;

    canvas.set_width(render_width);
    canvas.set_height(render_height);
}

pub fn render_frame(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    render_tiles(ctx, world);
    render_players(ctx, world);
}

fn render_tiles(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let tiles = &world.tiles;

    ctx.begin_path();

    for row in 0..world.height {
        for col in 0..world.width {
            let idx = (row * world.width + col) as usize;

            match tiles[idx] {
                Tile::Empty => {
                    draw_rect(ctx, row, col, "#FFF");
                },
                Tile::SoftBlock => {
                    draw_rect(ctx, row, col, "#D2B48C");
                },
                Tile::HardBlock => {
                    draw_rect(ctx, row, col, "#654321");
                },
                Tile::Bomb { timer: _, player_id } => {
                    let color = match player_id {
                        1 => "#A00",
                        2 => "#0A0",
                        3 => "#00A",
                        4 => "#AA0",
                        _ => "#000"
                    };
                    draw_rect(ctx, row, col, "#FFF");
                    draw_bomb(ctx, row, col, color);
                },
            };
        }
    }

    ctx.stroke();
}

fn draw_rect(ctx: &web_sys::CanvasRenderingContext2d, row: usize, col: usize, color: &str) {
    ctx.set_fill_style(&color.into());
    ctx.fill_rect(
        (col as u32 * CELL_SIZE) as f64,
        (row as u32 * CELL_SIZE) as f64,
        CELL_SIZE as f64,
        CELL_SIZE as f64
    );
}

fn draw_bomb(ctx: &web_sys::CanvasRenderingContext2d, row: usize, col: usize, color: &str) {
    ctx.begin_path();
    ctx.set_stroke_style(&color.into());
    ctx.move_to(
        (col as f64 + 0.25) * CELL_SIZE as f64,
        (row as f64 + 0.25) * CELL_SIZE as f64
    );
    ctx.line_to(
        (col as f64 + 0.41) * CELL_SIZE as f64,
        (row as f64 + 0.41) * CELL_SIZE as f64
    );
    ctx.stroke();
    ctx.close_path();
    ctx.begin_path();
    ctx.set_fill_style(&color.into());
    ctx.arc(
        (col as f64 + 0.6) * CELL_SIZE as f64,
        (row as f64 + 0.6) * CELL_SIZE as f64,
        15.0,
        0.0,
        2.0 * std::f64::consts::PI
    ).expect("Invalid radius");
    ctx.close_path();
    ctx.fill();
}

fn render_players(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let players = &world.players;

    for player in players.iter() {
        let color = match player.id {
            1 => "#F00",
            2 => "#0F0",
            3 => "#00F",
            4 => "#FF0",
            _ => "#000"
        };

        ctx.begin_path();
        ctx.set_fill_style(&color.into());
        ctx.arc(
            (player.position.0 + 0.5) * CELL_SIZE as f64,
            (player.position.1 + 0.5) * CELL_SIZE as f64,
            20.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).expect("Invalid radius");
        ctx.close_path();
        ctx.fill();
    }
}
