use super::board::Board;
use super::game::Game;
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
    fn valid(&self, board: &mut Board, move_: &Move, player: &Player) -> bool;
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
    fn valid(&self, board: &mut Board, move_: &Move, player: &Player) -> bool {
        fn check_capture(
            player_x: i64,
            player_y: i64,
            increment_x: i64,
            increment_y: i64,
            id: u8,
            board: &mut Board,
        ) -> u8 {
            let vals = if id == 1 { [2, 2, 1] } else { [1, 1, 2] };
            for i in 0..3 {
                let x = (player_x + increment_x * i) as usize;
                let y = (player_y + increment_y * i) as usize;
                if !board.is_in_bounds(x, y) || board.get_fcoo(x, y) != vals[i as usize] {
                    return 1;
                }
            }
            board.stone_captured[id as usize - 1] += 2;
            board.set_fcoo(player_x as usize, player_y as usize, 0);
            board.set_fcoo(
                (player_x + increment_x) as usize,
                (player_y + increment_y) as usize,
                0,
            );
            1
        }
        Game::apply_to_near_edges(move_, board, player.id, check_capture, Game::void_after);
        true
    }
    fn r#type(&self) -> RuleType {
        RuleType::CAPTURE
    }
}

impl Rule for FreeThrees {
    fn valid(&self, board: &mut Board, move_: &Move, player: &Player) -> bool {
        fn check_free_threes(
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
        pub fn add_total_free_threes(
            _v: (i64, i64),
            res: u8,
            _i: &Move,
            _d: &mut Board,
            _a: u8,
            count: u8,
        ) -> u8 {
            count + res
        }
        let free_three_count = Game::apply_to_near_edges(
            move_,
            board,
            player.id,
            check_free_threes,
            add_total_free_threes,
        );
        return player.free_threes + free_three_count <= 1;
    }
    fn r#type(&self) -> RuleType {
        RuleType::FREE_THREES
    }
}
