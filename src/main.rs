// Main Module
extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate image as im;
extern crate time;
extern crate rand;
extern crate ears;
extern crate xml;

use piston_window::*;
use time::PreciseTime;
use ears::AudioController;

// modules
mod player;
mod io;
mod level;
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
use player_hub::PlayerHub;
use item::Item;
use coord::Coordinate;
use rand::Rng;

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
/// players: Vector with all bots
/// bots: Vector of all bots
/// items: Vector of all Items
/// hub_one: Hub for Player_one
/// hub_two: Hub for Player_two
/// cam: the Camera
/// muted: For muting sounds
pub struct App<'a> {
    players: Vec<Option<Player<'a>>>,
    bots: Vec<Option<Bot<'a>>>,
    items: Vec<Item<'a>>,
    cam: Cam,
    hub_one: PlayerHub<'a>,
    hub_two: PlayerHub<'a>,
    muted: bool,
    glyph: Glyphs,
}

impl<'a> App<'a> {
    /// Constructor
    fn new(players: Vec<Option<Player<'a>>>,
           bots: Vec<Option<Bot<'a>>>,
           items: Vec<Item<'a>>,
           factory: gfx_device_gl::Factory)
           -> Self {

        let assets = find_folder::Search::ParentsThenKids(1, 1).for_folder("assets").unwrap();
        let ref font = assets.join("font.ttf");
        let glyphs = Glyphs::new(font, factory).unwrap();

        App {
            players: players,
            bots: bots,
            items: items,
            cam: Cam::new(CAMERA_BUF_X, CAMERA_BUF_Y),
            hub_one: PlayerHub::new(None),
            hub_two: PlayerHub::new(None),
            muted: false,
            glyph: glyphs,
        }
    }

