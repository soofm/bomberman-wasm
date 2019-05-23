use crate::models::Tiles;
use super::{CORNER, Direction};

pub trait Position {
    fn position(&self) -> (f64, f64);
    fn set_position(&mut self, x: f64, y: f64);
    fn move_in_direction(&mut self, dir: Direction, dist: f64, tiles: &Tiles) {
        let (x, y) = self.position();
        let target: f64;
        let index: i32;
        let adjusted: f64;
        let step: bool;

        // calculate crossed boundaries and store adjusted positions for later
        let res = match dir {
            Direction::Left | Direction::Right => {
                if dir == Direction::Left {
                    target = x - dist;
                    index = target.floor() as i32;
                    adjusted = x.floor();
                    step = target.floor() < x.floor();
                } else {
                    target = x + dist;
                    index = target.ceil() as i32;
                    adjusted = x.ceil();
                    step = target.ceil() > x.ceil();
                }    

                match step {
                    false => (target, y),
                    true => {
                        let yi = y.floor() as i32;
                        let yf = y.fract();
                        let blocked_up = tiles.is_blocked(index, yi);
                        let blocked_down = tiles.is_blocked(index, yi + 1);
                        if blocked_up && blocked_down {
                            (adjusted, y)
                        } else if blocked_up {
                            if yf < 1.0 - CORNER { (adjusted, y) } else { (target, y.ceil()) }
                        } else if blocked_down {
                            if yf > CORNER { (adjusted, y) } else { (target, y.floor()) }
                        } else {
                            (target, y)
                        }
                    }
                }
            },
            Direction::Up | Direction::Down => {
                if dir == Direction::Up {
                    target = y - dist;
                    index = target.floor() as i32;
                    adjusted = y.floor();
                    step = target.floor() < y.floor();
                } else {
                    target = y + dist;
                    index = target.ceil() as i32;
                    adjusted = y.ceil();
                    step = target.ceil() > y.ceil();
                }

                match step {
                    false => (x, target),
                    true => {
                        let xi = x.floor() as i32;
                        let xf = x.fract();
                        let blocked_left = tiles.is_blocked(xi, index);
                        let blocked_right = tiles.is_blocked(xi + 1, index);
                        if blocked_left && blocked_right {
                            (x, adjusted)
                        } else if blocked_left {
                            if xf < 1.0 - CORNER { (x, adjusted) } else { (x.ceil(), target) }
                        } else if blocked_right {
                            if xf > CORNER { (x, adjusted) } else { (x.floor(), target) }
                        } else {
                            (x, target)
                        }
                    }
                }
            },
        };

        self.set_position(res.0, res.1);
    }
}
