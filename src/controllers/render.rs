use crate::models::{BombState, Tile, World};

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
    render_bombs(ctx, world);
}

fn render_tiles(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let tiles = &world.tiles;

    draw_empty(
        ctx,
        (world.width as u32 * CELL_SIZE) as f64,
        (world.height as u32 * CELL_SIZE) as f64
    );

    for row in 0..world.height {
        for col in 0..world.width {
            let x = (col as u32 * CELL_SIZE) as f64;
            let y = (row as u32 * CELL_SIZE) as f64;

            match tiles.tile(col as usize, row as usize) {
                Tile::Empty => {},
                Tile::SoftBlock => {
                    draw_soft_block(ctx, x, y);
                },
                Tile::HardBlock => {
                    draw_hard_block(ctx, x, y);
                },
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
    ctx.fill_rect(x + 1.0, y + 1.0, size - 2.0, size - 2.0);
}

fn render_players(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    for player in world.players.iter() {
        if player.id > 4 { continue; }
        let color = match player.id {
            1 => "#F00",
            2 => "#0F0",
            3 => "#00F",
            4 => "#FF0",
            _ => "#000",
        };

        ctx.begin_path();
        ctx.set_fill_style(&color.into());
        ctx.arc(
            (player.position.0 + 0.5) * CELL_SIZE as f64,
            (player.position.1 + 0.5) * CELL_SIZE as f64,
            20.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).unwrap();
        ctx.close_path();
        ctx.fill();
    }
}

fn render_bombs(ctx: &web_sys::CanvasRenderingContext2d, world: &World) {
    let size = CELL_SIZE as f64;
    for bomb in world.bombs.get() {
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
                //ctx.set_stroke_style(&"#000".into());
                ctx.begin_path();
                ctx.move_to(x + (0.25 * size), y + (0.25 * size));
                ctx.line_to(x + (0.41 * size), y + (0.41 * size));
                ctx.stroke();
                ctx.close_path();
                ctx.begin_path();
                ctx.set_fill_style(&color.into());
                ctx.arc(
                    x + (0.6 * size),
                    y + (0.6 * size),
                    15.0,
                    0.0,
                    2.0 * std::f64::consts::PI
                ).unwrap();
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
