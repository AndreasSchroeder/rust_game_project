use level::Level;

/// trait for player and bots to interact with each other properly
pub trait Actor {
    fn is_alive(&self) -> bool;
    fn get_life(&self) -> i32;
    fn damage_taken(&mut self, dmg: i32);
    fn attack<T>(&mut self, level: &mut Level, enemy: &mut Vec<Option<T>>) where T: Actor;
}
