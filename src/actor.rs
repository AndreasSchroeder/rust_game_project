pub trait Actor {
    pub fn is_alive(&self) -> bool;
    //pub fn attack(&self);
    pub fn dying(&self);
}
