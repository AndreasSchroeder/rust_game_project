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

struct Effect<'a> {
    pub sprite: Option<&'a Sprite>,
    coord: Coordinate,
    start: PreciseTime,
}

impl<'a> Effect<'a> {
    pub fn new(coord: Coordinate) -> Self {
        Effect {
            sprite: None,
            coord: coord,
            start: PreciseTime::now(),
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
    effects: Vec<Effect<'a>>,
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
        let mut effect = Effect::new(coord);
        match (typ, direction) {
            (EffectOption::Dead, _) => {
                effect.set_sprite(self.map.get_sprite("explosion.png"));

            }
            (_, _) => {}

        }
        effect.reset_time();
        self.effects.push(effect);

    }
    pub fn on_update(&mut self, args: &UpdateArgs) {
        self.effects.retain(|ref i| i.start.to(PreciseTime::now()).num_milliseconds() <= 1000);

    }
    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        for e in &self.effects {
            e.render(g, view);
        }
    }
}

impl<'a> Renderable for Effect<'a> {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     self.start.to(PreciseTime::now()).num_milliseconds() as u64,
                     false);

        }
    }
}

pub enum EffectOption {
    Dagger,
    Spear,
    Sword,
    Dead,
}
