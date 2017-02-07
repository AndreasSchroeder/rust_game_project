// Module for reading sprides and saving them in a HashMap

use std::collections::HashMap;
use piston_window::*;
use io::sprite::Sprite;

/// HashMap for all given sprites
pub struct SpriteMap {
    pub map: HashMap<&'static str, Sprite>,
}

impl SpriteMap {
    /// Initialize the SpriteMap with given window, fills Map with all Sprites
    pub fn init(mut w: &mut PistonWindow) -> Self {
        let mut map2: HashMap<&'static str, Sprite> = HashMap::new();
        for i in SPRITES {
            map2.insert(i.image,
                        Sprite::fill_sprite(i.image, i.size_x as u32, i.size_y as u32, w));
        }

        SpriteMap { map: map2 }

    }

    /// creates empty SpriteMap
    pub fn new() -> Self {
        SpriteMap { map: HashMap::new() }
    }

    /// returns a Sprite with given Name
    pub fn get_sprite(&self, file: &'static str) -> Option<&Sprite> {
        self.map.get(file)
    }
}

/// Struct for representing Necessary information about sprites
struct SpriteModel {
    image: &'static str,
    size_x: usize,
    size_y: usize,
}

/// const with all Sprites
const SPRITES: &'static [SpriteModel] = &[SpriteModel {
                                              image: "bowman.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "chicken.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "chicken_brown.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "chicken_pink.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "chicken_white.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "explosion.png",
                                              size_x: 59,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "knight.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "paladin.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "swipe_broadsword.png",
                                              size_x: 64,
                                              size_y: 192,
                                          },
                                          SpriteModel {
                                              image: "swipe_dagger.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "swipe_longsword.png",
                                              size_x: 128,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "weapon_broadsword.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "weapon_dagger.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "weapon_spear.png",
                                              size_x: 64,
                                              size_y: 64,
                                          },
                                          SpriteModel {
                                              image: "wizard.png",
                                              size_x: 64,
                                              size_y: 64,
                                          }];
