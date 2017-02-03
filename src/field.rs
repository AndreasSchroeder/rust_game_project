use interactable::InteractableType;

#[derive(Clone)]
pub struct Field {
    tiles: u64, // Type of field
    passable: bool, // is field passable
    contains: Option<InteractableType>, // does field contains an object/enemy/player?
}

impl Field {
    pub fn new(typ: u64) -> Self {
        Field {
            tiles: typ,
            passable: get_passable(typ),
            contains: None,
        }
    }

    pub fn get_fieldstatus(&self) -> Option<InteractableType> {
        self.contains
    }

    pub fn set_fieldstatus(&mut self, o: InteractableType) {
        self.contains = Some(o);
    }

    pub fn free_fieldstatus(&mut self) {
        self.contains = None;
    }

    pub fn check_passable(&self) -> bool {

        self.passable &&
        if let None = self.contains {
            true
        } else {
            false
        }
    }

    pub fn get_id(&self) -> u64 {
        self.tiles
    }
}

// Hier wird bis jetzt statisch hinterlegt, ob ein Tile passable ist oder nicht
fn get_passable(id: u64) -> bool {
    match id {
        0 | 1 | 2 | 3 | 4 | 5 | 94 | 95 | 96 => true,
        6 => false,
        _ => false,
    }
}
