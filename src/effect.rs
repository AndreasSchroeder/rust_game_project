use io::sprite::Sprite;
use coord::Coordinate;
use renderable::Renderable;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use player::*;
use time::PreciseTime;
use all_sprites::SpriteMap;

pub struct Effect<'a> {
    pub sprite: Option<&'a Sprite>,
    pub coord: Coordinate,
    pub mirror_h: bool,
    pub degree: u32,
    start: PreciseTime,
}

impl<'a> Effect<'a> {
    pub fn new(coord: Coordinate) -> Self {
        Effect {
            sprite: None,
            coord: coord,
            start: PreciseTime::now(),
            mirror_h: false,
            degree: 0,
        }
    }
    pub fn set_sprite(&mut self, sprite: Option<&'a Sprite>) {
        self.sprite = sprite;
    }
    pub fn reset_time(&mut self) {
        self.start = PreciseTime::now();
    }
}
pub struct EffectHandler<'a> {
    pub effects: Vec<Effect<'a>>,
    map: &'a SpriteMap,
}
impl<'a> EffectHandler<'a> {
    pub fn new(map: &'a SpriteMap) -> Self {
        EffectHandler {
            effects: Vec::new(),
            map: map,
        }

    }


    pub fn handle(&mut self, coord: Coordinate, typ: EffectOption, direction: Direction) {
        let mut effect = Effect::new(coord.clone());
        match (typ, direction) {
            (EffectOption::Dead, _) => {
                effect.set_sprite(self.map.get_sprite("explosion.png"));

            }
            (EffectOption::Dagger, x) => {

                effect.set_sprite(self.map.get_sprite("swipe_dagger.png"));
                match x {
                    Direction::Up => {
                        effect.coord.force_move(0, -1);
                        effect.degree = 270
                    }
                    Direction::Down => {
                        effect.coord.force_move(0, 1);
                        effect.degree = 90
                    }
                    Direction::Left => {
                        effect.coord.force_move(-1, 0);
                        effect.mirror_h = true;
                    }
                    Direction::Right => {
                        effect.coord.force_move(1, 0);
                    }
                    _ => {}

                }
            }
            (EffectOption::Spear, x) => {
                effect.set_sprite(self.map.get_sprite("swipe_longsword.png"));
                match x {
                    Direction::Up => {
                        effect.coord.force_move(0, -1);
                        effect.degree = 270
                    }
                    Direction::Down => {
                        effect.coord.force_move(0, 1);
                        effect.degree = 90
                    }
                    Direction::Left => {
                        effect.coord.force_move(-1, 0);
                        effect.mirror_h = true;
                    }
                    Direction::Right => {
                        effect.coord.force_move(1, 0);
                    }
                    _ => {}

                }


            }
            (EffectOption::Sword, x) => {
                 effect.set_sprite(self.map.get_sprite("swipe_broadsword.png"));
                match x {
                    Direction::Up => {
                        effect.coord.force_move(-1, -1);
                        effect.degree = 270
                    }
                    Direction::Down => {
                        effect.coord.force_move(1, 1);
                        effect.degree = 90
                    }
                    Direction::Left => {
                        effect.coord.force_move(-1, -1);
                        effect.mirror_h = true;
                    }
                    Direction::Right => {
                        effect.coord.force_move(1, -1);
                    }
                    _ => {}

                }

            }
           

        }
        effect.reset_time();
        self.effects.push(effect);

    }
    pub fn on_update(&mut self, args: &UpdateArgs) {
        self.effects.retain(|ref i| i.start.to(PreciseTime::now()).num_milliseconds() <= 1000);

    }
}

impl<'a> Renderable for Effect<'a> {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     self.start.to(PreciseTime::now()).num_milliseconds() as u64,
                     self.mirror_h,
                     self.degree);

        }
    }
}

pub enum EffectOption {
    Dagger,
    Spear,
    Sword,
    Dead,
}
