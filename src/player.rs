use piston_window::*;
use creature::Creature;
use inventory::Inventory;
use actor::Actor;
use enums::InteractableType;
use interactable::Interactable;
use coord::Coordinate;

pub struct Player {
    pub creature: Creature,
    pub life: i32,
    pub dmg: i32,
    pub inv: Inventory,
    pub coord: Coordinate,
    pub last: LastKey,
    pub interactable_type: InteractableType,
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
        }
    }

    pub fn on_update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.

        match self.last {
            LastKey::Up => {
                self.creature.moves(0.0, -64.0);
            }
            LastKey::Down => {
                self.creature.moves(0.0, 64.0);
            }
            LastKey::Left => {
                self.creature.moves(-64.0, 0.0);
            }
            LastKey::Right => {
                self.creature.moves(64.0, 0.0);
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
                        InteractableType::player | InteractableType::bot => x.conv_to_actor().damage_taken(self.dmg),
                        InteractableType::useable => {},
                        InteractableType::collectable => {},
                    }
                },
                None => {},
            }
        }
    }



    fn dying(&self){}
}

impl Interactable for Player{
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
    }

    fn conv_to_actor(&mut self) -> &mut Actor {
        self
    }
}
