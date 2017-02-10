use level::Level;

pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn dying(&self);
    fn attack<T>(&mut self, level: &mut Level, enemy: &mut Vec<T>) where T: Actor;
}
