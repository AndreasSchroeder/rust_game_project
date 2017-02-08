use player::Weapon;
use renderable::Renderable;
use gfx_device_gl::Resources;
use gfx_device_gl::CommandBuffer;
use gfx_graphics::GfxGraphics;
use piston_window::*;

struct PlayerHub {
    name: String,
    life: u64,
    item: Weapon,
}

impl PlayerHub {
    pub fn new(name: &str) -> Self{
        PlayerHub {
            name: name.to_string(),
            life: 100,
            item: Weapon::Sword,
        }
    }
    pub fn set_item(&mut self, weapon: Weapon) {
        self.item = weapon;
    }

    pub fn get_item(&self) -> &Weapon {
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
            Weapon::Sword => "weapon_dagger_small.png",
            Weapon::Broadsword => "weapon_broadsword_small.png",
            Weapon::Spear => "weapon_spear_small.png",
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

impl Renderable for PlayerHub {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        //println!("player{:?}", match self.sprite {None => "None", _=> "Some"});
       

    }
}
