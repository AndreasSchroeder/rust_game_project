// Module for Representing and Rendering Sprite
extern crate find_folder;
extern crate image as im;

use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use im::GenericImage;

use SIZE_PER_TILE;
use BORDER_BETWEEN_TILES;

/// Struct for Sprites
/// sets: for every state of animation
/// animation: Time-border for every animation
/// animation_time: durration of animation
pub struct Sprite {
    set: Vec<Texture<Resources>>,
    animation: Vec<f64>,
    animation_time: u64,
    once: bool,
}

impl Sprite {
    /// Returns Animation-Step
    pub fn get_texture(&self, index: u64) -> Option<&Texture<Resources>> {
        self.set.get(index as usize)
    }

    /// Returns set of animations
    pub fn get_set(&mut self) -> &mut Vec<Texture<Resources>> {
        &mut self.set
    }

    /// Read all Sprites for Animation
    pub fn fill_sprite(path: &str, width: u32, heigth: u32, animation_time: u64, once: bool, mut w: &mut PistonWindow) -> Self {
        // Sprites are located here Create Path
        let sprite_path = match find_folder::Search::ParentsThenKids(2, 2).for_folder("Sprites") {
            Ok(res) => res.join(path),
            Err(_) => panic!("Folder not found!"),
        };
        // Create String from path
        let sprite_string = match sprite_path.to_str() {
            Some(res) => res,
            None => panic!("Sprite not found"),
        };

        // Open path an create image
        let mut ts = match im::open(sprite_string) {
            Ok(x) => x,
            Err(_) => panic!("Can't open {} in {}", path, sprite_string),
        };
        // set dimensions and caculate rows and collumns
        let (image_x, image_y) = ts.dimensions();
        let cols = image_x / width;
        let rows = image_y / heigth;
        let frames = rows * cols;
        // initialize vector
        let mut set: Vec<Texture<Resources>> = Vec::with_capacity((frames) as usize);

        // Read all sprites from image
        for i in 0..(rows) {
            for j in 0..(cols) {
                let tile = ts.crop(j * width, i * heigth, width, heigth).to_rgba();
                set.push(Texture::from_image(&mut w.factory, &tile, &TextureSettings::new())
                    .unwrap());
            }
        }
        // Calculate Time-borders for each animation
        let mut vec: Vec<f64> = Vec::with_capacity(frames as usize);
        let part = animation_time as f64 / frames as f64;
        for i in 0..frames {
            vec.push(((i as f64) * part));
        }
        // Create Sprite
        Sprite {
            animation_time: animation_time as u64,
            set: set,
            animation: vec,
            once: once,
        }
    }

    /// Render the sprite
    pub fn render(&self,
                  g: &mut GfxGraphics<Resources, CommandBuffer>,
                  view: math::Matrix2d,
                  dt: u64,
                  mirror_h: bool,
                  degree: u32) {
        if self.once && dt >= self.animation_time {}
        else{
            let mut frame = 0;
            let mut new_dt = dt % self.animation_time;
            // choose animation by time
            for (i, val) in self.animation.iter().enumerate() {
                if new_dt as f64 > *val {
                    frame = i;
                    new_dt = (dt % self.animation_time) - *val as u64;
                }
            }
        

            // render image
            image(&self.set[frame as usize],
                  if mirror_h {
                      // translate
                      view.flip_h().trans((-1 * (SIZE_PER_TILE + BORDER_BETWEEN_TILES) as i64) as f64,
                                          0.0)
                  } else if degree == 270 {
                      view.trans(0.0, (SIZE_PER_TILE + BORDER_BETWEEN_TILES) as f64)
                          .rot_deg(degree as f64)
                  } else if degree == 90 {
                      view.trans((SIZE_PER_TILE + BORDER_BETWEEN_TILES) as f64, 0.0)
                          .rot_deg(degree as f64)
                  } else {
                      view
                  },
                  g);
        }
    }
}
