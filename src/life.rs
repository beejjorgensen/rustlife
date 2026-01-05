//! Conway's Life
//!
//! This Module implements the state management for Conway's Life. The implementation is a naive
//! grid.
use rand::random;

/// Whether a cell is alive or dead.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifeCell {
    Dead,
    Alive,
}

/// Represents a Life grid.
pub struct Life {
    width: usize,
    height: usize,
    cells: [Vec<Vec<LifeCell>>; 2],
    cur_page: usize,
}

impl Life {
    /// Create a new Life object.
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            cells: [Vec::new(), Vec::new()],
            cur_page: 0,
        }
    }

    /// Initialize a Life object to a given width and height.
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

    /// Fill the grid with random values.
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

    /// Get the status of a cell.
    pub fn get_cell(&self, x: usize, y: usize) -> LifeCell {
        if x < self.width && y < self.height {
            self.cells[self.cur_page][y][x]
        } else {
            LifeCell::Dead
        }
    }

    /// Get the weight of a cell, `1` for alive, `0` for dead.
    fn get_cell_weight(&self, x: usize, y: usize) -> i32 {
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

    /// Set the status of a cell.
    pub fn set_cell(&mut self, x: usize, y: usize, state: LifeCell) {
        if x < self.width && y < self.height {
            self.cells[self.cur_page][y][x] = state;
        }
    }

    /// Toggle the status of a cell.
    pub fn toggle(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            match self.get_cell(x, y) {
                LifeCell::Alive => self.set_cell(x, y, LifeCell::Dead),
                LifeCell::Dead => self.set_cell(x, y, LifeCell::Alive),
            }
        }
    }

    /// Clear the life grid to Dead.
    pub fn clear(&mut self) {
        for row in &mut self.cells[self.cur_page] {
            row.fill(LifeCell::Dead);
        }
    }

    /*
    /// Get a row of the life grid.
    pub fn get_row(&self, y: usize) -> Option<&Vec<LifeCell>> {
        if y < self.height {
            Some(&(self.cells[y]))
        } else {
            None
        }
    }
    */

    /// Single step the life simuation.
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

    /// Resize the life grid.
    ///
    /// When shrinking, it truncates the values. When growing, it fills with dead cells.
    pub fn resize(&mut self, width: usize, height: usize) {
        for page in 0..=1 {
            self.cells[page].resize(height, vec![LifeCell::Dead; width]);

            for i in 0..height {
                self.cells[page][i].resize(width, LifeCell::Dead);
            }
        }

        self.width = width;
        self.height = height;
    }

    /// Get a reference to all the cells.
    pub fn get_cells(&self) -> &Vec<Vec<LifeCell>> {
        &self.cells[self.cur_page]
    }
}

impl Default for Life {
    fn default() -> Self {
        Self::new()
    }
}
