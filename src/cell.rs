#[derive(Clone)]
pub struct Cell {
    alive: bool,
}
impl Cell {
    pub fn new(alive: bool) -> Self {
        Self { alive }
    }
    pub fn change_state(&mut self, state: bool) {
        self.alive = state;
    }
}
