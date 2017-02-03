use actor::Actor;

pub trait Interactable {
    fn get_interactable_type(&self) -> InteractableType;
    fn conv_to_actor(&mut self) -> &mut Actor;
}

#[derive(Copy, Clone)]
pub enum InteractableType {
    Player(u64),
    Bot(u64),
    Useable(u64),
    Collectable(u64),
}
