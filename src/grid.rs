use crate::cell::Cell;
use rand::prelude::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}
impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::rng();
        let cells = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| Cell::new(rng.random_bool(0.3)))
                    .collect()
            })
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }
    pub fn set_new_cells(&mut self) {
        let mut new_cells: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.height {
            for column in 0..self.width {

                let up = (row + self.height - 1) % self.height;
                let down = (row + 1) % self.height;
                let left = (column + self.width - 1) % self.width;
                let right = (column + 1) % self.width;

                let neighbours: i32 = self.cells[up][left].get_state() as i32
                    + self.cells[up][column].get_state() as i32
                    + self.cells[up][right].get_state() as i32
                    + self.cells[row][left].get_state() as i32
                    + self.cells[row][right].get_state() as i32
                    + self.cells[down][left].get_state()
                        as i32
                    + self.cells[down][column].get_state() as i32
                    + self.cells[down][right].get_state() as i32;

                let is_alive = self.cells[row][column].get_state();
                if is_alive && (neighbours < 2 || neighbours > 3) {
                    new_cells.push((row, column));
                } else if !is_alive && neighbours == 3 {
                    new_cells.push((row, column));
                }
            }
        }
        new_cells
            .into_iter()
            .for_each(|(row, column)| self.cells[row][column].switch_state());
    }
}
