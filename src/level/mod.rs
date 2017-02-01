pub struct Level {
    data: Vec<Vec<u32>>,
    x: usize,
    y: usize,
}

impl Level {
    pub fn with_size(x: usize, y: usize) -> Self {
        Level {
            data: vec![vec![0; x]; y],
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

    pub fn get_data(&mut self) -> &mut Vec<Vec<u32>> {
        &mut self.data
    }
}
