pub mod tileset;
pub mod sprite;
pub mod all_sprites;

extern crate find_folder;
extern crate image as im;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use self::tileset::{Tileset, TILESET_HEIGHT, TILE_HEIGHT, TILESET_WIDTH, TILE_WIDTH};
use level::Level;
use field::Field;

const SCALE_FACTOR: f64 = 4.0;

pub fn read_tileset(path: &str, mut w: &mut PistonWindow) -> Tileset {

    let mut tileset = Tileset::new();

    let mut ts = match im::open(path) {
        Ok(x) => x,
        Err(i) => panic!("{:?}", i),
    };

    for i in 0..(TILESET_HEIGHT / TILE_HEIGHT) {
        for j in 0..(TILESET_WIDTH / TILE_WIDTH) {

            let tile = ts.crop(j * TILE_WIDTH, i * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT).to_rgba();

            tileset.get_set()
                .push(Texture::from_image(&mut w.factory, &tile, &TextureSettings::new()).unwrap());
        }
    }

    tileset
}

pub fn read_level(path: &str) -> Level {
    let mut f = match File::open(path) {
        Ok(res) => res,
        Err(e) => panic!(e),
    };

    let mut buffer = String::new();

    match f.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };

    let mut rows = buffer.lines().filter(|s| !s.is_empty());

    let first_row = match rows.next() {
        Some(x) => x,
        None => panic!("No valid Level-File"),
    };

    let mut sizes = first_row.split(" ");

    let x = match sizes.next() {
        Some(x) => x,
        None => panic!("No valid Level-File"),
    };
    let x_new = match usize::from_str(x) {
        Ok(a) => a,
        Err(_) => panic!("No valid Level-File"),
    };

    let y = match sizes.next() {
        Some(x) => x,
        None => panic!("No valid Level-File"),
    };
    let y_new = match usize::from_str(y) {
        Ok(a) => a,
        Err(_) => panic!("No valid Level-File"),
    };

    let mut level = Level::with_size(x_new, y_new);


    // Zeilen
    for (i, s) in rows.enumerate() {
        // Spalten
        for (j, c) in s.split(" ").filter(|s| !s.is_empty()).enumerate() {
            let n = match u64::from_str(c) {
                Ok(a) => a,
                Err(e) => panic!("{:?}", e),
            };
            level.get_data()[i][j] = Field::new(n);
        }
    }

    level
}

pub fn render_level(tileset: &Tileset,
                    g: &mut GfxGraphics<Resources, CommandBuffer>,
                    view: math::Matrix2d,
                    level: &mut Level) {
    for i in 0..level.get_height() {
        for j in 0..level.get_width() {
            let tile = match tileset.get_texture(level.get_data()[i][j].get_id()) {
                Some(x) => x,
                None => panic!("No texture found."),
            };

            render_tile(&tile,
                        g,
                        view,
                        j as u32 * TILE_HEIGHT,
                        i as u32 * TILE_WIDTH,
                        i as u32,
                        j as u32);
        }
    }
}

pub fn render_tile(texture: &Texture<Resources>,
                   g: &mut GfxGraphics<Resources, CommandBuffer>,
                   view: math::Matrix2d,
                   x_coord: u32,
                   y_coord: u32,
                   x_offset: u32,
                   y_offset: u32) {
    // Skaliere Tile um Faktor
    image(texture,
          view.trans(y_offset as f64, x_offset as f64)
              .scale(SCALE_FACTOR, SCALE_FACTOR)
              .trans(x_coord as f64, y_coord as f64),
          g);
}
