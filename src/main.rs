extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate vecmath;
extern crate image as im;
extern crate time;

use piston_window::*;
use time::{Duration, PreciseTime};

mod creature;
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

use camera::Cam;
use player::{Player, LastKey};
use creature::Creature;
use field::Field;
use interactable::Interactable;
use coord::Coordinate;
use io::{render_level, read_level};
use io::tileset::Tileset;
use level::Level;

//EINGABEN
const TWO_PLAYER: bool = true;
const SPRITE_P_1: &'static str = "warrior2.png";
const SPRITE_P_2: &'static str = "paladin.png";
const LEVEL_HEIGHT: u64 = 100;
const LEVEL_WIDTH: u64 = 100;

const WIDTH: i64 = 1600;
const HEIGHT: i64 = 900;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    player_one: Player,
    player_two: Option<Player>,
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
        }
    }

    fn on_draw(&mut self,
               args: &RenderArgs,
               mut w: &mut PistonWindow,
               e: &Event,
               tileset: &Tileset,
               mut level: &mut Level) {
        let player_one = &self.player_one.creature;
        let player_two = &self.player_two;
        w.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            let center_p1 = c.transform.trans((self.player_one.coord.get_x() * 65) as f64, (self.player_one.coord.get_y() * 65) as f64);
            let center_ts =c.transform.trans(0.0,0.0);

            render_level(&tileset, gl, center_ts, &mut level);

            player_one.render(gl, center_p1);
            if let Some(ref x) = *player_two {
                x.creature.render(gl, center_p1);
            }
        });
    }

    fn on_update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player_one.on_update(args);
        if let Some(ref mut x) = self.player_two {
            x.on_update(args);
        }
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
        let mut player_one = &mut self.player_one;
        App::load_sprite(w, &mut player_one, SPRITE_P_1);
        if let Some(ref mut x) = self.player_two {
            App::load_sprite(w, x, SPRITE_P_2);
        }

    }
    fn load_sprite(mut w: &mut PistonWindow, player: &mut Player, file: &str) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let tank_sprite = assets.join(file);
        let tank_sprite2 = Texture::from_path(&mut w.factory,
                                              &tank_sprite,
                                              Flip::None,
                                              &TextureSettings::new());
        match tank_sprite2 {
            Err(_) => {
                println!("Empty");
            }
            Ok(x) => {
                player.creature.set_sprite(x);
            }
        }

    }
}

fn main() {
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("spinning-square",
                                                       [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        //.fullscreen(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(TWO_PLAYER);
    app.on_load(&mut window);

    let mut events = window.events();

    let folder = match find_folder::Search::Kids(1).for_folder("tiles") {
        Ok(res) => res.join("tileset-pokemon_dawn.png"),
        Err(_) => panic!("Folder 'tiles' not found!"),
    };

    let file_path = match folder.to_str() {
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

    while let Some(e) = events.next(&mut window) {
        let now = start.to(PreciseTime::now()).num_milliseconds();
        //println!("{}", now);

        if let Some(r) = e.render_args() {
            app.on_draw(&r, &mut window, &e, &tileset, &mut level);
        }
        if let Some(i) = e.release_args() {
            app.on_input(i, false);
        }
        if let Some(i) = e.press_args() {
            app.on_input(i, true);
        }
        if now > 500 {
            if let Some(u) = e.update_args() {
                app.on_update(&u);
                start = PreciseTime::now();
            }
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
