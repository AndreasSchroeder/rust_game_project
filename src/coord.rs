// Module to represent World coordinates for Player, Bots, Camera, etc.
use camera::Range;
use level::Level;
use field::Field;
use player::LastKey;

/// Struct to represent the world coordinates
#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    x: u64,
    y: u64,
}


impl Coordinate {
    /// Creates a Coordinate wth the given x and y-value
    pub fn new(x: u64, y: u64) -> Self {
        Coordinate { x: x, y: y }
    }

    /// Getter for the x-value in the coordinates
    pub fn get_x(&self) -> u64 {
        self.x
    }

    /// Getter for the y-value in the coordinates
    pub fn get_y(&self) -> u64 {
        self.y
    }

    pub fn get_neighbours<'a>(&self, level: &'a Level) -> Vec<(&'a Field, LastKey)> {
        let mut vec: Vec<(&'a Field, LastKey)> = Vec::new();
        if self.x > 0 {
            vec.push((&level.get_field_at(self.x as usize - 1, self.y as usize), LastKey::Left));
        }
        if self.y > 0 {
            vec.push((&level.get_field_at(self.x as usize, self.y as usize - 1), LastKey::Up));
        }
        if self.y < level.get_width() as u64 {
            vec.push((&level.get_field_at(self.x as usize + 1, self.y as usize), LastKey::Right));
        }
        if self.y < level.get_height() as u64 {
            vec.push((&level.get_field_at(self.x as usize, self.y as usize + 1), LastKey::Down));
        }


        vec
    }

    /// Move the coordinates and prevents leaving the camera, leaving the level,
    /// or collide with other coordinates, except of the camera. Also checks, if
    /// field on new coordinate is passable
    pub fn move_coord_with_cam(&mut self, dx: i64, dy: i64, level: &mut Level, range: Range) {
        self.move_coord_without_cam(dx, dy, 0, 0, level);
        self.cam_border(range);
    }

    /// Sets the coordinate to the given new position (No checking)
    pub fn set_coord(&mut self, x: u64, y: u64) {
        self.x = x;
        self.y = y;
    }

    /// Sets the positon of the camera and prevents leaving Level
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

    /// Force the Coordinate to move without any checking
    pub fn force_move(&mut self, dx: i64, dy: i64) {
        self.x = (self.x as i64 + dx) as u64;
        self.y = (self.y as i64 + dy) as u64;
    }

    /// Move the coordinates and prevents leaving the level,
    /// or collide with other coordinates, except of the camera. Also checks, if
    /// field on new coordinate is passable
    pub fn move_coord_without_cam(&mut self,
                                  dx: i64,
                                  dy: i64,
                                  mut buf_x: u64,
                                  mut buf_y: u64,
                                  level: &mut Level) {
        // prevents leaving the the level if a buffer was given
        buf_x = if (level.get_width() as u64) < buf_x {
            level.get_width() as u64
        } else {
            buf_x
        };
        buf_y = if (level.get_height() as u64) < buf_y {
            level.get_height() as u64
        } else {
            buf_y
        };

        // checks leaving level
        let new_x = if (self.x as i64 + dx) < 0 {
            0
        } else if (self.x as i64 + dx) as u64 >= level.get_width() as u64 {
            level.get_width() as u64
        } else {
            (self.x as i64 + dx) as u64
        };
        let new_y = if (self.y as i64 + dy) < 0 {
            0
        } else if (self.y as i64 + dy) as u64 >= level.get_height() as u64 {
            level.get_height() as u64
        } else {
            (self.y as i64 + dy) as u64
        };

        /* player not at border */

        /* Check collision with unpassable fields, not with camera! (dx = dy = 0) */

        if dx != 0 || dy != 0 {
            let next_field =
                level.get_field_at((self.x as i64 + dx) as usize, (self.y as i64 + dy) as usize);
            if !next_field.check_passable() {
                return;
            }
        }
        /* Check end */

        if dx != 0 || dy != 0 {
            /* Update old position in field */
            level.get_data()[self.x as usize][self.y as usize].free_fieldstatus();
        }
        // sets coordinates after last checks
        self.x = if new_x < buf_x {
            buf_x
        } else if new_x > (level.get_width() as u64) - buf_x - 1 {
            level.get_width() as u64 - buf_x - 1
        } else {
            new_x
        };
        self.y = if new_y < buf_y {
            buf_y
        } else if new_y > (level.get_height() as u64) - buf_y - 1 {
            level.get_height() as u64 - buf_y - 1
        } else {
            new_y
        };
    }
}
