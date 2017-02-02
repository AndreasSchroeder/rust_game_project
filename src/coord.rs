#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    x: u64,
    y: u64,
}

impl Coordinate {
    pub fn new(x: u64, y: u64) -> Self {
        Coordinate { x: x, y: y }
    }
    pub fn origin() -> Self {
        Coordinate::new(0, 0)
    }

    pub fn get_x(&self) -> u64 {
        self.x
    }
    pub fn get_y(&self) -> u64 {
        self.y
    }

    pub fn clone_coord(&self) -> Self {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }
    pub fn move_coord(&mut self, dx: u64, dy: u64, level_width: u64, level_height: u64) {
        self.move_coord_with_buf(dx, dy, 0, 0, level_width, level_height);
    }
    pub fn set_coord(&mut self, x: u64, y: u64) {
        self.x = x;
        self.y = y;
    }
    pub fn move_coord_with_buf(&mut self, dx: u64, dy: u64, mut buf_x: u64, mut buf_y: u64, level_width: u64, level_height: u64) {
        buf_x = if level_width < buf_x {
            level_width
        } else {
            buf_x
        };
        buf_y = if level_height < buf_y {
            level_height
        } else {
            buf_y
        };
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        self.x = if new_x < buf_x {
            buf_x
        } else if new_x > level_width - buf_x {
            level_width - buf_x
        } else {
            new_x
        };
        self.y = if new_y < buf_y {
            buf_y
        } else if new_y > level_height - buf_y {
            level_height - buf_y
        } else {
            new_y
        }
    }
}
