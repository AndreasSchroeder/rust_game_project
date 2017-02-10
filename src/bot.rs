use piston_window::*;
use actor::Actor;
use interactable::InteractableType;
use interactable::Interactable;
use coord::Coordinate;
use camera::Range;
use io::sprite::Sprite;
use level::Level;
use rand::Rng;
use rand;
use renderable::Renderable;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use time::PreciseTime;
use player::LastKey;
use effect::EffectHandler;
use io::all_sprites::SpriteMap;
use sounds::SoundHandler;
use player::Player;
use field::Field;
use effect::EffectOption;

pub struct Bot<'a> {
    pub life: i32,
    pub dmg: i32,
    pub coord: Coordinate,
    pub interactable_type: InteractableType,
    pub sprite: Option<&'a Sprite>,
    level_w: u64,
    level_h: u64,
    old_state: usize,
    dt: PreciseTime,
    watch_rigth: bool,
    pub effect: EffectHandler<'a>,
}

impl<'a> Bot<'a> {
    pub fn new(x: u64, y: u64, id: u64, map: &'a SpriteMap) -> Self {
        Bot {
            coord: Coordinate::new(x, y),
            interactable_type: InteractableType::Bot(id),
            sprite: None,
            life: 100,
            dmg: 10,
            level_w: 0,
            level_h: 0,
            old_state: 0,
            dt: PreciseTime::now(),
            watch_rigth: false,
            effect: EffectHandler::new(map),
        }
    }

    pub fn set_borders(&mut self, (w, h): (u64, u64)) {
        self.level_w = w;
        self.level_h = h;

    }

    pub fn set_sprite(&mut self, sprite: Option<&'a Sprite>) {
        self.sprite = sprite;
    }

    pub fn on_update(&mut self, args: &UpdateArgs, mut level: &mut Level, state: usize, sounds: &mut SoundHandler, enemy: &mut Vec<Option<Player>>) {
        let mut rng = rand::thread_rng();



        if self.old_state != state {
            if self.dt.to(PreciseTime::now()).num_milliseconds() > 1000 {
                self.dt = PreciseTime::now();
            }
            let dir = rng.gen_range(0, 6);
            match dir {
                0 => {
                    //Up
                    self.coord.move_coord_without_cam(0, -1, 0, 0, level);
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(self.interactable_type);
                }
                1 => {
                    //Down
                    self.coord.move_coord_without_cam(0, 1, 0, 0, level);
                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(self.interactable_type);
                }
                2 => {
                    //Left
                    self.watch_rigth = false;
                    self.coord.move_coord_without_cam(-1, 0, 0, 0, level);

                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(self.interactable_type);

                }
                3 => {
                    //Right
                    self.watch_rigth = true;
                    self.coord.move_coord_without_cam(1, 0, 0, 0, level);

                    level.get_data()[self.coord.get_x() as usize][self.coord.get_y() as usize]
                        .set_fieldstatus(self.interactable_type);
                }
                _ =>  self.attack(level, enemy),
            }
            self.old_state = state;
 
        }
        self.effect.on_update(args);


        for e in &mut self.effect.effects {
            if !e.get_played() {
                sounds.play(e.get_sound_str());
                e.played();
            }
        }
    }
}

impl<'a> Actor for Bot<'a> {
    fn is_alive(&self) -> bool {
        self.life > 0
    }

    fn get_life(&self) -> i32 {
        self.life
    }

    fn damage_taken(&mut self, dmg: i32) {
        self.life -= dmg;
    }


    fn attack<T>(&mut self, level: &mut Level, enemy: &mut Vec<Option<T>>)
        where T: Actor
    {
       let targeting_fields: Vec<(& Field, LastKey)> = self.coord.get_neighbours(level);
        for (f, dir) in targeting_fields {
            if let Some(t) = f.get_fieldstatus(){
                if let InteractableType::Player(id_in_field) = t {
                    if let &mut Some(ref mut p) = &mut enemy[id_in_field as usize - 1]{
                        p.damage_taken(10);
                        self.effect.handle(self.coord, EffectOption::Chicken, dir);
                        return
                    }
                }
            }
        }
    }



    fn dying(&self) {}
}

impl<'a> Interactable for Bot<'a> {
    fn get_interactable_type(&self) -> InteractableType {
        self.interactable_type
    }
}

impl<'a> Renderable for Bot<'a> {
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
