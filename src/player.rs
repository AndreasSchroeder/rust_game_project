use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use time::PreciseTime;

use actor::Actor;
use interactable::InteractableType;
use interactable::Interactable;
use coord::Coordinate;
use camera::Range;
use level::Level;
use io::sprite::Sprite;
use renderable::Renderable;
use effect::{EffectHandler, EffectOption};
use io::all_sprites::SpriteMap;
use sounds::SoundHandler;

pub struct Player<'a> {
    pub life: i32,
    pub dmg: i32,
    pub coord: Coordinate,
    // last key pressed (can be Wait, in case no butten was pressed in this iteration)
    pub last: LastKey,
    // if a button is pressed currently
    pub pressed: bool,
    // stops the player from moving multiple times if button was pressed just once
    pub no_more: bool,
    pub interactable_type: InteractableType,
    pub sprite: Option<&'a Sprite>,
    pub weapon: EffectOption,
    pub effect: EffectHandler<'a>,
    // last direction moved to, used for player attack direction
    pub dir: LastKey,
    level_w: u64,
    level_h: u64,
    dt: PreciseTime,
    // bool to determined the direction the sprite is facing
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
                     range: Range,
                     level: &mut Level,
                     it: InteractableType,
                     sounds: &mut SoundHandler) {
        if self.is_alive() {
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
        }

        self.effect.on_update();
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

    pub fn clone_without_effects(&self, map: &'a SpriteMap) -> Self {
        Player {
            coord: self.coord.clone(),
            last: self.last.clone(),
            interactable_type: self.interactable_type.clone(),
            life: self.life.clone(),
            dmg: self.dmg.clone(),
            pressed: self.pressed.clone(),
            level_w: self.level_w.clone(),
            level_h: self.level_h.clone(),
            sprite: self.sprite.clone(),
            no_more: self.no_more.clone(),
            weapon: self.weapon.clone(),
            dt: self.dt.clone(),
            watch_rigth: self.watch_rigth.clone(),
            effect: EffectHandler::new(map),
            dir: self.dir.clone(),
            dead: self.dead.clone(),
            delay_attack: self.delay_attack.clone(),
        }
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
        // if statments to not have more then 100 HP or less then 0HP
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
        // Vector of targets to be attacked
        let mut targets = Vec::new();
        let pos = &self.coord.clone();
        if self.delay_attack == false {
            match self.dir {
                LastKey::Wait => {}
                _ => {
                    self.effect.handle(self.coord, self.weapon, self.dir);
                }
            }

            // Matching Player Weapon to get htbox
            match self.weapon {
                EffectOption::Dagger => {
                    // matching last move direction and filling targets vector
                    match self.dir {
                        LastKey::Up => {
                            targets.push(level.get_data()[(pos.get_x()) as usize][(pos.get_y() - 1) as usize].get_fieldstatus());
                        }
                        LastKey::Down => {
                            targets.push(level.get_data()[(pos.get_x()) as usize][(pos.get_y() + 1) as usize].get_fieldstatus());
                        }
                        LastKey::Left => {
                            targets.push(level.get_data()[(pos.get_x() - 1) as usize][(pos.get_y()) as usize].get_fieldstatus());
                        }
                        LastKey::Right => {
                            targets.push(level.get_data()[(pos.get_x() + 1) as usize][(pos.get_y()) as usize].get_fieldstatus());
                        }
                        _ => {}
                    }
                }
                EffectOption::Sword => {
                    //iterating over hitbox
                    for i in 0..3 {
                        match self.dir {
                            LastKey::Up => {
                                targets.push(level.get_data()[(pos.get_x() + i - 1) as usize][(pos.get_y() - 1) as usize].get_fieldstatus());
                            }
                            LastKey::Down => {
                                targets.push(level.get_data()[(pos.get_x() + i - 1) as usize][(pos.get_y() + 1) as usize].get_fieldstatus());

                            }
                            LastKey::Left => {
                                targets.push(level.get_data()[(pos.get_x() - 1) as usize][(pos.get_y() + i - 1) as usize].get_fieldstatus());

                            }
                            LastKey::Right => {
                                targets.push(level.get_data()[(pos.get_x() + 1) as usize][(pos.get_y() + i - 1) as usize].get_fieldstatus());

                            }
                            _ => {}
                        }
                    }
                }
                EffectOption::Spear => {
                    match self.dir {
                        LastKey::Up => {
                            targets.push(level.get_data()[(pos.get_x()) as usize][(pos.get_y() - 1) as usize].get_fieldstatus());
                            targets.push(level.get_data()[(pos.get_x()) as usize][(pos.get_y() - 2) as usize].get_fieldstatus());

                        }
                        LastKey::Down => {
                            targets.push(level.get_data()[(pos.get_x()) as usize][(pos.get_y() + 1) as usize].get_fieldstatus());
                            targets.push(level.get_data()[(pos.get_x()) as usize][(pos.get_y() + 2) as usize].get_fieldstatus());

                        }
                        LastKey::Left => {
                            targets.push(level.get_data()[(pos.get_x() - 1) as usize][(pos.get_y()) as usize].get_fieldstatus());
                            targets.push(level.get_data()[(pos.get_x() - 2) as usize][(pos.get_y()) as usize].get_fieldstatus());

                        }
                        LastKey::Right => {
                            targets.push(level.get_data()[(pos.get_x() + 1) as usize][(pos.get_y()) as usize].get_fieldstatus());
                            targets.push(level.get_data()[(pos.get_x() + 2) as usize][(pos.get_y()) as usize].get_fieldstatus());

                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            self.delay_attack = true;
        }


        // all enemies in targets vector take damage
        for t in targets {
            if let Some(x) = t {
                match x {
                    InteractableType::Player(_) => {}
                    InteractableType::Bot(id) => {
                        //x.conv_to_actor().damage_taken(self.dmg)
                        if let &mut Some(ref mut e) = &mut enemy[id as usize] {
                            if e.is_alive() {
                                e.damage_taken(self.dmg);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Interactable for Player<'a> {
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
    }
}

impl<'a> Renderable for Player<'a> {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     self.dt.to(PreciseTime::now()).num_milliseconds() as u64,
                     self.watch_rigth,
                     0);

        }
    }
}
