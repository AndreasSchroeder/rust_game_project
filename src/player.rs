use piston_window::*;
use creature::Creature;
use inventory::Inventory;
use actor::Actor;
use enums::InteractableType;
use interactable::Interactable;

pub struct Player {
    pub interactable_type: InteractableType,
    pub up_d: bool,
    pub down_d: bool,
    pub left_d: bool,
    pub right_d: bool,
    pub creature: Creature,
    pub life: i32,
    pub dmg: i32,
    pub inv: Inventory,
}


impl Player {
    pub fn new() -> Self {
        Player {
            interactable_type: InteractableType::player,
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
