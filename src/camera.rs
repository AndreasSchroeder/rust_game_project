use coord::Coordinate;
use player::Player;
use LEVEL_WIDTH;
use LEVEL_HEIGHT;

pub struct Cam {
    cord: Coordinate,
    buf_x: u64,
    buf_y: u64,
    range: Range,
}

impl Cam {
    pub fn new(buf_x: u64, buf_y: u64) -> Self {
        Cam {
            cord: Coordinate::new(0, 0),
            buf_x: buf_x,
            buf_y: buf_y,
            range: Range::new(),
        }
    }
    pub fn calc_coordinates(&mut self, coord1: Coordinate, coord2: Coordinate) {

        let new_x = (coord1.get_x() + coord2.get_x()) / 2;
        let new_y = (coord1.get_y() + coord2.get_y()) / 2;
        self.cord.set_coord(new_x, new_y);
        self.cord.move_coord_with_buf(0, 0, self.buf_x, self.buf_y);


    }
    pub fn get_range(&mut self) -> Range {
        self.range.calc_range(self.buf_x, self.buf_y, self.cord);
        self.range
    }
}
#[derive(Copy, Clone)]
struct Range {
    x_min: u64,
    y_min: u64,
    x_max: u64,
    y_max: u64,
}

impl Range {
    fn new() -> Self {
        Range{
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,           
        }
    }
    fn calc_range(&mut self, buf_x: u64, buf_y: u64, coord: Coordinate) {
        self.x_min = if coord.get_x() < buf_x { 0 } else { coord.get_x() - buf_x };
        self.y_min = if coord.get_y() < buf_y { 0 } else { coord.get_y() - buf_y };
        self.x_max = if coord.get_x() + buf_x > LEVEL_WIDTH { LEVEL_WIDTH } else { coord.get_x() - buf_x };
        self.y_min = if coord.get_y() + buf_y > LEVEL_HEIGHT{ LEVEL_HEIGHT } else { coord.get_y() - buf_y };

    }
}
