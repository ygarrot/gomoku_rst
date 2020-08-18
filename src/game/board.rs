// TODO: use a bitboard if performance are needed

use super::r#move::Move;

#[derive(Clone, Debug)]
pub struct Board {
    _board: Vec<u8>,
    pub size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            _board: vec![0; size * size],
            size,
        }
    }

    pub fn set(&mut self, m: &Move, val: u8) {
        self._board[m.y as usize * self.size + m.x as usize] = val;
    }

    pub fn get(&self, m: &Move) -> u8 {
        self._board[m.y as usize * self.size + m.x as usize]
    }

    pub fn get_fcoo(&self, x: usize, y: usize) -> u8 {
        self._board[y * self.size + x]
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    pub fn is_occupied(&self, m: &Move) -> bool {
        self.get(m) != 0
    }
}
