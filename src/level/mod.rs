use super::field::Field;
use super::interactable::Interactable;

pub struct Level {
    data: Vec<Vec<Field>>,
    x: usize,
    y: usize,
}

impl Level {
    pub fn with_size(x: usize, y: usize) -> Self {
        Level {
            data: vec![vec![Field::new(0); x]; y],
            x: x,
            y: y,
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_data(&mut self) -> &mut Vec<Vec<Field>> {
        &mut self.data
    }

    pub fn get_field_at(&self, x: usize, y: usize) -> &Field {
        &self.data[x][y]
    }
}
