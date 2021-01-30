use super::board::Board;
use super::player::Player;
use super::r#move::Move;
use core::fmt::Debug;

#[derive(PartialEq)]
pub enum RuleType {
    CONDITION,
    CONSEQUENCE,
    CAPTURE,
    FREE_THREES,
}

pub trait Rule {
    fn valid(&self, board: &mut Board, move_: &Move, player: &Player) -> bool ;
    fn r#type(&self) -> RuleType;
}

impl Debug for dyn Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rule")
    }
}

pub struct BaseRule {}
pub struct Capture {}
pub struct FreeThrees {}

impl Rule for BaseRule {
    fn valid(&self, board: &mut Board, move_: &Move, _player: &Player) -> bool {
        !board.is_occupied(move_.x as usize, move_.y as usize)
    }
    fn r#type(&self) -> RuleType {
        RuleType::CONDITION
    }
}

impl Rule for Capture {
    fn valid(&self, board: &mut Board, move_: &Move, player: & Player) -> bool {
        fn count_dir(
            player_x: i64,
            player_y: i64,
            increment_x: i64,
            increment_y: i64,
            id: u8,
            board: &mut Board,
        ) {
            let vals = if id == 1 { [2, 2, 1] } else { [1, 1, 2] };
            for i in 0..3 {
                let x = (player_x + increment_x * i) as usize;
                let y = (player_y + increment_y * i) as usize;
                if !board.is_in_bounds(x, y) || board.get_fcoo(x, y) != vals[i as usize] {
                    return;
                }
            }
            board.stone_captured[id as usize - 1] += 2;
            board.set_fcoo(player_x as usize, player_y as usize, 0);
            board.set_fcoo(
                (player_x + increment_x) as usize,
                (player_y + increment_y) as usize,
                0,
            );
        }

        for vec in [(0, 1), (1, 1), (1, 0), (1, -1)].iter() {
            for dir in [-1, 1].iter() {
                let n_vec = (vec.0 * dir, vec.1 * dir);
                count_dir(
                    move_.x as i64 + n_vec.0,
                    move_.y as i64 + n_vec.1,
                    n_vec.0,
                    n_vec.1,
                    player.id,
                    board,
                );
            }
        }
        true
    }
    fn r#type(&self) -> RuleType {
        RuleType::CAPTURE
    }
}

impl Rule for FreeThrees {
    fn valid(&self, board: &mut Board, move_: &Move, player: &Player) -> bool {
        fn count_dir(
            player_x: i64,
            player_y: i64,
            increment_x: i64,
            increment_y: i64,
            id: u8,
            board: &mut Board,
        ) -> u8 {
            let x_extremity = (player_x - increment_x * 2) as usize;
            let y_extremity = (player_y - increment_y * 2) as usize;
            if !board.is_in_bounds(x_extremity, y_extremity)
                || board.get_fcoo(x_extremity, y_extremity) != 0
            {
                return 0;
            }
            if !board.is_in_bounds(player_x as usize, player_y as usize) {
                return 0;
            }
            let vals = if board.get_fcoo(player_x as usize, player_y as usize) == id {
                [id, id, 0, 42]
            } else {
                [0, id, id, 0]
            };
            for i in 0..vals.len() as i64 {
                let x = (player_x + increment_x * i) as usize;
                let y = (player_y + increment_y * i) as usize;
                if vals[i as usize] == 42 {
                    continue;
                }
                if !board.is_in_bounds(x, y) || board.get_fcoo(x, y) != vals[i as usize] {
                    return 0;
                }
            }
            return 1;
        }

        let mut count = 0;
        for vec in [(0, 1), (1, 1), (1, 0), (1, -1)].iter() {
            for dir in [-1, 1].iter() {
                let n_vec = (vec.0 * dir, vec.1 * dir);
                count += count_dir(
                    move_.x as i64 + n_vec.0,
                    move_.y as i64 + n_vec.1,
                    n_vec.0,
                    n_vec.1,
                    player.id,
                    board,
                );
            }
        }
        return player.free_threes + count <= 1;
    }
    fn r#type(&self) -> RuleType {
        RuleType::FREE_THREES
    }
}
