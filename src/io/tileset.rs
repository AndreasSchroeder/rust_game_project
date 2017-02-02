use piston_window::*;
use gfx_device_gl::Resources;

pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;
pub const TILESET_HEIGHT: u32 = 2519;
pub const TILESET_WIDTH: u32 = 1504;

pub struct Tileset {
    set: Vec<Texture<Resources>>,
}

impl Tileset {
    pub fn new() -> Self {
        Tileset {
            set: Vec::with_capacity((TILESET_HEIGHT / TILE_HEIGHT) as usize *
                                    (TILESET_WIDTH / TILE_WIDTH) as usize),
        }
    }

    pub fn get_texture(&self, index: u32) -> Option<&Texture<Resources>> {
        self.set.get(index as usize)
    }

    pub fn get_set(&mut self) -> &mut Vec<Texture<Resources>> {
        &mut self.set
    }
}
