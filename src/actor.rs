use interactable::InteractableType;
use bot::Bot;
use player::LastKey;
use level::Level;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn attack(&mut self, level: &mut Level, bots: &mut Vec<Bot>);
    fn dying(&self);
}
