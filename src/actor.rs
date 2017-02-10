use interactable::InteractableType;
use bot::Bot;
use player::LastKey;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn attack(&mut self,
              target: Vec<Option<InteractableType>>,
              bots: &mut Vec<Bot>,
              dir: LastKey);
    fn dying(&self);
}
