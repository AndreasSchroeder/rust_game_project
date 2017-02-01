use interactable::Interactable;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn attack(&self, target: Vec<Option<&mut Interactable>>);
    fn dying(&self);
}
