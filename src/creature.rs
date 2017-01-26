use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;

use border_add;
use piston_window::*;

pub struct Creature {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub rect: [f64; 4],
    sprite: Option<Texture<Resources>>,
}
impl Creature {
    pub fn new() -> Self {
        Creature {
            x: (::WIDTH / 2) as f64,
            y: (::HEIGHT / 2) as f64,
            rot: 0.0,
            rect: rectangle::square(0.0, 0.0, 50.0),
            sprite: None,
        }
    }
    pub fn moves(&mut self, dx: f64, dy: f64) {
        self.x = border_add(self.x, dx, true);
        self.y = border_add(self.y, dy, false);
    }
    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }
    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        match self.sprite {
            None => {
                rectangle(::RED,
                          self.rect,
                          view.trans(self.x, self.y).trans(-50.0, -50.0),
                          g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered

            }
            Some(ref sprite) => {
                image(sprite,
                      view.trans(self.x, self.y).trans(-50.0, -50.0),
                      g);
            }
        }
    }
}
