use super::field::Field;

pub struct Level {
    // 2D-Array to maintain objects of the level
    data: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Level {
    pub fn with_size(x: usize, y: usize) -> Self {
        Level {
            // Initialize 2D-Array with empty field-objects
            data: vec![vec![Field::new(0); y]; x],
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
        //returns reference of field-objects
        &mut self.data
    }

    pub fn get_field_at(&self, x: usize, y: usize) -> &Field {
        // returns reference of field-object at specific index
        &self.data[x][y]
    }
}

// Clone level to save resources
impl Clone for Level {
    fn clone(&self) -> Self {
        let mut vec = vec![vec![Field::new(0); self.height]; self.width];
        for i in 0..self.width {
            for j in 0..self.height {
                vec[i][j] = self.data[i][j].clone();
            }
        }
        Level {
            data: vec,
            width: self.width,
            height: self.height,
        }
    }
}
