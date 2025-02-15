#[derive(Clone)]
pub struct Cell {
    alive: bool,
}
impl Cell {
    pub fn new(alive: bool) -> Self {
        Self { alive }
    }
    pub fn switch_state(&mut self) {
        self.alive = !self.alive;
    }
    pub fn get_state(&self) -> bool {
        self.alive
    }
    pub fn change_state(&mut self, state: bool) {
        self.alive = state;
    }
}
