use piston_window::*;

use inventory::Inventory;
use actor::Actor;
use interactable::InteractableType;
use interactable::Interactable;
use coord::Coordinate;
use camera::Range;
use level::Level;
use io::sprite::Sprite;
use bot::Bot;

pub enum Weapon {
    Sword,
    Spear,
    Broadsword,
}

pub struct Player {
    pub life: i32,
    pub dmg: i32,
    pub inv: Inventory,
    pub coord: Coordinate,
    pub last: LastKey,
    pub pressed: bool,
    pub no_more: bool,
    pub interactable_type: InteractableType,
    pub sprite: Option<Sprite>,
    pub weapon: Weapon,
    level_w: u64,
    level_h: u64,
}


impl Player {
    pub fn new(x: u64, y: u64, id: u64) -> Self {
        Player {
            coord: Coordinate::new(x, y),
            last: LastKey::Wait,
            interactable_type: InteractableType::Player(id),
            life: 100,
            dmg: 10,
            inv: Inventory::new(),
            pressed: false,
            level_w: 0,
            level_h: 0,
            sprite: None,
            no_more: true,
            weapon: Weapon::Sword,
        }
    }

    pub fn set_borders(&mut self, (w, h): (u64, u64)) {
        self.level_w = w;
        self.level_h = h;

    }

    pub fn set_sprite(&mut self, sprite: Sprite) {
        self.sprite = Some(sprite);
    }

    pub fn on_update(&mut self, args: &UpdateArgs, range: Range, level: &mut Level, it: InteractableType) {
        if self.no_more == true {
            match self.last {
                LastKey::Up => {
                    self.coord.move_coord_with_cam(0, -1, level, range);
                    self.no_more = false;
                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize].set_fieldstatus(it);
                    //self.creature.moves(0.0, -65.0);
                }
                LastKey::Down => {
                    self.coord.move_coord_with_cam(0, 1, level, range);
                    self.no_more = false;
                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize].set_fieldstatus(it);
                    //self.creature.moves(0.0, 65.0);
                }
                LastKey::Left => {
                    self.coord.move_coord_with_cam(-1, 0, level, range);
                    self.no_more = false;
                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize].set_fieldstatus(it);
                    //self.creature.moves(-65.0, 0.0);
                }
                LastKey::Right => {
                    self.coord.move_coord_with_cam(1, 0, level, range);
                    self.no_more = false;
                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize].set_fieldstatus(it);

                }
                _ => {}
            }
        }
        if !self.pressed {
            self.last = LastKey::Wait;
            self.no_more = true;
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

    fn attack(&self, target: Vec<Option<InteractableType>>, bots: &mut Vec<Bot>) {
        for t in target {
            match t {
                Some(x) => {
                    match x {
                        InteractableType::Player(_) => {}
                        InteractableType::Bot(id) => {
                            //x.conv_to_actor().damage_taken(self.dmg)
                            if bots[(id-1) as usize].is_alive {
                                bots[(id-1) as usize].damage_taken(self.dmg);
                            }

                            println!("{}", bots[(id-1) as usize].get_life());
                        }
                        InteractableType::Useable(_) => {}
                        InteractableType::Collectable(_) => {}
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
