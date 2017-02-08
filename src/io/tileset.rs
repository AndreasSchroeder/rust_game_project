use piston_window::*;
use gfx_device_gl::Resources;

pub struct Tileset {
    set: Vec<Texture<Resources>>,
    tile_width: u32,
    tile_height: u32,
    tileset_height: u32,
    tileset_width: u32,
}

impl Tileset {
    pub fn new(th: u32, tw: u32, tsh: u32, tsw: u32) -> Self {
        Tileset {
            set: Vec::with_capacity((tsh / th) as usize *
                                    (tsw / tw) as usize),
            tile_width: tw,
            tile_height: th,
            tileset_height: tsh,
            tileset_width: tsw,
        }
    }

    pub fn get_texture(&self, index: u64) -> Option<&Texture<Resources>> {
        self.set.get(index as usize)
    }

    pub fn get_set(&mut self) -> &mut Vec<Texture<Resources>> {
        &mut self.set
    }

    pub fn get_tile_height(&self) -> u32 {
        self.tile_height
    }
    pub fn get_tile_width(&self) -> u32 {
        self.tile_width
    }
    pub fn get_tileset_height(&self) -> u32 {
        self.tileset_height
    }
    pub fn get_tileset_width(&self) -> u32 {
        self.tileset_width
    }
}
