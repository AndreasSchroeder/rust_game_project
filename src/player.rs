use piston_window::*;
use creature::Creature;
use coord::Coordinate;

pub struct Player {
    pub creature: Creature,
    pub coord: Coordinate,
    pub last: LastKey,
}


impl Player {
    pub fn new(x: u64, y: u64) -> Self {
        Player {
            creature: Creature::new(),
            coord: Coordinate::new(x, y),
            last: LastKey::Wait,
        }
    }
    pub fn on_update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.

        match self.last {
            LastKey::Up => {
                self.creature.moves(0.0, -50.0);
            }
            LastKey::Down => {
                self.creature.moves(0.0, 50.0);
            }
            LastKey::Left => {
                self.creature.moves(-50.0, 0.0);
            }
            LastKey::Right => {
                self.creature.moves(50.0, 0.0);
            }
            _ => {}
        }
        self.last = LastKey::Wait;
    }
}

pub enum LastKey {
    Up,
    Down,
    Left,
    Right,
    Wait,
}
