extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate vecmath;
extern crate image as im;
extern crate time;
extern crate rand;

use piston_window::*;
use time::{Duration, PreciseTime};

mod player;
mod io;
mod level;
mod inventory;
mod item;
mod actor;
mod field;
mod interactable;
mod coord;
mod enums;
mod camera;

use camera::{Cam};
use player::{Player, LastKey};
use io::{ render_tile};
use io::tileset::{TILE_HEIGHT, TILE_WIDTH, Tileset};
use level::Level;
use io::sprite::Sprite;

//EINGABEN
const TWO_PLAYER: bool = true;
const SPRITE_P_1: &'static str = "warrior2.png";
const SPRITE_P_2: &'static str = "paladin.png";
const CAMERA_BUF_X: u64 = 4;
const CAMERA_BUF_Y: u64 = 4;

const WIDTH: i64 = 586;
const HEIGHT: i64 = 586;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct App {
    player_one: Player,
    player_two: Option<Player>,
    cam: Cam,
}

impl App {
    fn new(two_player: bool) -> Self {
        App {
            // 0,0 Dummy-Value
            player_one: Player::new(0, 0),
            player_two: if two_player {
                // 0,0 Dummy-Value
                Some(Player::new(0, 0))
            } else {
                None
            },
            cam: Cam::new(CAMERA_BUF_X, CAMERA_BUF_Y),
        }
    }


    fn on_draw(&mut self,
               mut w: &mut PistonWindow,
               e: &Event,
               tileset: &Tileset,
               level: &mut Level) {
        let player_one = &self.player_one.sprite;
        let player_two = &self.player_two;

        let range = self.cam.get_range();
        w.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let center_lv = c.transform.trans(0.0, 0.0);

            //render_level(&tileset, gl, center_lv, &mut level);
            for (h, j) in (range.x_min..range.x_max).enumerate() {
                for (w, i) in (range.y_min..range.y_max).enumerate() {
                    let tile = match tileset.get_texture(level.get_data()[i as usize][j as usize].get_id()) {
                    Some(x) => x,
                    None => panic!("No texture found."),
                    };
                    // DEBUG
                    //println!("{} {}", i, j);
                    render_tile(&tile, gl, center_lv,  h as u32 * TILE_HEIGHT,
                            w as u32 * TILE_WIDTH,
                            w as u32,
                            h as u32);
                }
            }
            if let Some(ref p1) = *player_one {
                let center_p1 = c.transform.trans(((self.player_one.coord.get_x() - range.x_min )* 65) as f64,
                                              ((self.player_one.coord.get_y() - range.y_min)* 65) as f64);
            p1.render(gl, center_p1, 0);
            }
            
            if let Some(ref p2) = *player_two {
                if let Some (ref x) =p2.sprite {


                let center_p2 = c.transform.trans(((p2.coord.get_x() - range.x_min) * 65) as f64,

                                              ((p2.coord.get_y() - range.y_min )* 65) as f64);
                 x.render(gl, center_p2, 1);
                 }

            }
        });
    }

    fn on_update(&mut self,
                 args: &UpdateArgs,) {
                let coord1 = self.player_one.coord.clone();
        let mut coord2 = coord1.clone();
        if let Some(ref p2) = self.player_two {
            coord2 = p2.coord.clone();
        }

                let range = self.cam.get_range_update();



        self.player_one.on_update(args, range);
        if let Some(ref mut x) = self.player_two {

            x.on_update(args, range);
        }
                self.cam.calc_coordinates(coord1, coord2);

    }

    fn on_input(&mut self, inp: Button, pressed: bool) {

        match inp {
            Button::Keyboard(Key::Up) => {
                if pressed {
                    self.player_one.last = LastKey::Up;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::Down) => {
                if pressed {
                    self.player_one.last = LastKey::Down;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::Left) => {
                if pressed {
                    self.player_one.last = LastKey::Left;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::Right) => {
                if pressed {
                    self.player_one.last = LastKey::Right;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::W) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Up;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::S) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Down;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::A) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Left;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::D) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Right;
                    }
                    x.pressed = pressed;
                }
            }
            _ => {}

        }
    }
    fn on_load(&mut self, mut w: &mut PistonWindow) {
       
    }
   
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("chicken_fight_3000_ultimate_tournament",
                                                       [WIDTH as u32, HEIGHT as u32]).exit_on_esc(true).fullscreen(false).resizable(false).build().unwrap();


    // Create a new game and run it.

    let mut app = App::new(TWO_PLAYER);
    app.on_load(&mut window);

    let mut events = window.events();

    let tiles = match find_folder::Search::Kids(1).for_folder("tiles") {
        Ok(res) => res.join("tileset-pokemon_dawn.png"),
        Err(_) => panic!("Folder 'tiles' not found!"),
    };

    let file_path = match tiles.to_str() {
        Some(res) => res,
        None => panic!("Tileset not found!"),
    };

    let folder_level = match find_folder::Search::Kids(0).for_folder("src") {
        Ok(res) => res.join("level1.lvl"),
        Err(_) => panic!("Folder 'src' not found!"),
    };

    let level_path = match folder_level.to_str() {
        Some(res) => res,
        None => panic!("Level not found!"),
    };

    let tileset = io::read_tileset(file_path, &mut window);

    let mut level = io::read_level(level_path);

    let mut start = PreciseTime::now();
    app.cam.set_borders((level.get_x() as u64, level.get_y()as u64));
    app.player_one.set_borders((level.get_x() as u64, level.get_y()as u64));
    app.player_one.set_sprite(Sprite::fill_sprite("knight.png",2,1,64,64,&mut window));
    if let Some(ref mut p2) = app.player_two {
        p2.set_borders((level.get_x() as u64, level.get_y()as u64));
        p2.set_sprite(Sprite::fill_sprite("knight.png",2,1,64,64,&mut window));
    }

    while let Some(e) = events.next(&mut window) {
        let now = start.to(PreciseTime::now()).num_milliseconds();
        //println!("{}", now);

        if let Some(_) = e.render_args() {
            app.on_draw(&mut window, &e, &tileset, &mut level);
        }
        if let Some(i) = e.release_args() {
            app.on_input(i, false);
        }
        if let Some(i) = e.press_args() {
            app.on_input(i, true);
        }
        
            if let Some(u) = e.update_args() {
                app.on_update(&u);
                start = PreciseTime::now();
            }
        

    }

}

//UTIL//////////////////////////////


fn border_add(add1: f64, add2: f64, width: bool) -> f64 {
    let border = (if width { WIDTH } else { HEIGHT }) as f64;
    let sum = add1 + add2;
    if sum < 0.0 {
        0.0
    } else if sum > border {
        border
    } else {
        sum
    }
}
