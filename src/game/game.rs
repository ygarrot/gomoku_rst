use super::player::Player;
use super::board::Board;
use super::r#move::Move;
use super::rules::{Rule, BaseRule, RuleType};

pub enum MoveError {
    GameEnded,
    MoveForbidden
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
    pub fn new(players: Vec<(&str, bool)>, board_size: usize, starter: u8, rules: Vec<&str>) -> Game {
        let mut p_vec: Vec<Player> = vec![];
        
        for (i, p) in players.iter().enumerate() {
            p_vec.push(Player{id: i as u8, name: p.0.to_string(), is_ai: p.1});
        }

        let mut r_vec: Vec<Box<dyn Rule>> = vec![];
        for rulename in rules.iter() {
            r_vec.push(
                match rulename {
                    &"Base" => Box::new(BaseRule{}),
                    _ => Box::new(BaseRule{})
                }
            );
        }

        Game {
            rules: r_vec,
            players: p_vec,
            board: Board::new(board_size),
            player_turn: starter,
            global_turn: 1
        }
    }

    pub fn r#move(&mut self, move_: &Move, emulated: Option<&mut Board>) -> Result<&Game, MoveError> {
        let board = match emulated {
            Some(b) => b,
            None => &mut self.board
        };

        for rule in self.rules.iter() {
            if rule.r#type() == RuleType::CONDITION && !rule.valid(board, move_) {
                return Err(MoveError::MoveForbidden);
            }
        }

        board.set(move_, self.player_turn);

        match Game::_apply_move_consequences(move_, board)? { //Maybe a vec of moves is needed
            Some(m) => board.set(m, self.player_turn),
            None => ()
        }

        self.global_turn += 1;
        self.player_turn = self.global_turn as u8 % 2 + 1;

        Ok(self)
    }

    fn _apply_move_consequences<'a>(move_: &Move, board: &Board) -> Result<Option<&'a Move>, MoveError> {
        //check for captures, winning conditions...

        //if game_has_ended {
        // Err(MoveError::GameEnded)
        //}

        Ok(None)
    }
}