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

pub struct Item<'a> {
    pub sprite: Option<&'a Sprite>,
    pub heal: u32,
    pub id: u64,
    pub coord: Coordinate,
    pub item_type: Option<ItemType>,
    pub gone: bool,
}

impl<'a> Item<'a> {
    pub fn new(x: u64, y: u64, i: u64) -> Self {
        Item {
            sprite: None,
            heal: 0,
            coord: Coordinate::new(x,y),
            item_type: None,
            id: i,
            gone: false,
        }
    }

    pub fn load_sprite(&mut self, map: &'a SpriteMap, sprite_str: String) {
        self.sprite = map.map.get(sprite_str.as_str());
        println!("{}", sprite_str);
        self.item_type = match sprite_str.as_str() {
            "weapon_dagger.png" => Some(ItemType::Weapon(EffectOption::Dagger)),
            "weapon_broadsword.png" => Some(ItemType::Weapon(EffectOption::Sword)), 
            "weapon_spear.png" => Some(ItemType::Weapon(EffectOption::Spear)),
            "Heart_10.png" => Some(ItemType::Heal(10)),
            "Heart_20.png" => Some(ItemType::Heal(20)),
            _ => None,
        }
    }

    pub fn collect(&mut self, player: &mut Player<'a>) {
        if player.coord.get_x() == self.coord.get_x() && player.coord.get_y() ==self.coord.get_y() && !self.get_gone(){
            if let Some(ref item) = self.item_type {
                match *item {
                    ItemType::Weapon(weapon) => player.weapon = weapon,
                    ItemType::Heal(heal) => player.damage_taken(heal as i32 * -1),
                }
            }
            self.gone();
        }    
    }

    pub fn get_gone(&self) -> bool {
        self.gone
    }

    pub fn gone(&mut self) {
        self.gone = true;
    }
}

pub enum ItemType {
    Weapon(EffectOption),
    Heal(u8),
}

impl<'a> Renderable for Item<'a> {
     fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        if let Some(ref x) = self.sprite {
            x.render(g,
                     view,
                     0,
                     false,
                     0);

        } else {
            //println!("None");
        }

     }
}

