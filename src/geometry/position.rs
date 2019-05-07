use crate::models::Tiles;

pub trait Position {
    fn position(&self) -> (f64, f64);
    fn set_position(&mut self, x: f64, y: f64);
    fn move_to(&mut self, x: f64, y: f64, tiles: &Tiles) {
        let (ox, oy) = self.position();
        let adj_x: f64;
        let adj_y: f64;
        let col: i32;
        let row: i32;
        let mut res: (f64, f64) = (x, y);

        // calculate crossed boundaries and store adjusted positions for later
        if x.floor() < ox.floor() {
            adj_x = ox.floor();
            col = x.floor() as i32;
        } else if x.ceil() > ox.ceil() {
            adj_x = ox.ceil();
            col = x.ceil() as i32;
        } else {
            adj_x = x;
            col = x.floor() as i32;
        }

        if y.floor() < oy.floor() {
            adj_y = oy.floor();
            row = y.floor() as i32;
        } else if y.ceil() > oy.ceil() {
            adj_y = oy.ceil();
            row = y.ceil() as i32;
        } else {
            adj_y = y;
            row = y.floor() as i32;
        }

        if adj_x != x && adj_y != y {
            let ocol = if adj_x < x { col - 1 } else { col + 1 };
            let orow = if adj_y < y { row - 1} else { row + 1 };

            let mut blocked_x = tiles.is_blocked(col, orow);
            let mut blocked_y = tiles.is_blocked(ocol, row);

            if !blocked_x && !blocked_y && tiles.is_blocked(col, row) {
                blocked_x = true;
                blocked_y = true;
            }

            res = (if blocked_x { adj_x } else { x }, if blocked_y { adj_y } else { y });
        } else if adj_x != x {
            if tiles.is_blocked(col, row) || (y != y.trunc() && tiles.is_blocked(col, row + 1)) {
                res = (adj_x, y);
            }
        } else if adj_y != y {
            if tiles.is_blocked(col, row) || (x != x.trunc() && tiles.is_blocked(col + 1, row)) {
                res = (x, adj_y);
            }
        }
        
        self.set_position(res.0, res.1);
    }
}
