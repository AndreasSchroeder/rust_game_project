
use bot::Bot;
use level::Level;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn attack(&mut self, level: &mut Level, bots: &mut Vec<Option<Bot>>);
    fn dying(&self);
}
