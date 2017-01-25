extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::*;

const OPENGL: OpenGL = OpenGL::V3_2;
const WIDTH: i64 = 400;
const HEIGHT: i64 = 400;

pub struct Creature {
    x: f64,
    y: f64,
    rot: f64,
    rect: [f64; 4],
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
    player: Creature,
}

impl Creature {
    fn new() -> Self {
        Creature {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64,
            rot: 0.0,
            rect: rectangle::square(0.0, 0.0, 50.0),
        }
    }
    fn moves(&mut self, dx: f64, dy: f64) {
        self.x = border_add(self.x, dx, true);
        self.y = border_add(self.y, dy, false);
    }
}

impl App {
    fn new() -> Self {
        App {
            gl: GlGraphics::new(OPENGL), // OpenGL drawing backend.
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
            player: Creature::new(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {


        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


        let square_length = 20;
        let field_width = WIDTH / square_length - 2;
        let field_heigth = HEIGHT / square_length - 2;
        let mut vec: Vec<[f64; 4]> = Vec::new();

        for i in 0..field_heigth {
            for j in 0..field_width {
                vec.push(rectangle::square((20 + square_length * j) as f64,
                                           (20 + square_length * i) as f64,
                                           square_length as f64));
            }
        }
        let (x, y) = (self.player.x as f64, self.player.y as f64);
        let player = self.player.rect;
                let rotation = self.player.rot;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            for i in 0..field_heigth {
                for j in 0..field_width {
                    rectangle(if i % 2 == j % 2 { BLACK } else { WHITE },
                              vec[(i * field_width + j) as usize],
                              c.transform,
                              gl);
                }
            }
            
            rectangle(RED, player, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.rot += 2.0 * args.dt;
        if self.up_d {
            self.player.moves(0.0, -1.0);
        }
        if self.down_d {
            self.player.moves(0.0, 1.0);
        }
        if self.right_d {
            self.player.moves(1.0, 0.0);
        }
        if self.left_d {
            self.player.moves(-1.0, 0.0);
        }
    }

    fn on_input(&mut self, inp: Button, pressed: bool) {

        match inp {
            Button::Keyboard(Key::Up) => {
                self.up_d = pressed;
            }
            Button::Keyboard(Key::Down) => {
                self.down_d = pressed;
            }
            Button::Keyboard(Key::Left) => {
                self.left_d = pressed;
            }
            Button::Keyboard(Key::Right) => {
                self.right_d = pressed;
            }
            _ => {}

        }
    }
}

fn main() {
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH as u32 , HEIGHT as u32])
        .opengl(OPENGL)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();

    let mut events = window.events();


    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
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
