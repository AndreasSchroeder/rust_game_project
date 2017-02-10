//use inventory::Inventory;
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
use effect::{EffectHandler, EffectOption, Effect};
use io::all_sprites::SpriteMap;
use sounds::SoundHandler;

pub struct Player<'a> {
    pub life: i32,
    pub dmg: i32,
    //pub inv: Inventory,
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
    pub delay_attack: bool,
}


impl<'a> Player<'a> {
    pub fn new(x: u64, y: u64, id: u64, map: &'a SpriteMap) -> Self {
        Player {
            coord: Coordinate::new(x, y),
            last: LastKey::Wait,
            interactable_type: InteractableType::Player(id),
            life: 100,
            dmg: 10,
            //inv: Inventory::new(),
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
            delay_attack: false,
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
                     it: InteractableType,
                      sounds: &mut SoundHandler) {
        if self.dt.to(PreciseTime::now()).num_milliseconds() > 1000 {
            self.dt = PreciseTime::now();
        }
        if self.no_more == true {
            match self.last {
                LastKey::Up => {
                    self.coord.move_coord_with_cam(0, -1, level, range);

                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);
                    //self.creature.moves(0.0, -65.0);
                }
                LastKey::Down => {
                    self.coord.move_coord_with_cam(0, 1, level, range);

                    /* Update new position in field */
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);
                    //self.creature.moves(0.0, 65.0);
                }
                LastKey::Left => {
                    self.watch_rigth = false;
                    self.coord.move_coord_with_cam(-1, 0, level, range);

                    /* Update new position in field */

                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);

                    //self.creature.moves(-65.0, 0.0);
                }
                LastKey::Right => {
                    self.watch_rigth = true;
                    self.coord.move_coord_with_cam(1, 0, level, range);

                    /* Update new position in field */

                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(it);

                }
                _ => {}
            }
            self.no_more = false;
        }
        if !self.pressed {
            self.last = LastKey::Wait;
            self.no_more = true;
        }

        self.effect.on_update(args);
        for e in &mut self.effect.effects {
            if !e.get_played() {
                sounds.play(e.get_sound_str());
                e.played();
            }
        }
    }

    pub fn get_effect_handler(&self) -> &EffectHandler {
        &self.effect
    }
    pub fn get_effects(&mut self) -> &'a [Effect] {
        &mut self.effect.effects
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
        self.life = if self.life - dmg > 100 {
            100
        } else if self.life - dmg < 0 {
            0
        } else {
            self.life - dmg
        };
    }

    fn attack<T>(&mut self, level: &mut Level, enemy: &mut Vec<Option<T>>)
        where T: Actor
    {
        let mut targets = Vec::new();
        let pos = &self.coord.clone();
        if self.delay_attack == false {
            match self.dir {
                LastKey::Wait => {},
                _ => {
                    self.effect.handle(self.coord, self.weapon, self.dir);
                },
            }

            match self.weapon {
                EffectOption::Dagger => {
                    match self.dir {
                        LastKey::Up => {
                            targets.push(level.get_data()[(pos.get_x() - 1) as usize][pos.get_y() as usize].get_fieldstatus());

                        },
                        LastKey::Down => {
                            targets.push(level.get_data()[(pos.get_x() + 1) as usize][pos.get_y() as usize].get_fieldstatus());

                        },
                        LastKey::Left => {
                            targets.push(level.get_data()[pos.get_x() as usize][(pos.get_y() -1) as usize].get_fieldstatus());

                        }
                        LastKey::Right => {
                            targets.push(level.get_data()[pos.get_x() as usize][(pos.get_y() +1) as usize].get_fieldstatus());
                        },
                        _ => {},
                    }
                },
                EffectOption::Sword => {
                    for i in 0..3{
                        match self.dir {
                            LastKey::Up => {
                                targets.push(level.get_data()[(pos.get_x() - 1) as usize][(pos.get_y() + i - 1) as usize].get_fieldstatus());

                            },
                            LastKey::Down => {
                                targets.push(level.get_data()[(pos.get_x() + 1) as usize][(pos.get_y() + i - 1) as usize].get_fieldstatus());

                            },
                            LastKey::Left => {
                                targets.push(level.get_data()[(pos.get_x() + i - 1) as usize][(pos.get_y() -1) as usize].get_fieldstatus());

                            },
                            LastKey::Right => {
                                targets.push(level.get_data()[(pos.get_x() + i - 1) as usize][(pos.get_y() +1) as usize].get_fieldstatus());

                            },
                            _ => {},
                        }
                    }
                },
                EffectOption::Spear => {
                    match self.dir {
                        LastKey::Up => {
                            targets.push(level.get_data()[(pos.get_x() - 1) as usize][pos.get_y() as usize].get_fieldstatus());
                            targets.push(level.get_data()[(pos.get_x() - 2) as usize][pos.get_y() as usize].get_fieldstatus());

                        },
                        LastKey::Down => {
                            targets.push(level.get_data()[(pos.get_x() + 1) as usize][pos.get_y() as usize].get_fieldstatus());
                            targets.push(level.get_data()[(pos.get_x() + 2) as usize][pos.get_y() as usize].get_fieldstatus());

                        },
                        LastKey::Left => {
                            targets.push(level.get_data()[pos.get_x() as usize][(pos.get_y() - 1) as usize].get_fieldstatus());
                            targets.push(level.get_data()[pos.get_x() as usize][(pos.get_y() - 2) as usize].get_fieldstatus());

                        },
                        LastKey::Right => {
                            targets.push(level.get_data()[pos.get_x() as usize][(pos.get_y() + 1) as usize].get_fieldstatus());
                            targets.push(level.get_data()[pos.get_x() as usize][(pos.get_y() + 2) as usize].get_fieldstatus());

                        },
                        _ => {},
                    }
                },
                _ => {},
            }
            self.delay_attack = true;
     }


        for t in targets {
            if let Some(x) = t {
                match x {
                    InteractableType::Player(_) => {}
                    InteractableType::Bot(id) => {
                        //x.conv_to_actor().damage_taken(self.dmg)
                        if let &mut Some(ref mut e) = &mut enemy[id as usize]{
                            if e.is_alive() {
                                e.damage_taken(self.dmg);
                            }
                        }
                    }

                    InteractableType::Useable(_) => {}
                    InteractableType::Collectable(_) => {}
                }
            }
        }
    }

    fn dying(&self) {}
}

impl<'a> Interactable for Player<'a> {
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
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
