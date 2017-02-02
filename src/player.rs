use piston_window::*;
use creature::Creature;
use inventory::Inventory;
use actor::Actor;
use enums::InteractableType;
use interactable::Interactable;
use coord::Coordinate;
use camera::Range;

pub struct Player {
    pub creature: Creature,
    pub life: i32,
    pub dmg: i32,
    pub inv: Inventory,
    pub coord: Coordinate,
    pub last: LastKey,
    pub pressed: bool,
    pub interactable_type: InteractableType,
    level_w: u64,
    level_h: u64,
}


impl Player {
    pub fn new(x: u64, y: u64) -> Self {
        Player {
            creature: Creature::new(),
            coord: Coordinate::new(x, y),
            last: LastKey::Wait,
            interactable_type: InteractableType::player,
            life: 100,
            dmg: 10,
            inv: Inventory::new(),
            pressed: false,
            level_w: 0,
            level_h: 0
        }
    }
    pub fn set_borders(&mut self, (w,h): (u64, u64)) {
        self.level_w = w;
        self.level_h = h;

    }

    pub fn on_update(&mut self, args: &UpdateArgs, range: Range) {
        // Rotate 2 radians per second.

        match self.last {
            LastKey::Up => {
                self.coord.move_coord_with_cam(0, -1, self.level_w, self.level_h, range);
                //self.creature.moves(0.0, -65.0);
            }
            LastKey::Down => {
                self.coord.move_coord_with_cam(0, 1, self.level_w, self.level_h, range);
                //self.creature.moves(0.0, 65.0);
            }
            LastKey::Left => {
                self.coord.move_coord_with_cam(-1, 0, self.level_w, self.level_h, range);                
                //self.creature.moves(-65.0, 0.0);
            }
            LastKey::Right => {
                self.coord.move_coord_with_cam(1, 0, self.level_w, self.level_h, range);                
                //self.creature.moves(65.0, 0.0);
            }
            _ => {}
        }
        if !self.pressed {
            self.last = LastKey::Wait;
        }
    }
}

pub enum LastKey {
    Up,
    Down,
    Left,
    Right,
    Wait,
}
impl Actor for Player {
    fn is_alive(&self) -> bool {
        self.life > 0
    }

    fn get_life(&self) -> i32 {
        self.life
    }
    fn damage_taken(&mut self, dmg: i32) {
        self.life -= dmg;
    }

    fn attack(&self, target: Vec<Option<&mut Interactable>>) {
        for t in target {
            match t {
                Some(x) => {
                    match x.get_interactable_type() {
                        InteractableType::player | InteractableType::bot => {
                            x.conv_to_actor().damage_taken(self.dmg)
                        }
                        InteractableType::useable => {}
                        InteractableType::collectable => {}
                    }
                }
                None => {}
            }
        }
    }



    fn dying(&self) {}
}

impl Interactable for Player {
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
    }

    fn conv_to_actor(&mut self) -> &mut Actor {
        self
    }
}
