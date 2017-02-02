use super::field::Field;
use super::interactable::Interactable;

pub struct Level<'a> {
    data: Vec<Vec<Field<'a, Option<&'a Interactable>>>>,
    x: usize,
    y: usize,
}

impl<'a> Level<'a> {
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

    pub fn get_data(&mut self) -> &mut Vec<Vec<Field<'a, Option<&'a Interactable>>>> {
        &mut self.data
    }
}
