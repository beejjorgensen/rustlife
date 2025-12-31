#[derive(Debug, Clone, Copy)]
enum LifeCell {
    Dead,
    Alive,
}

struct Life {
    cells: Vec<Vec<LifeCell>>,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Vec<LifeCell>> = Vec::new();

        for row_index in 0..height {
            cells.push(vec![LifeCell::Dead; width]);
        }

        Self {
            cells
        }
    }

    pub fn get_width(&self) -> usize {
        self.cells.get(0).len()
    }

    pub fn get_height(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell(&self, x: usize, y: usize) -> LifeCell {
        if x < self.get_width() && y < self.heigh_height() {
            self.cells[y][x]
        } else {
            LifeCell::Dead
        }
    }

    pub fn set_cell(&self, x: usize, y: usize, state: LifeCell) {
        if x < self.get_width() && y < self.heigh_height() {
            self.cells[y][x] = state;
        }
    }

    pub fn get_row(&self, y: usize) -> Option<&Vec<LifeCell>> {
        if y < self.heigh_height() {
            Some(&(self.cells[y]))
        } else {
            None
        }
    }
}
