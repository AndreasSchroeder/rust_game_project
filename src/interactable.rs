pub trait Interactable {
    fn get_interactable_type(&self) -> InteractableType;
}

/// enum to differentiate between Player and Bots or other types which can be added
#[derive(Copy, Clone, Debug)]
pub enum InteractableType {
    Player(u64),
    Bot(u64),
}
