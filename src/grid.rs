use std::u8;

use crate::cell::Cell;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
   pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::new(false); width*height];
        Self {width,height,cells}
   }
}
