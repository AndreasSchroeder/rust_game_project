// Main Module
extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate vecmath;
extern crate image as im;
extern crate time;
extern crate rand;
extern crate ears;
extern crate xml;

use piston_window::*;
use time::PreciseTime;

// modules
mod player;
mod io;
mod level;
//mod inventory;
mod item;
mod actor;
mod field;
mod interactable;
mod coord;
mod camera;
mod bot;
mod renderable;
mod effect;
mod sounds;
mod util;
mod player_hub;

// own uses
use interactable::Interactable;
use util::{coord_to_pixel_x, coord_to_pixel_y};
use camera::Cam;
use player::{Player, LastKey};
use bot::Bot;
use actor::Actor;
use io::render_tile;
use io::tileset::Tileset;
use io::xml::load_xml;
use level::Level;
use effect::EffectOption;
use interactable::InteractableType;
use renderable::Renderable;
use io::all_sprites::SpriteMap;
use std::process;
use sounds::SoundHandler;
use ears::AudioController;
use player_hub::PlayerHub;
use item::Item;
use coord::Coordinate;

//EINGABEN
const HUB_UP: u64 = 52;
const CAM_BORDER: u64 = 20;
const SIZE_PER_TILE: u64 = 64;
const BORDER_BETWEEN_TILES: u64 = 1;
const CAMERA_BUF_X: u64 = 8;
const CAMERA_BUF_Y: u64 = 4;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREY: [f32; 4] = [0.5, 0.5, 0.5, 0.5];
const GAME_NAME_PART1: &'static str = "Chicken Fight 3000";
const GAME_NAME_PART2: &'static str = "Ultimate Tournament";

/// struct for Settings
pub struct Settings {
    pub sprite_map: SpriteMap,
}

impl Settings {
    /// Constructor
    pub fn new(mut w: &mut PistonWindow) -> Self {
        let sprite_map = SpriteMap::init(w);
        Settings { sprite_map: sprite_map }
    }
}

/// Struct for the app
/// player_one: the first Player
/// Player_two: teh seconde Player if available
/// bots: Vector of all bots
/// cam: the Camera
pub struct App<'a> {
    players: Vec<Option<Player<'a>>>,
    bots: Vec<Option<Bot<'a>>>,
    items: Vec<Item<'a>>,
    cam: Cam,
    hub_one: PlayerHub<'a>,
    hub_two: PlayerHub<'a>,
    muted: bool,
}

impl<'a> App<'a> {
    /// Constructor
    fn new(players: Vec<Option<Player<'a>>>, bots: Vec<Option<Bot<'a>>>,   items: Vec<Item<'a>>) -> Self {

        App {
            players: players,
            bots: bots,
            items: items,
            cam: Cam::new(CAMERA_BUF_X, CAMERA_BUF_Y),
            hub_one: PlayerHub::new("Player One", None),
            hub_two: PlayerHub::new("Player Two", None),
            muted: false,
        }
    }

