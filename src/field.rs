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
    // gets a certain fieldstatus
    pub fn get_fieldstatus(&self) -> Option<InteractableType> {
        self.contains
    }
    // sets a certain fieldstatus (f.e.: Player move on Field)
    pub fn set_fieldstatus(&mut self, o: InteractableType) {
        self.contains = Some(o);
    }
    // set the fieldstatus to None (f.e. when an enemy dies)
    pub fn free_fieldstatus(&mut self) {
        self.contains = None;
    }
    // checks the passability of a field
    pub fn check_passable(&self) -> bool {

        self.passable &&
        if let None = self.contains {
            true
        } else {
            false
        }
    }
    //gets a certain tiles id
    pub fn get_id(&self) -> u64 {
        self.tiles
    }
}

// define if a tile is passable or not (hard-code because of time constrains)
fn get_passable(id: u64) -> bool {
    match id {
        9 | 10 | 11 | 21 | 22 | 23 | 33 | 34 | 35 => false,

        _ => true,
    }
}
