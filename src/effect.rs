// Module for representing effects

use io::sprite::Sprite;
use coord::Coordinate;
use renderable::Renderable;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use player::*;
use time::PreciseTime;
use io::all_sprites::SpriteMap;

/// Struct for representing Effects.
/// sprites: Sprite of effect
/// coord: Coordinate of effect
/// mirror_h: true if mirror horizontal
/// degree for Rotation
/// start: Time for checking unique render cycle
pub struct Effect<'a> {
    pub sprite: Option<&'a Sprite>,
    pub coord: Coordinate,
    pub mirror_h: bool,
    pub degree: u32,
    start: PreciseTime,
    sound: &'a str,
    sound_played: bool,
}

impl<'a> Effect<'a> {
    /// Constructor for Effect
    pub fn new(coord: Coordinate) -> Self {
        Effect {
            sprite: None,
            coord: coord,
            start: PreciseTime::now(),
            mirror_h: false,
            degree: 0,
            sound: "",
            sound_played: false,
        }
    }
    /// Sets the sprite of the effect
    pub fn set_sprite(&mut self, sprite: Option<&'a Sprite>) {
        self.sprite = sprite;
    }
    /// Resets the time
    pub fn reset_time(&mut self) {
        self.start = PreciseTime::now();
    }

    pub fn set_sound_str(&mut self, sound: &'a str) {
        self.sound = sound;
    }

    pub fn get_sound_str(&self) -> &str {
        self.sound
    }

    pub fn get_played(&self) -> bool {
        self.sound_played
    }

    pub fn played(&mut self) {
        self.sound_played = true;
    }
}

/// Struct for Handeling the sprites
/// effects: Vec with all active Effects
/// map: SpriteMap with all sprites
pub struct EffectHandler<'a> {
    pub effects: Vec<Effect<'a>>,
    map: &'a SpriteMap,
}
impl<'a> EffectHandler<'a> {
    /// Constructor
    pub fn new(map: &'a SpriteMap) -> Self {
        EffectHandler {
            effects: Vec::new(),
            map: map,
        }
    }

    /// hadles an effect with given parameters
    /// coord: Coordinate of Effect
    /// typ: Type of Effect (Dead, Dagger-Attack...)
    /// Direction of Effect
    pub fn handle(&mut self, coord: Coordinate, typ: EffectOption, direction: LastKey) {
        // Clones given Coordinates
        let mut effect = Effect::new(coord.clone());

        // Checks for Type and direction
        match (typ, direction) {
            // if Dead no direction needed
            (EffectOption::Dead, _) => {
                effect.set_sprite(self.map.get_sprite("explosion.png".to_string()));
                effect.set_sound_str("Dead.ogg");
            }
            // Dagger-Attack
            (EffectOption::Dagger, x) => {
                effect.set_sprite(self.map.get_sprite("swipe_dagger.png".to_string()));
                effect.set_sound_str("Dagger.ogg");
                // Match direction
                match x {
                    // Moves coordinate up and rotate effect
                    LastKey::Up => {
                        effect.coord.force_move(0, -1);
                        effect.degree = 270
                    }
                    // Moves Coordinate down and rotate effect
                    LastKey::Down => {
                        effect.coord.force_move(0, 1);
                        effect.degree = 90
                    }
                    // Moves Coordinate left and mirror effect
                    LastKey::Left => {
                        effect.coord.force_move(-1, 0);
                        effect.mirror_h = true;
                    }
                    // Moves Coordinate right
                    LastKey::Right => {
                        effect.coord.force_move(1, 0);
                    }
                    _ => {}

                }
            }
            // Same as Dagger
            (EffectOption::Spear, x) => {
                effect.set_sprite(self.map.get_sprite("swipe_longsword.png".to_string()));
                effect.set_sound_str("Spear.ogg");
                match x {
                    LastKey::Up => {
                        effect.coord.force_move(0, -1);
                        effect.degree = 270
                    }
                    LastKey::Down => {
                        effect.coord.force_move(0, 1);
                        effect.degree = 90
                    }
                    LastKey::Left => {
                        effect.coord.force_move(-1, 0);
                        effect.mirror_h = true;
                    }
                    LastKey::Right => {
                        effect.coord.force_move(1, 0);
                    }
                    _ => {}
                }
            }
            // Same as Sword, only moving of Coordinates changes
            (EffectOption::Sword, x) => {
                effect.set_sprite(self.map.get_sprite("swipe_broadsword.png".to_string()));
                effect.set_sound_str("Sword.ogg");
                match x {
                    LastKey::Up => {
                        effect.coord.force_move(-1, -1);
                        effect.degree = 270
                    }
                    LastKey::Down => {
                        effect.coord.force_move(1, 1);
                        effect.degree = 90
                    }
                    LastKey::Left => {
                        effect.coord.force_move(-1, -1);
                        effect.mirror_h = true;
                    }
                    LastKey::Right => {
                        effect.coord.force_move(1, -1);
                    }
                    _ => {}
                }
            }
            (EffectOption::Chicken, x) => {
                effect.set_sprite(self.map.get_sprite("swipe_enemy.png".to_string()));
                effect.set_sound_str("Chicken.ogg");
                // Match direction
                match x {
                    // Moves coordinate up and rotate effect
                    LastKey::Up => {
                        effect.coord.force_move(0, -1);
                        effect.degree = 270
                    }
                    // Moves Coordinate down and rotate effect
                    LastKey::Down => {
                        effect.coord.force_move(0, 1);
                        effect.degree = 90
                    }
                    // Moves Coordinate left and mirror effect
                    LastKey::Left => {
                        effect.coord.force_move(-1, 0);
                        effect.mirror_h = true;
                    }
                    // Moves Coordinate right
                    LastKey::Right => {
                        effect.coord.force_move(1, 0);
                    }
                    _ => {}

                }
            }
        }
        // Resets time and push to active effects
        effect.reset_time();
        self.effects.push(effect);
    }

    /// For Updating the active effects
    pub fn on_update(&mut self) {
        // If effect lasts longer than 1 sec, effect was rendered and can be removed
        self.effects.retain(|ref i| i.start.to(PreciseTime::now()).num_milliseconds() <= 1000);
    }
}

impl<'a> Renderable for Effect<'a> {
    /// Redners Effect
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     self.start.to(PreciseTime::now()).num_milliseconds() as u64,
                     self.mirror_h,
                     self.degree);

        }
    }
}

/// Enum with all EffectTypes
#[derive(Copy, Clone, Debug)]
pub enum EffectOption {
    Dagger,
    Spear,
    Sword,
    Dead,
    Chicken,
}
