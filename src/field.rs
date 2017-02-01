use interactable::Interactable;

pub struct Field<'a, T: 'a> {
    tiles: u64, // Type of field
    passable: bool, // is field passable
    contains: Option<&'a T>, // does field contains an object/enemy/player?
}

impl<'a, T> Field<'a, T> {
    pub fn new(typ: u64, pass: bool) -> Self {
        Field {
            tiles: typ,
            passable: pass,
            contains: None,
        }
    }

    pub fn get_fieldstatus(&self) -> Option<&'a T>
        where T: Interactable
    {
        self.contains
    }

    pub fn set_fieldstatus(&mut self, o: Option<&'a T>) {
        self.contains = o;
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
}
