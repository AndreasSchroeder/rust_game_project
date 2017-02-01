use coord::Coordinate;
use player::Player;

pub struct Cam<'a, 'b> {
    cord: Coordinate,
    buf_x: u64,
    buf_y: u64,
    player_one: Option<&'a Player>,
    player_two: Option<&'b Player>,
}

impl<'a, 'b> Cam<'a, 'b> {
    pub fn new(cord: Coordinate, buf_x: u64, buf_y: u64) -> Self {
        Cam {
            cord: cord,
            buf_x: buf_x,
            buf_y: buf_y,
            player_one: None,
            player_two: None,
        }
    }
    pub fn calc_coordinates(&mut self) {
        let p_one = if let None = self.player_one {
            false
        } else {
            true
        };
        let p_two = if let None = self.player_two {
            false
        } else {
            true
        };
        if !p_one && !p_two {
            self.cord.set_coord(0, 0);
        } else if p_one != p_two {
            if p_one {
                self.cord.set_coord(self.player_one.unwrap().coord.get_x(),
                                    self.player_one.unwrap().coord.get_y());
                self.cord.move_coord_with_buf(0, 0, self.buf_x, self.buf_y);
            } else {
                self.cord.set_coord(self.player_two.unwrap().coord.get_x(),
                                    self.player_two.unwrap().coord.get_y());
                self.cord.move_coord_with_buf(0, 0, self.buf_x, self.buf_y);
            }
        } else {
            let new_x = (self.player_one.unwrap().coord.get_x() +
                         self.player_two.unwrap().coord.get_x()) / 2;
            let new_y = (self.player_one.unwrap().coord.get_y() +
                         self.player_two.unwrap().coord.get_y()) / 2;
            self.cord.set_coord(new_x, new_y);
            self.cord.move_coord_with_buf(0, 0, self.buf_x, self.buf_y);

        }
    }
    pub fn set_player_one(&mut self, player: &'a Player) {
        self.player_one = Some(player);

    }
    pub fn clear_player_one(&mut self) {
        self.player_one = None;
    }

    pub fn set_player_two(&mut self, player: &'b Player) {
        self.player_two = Some(player);

    }
    pub fn clear_player_two(&mut self) {
        self.player_two = None;
    }
}
