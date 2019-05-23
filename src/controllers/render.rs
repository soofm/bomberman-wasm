use crate::models::{BombState, Tile, World, PowerupType};

const CELL_SIZE: u32 = 50;

pub fn render_bg(canvas: &web_sys::HtmlCanvasElement, world: &World) {
    let render_width = world.tiles.width as u32 * CELL_SIZE;
    let render_height = world.tiles.height as u32 * CELL_SIZE;

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

    draw_empty(
        ctx,
        (world.tiles.width as u32 * CELL_SIZE) as f64,
        (world.tiles.height as u32 * CELL_SIZE) as f64
    );

    for row in 0..world.tiles.height {
        for col in 0..world.tiles.width {
            let x = (col as u32 * CELL_SIZE) as f64;
            let y = (row as u32 * CELL_SIZE) as f64;

            match tiles.get(col, row) {
                Tile::SoftBlock => {
                    draw_soft_block(ctx, x, y);
                },
                Tile::HardBlock => {
                    draw_hard_block(ctx, x, y);
                },
                Tile::Powerup(powerup) => {
                    draw_powerup(ctx, x, y, powerup);
                },
                _ => {}
            };
        }
    }
}

fn draw_empty(ctx: &web_sys::CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.set_fill_style(&"#FFF".into());
    ctx.fill_rect(0.0, 0.0, width, height);
}

fn draw_soft_block(ctx: &web_sys::CanvasRenderingContext2d, x: f64, y: f64) {
    let size = CELL_SIZE as f64;
    ctx.set_fill_style(&"#000".into());
    ctx.fill_rect(x, y, size, size);
    ctx.set_fill_style(&"#D2B48C".into());
    ctx.fill_rect(x + 1.0, y + 1.0, size - 2.0, size - 2.0);
    ctx.begin_path();
    ctx.set_stroke_style(&"#000".into());
    ctx.move_to(x + (0.5 * size), y);
    ctx.line_to(x + (0.5 * size), y + (0.25 * size));
    ctx.move_to(x, y + (0.25 * size));
    ctx.line_to(x + size, y + (0.25 * size));
    ctx.move_to(x, y + (0.55 * size));
    ctx.line_to(x + size, y + (0.55 * size));
    ctx.move_to(x, y + (0.85 * size));
    ctx.line_to(x + size, y + (0.85 * size));
    ctx.move_to(x + (0.3 * size), y + (0.25 * size));
    ctx.line_to(x + (0.3 * size), y + (0.55 * size));
    ctx.move_to(x + (0.8 * size), y + (0.25 * size));
    ctx.line_to(x + (0.8 * size), y + (0.55 * size));
    ctx.move_to(x + (0.5 * size), y + (0.55 * size));
    ctx.line_to(x + (0.5 * size), y + (0.85 * size));
    ctx.move_to(x + (0.3 * size), y + (0.85 * size));
    ctx.line_to(x + (0.3 * size), y + size);
    ctx.move_to(x + (0.8 * size), y + (0.85 * size));
    ctx.line_to(x + (0.8 * size), y + size);
    ctx.stroke();
}

fn draw_hard_block(ctx: &web_sys::CanvasRenderingContext2d, x: f64, y: f64) {
    let size = CELL_SIZE as f64;
    ctx.set_fill_style(&"#000".into());
    ctx.fill_rect(x, y, size, size);
    ctx.set_fill_style(&"#333".into());
    ctx.fill_rect(x + 3.0, y + 3.0, size - 6.0, size - 6.0);
}

