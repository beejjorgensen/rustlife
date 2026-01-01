use rand::random;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifeCell {
    Dead,
    Alive,
}

pub struct Life {
    width: usize,
    height: usize,
    cells: [Vec<Vec<LifeCell>>; 2],
    cur_page: usize,
}

impl Life {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            cells: [Vec::new(), Vec::new()],
            cur_page: 0,
        }
    }

    pub fn init(&mut self, width: usize, height: usize) {
        self.cells[0].clear();
        self.cells[1].clear();
        self.width = width;
        self.height = height;
        self.cur_page = 0;

        for _ in 0..height {
            self.cells[0].push(vec![LifeCell::Dead; width]);
            self.cells[1].push(vec![LifeCell::Dead; width]);
        }
    }

    pub fn randomize(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let v: f32 = random();

                self.set_cell(
                    x,
                    y,
                    if v < 0.5 {
                        LifeCell::Alive
                    } else {
                        LifeCell::Dead
                    },
                );
            }
        }
    }

    /*
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    */

    pub fn get_cell(&self, x: usize, y: usize) -> LifeCell {
        if x < self.width && y < self.height {
            self.cells[self.cur_page][y][x]
        } else {
            LifeCell::Dead
        }
    }

    pub fn get_cell_weight(&self, x: usize, y: usize) -> i32 {
        if x < self.width && y < self.height {
            if self.cells[self.cur_page][y][x] == LifeCell::Alive {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: LifeCell) {
        if x < self.width && y < self.height {
            self.cells[self.cur_page][y][x] = state;
        }
    }

    /*
    pub fn get_row(&self, y: usize) -> Option<&Vec<LifeCell>> {
        if y < self.height {
            Some(&(self.cells[y]))
        } else {
            None
        }
    }
    */

    pub fn step(&mut self) {
        let other_page = if self.cur_page == 0 { 1 } else { 0 };

        for y in 0..self.height {
            for x in 0..self.width {
                let n = if y > 0 { y - 1 } else { self.height - 1 };
                let s = if y < self.height - 1 { y + 1 } else { 0 };
                let w = if x > 0 { x - 1 } else { self.width - 1 };
                let e = if x < self.width - 1 { x + 1 } else { 0 };

                let count = self.get_cell_weight(w, n)
                    + self.get_cell_weight(x, n)
                    + self.get_cell_weight(e, n)
                    + self.get_cell_weight(w, y)
                    + self.get_cell_weight(e, y)
                    + self.get_cell_weight(w, s)
                    + self.get_cell_weight(x, s)
                    + self.get_cell_weight(e, s);

                let alive = self.get_cell(x, y) == LifeCell::Alive;

                if alive {
                    self.cells[other_page][y][x] = if count == 2 || count == 3 {
                        LifeCell::Alive
                    } else {
                        LifeCell::Dead
                    };
                } else {
                    self.cells[other_page][y][x] = if count == 3 {
                        LifeCell::Alive
                    } else {
                        LifeCell::Dead
                    }
                }
            } // for x
        } // for y

        self.cur_page = other_page;
    }

    pub fn get_cells(&self) -> &Vec<Vec<LifeCell>> {
        &self.cells[self.cur_page]
    }
}
