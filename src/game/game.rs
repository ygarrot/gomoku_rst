use super::player::Player;
use super::board::Board;
use super::r#move::Move;

pub enum MoveError {
    GameEnded,
    MoveForbidden
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    //rules: Vec<Rules>
    board: Board,
    player_turn: u8,
    global_turn: u32,
}

impl Game {
    pub fn new(_players: Vec<(&str, bool)>, board_size: usize, starter: u8) -> Game {
        let mut p_vec: Vec<Player> = vec![];
        
        for (i, p) in _players.iter().enumerate() {
            p_vec.push(Player{id: i as u8, name: p.0.to_string(), is_ai: p.1});
        }

        Game {
            players: p_vec,
            board: Board::new(board_size),
            player_turn: starter,
            global_turn: 1
        }
    }

    pub fn r#move(&mut self, move_: &Move, emulated: Option<&mut Board>) -> Result<&Game, MoveError> {
        // Check ruleset
        let mut board = match emulated {
            Some(b) => b,
            None => &mut self.board
        };

        board.set(move_);

        match Game::_apply_move_consequences(move_, board)? { //Maybe a vec of moves is needed
            Some(m) => board.set(m),
            None => ()
        }

        self.global_turn += 1;
        self.player_turn = (self.player_turn + 1) % self.players.len() as u8;

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