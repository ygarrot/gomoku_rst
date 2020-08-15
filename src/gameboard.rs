const SIZE: usize = 9;

pub struct Gameboard {
    pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }
}
