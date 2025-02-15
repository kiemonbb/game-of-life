use core::time;
use std::thread;

use grid::Grid;
use renderer::render_grid;

pub mod cell;
pub mod grid;
pub mod renderer;

fn main() {
    let mut grid = Grid::new(15, 15);
    //simple game loop
    loop {
        if false {
            break;
        }
        render_grid(&grid);
        grid.set_new_cells();
        thread::sleep(time::Duration::from_millis(1000));
    }
}
