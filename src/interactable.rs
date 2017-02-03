use enums::InteractableType;
use actor::Actor;

pub trait Interactable {
    fn get_interactable_type(&self) -> InteractableType;
    fn conv_to_actor(&mut self) -> &mut Actor;
}