fn draw_powerup(ctx: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, powerup: PowerupType) {
    let size = CELL_SIZE as f64;
    ctx.set_line_width(3.0);
    if powerup == PowerupType::BombNumber {
        ctx.begin_path();
        ctx.set_fill_style(&"#4A304A".into());
        ctx.arc(
            x + (0.5 * size),
            y + (0.5 * size),
            15.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).unwrap();
        ctx.close_path();
        ctx.fill();
        ctx.begin_path();
        ctx.set_stroke_style(&"#FFF".into());
        ctx.move_to(x + (0.5 * size), y + (0.42 * size));
        ctx.line_to(x + (0.5 * size), y + (0.58 * size));
        ctx.move_to(x + (0.42 * size), y + (0.5 * size));
        ctx.line_to(x + (0.58 * size), y + (0.5 * size));
        ctx.stroke();
        ctx.close_path();
    } else if powerup == PowerupType::BombPower {
        ctx.begin_path();
        ctx.set_fill_style(&"#FF8C00".into());
        ctx.arc(
            x + (0.5 * size),
            y + (0.5 * size),
            15.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).unwrap();
        ctx.close_path();
        ctx.fill();
        ctx.begin_path();
        ctx.set_stroke_style(&"#FFF".into());
        ctx.move_to(x + (0.5 * size), y + (0.42 * size));
        ctx.line_to(x + (0.5 * size), y + (0.58 * size));
        ctx.move_to(x + (0.46 * size), y + (0.46 * size));
        ctx.line_to(x + (0.5 * size), y + (0.42 * size));
        ctx.line_to(x + (0.54 * size), y + (0.46 * size));
        ctx.stroke();
        ctx.close_path();
    } else if powerup == PowerupType::Speed {
        ctx.begin_path();
        ctx.set_stroke_style(&"#32CD32".into());
        ctx.move_to(x + (0.3 * size), y + (0.33 * size));
        ctx.line_to(x + (0.7 * size), y + (0.33 * size));
        ctx.move_to(x + (0.3 * size), y + (0.5 * size));
        ctx.line_to(x + (0.7 * size), y + (0.5 * size));
        ctx.move_to(x + (0.3 * size), y + (0.67 * size));
        ctx.line_to(x + (0.7 * size), y + (0.67 * size));
        ctx.stroke();
        ctx.close_path();
    } else if powerup == PowerupType::Boots {
        ctx.begin_path();
        ctx.set_stroke_style(&"#000".into());
        ctx.set_fill_style(&"#654321".into());
        ctx.move_to(x + (0.38 * size), y + (0.2 * size));
        ctx.line_to(x + (0.58 * size), y + (0.2 * size));
        ctx.line_to(x + (0.58 * size), y + (0.44 * size));
        ctx.arc(
            x + (0.58 * size),
            y + (0.6 * size),
            0.16 * size,
            1.5 * std::f64::consts::PI,
            0.5 * std::f64::consts::PI
        ).unwrap();
        ctx.line_to(x + (0.38 * size), y + (0.76 * size));
        ctx.line_to(x + (0.38 * size), y + (0.2 * size));
        ctx.stroke();
        ctx.close_path();
        ctx.fill();
    }
    ctx.set_line_width(1.0);
}

fn render_players(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let size = CELL_SIZE as f64;
    for player in world.players.iter() {
        let x = player.position.0 * size;
        let y = player.position.1 * size;
        let color = match player.id {
            1 => "#F00",
            2 => "#0F0",
            3 => "#00F",
            4 => "#FF0",
            _ => "#000",
        };

        if player.is_alive {
            ctx.begin_path();
            ctx.set_fill_style(&color.into());
            ctx.arc(x + (0.5 * size), y + (0.5 * size), 20.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            ctx.close_path();
            ctx.fill();
        } else {
            ctx.begin_path();
            ctx.set_line_width(3.0);
            ctx.set_stroke_style(&color.into());
            ctx.move_to(x + (0.2 * size), y + (0.2 * size));
            ctx.line_to(x + (0.8 * size), y + (0.8 * size));
            ctx.move_to(x + (0.8 * size), y + (0.2 * size));
            ctx.line_to(x + (0.2 * size), y + (0.8 * size));
            ctx.stroke();
            ctx.set_line_width(1.0);
            ctx.close_path();
        }
    }
}

fn render_bombs(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let size = CELL_SIZE as f64;
    for bomb in world.bombs.iter() {
        let color = match bomb.player_id {
                1 => "#A00",
                2 => "#0A0",
                3 => "#00A",
                4 => "#AA0",
                _ => "#000"
        };
        let x = bomb.position.0 * size;
        let y = bomb.position.1 * size;

        match bomb.state {
            BombState::Armed => {
                ctx.begin_path();
                ctx.move_to(x + (0.25 * size), y + (0.25 * size));
                ctx.line_to(x + (0.41 * size), y + (0.41 * size));
                ctx.stroke();
                ctx.close_path();
                ctx.begin_path();
                ctx.set_fill_style(&color.into());
                ctx.arc(x + (0.6 * size), y + (0.6 * size), 15.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                ctx.close_path();
                ctx.fill();
            },
            BombState::Exploding { left, right, up, down } => {
                ctx.begin_path();
                ctx.set_fill_style(&color.into());

                ctx.move_to(x + (0.1 * size), y + (0.1 * size));

                ctx.line_to(x + (0.1 * size), y - (up as f64 * size));
                ctx.line_to(x + (0.9 * size), y - (up as f64 * size));
                ctx.line_to(x + (0.9 * size), y + (0.1 * size));

                ctx.line_to(x + ((right as f64 + 1.0) * size), y + (0.1 * size));
                ctx.line_to(x + ((right as f64 + 1.0) * size), y + (0.9 * size));
                ctx.line_to(x + (0.9 * size), y + (0.9 * size));

                ctx.line_to(x + (0.9 * size), y + (down as f64 + 1.0) * size);
                ctx.line_to(x + (0.1 * size), y + (down as f64 + 1.0) * size);
                ctx.line_to(x + (0.1 * size), y + (0.9 * size));
                
                ctx.line_to(x - (left as f64 * size), y + (0.9 * size));
                ctx.line_to(x - (left as f64 * size), y + (0.1 * size));
                ctx.line_to(x + (0.1 * size), y + (0.1 * size));

                ctx.close_path();

                ctx.fill();                
            },
            _ => {}
        }
    }
}