    /// Handle the rendering of all Objects
    fn on_draw(&mut self,
               mut w: &mut PistonWindow,
               e: &Event,
               tileset: &Tileset,
               level: &mut Level) {
        // Calculate Range of the camera
        let range = self.cam.get_range();

        // draw in 2D
        w.draw_2d(e, |c, gl| {
            // Clear the screen with black
            clear(BLACK, gl);

            // Render Hubs only if player is available
            if let Some(ref p) = self.players[0] {
                if p.is_alive() {
                    let center_hub_one = c.transform.trans(10.0, 10.0);
                    self.hub_one.render(gl, center_hub_one);
                }
            }
            if let Some(ref p) = self.players[1] {
                if p.is_alive() {
                    let center_hub_two = c.transform.trans(280.0, 10.0);
                    self.hub_two.render(gl, center_hub_two);
                }
            }

            // Position for Tileset in Window
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

            // Render all in-camera items
            for i in &mut self.items {
                    if i.coord.get_x() >= range.x_min &&  i.coord.get_x() < range.x_max &&
                            i.coord.get_y() >= range.y_min && i.coord.get_y() < range.y_max {
                        // position in window for current item
                        let center_b1 = c.transform.trans(coord_to_pixel_x(i.coord.get_x(), range.x_min ),
                                                          coord_to_pixel_y(i.coord.get_y(), range.y_min));
                        // render current item
                        i.render(gl, center_b1);
                }
            }

            // Render all players if not None
            for x in &mut self.players {
                if let &mut Some(ref mut p) = x {

                    // position of Player in Pixel coordinates
                    let center_p = c.transform.trans(coord_to_pixel_x(p.coord.get_x(), range.x_min),
                                                  coord_to_pixel_y(p.coord.get_y(), range.y_min));
                    if p.is_alive() {
                    // render player one
                        p.render(gl, center_p);
                    }
                    // render all effects of player
                    for e in &p.get_effect_handler().effects {
                        // Render only Effects in Camera
                        if e.coord.get_x() >= range.x_min &&  e.coord.get_x() < range.x_max &&
                                e.coord.get_y() >= range.y_min && e.coord.get_y() < range.y_max {
                            // Pixel coordinates of player
                            let center = c.transform.trans(coord_to_pixel_x(e.coord.get_x(), range.x_min) ,
                                                          coord_to_pixel_y(e.coord.get_y(), range.y_min));
                            // Render effect
                            e.render(gl, center);
                        }
                    }

                }
            }

            // Render all bots
            for x in &mut self.bots {
                if let &mut Some(ref mut b) = x {
                    // Render only Bots in Camera
                    if b.coord.get_x() >= range.x_min &&  b.coord.get_x() < range.x_max &&
                        b.coord.get_y() >= range.y_min && b.coord.get_y() < range.y_max {
                        // Pixel coordinates of bot
                        let center_b1 = c.transform.trans(coord_to_pixel_x(b.coord.get_x(), range.x_min ),
                                                          coord_to_pixel_y(b.coord.get_y(), range.y_min));
                        // only render if alive. Bot can be dead, but not None, untill all his effects have rendered
                        if b.is_alive(){
                            b.render(gl, center_b1);
                        }

                        // Render all Effects in Camera from bot
                        for e in &b.effect.effects {
                            if e.coord.get_x() >= range.x_min &&  e.coord.get_x() < range.x_max &&
                                e.coord.get_y() >= range.y_min && e.coord.get_y() < range.y_max {
                                // pixel coordinate of effect
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
    /// Updates all Players, Bots, effects, items and camera
    fn on_update(&mut self, level: &mut Level, state: usize, mut sounds: &mut SoundHandler) {
        // Update range
        let range = self.cam.get_range_update();
        // Update Players
        for x in &mut self.players.iter_mut() {
            if let &mut Some(ref mut p) = x {
                let id = if let InteractableType::Player(x) = p.get_interactable_type() {
                    x
                } else {
                    0
                };
                // Update Player
                p.on_update(range, level, InteractableType::Player(id), &mut sounds);
                // Update hubs
                if id == 1 {
                    self.hub_one.on_update(&p);
                } else if id == 2 {
                    self.hub_two.on_update(&p);
                }
                if p.is_alive() {
                    // check if on item and update item
                    for i in &mut self.items {
                        i.collect(p);
                    }

                    // update all effects of player
                    for e in &mut p.effect.effects {
                        if !e.get_played() {
                            sounds.play(e.get_sound_str());
                            e.played();
                        }
                    }
                } else {
                    // on player death handle death effect
                    if !p.dead {
                        p.effect.handle(p.coord, EffectOption::PlayerDeath, LastKey::Wait);
                        p.dead = true;
                    }
                    match level.get_data()[p.coord.get_x() as usize][p.coord.get_y() as usize].get_fieldstatus() {
                        Some(InteractableType::Player(i)) => {
                            match p.get_interactable_type() {
                                InteractableType::Player(a) => {
                                    if i == a {
                                        // free the field, on which the player stood
                                        level.get_data()[p.coord.get_x() as usize][p.coord.get_y() as usize].free_fieldstatus();
                                    }
                                },
                                _ => (),
                            }
                        },
                        _ => (),
                    }
                }
            }
        }
        // Set player to None, when death effect ends
        for i in 0..self.players.len() {
            let mut delete = false;
            if let Some(ref p) = self.players[i] {
                if p.dead {
                    delete = p.effect.effects.len() == 0;
                }
            }
            if delete {
                self.players[i] = None;
            }
        }
        // Vec with dead bots
        let mut dead = Vec::new();
        for x in &mut self.bots {
            if let &mut Some(ref mut b) = x {
                // if bot dead
                if !b.is_alive() {
                    // and not set dead yet (two bools, because last effect have to render)
                    if !b.dead {
                        // create last effect
                        b.effect.handle(b.coord, EffectOption::Dead, LastKey::Wait);
                        b.dead = true;
                    }
                    // Field not longer occupied
                    level.get_data()[b.coord.get_x() as usize][b.coord.get_y() as usize]
                        .free_fieldstatus();
                    // If Bot, add to dead bots
                    if let InteractableType::Bot(i) = b.get_interactable_type() {
                        dead.push(i);
                    }
                }
                // update bot (In Bot no update if already dead. Only updates of effects)
                b.on_update(level, state, &mut sounds, &mut self.players);
            }
        }
        // Delete all dead Bots (Set to None)
        for d in dead {
            let mut delete = false;
            if let Some(ref b) = self.bots[d as usize] {
                delete = b.effect.effects.len() == 0;
            }
            if delete {
                self.bots[d as usize] = None;
            }
        }

        // Update Camera
        let (coord1, coord2) = match (&self.players[0], &self.players[1]) {
            (&None, &None) => (Coordinate::new(0, 0), Coordinate::new(0, 0)),
            (&None, &Some(ref y)) => (y.coord.clone(), y.coord.clone()),
            (&Some(ref x), &None) => (x.coord.clone(), x.coord.clone()),
            (&Some(ref x), &Some(ref y)) => (x.coord.clone(), y.coord.clone()),
        };
        self.cam.calc_coordinates(coord1, coord2, level);
        self.items.retain(|ref i| !i.get_gone());

    }

    fn show_ingame_menu(&mut self, window: &mut PistonWindow, sounds: &mut SoundHandler) {
        // Show menu
        let mut settings = true;

        let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                     CAM_BORDER * 2) as u32;

        // Create menu list
        let mut sub_start_menu = Vec::with_capacity(3);
        sub_start_menu.push("Resume");
        sub_start_menu.push(match self.muted {
            true => "Unmute",
            false => "Mute",
        });
        sub_start_menu.push("Exit Game");
        let sub_menu_size = sub_start_menu.len();
        let mut sub_active_index = 0;

        // render window
        while let Some(a) = window.next() {
            if !settings {
                break;
            }
            if let Some(_) = a.render_args() {
                window.draw_2d(&a, |c, gl| {
                    // Clear the screen.
                    clear(BLACK, gl);

                    // Render menu
                    text::Text::new_color(WHITE, 32).draw("Game Paused",
                                                          &mut self.glyph,
                                                          &c.draw_state,
                                                          c.transform
                                                              .trans(width as f64 / 2.0 - 150.0,
                                                                     100.0),
                                                          gl);

                    let mut distance = 0.0;

                    // Render menu list elements
                    for s in &sub_start_menu {
                        let color = match &sub_start_menu[sub_active_index] == s {
                            true => WHITE,
                            false => GREY,
                        };

                        text::Text::new_color(color, 32).draw(s,
                                                              &mut self.glyph,
                                                              &c.draw_state,
                                                              c.transform
                                                                  .trans(width as f64 / 2.0 -
                                                                         100.0,
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
                                    if let Some(music) = sounds.map.get_mut("Background.ogg"){
                                        music.set_volume(1.0 / 16.0);
                                    }
                                }
                            }
                            // Back
                            2 => process::exit(1),
                            _ => (),
                        }
                    }
                    // adjust intern menu moves
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
        // Match Key
        match inp {
            // Activate Menu
            Button::Keyboard(Key::Escape) => {
                if pressed {
                    self.show_ingame_menu(window, sounds);
                }
            }
            // Up Player One
            Button::Keyboard(Key::Up) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Up;
                        p.dir = LastKey::Up;
                    }
                    p.pressed = pressed;
                }
            }
            // Down Player One
            Button::Keyboard(Key::Down) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Down;
                        p.dir = LastKey::Down;
                    }
                    p.pressed = pressed;
                }
            }
            // Left Player One
            Button::Keyboard(Key::Left) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Left;
                        p.dir = LastKey::Left;
                    }
                    p.pressed = pressed;
                }
            }
            // Right Player One
            Button::Keyboard(Key::Right) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.last = LastKey::Right;
                        p.dir = LastKey::Right;
                    }
                    p.pressed = pressed;
                }
            }
            // Up Player Two
            Button::Keyboard(Key::W) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Up;
                        x.dir = LastKey::Up;
                    }
                    x.pressed = pressed;
                }
            }
            // Down Player Two
            Button::Keyboard(Key::S) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Down;
                        x.dir = LastKey::Down;
                    }
                    x.pressed = pressed;
                }
            }
            // Left Player Two
            Button::Keyboard(Key::A) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Left;
                        x.dir = LastKey::Left;
                    }
                    x.pressed = pressed;
                }
            }
            // Right Player Two
            Button::Keyboard(Key::D) => {
                if let &mut Some(ref mut x) = &mut self.players[1] {
                    if pressed {
                        x.last = LastKey::Right;
                        x.dir = LastKey::Right;
                    }
                    x.pressed = pressed;
                }
            }
            // Attack Player One
            Button::Keyboard(Key::Return) => {
                if let &mut Some(ref mut p) = &mut self.players[0] {
                    if pressed {
                        p.attack(level, &mut self.bots);
                    } else {
                        p.delay_attack = false;
                    }
                }
            }
            // Attack Player Two
            Button::Keyboard(Key::Space) => {
                if let &mut Some(ref mut p) = &mut self.players[1] {
                    if pressed {
                        p.attack(level, &mut self.bots);
                    } else {
                        p.delay_attack = false;
                    }
                }
            }
            _ => {}
        }
    }
    fn background_music(&self, sounds: &mut SoundHandler, game_over: bool) {
        if !self.muted{
            let background_sound = sounds.map.get_mut("Background.ogg");
            if let Some(music) = background_sound {
                if game_over {
                    if music.is_playing() {
                        music.stop();
                    }
                } else {
                    if !music.is_playing() {
                        music.play();
                    }
                    music.set_volume(1.0 / 16.0);
                }
            }
        }
    }
}

