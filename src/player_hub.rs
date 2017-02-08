use effect::EffectOption;
use renderable::Renderable;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;
use io::all_sprites::SpriteMap;
use player::Player;

pub struct PlayerHub<'a> {
    name: String,
    life: u64,
    item: EffectOption,
    map: Option<&'a SpriteMap>,
}

impl<'a> PlayerHub<'a> {
    pub fn new(name: &str, map: Option<&'a SpriteMap>) -> Self{
        PlayerHub {
            name: name.to_string(),
            life: 100,
            item: EffectOption::Dagger,
            map: map,

        }
    }
    pub fn on_update(&mut self, player: &Player) {
        self.set_item(player.weapon);
        self.set_life(player.life as u64);

    }

    pub fn set_map(&mut self, map: &'a SpriteMap){
        self.map = Some(map)
    }

    pub fn set_item(&mut self, weapon: EffectOption) {
        self.item = weapon;
    }

    pub fn get_item(&self) -> &EffectOption {
        &self.item
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_life(&self) -> u64 {
        self.life
    }

    pub fn set_life (&mut self, life: u64) {
        self.life = life;
    }

    pub fn get_sprite_item(&self) -> &str {
        match self.item {
            EffectOption::Dagger => "weapon_dagger_small.png",
            EffectOption::Sword => "weapon_broadsword_small.png",
            EffectOption::Spear => "weapon_spear_small.png",
            _ => panic!(""),
        }
    }

    pub fn get_sprite_heart(&self) -> Vec<&str> {
        let mut left_life: i64 = self.life as i64;
        let mut vec: Vec<&str> = Vec::new();
        for _ in 0..5 {
            if left_life -20 >= 0 {
                vec.push("Heart_full.png");
            } else if left_life -10 >= 0 {
                vec.push("Heart_half.png");
            } else {
                vec.push("Heart_empty.png");
            }
            left_life -= 20;
        }
        vec
    }



}

impl<'a> Renderable for PlayerHub<'a> {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        
        if let Some(map) = self.map {
            let mut counter = 0;
            for (i, val) in self.get_sprite_heart().iter().enumerate() {
                let sprite = map.get_sprite(val.to_string());
                if let Some(x) = sprite {
                    x.render(g, view.trans(i as f64 * 33.0, 0.0), 0, false, 0);
                }
                counter = i;
            }
            let sprite_string = self.get_sprite_item().to_string();
            if let Some(x) = map.get_sprite(sprite_string) {
                x.render(g, view.trans((counter + 1) as f64 * 33.0, 0.0), 0, false, 0);
            }
        }
    }
}