    /// Handle the rendering of all Objects
    fn on_draw(&mut self,
               mut w: &mut PistonWindow,
               e: &Event,
               tileset: &Tileset,
               level: &mut Level) {
        // Range of the camera
        let range = self.cam.get_range();

        // draw in 2D
        w.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let center_hub_one = c.transform.trans(10.0, 10.0);
            self.hub_one.render(gl, center_hub_one);

            let center_hub_two = c.transform.trans(280.0, 10.0);
            self.hub_two.render(gl, center_hub_two);

            let center_lv = c.transform.trans(0.0, 0.0);

            // render all Tiles if in Camera range
            for (h, j) in (range.x_min..range.x_max).enumerate() {
                for (w, i) in (range.y_min..range.y_max).enumerate() {
                    // get tile
                    let tile = match tileset.get_texture(level.get_data()[j as usize][i as usize].get_id()) {
                    Some(x) => x,
                    None => panic!("No texture found."),
                    };
                    // render tile
                    render_tile(&tile, gl, center_lv,
                            (h as u32 * tileset.get_tile_width()) + (CAM_BORDER/4) as u32,
                            (w as u32 * tileset.get_tile_height()) + (HUB_UP/4) as u32,
                            w as u32,
                            h as u32);
                }
            }

            // Render all in Camera items
            for i in &mut self.items {
                    if i.coord.get_x() >= range.x_min &&  i.coord.get_x() < range.x_max &&
                        i.coord.get_y() >= range.y_min && i.coord.get_y() < range.y_max {

                        let center_b1 = c.transform.trans(coord_to_pixel_x(i.coord.get_x(), range.x_min ),
                                                          coord_to_pixel_y(i.coord.get_y(), range.y_min));
                        i.render(gl, center_b1);
                }
            }

            for x in &mut self.players {
                if let &mut Some(ref mut p) = x {
                    // position of Player one in Pixel coordinates
                    let center_p = c.transform.trans(coord_to_pixel_x(p.coord.get_x(), range.x_min),
                                                  coord_to_pixel_y(p.coord.get_y(), range.y_min));

                    // render player one
                    p.render(gl, center_p);
                    for e in &p.get_effect_handler().effects {
                        // Rendr all Effects in Camera
                        if e.coord.get_x() >= range.x_min &&  e.coord.get_x() < range.x_max &&
                                e.coord.get_y() >= range.y_min && e.coord.get_y() < range.y_max {
                            let center = c.transform.trans(coord_to_pixel_x(e.coord.get_x(), range.x_min) ,
                                                          coord_to_pixel_y(e.coord.get_y(), range.y_min));
                            e.render(gl, center);
                        }
                    }

                }
            }

            // Render all bots
            for x in &mut self.bots {
                if let &mut Some(ref mut b) = x {
                    if b.coord.get_x() >= range.x_min &&  b.coord.get_x() < range.x_max &&
                        b.coord.get_y() >= range.y_min && b.coord.get_y() < range.y_max {

                        let center_b1 = c.transform.trans(coord_to_pixel_x(b.coord.get_x(), range.x_min ),
                                                          coord_to_pixel_y(b.coord.get_y(), range.y_min));
                        b.render(gl, center_b1);
                        // Render all Effects in Camera
                        for e in &b.effect.effects {
                        if e.coord.get_x() >= range.x_min &&  e.coord.get_x() < range.x_max &&
                            e.coord.get_y() >= range.y_min && e.coord.get_y() < range.y_max {
                            let center = c.transform.trans(coord_to_pixel_x(e.coord.get_x(), range.x_min) ,
                                                      coord_to_pixel_y(e.coord.get_y(), range.y_min));
                            e.render(gl, center);
                        }
                    }
                }
            }
            }
        });
    }
    /// Updates all Players, Bots, effects and camera
    fn on_update(&mut self,
                 args: &UpdateArgs,
                 level: &mut Level,
                 state: usize,
                 mut sounds: &mut SoundHandler) {
        // Update Coordinates


        let (coord1, coord2) = match (&self.players[0], &self.players[1]) {
            (&None, &None) => (Coordinate::new(0,0), Coordinate::new(0,0)),
            (&None, &Some(ref y)) => (Coordinate::new(0,0), y.coord.clone()),
            (&Some(ref x), &None) => (x.coord.clone(), Coordinate::new(0,0)),
            (&Some(ref x), &Some(ref y)) => (x.coord.clone(), y.coord.clone()),
        };
        // Update range with coordinates
        let range = self.cam.get_range_update();
        // Update Player one
        for (i, x) in &mut self.players.iter_mut().enumerate() {
            if let &mut Some(ref mut p) = x {
                let id = if let InteractableType::Player(x) = p.get_interactable_type() {x} else {42};
                p.on_update(args, range, level, InteractableType::Player(id), &mut sounds);
                if id == 1 {
                    self.hub_one.on_update(&p);
                } else if id == 2 {
                    self.hub_two.on_update(&p);
                }


                for i in &mut self.items {
                    i.collect(p);
                }
                for e in &mut p.effect.effects{
                    if !e.get_played(){
                        sounds.play(e.get_sound_str());
                        e.played();
                    }
                }
            }
        }
        for x in &mut self.bots {
            if let &mut Some(ref mut b) = x{
                b.on_update(args, range, level, state, &mut sounds)
            }
        }
        // Update Camera
        self.cam.calc_coordinates(coord1, coord2, level);
        self.items.retain(|ref i| !i.get_gone());

    }

    fn show_ingame_menu(&mut self, window: &mut PistonWindow, sounds: &mut SoundHandler) {
        // Show menu
        let mut settings = true;

        let assets = find_folder::Search::ParentsThenKids(1, 1).for_folder("assets").unwrap();
        let ref font = assets.join("font.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();

        let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                     CAM_BORDER * 2) as u32;

        let mut sub_start_menu = Vec::with_capacity(3);
        sub_start_menu.push("Resume");
        sub_start_menu.push(match self.muted { true => "Unmute", false => "Mute" });
        sub_start_menu.push("Exit Game");
        let sub_menu_size = sub_start_menu.len();
        let mut sub_active_index = 0;

        while let Some(a) = window.next() {
            if !settings {
                break;
            }
            if let Some(_) = a.render_args() {
                window.draw_2d(&a, |c, gl| {
                    // Clear the screen.
                    clear(BLACK, gl);

                    // Render menu
                    text::Text::new_color(WHITE, 32)
                        .draw("Game Paused",
                              &mut glyphs,
                              &c.draw_state,
                              c.transform
                                  .trans(width as f64 / 2.0 - 150.0, 100.0),
                              gl);

                    let mut distance = 0.0;

                    for s in &sub_start_menu {
                        let color =
                            match &sub_start_menu[sub_active_index] == s {
                                true => WHITE,
                                false => GREY,
                            };

                        text::Text::new_color(color, 32)
                            .draw(s,
                                  &mut glyphs,
                                  &c.draw_state,
                                  c.transform
                                      .trans(width as f64 / 2.0 - 100.0,
                                             300.0 + distance),
                                  gl);
                        distance += 50.0;
                    }
                });
            }
            if let Some(b) = a.press_args() {
                match b {
                    /* Check arrow keys for menu */
                    Button::Keyboard(Key::Return) => {
                        match sub_active_index {
                            // Resume
                            0 => {
                                settings = false;
                            }
                            // Mute
                            1 => {
                                if sub_start_menu[1] == "Mute" {
                                    for sound in sounds.map.values_mut() {
                                        sound.set_volume(0.0);
                                        self.muted = true;
                                    }
                                    sub_start_menu[1] = "Unmute";
                                } else {
                                    for sound in sounds.map.values_mut() {
                                        sound.set_volume(1.0);
                                        self.muted = false;
                                    }
                                    sub_start_menu[1] = "Mute";
                                }
                            }
                            // Back
                            2 => process::exit(1),
                            _ => (),
                        }
                    }
                    Button::Keyboard(Key::Down) => {
                        if sub_active_index == sub_menu_size - 1 {
                            sub_active_index = 0;
                        } else {
                            sub_active_index += 1;
                        }
                    }
                    Button::Keyboard(Key::Up) => {
                        if sub_active_index == 0 {
                            sub_active_index = sub_menu_size - 1;
                        } else {
                            sub_active_index -= 1;
                        }
                    }
                    Button::Keyboard(Key::Escape) => {
                        settings = false;
                    }
                    _ => (),
                }
            }
        }
    }

    /// Handles Input
    fn on_input(&mut self,
                inp: Button,
                pressed: bool,
                level: &mut Level,
                sounds: &mut SoundHandler,
                window: &mut PistonWindow) {

        match inp {
            Button::Keyboard(Key::Escape) => {
                if pressed {
                    self.show_ingame_menu(window, sounds);
                }
            }
            Button::Keyboard(Key::Q) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {


                        p.life -= 10;
                        p.weapon = EffectOption::Spear;
                    }
                    p.pressed = pressed;
                }
            }
            Button::Keyboard(Key::Up) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Up;
                        p.dir = LastKey::Up;
                    }
                    p.pressed = pressed;
                }
            }
            Button::Keyboard(Key::Down) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Down;
                        p.dir = LastKey::Down;
                    }
                    p.pressed = pressed;
                }
            }
            Button::Keyboard(Key::Left) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Left;
                        p.dir = LastKey::Left;
                    }
                    p.pressed = pressed;
                }
            }
            Button::Keyboard(Key::Right) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Right;
                        p.dir = LastKey::Right;
                    }
                    p.pressed = pressed;
                }
            }
            Button::Keyboard(Key::W) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Up;
                        x.dir = LastKey::Up;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::S) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Down;
                        x.dir = LastKey::Down;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::A) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Left;
                        x.dir = LastKey::Left;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::D) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Right;
                        x.dir = LastKey::Right;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::Return) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.attack(level, &mut self.bots);
                    }
                }
            }
            Button::Keyboard(Key::Space) => {
                if let &mut Some(ref mut p) = &mut self.players[1] {
                    if pressed {
                        p.attack(level, &mut self.bots);
                    }
                }
            }
            _ => {}

        }
    }
}


