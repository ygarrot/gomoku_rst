use super::r#move::Move;
use serde::Serialize;

// TODO: use a bitboard if performance are needed
#[derive(Clone, Debug, Serialize)]
pub struct Board {
    pub _board: Vec<u8>,
    pub size: usize,
    pub stone_captured: [u8; 2],

    pub depth: usize,
    pub player_turn: u8,
    pub global_turn:u32
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            _board: vec![0; size * size],
            stone_captured: [0,0],
            size,
            depth:0,
            player_turn:0,
            global_turn:0,
        }
    }

    pub fn serialize(&mut self, new_depth:usize, new_player_turn:u8, new_turn:u32)-> String
    {
        self.depth=new_depth;
        self.player_turn=new_player_turn;
        self.global_turn=new_turn;
        serde_json::to_string(&self).unwrap()
    }
    pub fn set(&mut self, m: &Move, val: u8) {
        self._board[m.y as usize * self.size + m.x as usize] = val;
    }

    pub fn set_fcoo(&mut self, x: usize, y: usize, val: u8) {
        self._board[y as usize * self.size + x as usize] = val;
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

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.get_fcoo(x, y) != 0
    }

    pub fn get_vec_bit(vx: f32, vy: f32) -> u8 {
        u8::pow(2, ((vy.atan2(vx).to_degrees() as i32).rem_euclid(180) / 45) as u32)   
    }

    pub fn display(&self)
    {
        println!("{:?}", self._board);
    }
    pub fn get_score(&self, player: u8) -> i64 {
        let mut score = 0;
        let mut checks: Vec<u8> = vec![Default::default(); self.size * self.size];

        for y in 0..self.size {
            for x in 0..self.size {
                if self.get_fcoo(x, y) == player {
                    // println!("Cell {} {} : {}", x, y, self.count_cell_score(x, y, player));
                    score += self.count_cell_score(x, y, player, &mut checks);
                }
            }
        }
        score
    }

    fn count_dir(&self, x: usize, y: usize, vx: i32, vy: i32, player: u8, checks: &mut Vec<u8>) -> (bool, i64) {
        if !self.is_in_bounds(x, y) {
            return (true, 0);
        }
        if self.get_fcoo(x, y) != player {
            return (self.is_occupied(x, y), 0);
        }
        checks[y * self.size + x] |= Board::get_vec_bit(vx as f32, vy as f32);
        let res = self.count_dir((x as i32 + vx) as usize, (y as i32 + vy) as usize, vx, vy, player, checks);
        return (res.0, 1 + res.1);
    }

    fn count_cell_score(&self, x: usize, y: usize, player: u8, checks: &mut Vec<u8>) -> i64 {
        let mut cell_score = 0;
        let mut dir_save = (false, 0);

        for vec in [(0, 1), (1, -1), (1, 1), (1, 0)].iter() {
            for dir in [-1, 1].iter() {
                let n_vec: (i32, i32) = (vec.0 * dir, vec.1 * dir);
                let checker = Board::get_vec_bit(n_vec.0 as f32, n_vec.1 as f32);
                if checks[y * self.size + x] & checker != checker {
                    let res = self.count_dir(
                        (x as i32 + n_vec.0) as usize,
                        (y as i32 + n_vec.1) as usize,
                        n_vec.0,
                        n_vec.1,
                        player,
                        checks,
                    );
                    if *dir == -1 {
                        dir_save = res;
                    } else if (!dir_save.0 || !res.0) && (res.1 + dir_save.1 != 0) {
                        cell_score += (1 + res.1 + dir_save.1).pow(5);
                    }
                }
            }
            checks[y * self.size + x] |= Board::get_vec_bit(vec.0 as f32, vec.1 as f32);
        }
        cell_score
    }
}
