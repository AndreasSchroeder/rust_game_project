pub struct Field {
    tiles: u64,          // Type of field 
    passable: bool,      // is field passable
    contains: Option<u64>, // does field contains an object/enemy/player?

}

impl Field {
    pub fn new(typ: u64, pass: bool) -> Self {
        Field {
                tiles: typ,
                passable: pass,
                contains: None,
        }
    }

    pub fn get_fieldstatus(&self) -> Option<u64> {
        self.contains
    }

    pub fn set_fieldstatus(&mut self, o: Option<u64>) {
        self.contains = o;
    }

    pub fn free_fieldstatus(&mut self) {
        self.contains = None;
    }
}