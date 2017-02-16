pub trait Interactable {
    fn get_interactable_type(&self) -> InteractableType;
}

#[derive(Copy, Clone, Debug)]
pub enum InteractableType {
    Player(u64),
    Bot(u64),
}
