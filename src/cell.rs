

#[derive(Copy,Clone)]
pub struct Cell(u8);

impl Cell {
    pub fn selected(self) -> bool {
        self.0 < 0
    }
}

