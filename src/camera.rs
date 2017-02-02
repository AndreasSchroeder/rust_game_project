use coord::Coordinate;
use player::Player;

pub struct Cam {
    cord: Coordinate,
    buf_x: u64,
    buf_y: u64,
}

impl Cam {
    pub fn new(buf_x: u64, buf_y: u64) -> Self {
        Cam {
            cord: Coordinate::new(0, 0),
            buf_x: buf_x,
            buf_y: buf_y,
        }
    }
    pub fn calc_coordinates(&mut self, coord1: Coordinate, coord2: Coordinate) {

        let new_x = (coord1.get_x() + coord2.get_x()) / 2;
        let new_y = (coord1.get_y() + coord2.get_y()) / 2;
        self.cord.set_coord(new_x, new_y);
        self.cord.move_coord_with_buf(0, 0, self.buf_x, self.buf_y);


    }
}
