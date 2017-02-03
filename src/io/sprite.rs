extern crate find_folder;
extern crate image as im;

use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;

pub struct Sprite {
    set: Vec<Texture<Resources>>,
    frames: usize,
}

impl Sprite {
    pub fn get_texture(&self, index: u64) -> Option<&Texture<Resources>> {
        self.set.get(index as usize)
    }

    pub fn get_set(&mut self) -> &mut Vec<Texture<Resources>> {
        &mut self.set
    }
    pub fn fill_sprite(path: &str,
                       rows: u32,
                       cols: u32,
                       width: u32,
                       heigth: u32,
                       mut w: &mut PistonWindow)
                       -> Self {
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
        let mut set: Vec<Texture<Resources>> = Vec::with_capacity((rows * cols) as usize);

        for i in 0..(rows) {
            for j in 0..(cols) {
                let tile = ts.crop(j * width, i * heigth, width, heigth).to_rgba();


                set.push(Texture::from_image(&mut w.factory, &tile, &TextureSettings::new())
                    .unwrap());
            }
        }
        Sprite {
            frames: (rows * cols) as usize,
            set: set,
        }
    }
    pub fn render(&self,
                  g: &mut GfxGraphics<Resources, CommandBuffer>,
                  view: math::Matrix2d,
                  frame: u64) {

        image(&self.set[frame as usize], view, g);


    }
}
