use ::LEVEL_WIDTH;
use ::LEVEL_HEIGHT;

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
    
    pub fn clone_coord(&self) -> Self{
        Coordinate{
            x: self.x,
            y: self.y,
        }
    }
    pub fn move_coord(&mut self, dx: u64, dy: u64) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        self.x = if new_x < 0 {
            0
        } else if new_x > LEVEL_WIDTH {
            LEVEL_WIDTH
        }
        else {
            new_x
        };
        self.y = if new_y < 0 {
            0
        } else if new_y > LEVEL_WIDTH {
            LEVEL_WIDTH
        }
        else {
            new_y
        }
    }
}
