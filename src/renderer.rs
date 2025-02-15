use crate::grid::Grid;
fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
}
pub fn render_grid(grid: &Grid) {
    for cell_row in grid.cells.iter() {
        for cell in cell_row {
            if cell.get_state() {
                print!("O");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    clear_terminal();
}
