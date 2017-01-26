extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate vecmath;

use piston_window::*;

mod creature;

use creature::Creature;

const WIDTH: i64 = 400;
const HEIGHT: i64 = 400;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


pub struct App {
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
    player: Creature,
}



impl App {
    fn new() -> Self {
        App {
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
            player: Creature::new(),
        }
    }

    fn on_draw(&mut self, args: &RenderArgs, mut w: &mut PistonWindow, e: &Event) {
        let square_length = 20;
        let field_width = WIDTH / square_length - 2;
        let field_heigth = HEIGHT / square_length - 2;
        let mut vec: Vec< [f64; 4] > = Vec::new();
        for i in 0..field_heigth {
            for j in 0..field_width {
                vec.push(rectangle::square((20 + square_length * j) as f64,
                                           (20 + square_length * i) as f64,
                                           square_length as f64));
            }
        }
        let (x, y) = (self.player.x as f64, self.player.y as f64);
        let player = &self.player;
        let rotation = self.player.rot;
        w.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            let center = c.transform.trans(0.0, 0.0);

            // Draw a box rotating around the middle of the screen.
            for i in 0..field_heigth {
                for j in 0..field_width {
                    rectangle(if i % 2 == j % 2 { BLACK } else { WHITE },
                              vec[(i * field_width + j) as usize],
                              c.transform.trans(1.0,1.0),
                              gl);
                }
            }

            player.render(gl, center);
        });
    }

    fn on_update(&mut self, args: &UpdateArgs) {
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
    fn on_load(&mut self, mut w: &mut PistonWindow) {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let tank_sprite = assets.join("warrior2.png");
    let tank_sprite2 = Texture::from_path(
            &mut w.factory,
            &tank_sprite,
            Flip::None,
            &TextureSettings::new());
    match tank_sprite2 {
        Err(_) => {
            println!("Empty");
        },
        Ok(x) => {
            println!("Not Empty");
            self.player.set_sprite(x);
        }
    }        
    
    }
}

fn main() {
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("spinning-square", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();
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
