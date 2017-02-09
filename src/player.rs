use inventory::Inventory;
use actor::Actor;
use interactable::InteractableType;
use interactable::Interactable;
use coord::Coordinate;
use camera::Range;
use level::Level;
use io::sprite::Sprite;
use bot::Bot;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use time::PreciseTime;
use renderable::Renderable;
use effect::{EffectHandler, EffectOption};
use io::all_sprites::SpriteMap;

pub struct Player<'a> {
    pub life: i32,
    pub dmg: i32,
    pub inv: Inventory,
    pub coord: Coordinate,
    pub last: LastKey,
    pub pressed: bool,
    pub no_more: bool,
    pub interactable_type: InteractableType,
    pub sprite: Option<&'a Sprite>,
    pub weapon: EffectOption,
    pub effect: EffectHandler<'a>,
    pub dir: LastKey,
    level_w: u64,
    level_h: u64,
    dt: PreciseTime,
    watch_rigth: bool,
    pub dead: bool,
}


impl<'a> Player<'a> {
    pub fn new(x: u64, y: u64, id: u64, map: &'a SpriteMap) -> Self {
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
            weapon: EffectOption::Dagger,
            dt: PreciseTime::now(),
            watch_rigth: false,
            effect: EffectHandler::new(map),
            dir: LastKey::Wait,
            dead: false,
        }
    }

    pub fn set_borders(&mut self, (w, h): (u64, u64)) {
        self.level_w = w;
        self.level_h = h;

    }

    pub fn set_sprite(&mut self, sprite: Option<&'a Sprite>) {
        self.sprite = sprite;
    }

    pub fn on_update(&mut self,
                     args: &UpdateArgs,
                     range: Range,
                     level: &mut Level,
                     it: InteractableType) {
        if self.dt.to(PreciseTime::now()).num_milliseconds() > 1000 {
            self.dt = PreciseTime::now();
        }
        if self.no_more == true {
            match self.last {
                LastKey::Up => {
                    self.coord.move_coord_with_cam(0, -1, level, range);
                    self.no_more = false;
                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);
                    //self.creature.moves(0.0, -65.0);
                }
                LastKey::Down => {
                    self.coord.move_coord_with_cam(0, 1, level, range);
                    self.no_more = false;
                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);
                    //self.creature.moves(0.0, 65.0);
                }
                LastKey::Left => {
                    self.watch_rigth = false;
                    self.coord.move_coord_with_cam(-1, 0, level, range);
                    self.no_more = false;
                    /* Update new position in field */

                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);

                    //self.creature.moves(-65.0, 0.0);
                }
                LastKey::Right => {
                    self.watch_rigth = true;
                    self.coord.move_coord_with_cam(1, 0, level, range);
                    self.no_more = false;
                    /* Update new position in field */

                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);

                }
                _ => {}
            }
        }
        if !self.pressed {
            self.last = LastKey::Wait;
            self.no_more = true;
        }

        self.effect.on_update(args);
    }

    pub fn get_effect_handler(&self) -> &EffectHandler {
        &self.effect
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LastKey {
    Up,
    Down,
    Left,
    Right,
    Wait,
}

impl<'a> Actor for Player<'a> {
    fn is_alive(&self) -> bool {
        self.life > 0
    }

    fn get_life(&self) -> i32 {
        self.life
    }

    fn damage_taken(&mut self, dmg: i32) {
        self.life -= dmg;
    }

    fn attack(&mut self,
              target: Vec<Option<InteractableType>>,
              bots: &mut Vec<Bot>,
              dir: LastKey) {
        println!(" weapon: {:?}; direction: {:?}", self.weapon, dir);
        self.effect.handle(self.coord, self.weapon, dir);
        for t in target {
            match t {
                Some(x) => {
                    match x {
                        InteractableType::Player(_) => {}
                        InteractableType::Bot(id) => {
                            //x.conv_to_actor().damage_taken(self.dmg)
                            if bots[id as usize].is_alive() {
                                bots[id as usize].damage_taken(self.dmg);
                            }

                            println!("{}", bots[id as usize].get_life());
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

impl<'a> Interactable for Player<'a> {
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
    }

    fn conv_to_actor(&mut self) -> &mut Actor {
        self
    }
}

impl<'a> Renderable for Player<'a> {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        //println!("player{:?}", match self.sprite {None => "None", _=> "Some"});
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     self.dt.to(PreciseTime::now()).num_milliseconds() as u64,
                     self.watch_rigth,
                     0);

        }
    }
}
