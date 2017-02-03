use piston_window::*;
use inventory::Inventory;
use actor::Actor;
use enums::InteractableType;
use interactable::Interactable;
use coord::Coordinate;
use camera::Range;
use io::sprite::Sprite;
use rand::Rng;
use rand;

pub struct Bot {
    pub life: i32,
    pub dmg: i32,
    pub coord: Coordinate,
    pub interactable_type: InteractableType,
    pub sprite: Option<Sprite>,
    level_w: u64,
    level_h: u64,
}

impl Bot {
    pub fn new(x: u64, y: u64) -> Self {
        Bot {
            coord: Coordinate::new(x, y),
            interactable_type: InteractableType::Bot,
            sprite: None,
            life: 100,
            dmg: 10,
            level_w: 0,
            level_h: 0
        }
    }

    pub fn set_borders(&mut self, (w,h): (u64, u64)) {
        self.level_w = w;
        self.level_h = h;

    }

    pub fn set_sprite(&mut self, sprite: Sprite) {
        self.sprite = Some(sprite);
    }

    pub fn on_update(&mut self, args: &UpdateArgs, range: Range) {
        let mut rng = rand::thread_rng();

        let dir = rng.gen_range(0, 4);

        match dir {
            0 => {
                //Up
                self.coord.move_coord_without_cam(0, -1, 0, 0, self.level_w, self.level_h);
            },
            1 => {
                //Down
                self.coord.move_coord_without_cam(0, 1, 0, 0, self.level_w, self.level_h);
            },
            2 => {
                //Left
                self.coord.move_coord_without_cam(-1, 0, 0, 0, self.level_w, self.level_h);
            },
            3 => {
                //Right
                self.coord.move_coord_without_cam(1, 0, 0, 0, self.level_w, self.level_h);
            },
            _ => {},
        }

    }
}

impl Actor for Bot {
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
                        InteractableType::Player | InteractableType::Bot => {
                            x.conv_to_actor().damage_taken(self.dmg)
                        }
                        InteractableType::Useable => {}
                        InteractableType::Collectable => {}
                    }
                }
                None => {}
            }
        }
    }



    fn dying(&self) {}
}

impl Interactable for Bot {
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
    }

    fn conv_to_actor(&mut self) -> &mut Actor {
        self
    }
}
