extern crate find_folder;
extern crate image as im;

use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use im::GenericImage;


pub struct Sprite {
    set: Vec<Texture<Resources>>,
    animation: Vec<f64>,
    frames: usize,
}

impl Sprite {
    pub fn get_texture(&self, index: u64) -> Option<&Texture<Resources>> {
        self.set.get(index as usize)
    }

    pub fn get_set(&mut self) -> &mut Vec<Texture<Resources>> {
        &mut self.set
    }
    pub fn fill_sprite(path: &str, width: u32, heigth: u32, mut w: &mut PistonWindow) -> Self {
        let sprite_path = match find_folder::Search::ParentsThenKids(2, 2).for_folder("Sprites") {
            Ok(res) => res.join(path),
            Err(_) => panic!("Folder not found!"),
        };
        let sprite_string = match sprite_path.to_str() {
            Some(res) => res,
            None => panic!("Sprite not found"),
        };


        let mut ts = match im::open(sprite_string) {
            Ok(x) => x,
            Err(_) => panic!("Can't open {} in {}", path, sprite_string),
        };
        let (image_x, image_y) = ts.dimensions();
        let cols = image_x / width;
        let rows = image_y / heigth;
        let frames = rows * cols;
        let mut set: Vec<Texture<Resources>> = Vec::with_capacity((frames) as usize);

        for i in 0..(rows) {
            for j in 0..(cols) {
                let tile = ts.crop(j * width, i * heigth, width, heigth).to_rgba();


                set.push(Texture::from_image(&mut w.factory, &tile, &TextureSettings::new())
                    .unwrap());
            }
        }
        let mut vec: Vec<f64> = Vec::with_capacity(frames as usize);
        let part = 1000.0 / frames as f64;
        for i in 0..frames {
            vec.push(((i as f64) * part));
        }
        Sprite {
            frames: frames as usize,
            set: set,
            animation: vec,
        }
    }
    pub fn render(&self,
                  g: &mut GfxGraphics<Resources, CommandBuffer>,
                  view: math::Matrix2d,
                  dt: u64,
                  mirror: bool) {
        let mut frame = 0;
        let mut new_dt = dt;
        for (i, val) in self.animation.iter().enumerate() {
            if new_dt as f64 > *val {
                frame = i;
                new_dt = dt - *val as u64;
            }
        }

        //println!("render");
        image(&self.set[frame as usize],
              if mirror {
                  view.flip_h().trans(-65.0, 0.0)
              } else {
                  view
              },
              g);


    }
}
