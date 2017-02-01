use item::Item;

pub struct Inventory {
    pub items: Vec<Item>,
    pub limit: u32,
    pub slots_used: u32,
}

impl Inventory {
    pub fn new() -> Self{
        Inventory{
            items: Vec::new(),
            limit: 10,
            slots_used: 0,
        }
    }

    pub fn pick_up_item(&mut self, i: Item){
        let pos = self.items.iter().position(|ref x| x.id == i.id);
        match pos {
            Some(x) => self.items[x].amount += 1,
            None => {
                if self.slots_used < self.limit {
                    self.items.push(i);
                    self.slots_used += 1;
                } else {
                    println!("Inventory full!");
                }

            },
        }
    }

    pub fn use_item(&mut self, index: usize){
        self.items[index].amount -=1;
        if self.items[index].amount == 0 {
            self.items.remove(index);
        }
    }
}