fn select_player(window: &mut PistonWindow, app: &mut App) -> bool {
    // Select whether you want to play with one or two players
    let mut two_players = false;
    let mut select = true;

    let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                 CAM_BORDER * 2) as u32;

    let start_menu = vec!["One Player", "Two Players"];
    let mut active_index = 0;

    // render window
    while let Some(a) = window.next() {
        if !select {
            break;
        }
        if let Some(_) = a.render_args() {
            window.draw_2d(&a, |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);

                // Render menu
                let mut distance = 0.0;

                for s in &start_menu {
                    let color = match &start_menu[active_index] == s {
                        true => WHITE,
                        false => GREY,
                    };

                    text::Text::new_color(color, 32).draw(s,
                                                          &mut app.glyph,
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
                    match active_index {
                        // One Player
                        0 => {
                            two_players = false;
                        }
                        // Two Players
                        1 => {
                            two_players = true;
                        }
                        _ => (),
                    };
                    select = false;
                }
                // adjust intern menu moves
                Button::Keyboard(Key::Down) => {
                    if active_index == start_menu.len() - 1 {
                        active_index = 0;
                    } else {
                        active_index += 1;
                    }
                }
                Button::Keyboard(Key::Up) => {
                    if active_index == 0 {
                        active_index = start_menu.len() - 1;
                    } else {
                        active_index -= 1;
                    }
                }
                _ => (),
            }
        }
    }
    two_players
}

