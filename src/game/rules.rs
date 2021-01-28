use super::board::Board;
use super::r#move::Move;
use core::fmt::Debug;

#[derive(PartialEq)]
pub enum RuleType {
    CONDITION,
    CONSEQUENCE,
    CAPTURE
}

pub trait Rule {
    fn valid(&self, b: &Board, m: &Move) -> bool;
    fn r#type(&self) -> RuleType;
    fn capture(&self, board: &mut Board, move_: &Move,id:u8) -> bool;
}

impl Debug for dyn Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rule")
    }
}

pub struct BaseRule {}
pub struct Capture {}

impl Rule for BaseRule {
    fn valid(&self, b: &Board, m: &Move) -> bool {
        !b.is_occupied(m.x as usize, m.y as usize)
    }

    fn r#type(&self) -> RuleType { RuleType::CONDITION }
    fn capture(&self, board: &mut Board, move_: &Move,id:u8) -> bool{true}
}

impl Rule for Capture{
    fn valid(&self, b: &Board, m: &Move) -> bool{true}
    fn capture(&self, board: &mut Board, move_: &Move,player_id:u8) -> bool {
        fn count_dir(player_x: i64, player_y: i64, increment_x: i64,
            increment_y: i64, id: u8, board: &mut Board) {
                let vals = if id == 1 {[2, 2, 1]} else {[1, 1, 2]};
                for i in 0..3
                {
                    let x = (player_x + increment_x * i) as usize;
                    let y = (player_y + increment_y * i) as usize;
            if (!board.is_in_bounds(x,y) ||
                 board.get_fcoo(x, y) != vals[i as usize])
            {
                    return ;
            }
        }
                            board.set_fcoo(player_x as usize, player_y as usize, 0);
                            board.set_fcoo((player_x + increment_x) as usize, (player_y + increment_y) as usize, 0);
                    }

        for vec in [(0, 1), (1, 1), (1, 0), (1, -1)].iter() {
            for dir in [-1, 1].iter() {
                let n_vec = (vec.0 * dir, vec.1 * dir);
                count_dir(
                    move_.x as i64 + n_vec.0,
                    move_.y as i64 + n_vec.1,
                    n_vec.0,
                    n_vec.1,
                    player_id,
                    board,
                );
            }
        }
        true
    }   
    fn r#type(&self) -> RuleType { RuleType::CAPTURE }
}
