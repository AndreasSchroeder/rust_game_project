use camera::Range;

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

    pub fn move_coord_with_cam(&mut self,
                               dx: i64,
                               dy: i64,
                               level_width: u64,
                               level_height: u64,
                               range: Range) {
        self.move_coord_without_cam(dx, dy, 0, 0, level_width, level_height);
        self.cam_border(range);
    }
    pub fn set_coord(&mut self, x: u64, y: u64) {
        self.x = x;
        self.y = y;
    }
    fn cam_border(&mut self, range: Range) {
        self.x = if self.x < range.x_min {
            range.x_min
        } else if self.x >= range.x_max {
            range.x_max - 1
        } else {
            self.x
        };
        self.y = if self.y < range.y_min {
            range.y_min
        } else if self.y >= range.y_max {
            range.y_max - 1
        } else {
            self.y
        };
    }

    pub fn move_coord_without_cam(&mut self,
                                  dx: i64,
                                  dy: i64,
                                  mut buf_x: u64,
                                  mut buf_y: u64,
                                  level_width: u64,
                                  level_height: u64) {
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
        let new_x = if (self.x as i64 + dx) < 0 {
            0
        } else {
            (self.x as i64 + dx) as u64
        };
        let new_y = if (self.y as i64 + dy) < 0 {
            0
        } else {
            (self.y as i64 + dy) as u64
        };
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
