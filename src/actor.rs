use interactable::InteractableType;
use bot::Bot;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn attack(&self, target: Vec<Option<InteractableType>>, bots: &mut Vec<Bot>);
    fn dying(&self);
}
