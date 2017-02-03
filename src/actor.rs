use interactable::Interactable;
use creature::Creature;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn get_creature(&mut self) -> &mut Creature;
    fn damage_taken(&mut self, dmg: i32);
    fn attack(&self, target: Vec<Option<&mut Interactable>>);
    fn dying(&self);
}
