#[derive(Debug, Clone, Copy)]
pub enum LifeCell {
    Dead,
    Alive,
}

pub struct Life {
    width: usize,
    height: usize,
    cells: Vec<Vec<LifeCell>>,
}

impl Life {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }

    pub fn init(&mut self, width: usize, height: usize) {
        self.cells.clear();
        self.width = width;
        self.height = height;

        for _ in 0..height {
            self.cells.push(vec![LifeCell::Dead; width]);
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_cell(&self, x: usize, y: usize) -> LifeCell {
        if x < self.width && y < self.height {
            self.cells[y][x]
        } else {
            LifeCell::Dead
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: LifeCell) {
        if x < self.width && y < self.height {
            self.cells[y][x] = state;
        }
    }

    pub fn get_row(&self, y: usize) -> Option<&Vec<LifeCell>> {
        if y < self.height {
            Some(&(self.cells[y]))
        } else {
            None
        }
    }
}
