use super::board::Board;
use super::player::Player;
use super::r#move::Move;
use super::rules::{BaseRule, Rule, RuleType, Capture};

pub enum MoveError {
    GameEnded,
    MoveForbidden,
}

#[derive(Debug)]
pub struct Game {
    rules: Vec<Box<dyn Rule>>,
    pub players: Vec<Player>,
    pub board: Board,
    pub player_turn: u8,
    pub global_turn: u32,
}

impl Game {
    pub fn new(
        players: Vec<(&str, bool)>,
        board_size: usize,
        starter: u8,
        rules: Vec<&str>,
    ) -> Game {
        let mut p_vec: Vec<Player> = vec![];

        for (i, p) in players.iter().enumerate() {
            p_vec.push(Player {
                id: i as u8,
                name: p.0.to_string(),
                is_ai: p.1,
            });
        }

        let mut r_vec: Vec<Box<dyn Rule>> = vec![];
        for rulename in rules.iter() {
            r_vec.push(match rulename {
                &"Base" => Box::new(BaseRule {}),
                &"Capture" => Box::new(Capture {}),
                _ => Box::new(BaseRule {}),
            });
        }

        Game {
            rules: r_vec,
            players: p_vec,
            board: Board::new(board_size),
            player_turn: starter,
            global_turn: 0,
        }
    }

    pub fn r#move(
        &mut self,
        move_: &Move,
        emulated: Option<&mut Board>,
        val: Option<u8>
    ) -> Result<&Game, MoveError> {
        let emu = match emulated {
            Some(b) => (b, true),
            None => (&mut self.board, false),
        };
        let board = emu.0;
        let v = if emu.1 {val.unwrap()} else {self.player_turn};

        for rule in self.rules.iter() {
            if rule.r#type() == RuleType::CAPTURE && !rule.capture(board, move_) {
                return Err(MoveError::MoveForbidden);
            }
        }
        board.set(move_, v);

        match Game::apply_move_consequences(move_, board)? {
            //Maybe a vec of moves is needed
            Some(m) => board.set(m, v),
            None => (),
        }

        if !emu.1 {
            self.global_turn += 1;
            self.player_turn = self.global_turn as u8 % 2 + 1;
        }

        println!("player turn: {:?} id: {:?}", self.players[(self.player_turn - 1) as usize].name,self.players[(self.player_turn - 1) as usize].id);
        println!("Player 1 score {}", self.board.get_score(1));
        println!("Player 2 score {}", self.board.get_score(2));
        
        Ok(self)
    }

    fn apply_move_consequences<'a>(
        move_: &Move,
        board: &Board,
    ) -> Result<Option<&'a Move>, MoveError> {
        //check for captures, winning conditions...

        if Game::game_has_ended(move_, board) {
            return Err(MoveError::GameEnded);
        }

        Ok(None)
    }

    pub fn check_restrictions(&self, mve: &Move, board: &mut Board) -> Result<(), MoveError> {
        for rule in self.rules.iter() {
            if rule.r#type() == RuleType::CONDITION && !rule.valid(board, mve) {
                return Err(MoveError::MoveForbidden);
            }

            if rule.r#type() == RuleType::CONDITION && !rule.valid(board, mve) {
                return Err(MoveError::MoveForbidden);
            }
        }
        Ok(())
    }

    pub fn game_has_ended(move_: &Move, board: &Board) -> bool {
        fn count_dir(px: i64, py: i64, vx: i64, vy: i64, val: u8, b: &Board) -> u8 {
            return if b.is_in_bounds(px as usize, py as usize)
                && b.get_fcoo(px as usize, py as usize) == val
            {
                1 + count_dir(px + vx, py + vy, vx, vy, val, b)
            } else {
                0
            };
        }

        let p_id = board.get(move_);

        for vec in [(0, 1), (1, 1), (1, 0), (1, -1)].iter() {
            let mut count = 1;
            for dir in [-1, 1].iter() {
                let n_vec = (vec.0 * dir, vec.1 * dir);
                count += count_dir(
                    move_.x as i64 + n_vec.0,
                    move_.y as i64 + n_vec.1,
                    n_vec.0,
                    n_vec.1,
                    p_id,
                    board,
                );
                if count > 4 {
                    return true;
                }
            }
        }
        false
    }
}
