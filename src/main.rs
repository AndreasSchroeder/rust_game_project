extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate vecmath;

use piston_window::*;

mod creature;
mod player;
mod inventory;
mod item;
mod actor;
mod field;
mod object;
mod coord;


use player::Player;
use creature::Creature;
use field::Field;
use object::Object;
use coord::Coordinate;


//EINGABEN
const TWO_PLAYER: bool = true;
const SPRITE_P_1: &'static str = "warrior2.png";
const SPRITE_P_2: &'static str = "paladin.png";
const LEVEL_HEIGHT: u64 = 100;
const LEVEL_WIDTH: u64 = 100;

const WIDTH:  i64 = 1200;
const HEIGHT: i64 = 600;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
//const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
//const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn new(x: f64, y: f64) -> Self {
        Coord {
            x: x,
            y: y,
        }
    }
    fn origin() -> Self {
        Coord::new(0.0,0.0)
    }
    fn get_coord(&self) ->(f64, f64) {
        (self.x, self.y)
    }
}

pub struct App {
    player_one: Player,
    player_two: Option<Player>,
}

impl App {
    fn new(two_player: bool) -> Self {
        App {
            player_one: Player::new(),
            player_two: if two_player {
                Some(Player::new())
            } else {
                None
            },
        }
    }

    fn on_draw(&mut self, args: &RenderArgs, mut w: &mut PistonWindow, e: &Event) {
        let player_one = &self.player_one.creature;
        let player_two = &self.player_two;
        w.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            let center = c.transform.trans(0.0, 0.0);
            player_one.render(gl, center);
            if let Some(ref x) = *player_two {
                x.creature.render(gl, center);
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
                self.player_one.up_d = pressed;
            }
            Button::Keyboard(Key::Down) => {
                self.player_one.down_d = pressed;
            }
            Button::Keyboard(Key::Left) => {
                self.player_one.left_d = pressed;
            }
            Button::Keyboard(Key::Right) => {
                self.player_one.right_d = pressed;
            }
            Button::Keyboard(Key::W) => {
                if let Some(ref mut x) = self.player_two {
                    x.up_d = pressed;
                }
            }
            Button::Keyboard(Key::S) => {
                if let Some(ref mut x) = self.player_two {
                    x.down_d = pressed;
                }
            }
            Button::Keyboard(Key::A) => {
                if let Some(ref mut x) = self.player_two {
                    x.left_d = pressed;
                }
            }
            Button::Keyboard(Key::D) => {
                if let Some(ref mut x) = self.player_two {
                    x.right_d = pressed;
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
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(TWO_PLAYER);
    app.on_load(&mut window);

    let mut events = window.events();


    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.on_draw(&r, &mut window, &e);
        }

        if let Some(u) = e.update_args() {
            app.on_update(&u);
        }

        if let Some(i) = e.press_args() {
            app.on_input(i, true);
        }
        if let Some(i) = e.release_args() {
            app.on_input(i, false);
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
