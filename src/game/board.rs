// TODO: use a bitboard if performance are needed

use super::r#move::Move;

#[derive(Debug)]
pub struct Board {
    _board: Vec<u8>,
    size: usize
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            _board: vec![0; size * size],
            size
        }
    }

    pub fn set(&mut self, m: &Move) {
        self._board[m.y as usize * self.size + m.x as usize] = m.player.id;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 { // Maybe return Result<>
        self._board[y * self.size + x]
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == 0
    }
}