fn show_menu(e: Event,
    mut window: &mut PistonWindow,
    sounds: &mut SoundHandler,
    start_menu: &[&str],
    ai: u32,
    app: &mut App,
    menu_bots: &mut Vec<Bot>,
    start: PreciseTime) -> (bool, bool, u32, i64) {
    let mut rng = rand::thread_rng();
    let mut state = start.to(PreciseTime::now()).num_milliseconds();
    let mut active_index = ai;
    let mut start_game = false;
    let mut two_players = false;
    let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
             CAM_BORDER * 2) as u32;
    let height = ((((CAMERA_BUF_Y * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                  CAM_BORDER + HUB_UP) as u32;
    if let Some(i) = e.press_args() {
        match i {
            /* Check arrow keys for menu */
            Button::Keyboard(Key::Return) => {
                match active_index {
                    // Start Game
                    0 => {
                        start_game = true;
                        // Spielerauswahl
                        // Mute Welcome Sound
                        match sounds.map.get_mut("Welcome.ogg") {
                            Some(s) => s.set_volume(0.0),
                            None => (),
                        };
                        two_players = select_player(window, app);
                    }
                    // Load Game
                    1 => (),
                    // Settings
                    2 => {
                        // Submenu Settings
                        let mut settings = true;

                        let mut sub_start_menu = Vec::with_capacity(3);
                        sub_start_menu.push("Show Controls");
                        sub_start_menu.push(match app.muted {
                            true => "Unmute",
                            false => "Mute",
                        });
                        sub_start_menu.push("Back");
                        let sub_menu_size = sub_start_menu.len();
                        let mut sub_active_index = 0;

                        // render window
                        while let Some(a) = window.next() {
                            if !settings {
                                break;
                            }
                            if let Some(_) = a.render_args() {
                                window.draw_2d(&a, |c, gl| {
                                    // Clear the screen.
                                    clear(BLACK, gl);

                                    // Render menu
                                    text::Text::new_color(WHITE, 32).draw(start_menu[2],
                                                                          &mut app.glyph,
                                                                          &c.draw_state,
                                                                          c.transform
                                                                              .trans(width as f64 /
                                                                                     2.0 -
                                                                                     80.0,
                                                                                     100.0),
                                                                          gl);

                                    let mut distance = 0.0;

                                    for s in &sub_start_menu {
                                        let color = match &sub_start_menu[sub_active_index] == s {
                                            true => WHITE,
                                            false => GREY,
                                        };

                                        text::Text::new_color(color, 32).draw(s,
                                                                              &mut app.glyph,
                                                                              &c.draw_state,
                                                                              c.transform
                                                                                  .trans(width as
                                                                                         f64 /
                                                                                         2.0 -
                                                                                         100.0,
                                                                                         300.0 +
                                                                                         distance),
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
                                            // Controls
                                            0 => {
                                                show_controls(&mut window, app);
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
                                    Button::Keyboard(Key::Escape) => {
                                        settings = false;
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
            // adjust intern menu moves
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

            // create moving start screen
            let temp = if state <= 500 { 0 } else { 1 };

            if temp == 1 {
                // Left chicken
                let dir = rng.gen_range(0, 4);

                let mut new_x = menu_bots[0].coord.get_x();
                let mut new_y = menu_bots[0].coord.get_y();

                match dir {
                    // UP
                    0 => {
                        if new_y >= 50 {
                            new_y -= 50;
                        }
                    }
                    // DOWN
                    1 => {
                        if new_y <= (height - 50) as u64 {
                            new_y += 50;
                        }
                    }
                    // LEFT
                    2 => {
                        if new_x >= 50 {
                            new_x -= 50;
                        }
                    }
                    // RIGHT
                    3 => {
                        if new_x <= 200 as u64 {
                            new_x += 50;
                        }
                    }
                    // ELSE
                    _ => (),
                };

                menu_bots[0].coord.set_coord(new_x, new_y);

                // Right chicken
                let dir = rng.gen_range(0, 4);

                new_x = menu_bots[1].coord.get_x();
                new_y = menu_bots[1].coord.get_y();

                match dir {
                    // UP
                    0 => {
                        if new_y >= 50 {
                            new_y -= 50;
                        }
                    }
                    // DOWN
                    1 => {
                        if new_y <= (height - 50) as u64 {
                            new_y += 50;
                        }
                    }
                    // LEFT
                    2 => {
                        if new_x >= 850 {
                            new_x -= 50;
                        }
                    }
                    // RIGHT
                    3 => {
                        if new_x <= 1000 as u64 {
                            new_x += 50;
                        }
                    }
                    // ELSE
                    _ => (),
                };

                menu_bots[1].coord.set_coord(new_x, new_y);
            }

            menu_bots[0].render(gl, c.transform.trans(menu_bots[0].coord.get_x() as f64, menu_bots[0].coord.get_y() as f64));
            menu_bots[1].render(gl, c.transform.trans(menu_bots[1].coord.get_x() as f64, menu_bots[1].coord.get_y() as f64));

            if temp == 1 {
                state = 0;
            }

            // Render menu
            text::Text::new_color(WHITE, 32).draw(GAME_NAME_PART1,
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 180.0, 100.0),
                                                  gl);
            text::Text::new_color(WHITE, 32).draw(GAME_NAME_PART2,
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 200.0, 150.0),
                                                  gl);

            let mut distance = 0.0;

            for s in start_menu {
                let color = match &start_menu[active_index as usize] == s {
                    true => WHITE,
                    false => GREY,
                };

                text::Text::new_color(color, 32).draw(s,
                                                      &mut app.glyph,
                                                      &c.draw_state,
                                                      c.transform
                                                          .trans(width as f64 / 2.0 - 100.0,
                                                                 400.0 + distance),
                                                      gl);
                distance += 50.0;
            }
        });
    }
    (start_game, two_players, active_index, state)

}

fn show_controls(window: &mut PistonWindow, app: &mut App) {
    let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
             CAM_BORDER * 2) as u32;

    while let Some(a) = window.next() {
        if let Some(_) = a.render_args() {
            window.draw_2d(&a, |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);


                text::Text::new_color(WHITE, 32).draw("Controls",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 80.0, 100.0),
                                                  gl);

                /* Controls Player 1 */
                text::Text::new_color(GREY, 32).draw("Player 1",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 - 100.0, 150.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("Movement:",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 - 150.0, 250.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("[W]",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 - 50.0, 350.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("[A]    [S]    [D]",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 - 150.0, 400.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("Attack: [SPACE]",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 - 150.0, 500.0),
                                                  gl);

                /* Controls Player 2 */
                text::Text::new_color(GREY, 32).draw("Player 2",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 * 3.0 - 50.0, 150.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("Movement:",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 * 3.0 - 150.0, 250.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("[▲]",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 * 3.0 - 15.0, 350.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("[◀]    [▼]    [▶]",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 * 3.0 - 150.0, 400.0),
                                                  gl);

                text::Text::new_color(GREY, 32).draw("Attack: [RETURN]",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 4.0 * 3.0 - 150.0, 500.0),
                                                  gl);

                text::Text::new_color(WHITE, 32).draw("Back",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 50.0, 600.0),
                                                  gl);
            });
        }
        if let Some(i) = a.press_args() {
            match i {
                Button::Keyboard(Key::Return) |
                Button::Keyboard(Key::Escape) => {
                    return;
                },
                _ => (),
            }
        }
    }
}

/// Main1
fn main() {
    // Calculate size of Window
    let width = ((((CAMERA_BUF_X * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                 CAM_BORDER * 2) as u32;
    let height = ((((CAMERA_BUF_Y * 2) + 1) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) +
                  CAM_BORDER + HUB_UP) as u32;
    // Create Window
    let mut window: PistonWindow =
        WindowSettings::new(format!("{} {}", GAME_NAME_PART1, GAME_NAME_PART2),
                            [width, height])
            .exit_on_esc(false)
            .fullscreen(false)
            .resizable(false)
            .build()
            .unwrap();

    // CreateEventHandler
    let mut events = window.events();

    // Create map for sprites and load all sprites
    let map = Settings::new(&mut window).sprite_map;

    // Load XML with all neccessary Data
    let folder_level = match find_folder::Search::Kids(0).for_folder("src") {
        Ok(res) => res.join("level1.xml"),
        Err(_) => panic!("Folder 'src' not found!"),
    };
    // Save Path
    let level_path = match folder_level.to_str() {
        Some(res) => res,
        None => panic!("Level not found!"),
    };

    // Create SoundHandler
    let mut sounds = SoundHandler::fill();

    let mut exit = false;

    // Load all neccessary Data (takes a while)
    let (lv, ts, bots, players, items) = load_xml(level_path, &map, &mut window.factory);

    // Exit Game via process::exit(), so infinite loop until player exits game
    while !exit {
        exit = true;

        let tileset = &ts;

        // save level
        let mut level = lv.clone();

        // Clone players and bots for Re-Use in next round
        let mut p2 = Vec::with_capacity(players.len());
        for p in &players {
            match p {
                &Some(ref a) => {
                    p2.push(Some(a.clone_without_effects(&map)));
                },
                &None => p2.push(None),
            }
        }
        let mut b2 = Vec::with_capacity(bots.len());
        for b in &bots {
            match b {
                &Some(ref a) => {
                    b2.push(Some(a.clone_without_effects(&map)));
                },
                &None => b2.push(None),
            }
        }

        // Create new app with one or two players
        let mut app = App::new(p2, b2, items.clone(), window.factory.clone());

        // insert players in level
        for x in &mut app.players {
            if let &mut Some(ref mut p) = x {
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
        if let Some(_) = app.players[0] {
            app.hub_one.set_map(&map);
        }
        // load sprite for player two and sets border
        if let Some(_) = app.players[1] {
            app.hub_two.set_map(&map);
        }

        // Load sprite for each bot and set borders
        for x in &mut app.bots {
            if let &mut Some(ref mut b) = x {
                b.set_borders((level.get_width() as u64, level.get_height() as u64));
                // insert bot into level
                level.get_data()[b.coord.get_x() as usize][b.coord.get_y() as usize]
                    .set_fieldstatus(b.get_interactable_type());
            }
        }
        // bool for testing if game is started
        let mut start_game = false;

        let start_menu = vec!["Start Game", "Load Game", "Settings", "Exit"];

        let mut active_index = 0;
        sounds.play("Welcome.ogg");

        let mut menu_bots = Vec::with_capacity(10);

        // Create bots for animated start screen
        let mut b = Bot::new(150, 200, 100, false, 1, &map);
        b.set_sprite(map.get_sprite("chicken_white.png".to_string()));
        menu_bots.push(b);
        let mut b2 = Bot::new(900, 400, 100, false, 2, &map);
        b2.set_sprite(map.get_sprite("chicken_pink.png".to_string()));
        menu_bots.push(b2);

        let mut game_over = false;
        let mut winning = false;

        // Start the game with the start screen
        while let Some(e) = events.next(&mut window) {
            if !game_over && !winning {
                if !start_game {
                    // show start menu
                    let (is_start, two_players, index, state) = show_menu(e,
                                                                &mut window,
                                                                &mut sounds,
                                                                &start_menu,
                                                                active_index,
                                                                &mut app,
                                                                &mut menu_bots,
                                                                start);

                    start_game = is_start;

                    active_index = index;
                    if start_game && !two_players {
                        // discard settings of player two
                        if let Some(ref mut p) = app.players[1] {
                            level.get_data()[p.coord.get_x() as usize][p.coord.get_y() as usize]
                                .free_fieldstatus();
                        }
                        app.players[1] = None;
                    }

                    if state == 0 {
                        start = PreciseTime::now();
                    }
                } else {
                    // End of loading, start game
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
                    // If Key-release-Event
                    if let Some(i) = e.press_args() {

                        app.on_input(i, true, &mut level, &mut sounds, &mut window);
                    }
                    // if update
                    if let Some(_) = e.update_args() {
                        app.on_update(&mut level, state, &mut sounds);

                        // Game over if both dead
                        game_over = app.players.iter().fold(true, |acc, x| {
                            acc && match *x { None => true, Some(_) => false }
                        });

                        // Winning if all bots dead
                        winning = app.bots.iter().fold(true, |acc, x| {
                            acc && match *x { None => true, Some(ref b) => {
                                b.passive
                            } }
                        });
                    }
                    // restart time if 1 second over
                    if now > 1000 {
                        start = PreciseTime::now();
                    }

                    app.background_music(&mut sounds, game_over || winning);
                }
            } else {
                break;
            }
        }

        /* Display Game Over */
        if game_over {

            // TODO: Play Game Over music



            while let Some(a) = events.next(&mut window) {
                if let Some(_) = a.render_args() {
                    window.draw_2d(&a, |c, gl| {
                        // Clear the screen.
                        clear(BLACK, gl);

                        text::Text::new_color(WHITE, 64).draw("Game Over",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 200.0, 200.0),
                                                  gl);

                        text::Text::new_color(WHITE, 32).draw("Go to Main Menu",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 150.0, 400.0),
                                                  gl);
                    });
                }
                // Return to main menu
                if let Some(i) = a.press_args() {
                    if i == Button::Keyboard(Key::Return) {
                        exit = false;
                        break;
                    }
                }
            }
        }

        if winning {
            while let Some(a) = events.next(&mut window) {
                if let Some(_) = a.render_args() {
                    window.draw_2d(&a, |c, gl| {
                        // Clear the screen.
                        clear(BLACK, gl);

                        text::Text::new_color(WHITE, 64).draw("You win!",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 150.0, 200.0),
                                                  gl);

                        text::Text::new_color(WHITE, 32).draw("Go to Main Menu",
                                                  &mut app.glyph,
                                                  &c.draw_state,
                                                  c.transform
                                                      .trans(width as f64 / 2.0 - 150.0, 400.0),
                                                  gl);
                    });
                }
                // Return to main menu
                if let Some(i) = a.press_args() {
                    if i == Button::Keyboard(Key::Return) {
                        exit = false;
                        break;
                    }
                }
            }
        }
    }
}
