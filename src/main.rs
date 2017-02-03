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
mod camera;
mod bot;

use camera::Cam;
use player::{Player, LastKey};
use bot::Bot;
use io::render_tile;
use io::tileset::{TILE_HEIGHT, TILE_WIDTH, Tileset};
use level::Level;
use actor::Actor;
use interactable::Interactable;
use interactable::InteractableType;
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
    bots: Vec<Bot>,
    cam: Cam,
}

impl App {
    fn new(two_player: bool) -> Self {
        App {
            // 0,0 Dummy-Value
            player_one: Player::new(1, 1, 1),
            player_two: if two_player {
                // 0,0 Dummy-Value
                Some(Player::new(1, 2, 2))
            } else {
                None
            },
            bots: vec![Bot::new(2, 2, 1), Bot::new(4, 4, 2), Bot::new(6, 6, 3), Bot::new(8, 6, 4), Bot::new(18, 18, 5)],
            cam: Cam::new(CAMERA_BUF_X, CAMERA_BUF_Y),
        }
    }


    fn on_draw(&mut self,
               mut w: &mut PistonWindow,
               e: &Event,
               tileset: &Tileset,
               level: &mut Level,
               state: usize) {
        let range = self.cam.get_range();

        w.draw_2d(e, |c, gl| {
            let player_one = &self.player_one.sprite;
            let player_two = &self.player_two;

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

                p1.render(gl, center_p1, state as u64);
            }

            if let Some(ref p2) = *player_two {
                if let Some (ref x) =p2.sprite {


                let center_p2 = c.transform.trans(((p2.coord.get_x() - range.x_min) * 65) as f64,

                                              ((p2.coord.get_y() - range.y_min )* 65) as f64);
                 x.render(gl, center_p2, state as u64);
                 }

            }

            for b in &mut self.bots {

                if let Some(ref br) = b.sprite {
                    if b.coord.get_x() >= range.x_min &&  b.coord.get_x() < range.x_max &&
                        b.coord.get_y() >= range.y_min && b.coord.get_y() < range.y_max {

                        let center_b1 = c.transform.trans(((b.coord.get_x() - range.x_min )* 65) as f64,
                                                          ((b.coord.get_y() - range.y_min)* 65) as f64);
                        br.render(gl, center_b1, state as u64);
                    }
                }

            }
        });
    }

    fn on_update(&mut self,
                 args: &UpdateArgs,
                 level: &mut Level,
                 state: usize)
    {
        let coord1 = self.player_one.coord.clone();
        let mut coord2 = coord1.clone();

        if let Some(ref p2) = self.player_two {
            coord2 = p2.coord.clone();
        }

        let range = self.cam.get_range_update();

        self.player_one.on_update(args, range, level, InteractableType::Player(1));

        if let Some(ref mut x) = self.player_two {
            x.on_update(args, range, level, InteractableType::Player(2));
        }

        for b in &mut self.bots {
            b.on_update(args, range, level, state);
        }

        self.cam.calc_coordinates(coord1, coord2, level);
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
            Button::Keyboard(Key::Space) => {
                if pressed {
                    println!("Space!!!");
                }
            }
            _ => {}

        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("chicken_fight_3000_ultimate_tournament",
                                                       [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(false)
        .build()
        .unwrap();


    // Create a new game and run it.

    let mut app = App::new(TWO_PLAYER);

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

    // insert players in level
    level.get_data()[app.player_one.coord.get_x() as usize][app.player_one.coord.get_y() as usize].set_fieldstatus(InteractableType::Player(1));
    if let Some(ref p2) = app.player_two {
        level.get_data()[p2.coord.get_x() as usize][p2.coord.get_y() as usize].set_fieldstatus(InteractableType::Player(2));
    }

    // insert bots in level
    for b in &mut app.bots {
        level.get_data()[b.coord.get_x() as usize][b.coord.get_y() as usize].set_fieldstatus(b.get_interactable_type());
    }

    let mut start = PreciseTime::now();
    app.cam.set_borders((level.get_width() as u64, level.get_height()as u64));
    app.player_one.set_sprite(Sprite::fill_sprite("knight.png",2,1,64,64,&mut window));

    if let Some(ref mut p2) = app.player_two {
        p2.set_borders((level.get_width() as u64, level.get_height() as u64));
        p2.set_sprite(Sprite::fill_sprite("paladin.png", 2, 1, 64, 64, &mut window));
    }

    for b in &mut app.bots {
        b.set_borders((level.get_width() as u64, level.get_height()as u64));
        b.set_sprite(Sprite::fill_sprite("chicken_pink.png",2,1,64,64,&mut window));
    }

    while let Some(e) = events.next(&mut window) {
        let now = start.to(PreciseTime::now()).num_milliseconds();
        if now > 1000 {
            start = PreciseTime::now();
        }
        let state = if now <= 500 { 0 } else { 1 };

        if let Some(_) = e.render_args() {
            app.on_draw(&mut window, &e, &tileset, &mut level, state);
        }
        if let Some(i) = e.release_args() {
            app.on_input(i, false);
        }
        if let Some(i) = e.press_args() {
            app.on_input(i, true);
        }

        if let Some(u) = e.update_args() {
            app.on_update(&u, &mut level, state);
        }
    }
}