fn show_menu(e: Event, window: &mut PistonWindow, sounds: &mut SoundHandler, glyphs: &mut Glyphs, start_menu: &[&str], ai: u32, app: &mut App) -> (bool, u32) {
    let mut active_index = ai;
    let mut start_game = false;
    let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
             CAM_BORDER * 2) as u32;
    if let Some(i) = e.press_args() {
        match i {
            /* Check arrow keys for menu */
            Button::Keyboard(Key::Return) => {
                match active_index {
                    // Start Game
                    0 => start_game = true,
                    // Load Game
                    1 => (),
                    // Settings
                    2 => {
                        // Submenu Settings
                        let mut settings = true;

                        let mut sub_start_menu =
                            vec!["Fullscreen (not working yet)", "Mute", "Back"];
                        let sub_menu_size = sub_start_menu.len();
                        let mut sub_active_index = 0;

                        while let Some(a) = window.next() {
                            if !settings {
                                break;
                            }
                            if let Some(_) = a.render_args() {
                                window.draw_2d(&a, |c, gl| {
                                    // Clear the screen.
                                    clear(BLACK, gl);

                                    // Render menu
                                    text::Text::new_color(WHITE, 32)
                                        .draw(start_menu[2],
                                              glyphs,
                                              &c.draw_state,
                                              c.transform
                                                  .trans(width as f64 / 2.0 - 80.0, 100.0),
                                              gl);

                                    let mut distance = 0.0;

                                    for s in &sub_start_menu {
                                        let color =
                                            match &sub_start_menu[sub_active_index] == s {
                                                true => WHITE,
                                                false => GREY,
                                            };

                                        text::Text::new_color(color, 32)
                                            .draw(s,
                                                  glyphs,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 100.0,
                                                             300.0 + distance),
                                                  gl);
                                        distance += 50.0;
                                    }
                                });
                            }
                            if let Some(b) = a.press_args() {
                                match b {
                                    /* Check arrow keys for menu */
                                    Button::Keyboard(Key::Return) => {
                                        match sub_active_index {
                                            // Fullscreen
                                            0 => {
                                                /* Neue Zuweisung funktioniert leider nicht (panickt!)
                                                   Neue Idee gesucht
                                                 window = WindowSettings::new(format!("{} {}", GAME_NAME_PART1, GAME_NAME_PART2),
                                                            [width, height])
                                                    .exit_on_esc(true)
                                                    .fullscreen(true)
                                                    .resizable(false)
                                                    .build()
                                                    .unwrap();*/
                                            }
                                            // Mute
                                            1 => {
                                                if sub_start_menu[1] == "Mute" {
                                                    for sound in sounds.map.values_mut() {
                                                        sound.set_volume(0.0);
                                                        app.muted = true;
                                                    }
                                                    sub_start_menu[1] = "Unmute";
                                                } else {
                                                    for sound in sounds.map.values_mut() {
                                                        sound.set_volume(1.0);
                                                        app.muted = false;
                                                    }
                                                    sub_start_menu[1] = "Mute";
                                                }
                                            }
                                            // Back
                                            2 => settings = false,
                                            _ => (),
                                        }
                                    }
                                    Button::Keyboard(Key::Down) => {
                                        if sub_active_index == sub_menu_size - 1 {
                                            sub_active_index = 0;
                                        } else {
                                            sub_active_index += 1;
                                        }
                                    }
                                    Button::Keyboard(Key::Up) => {
                                        if sub_active_index == 0 {
                                            sub_active_index = sub_menu_size - 1;
                                        } else {
                                            sub_active_index -= 1;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    // Exit
                    3 => process::exit(1),
                    _ => (),
                }
            }
            Button::Keyboard(Key::Down) => {
                if active_index == (start_menu.len() - 1) as u32 {
                    active_index = 0;
                } else {
                    active_index += 1;
                }
            }
            Button::Keyboard(Key::Up) => {
                if active_index == 0 {
                    active_index = (start_menu.len() - 1) as u32;
                } else {
                    active_index -= 1;
                }
            }
            _ => (),
        }
    }
    if let Some(_) = e.render_args() {
        window.draw_2d(&e, |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Render menu
            text::Text::new_color(WHITE, 32).draw(GAME_NAME_PART1,
                                                  glyphs,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 180.0,
                                                             100.0),
                                                  gl);
            text::Text::new_color(WHITE, 32).draw(GAME_NAME_PART2,
                                                  glyphs,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 200.0,
                                                             150.0),
                                                  gl);

            let mut distance = 0.0;

            for s in start_menu {
                let color = match &start_menu[active_index as usize] == s {
                    true => WHITE,
                    false => GREY,
                };

                text::Text::new_color(color, 32).draw(s,
                                                      glyphs,
                                                      &c.draw_state,
                                                      c.transform
                                                          .trans(width as f64 / 2.0 -
                                                                 100.0,
                                                                 400.0 + distance),
                                                      gl);
                distance += 50.0;
            }
        });
    }
    (start_game, active_index)
}

/// Main
fn main() {
    let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                 CAM_BORDER * 2) as u32;
    let height = ((((CAMERA_BUF_Y * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                  CAM_BORDER + HUB_UP) as u32;
    let mut window: PistonWindow =
        WindowSettings::new(format!("{} {}", GAME_NAME_PART1, GAME_NAME_PART2),
                            [width, height])
            .exit_on_esc(false)
            .fullscreen(false)
            .resizable(false)
            .build()
            .unwrap();

    // Create window
    let mut events = window.events();

    // Create map for sprites and load all sprites
    let map = Settings::new(&mut window).sprite_map;

    // Lade XML und erstelle daraus das Level, das Tileset, die Player und die Bots
    let folder_level = match find_folder::Search::Kids(0).for_folder("src") {
        Ok(res) => res.join("level1.xml"),
        Err(_) => panic!("Folder 'src' not found!"),
    };

    let level_path = match folder_level.to_str() {
        Some(res) => res,
        None => panic!("Level not found!"),
    };

    let (lv, ts, bots, players, items) = load_xml(level_path, &map, &mut window);

    let tileset = ts;

    let mut level = lv;

    // Create SoundHandler
    let mut sounds = SoundHandler::fill();

    // Create new app with one or two players
    let mut app = App::new(players, bots, items);

    // insert players in level
    for x in &mut app.players {
        if let  &mut Some(ref mut p) =  x {
            level.get_data()[p.coord.get_x() as usize][p.coord.get_y() as usize]
                .set_fieldstatus(p.get_interactable_type());
            p.set_borders((level.get_width() as u64, level.get_height() as u64));

        }
    }

    // Start counter
    let mut start = PreciseTime::now();

    // set Level-borders to camera
    app.cam.set_borders((level.get_width() as u64, level.get_height() as u64));


    // Set hubs
    if let Some(ref mut p1) = app.players[0] {
        app.hub_one.set_map(&map);
    }
    // load sprite for player two and sets border
    if let Some(ref mut p2) = app.players[1] {
        app.hub_two.set_map(&map);
    }

    // Load sprite for each bot and set borders
    for x in &mut app.bots {
        if let &mut Some(ref mut b) = x {
                b.set_borders((level.get_width() as u64, level.get_height() as u64));
        }
    }

    let mut start_game = false;

    let assets = find_folder::Search::ParentsThenKids(1, 1).for_folder("assets").unwrap();
    let ref font = assets.join("font.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    let start_menu = vec!["Start Game", "Load Game", "Settings", "Exit"];

    let mut active_index = 0;
    sounds.play("Welcome.ogg");

    while let Some(e) = events.next(&mut window) {
        if !start_game {
            let (start, index) = show_menu(e, &mut window, &mut sounds, &mut glyphs, &start_menu, active_index, &mut app);
            start_game = start;
            active_index = index;
        } else {
            // End of Loading start game
            // Calculate Milliseconds
            let now = start.to(PreciseTime::now()).num_milliseconds();

            // Calculate state
            let state = if now <= 500 { 0 } else { 1 };

            // If Render-Event
            if let Some(_) = e.render_args() {
                app.on_draw(&mut window, &e, &tileset, &mut level);
            }

            // If Key-Press-Event
            if let Some(i) = e.release_args() {
                app.on_input(i, false, &mut level, &mut sounds, &mut window);
            }
            // If Key-releas-Event
            if let Some(i) = e.press_args() {

                app.on_input(i, true, &mut level, &mut sounds, &mut window);
            }
            {
                // if update
                if let Some(u) = e.update_args() {
                    app.on_update(&u, &mut level, state, &mut sounds);
                }

                // restart time if 1 second over
                if now > 1000 {
                    start = PreciseTime::now();
                }
            }
        }
    }
}
