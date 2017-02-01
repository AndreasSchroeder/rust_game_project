use item::Item;

pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self{
        Inventory{
            items: Vec::new(),
        }
    }
}
