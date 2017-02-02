use coord::Coordinate;

pub struct Cam {
    coord: Coordinate,
    buf_x: u64,
    buf_y: u64,
    range: Range,
    level_w: u64,
    level_h: u64,
}

impl Cam {
    pub fn new(buf_x: u64, buf_y: u64) -> Self {
        Cam {
            coord: Coordinate::new(0, 0),
            buf_x: buf_x,
            buf_y: buf_y,
            range: Range::new(),
            level_w: 0,
            level_h: 0,
        }
    }
    pub fn set_borders(&mut self, (w, h): (u64, u64)) {
        self.level_w = w;
        self.level_h = h;

    }

    pub fn calc_coordinates(&mut self, coord1: Coordinate, coord2: Coordinate) {

        let new_x = (coord1.get_x() + coord2.get_x()) / 2;
        let new_y = (coord1.get_y() + coord2.get_y()) / 2;
        self.coord.set_coord(new_x, new_y);
        self.coord.move_coord_without_cam(0, 0, self.buf_x, self.buf_y, self.level_w, self.level_h);


    }
    pub fn get_range_update(&mut self) -> Range {
        self.range = Range::calc_range(self.buf_x, self.buf_y, self);
        self.range
    }
    pub fn get_range(&mut self) -> Range {
        self.range
    }
}
#[derive(Copy, Clone)]
pub struct Range {
    pub x_min: u64,
    pub y_min: u64,
    pub x_max: u64,
    pub y_max: u64,
}

impl Range {
    fn new() -> Self {
        Range {
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,
        }
    }
    fn calc_range(buf_x: u64, buf_y: u64, cam: &mut Cam) -> Self {
        let mut new = Range::new();
        new.x_min = if cam.coord.get_x() < buf_x {
            0
        } else {
            cam.coord.get_x() - buf_x
        };
        new.y_min = if cam.coord.get_y() < buf_y {
            0
        } else {
            cam.coord.get_y() - buf_y
        };
        new.x_max = if cam.coord.get_x() + buf_x + 1 > cam.level_w {
            cam.level_w
        } else {
            cam.coord.get_x() + buf_x + 1
        };
        new.y_max = if cam.coord.get_y() + buf_y + 1 > cam.level_h {
            cam.level_h
        } else {
            cam.coord.get_y() + buf_y + 1
        };
        // DEBUG
        /*println!("Camera.rs Debug: {} {} {} {} coord: {:?}",
                 new.x_max,
                 new.x_min,
                 new.y_max,
                 new.y_min,
                 cam.coord); */
        new

    }
}
