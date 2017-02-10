// Module for SoundHandler

use effect::EffectOption;
use io::sprite::Sprite;
use coord::Coordinate;
use renderable::Renderable;
use io::all_sprites::SpriteMap;
use player::Player;
use actor::Actor;

use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use time::PreciseTime;

/// Represents An Item
/// sprite: The Sprite of the Item
/// heal: > 0 if Item heals player
/// coord: Coordinate of ItemType
/// item_type: The Type of the ItemType
/// gone: If item is collected, it is gone and will be wasted
pub struct Item<'a> {
    pub sprite: Option<&'a Sprite>,
    pub heal: u32,
    pub coord: Coordinate,
    pub item_type: Option<ItemType>,
    pub gone: bool,
    pub dt: PreciseTime,
}


impl<'a> Item<'a> {
    /// Constructor
    pub fn new(x: u64, y: u64) -> Self {
        Item {
            sprite: None,
            heal: 0,
            coord: Coordinate::new(x, y),
            item_type: None,
            gone: false,
            dt: PreciseTime::now(),
        }
    }

    /// Loads the sprite by given string from SpriteMap
    pub fn load_sprite(&mut self, map: &'a SpriteMap, sprite_str: String) {
        self.sprite = map.map.get(sprite_str.as_str());
        println!("{}", sprite_str);
        self.item_type = match sprite_str.as_str() {
            //If Weapon
            "weapon_dagger.png" => Some(ItemType::Weapon(EffectOption::Dagger)),
            "weapon_broadsword.png" => Some(ItemType::Weapon(EffectOption::Sword)), 
            "weapon_spear.png" => Some(ItemType::Weapon(EffectOption::Spear)),
            // If Heal Item
            "Heart_10.png" => Some(ItemType::Heal(10)),
            "Heart_20.png" => Some(ItemType::Heal(20)),
            _ => None,
        }
    }

    /// collects an Item if Player is on same field.
    pub fn collect(&mut self, player: &mut Player<'a>) {
        if self.dt.to(PreciseTime::now()).num_milliseconds() > 1000 {
            self.dt = PreciseTime::now();
        }
        if player.coord.get_x() == self.coord.get_x() &&
           player.coord.get_y() == self.coord.get_y() && !self.get_gone() {
            if let Some(ref item) = self.item_type {
                match *item {
                    // Cahnge Weapon
                    ItemType::Weapon(weapon) => player.weapon = weapon,
                    // Heal
                    ItemType::Heal(heal) => player.damage_taken(heal as i32 * -1),
                }
            }
            self.gone();
        }
    }

    /// getter for gone
    pub fn get_gone(&self) -> bool {
        self.gone
    }

    /// Item will be gone
    pub fn gone(&mut self) {
        self.gone = true;
    }
}

/// Types of Items
pub enum ItemType {
    Weapon(EffectOption),
    Heal(u8),
}

impl<'a> Renderable for Item<'a> {
    /// Renders the Item
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     self.dt.to(PreciseTime::now()).num_milliseconds() as u64,
                     false,
                     0);
        } else {
            //println!("None");
        }

    }
}
