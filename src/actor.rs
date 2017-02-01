trait Actor {
    fn is_alive(&self) -> bool;
    fn attack(&self, &mut Actor);
    fn dead(&self);
}
