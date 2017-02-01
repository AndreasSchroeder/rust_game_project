use piston_window::*;
use creature::Creature;
use inventory::Inventory;

pub struct Player {
    pub up_d: bool,
    pub down_d: bool,
    pub left_d: bool,
    pub right_d: bool,
    pub creature: Creature,
    pub life: u32,
    pub dmg: i32,
    pub inv: Inventory,
}


impl Player {
    pub fn new() -> Self {
        Player {
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
            creature: Creature::new(),
            life: 100,
            dmg: 10,
            inv: Inventory::new(),
        }
    }
    pub fn on_update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        if self.up_d {
            self.creature.moves(0.0, -1.0);
        }
        if self.down_d {
            self.creature.moves(0.0, 1.0);
        }
        if self.right_d {
            self.creature.moves(1.0, 0.0);
        }
        if self.left_d {
            self.creature.moves(-1.0, 0.0);
        }
    }
}
