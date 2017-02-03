use super::field::Field;

pub struct Level {
    data: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Level {
    pub fn with_size(x: usize, y: usize) -> Self {
        Level {
            data: vec![vec![Field::new(0); x]; y],
            width: x,
            height: y,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_data(&mut self) -> &mut Vec<Vec<Field>> {
        &mut self.data
    }

    pub fn get_field_at(&self, x: usize, y: usize) -> &Field {
        &self.data[x][y]
    }
}
