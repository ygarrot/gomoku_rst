use super::board::Board;
use super::player::Player;
use super::r#move::Move;
use super::rules::{BaseRule, Capture, FreeThrees, Rule, RuleType};

pub enum MoveError {
    GameEnded,
    MoveForbidden,
}
static MAX_STONE_CAPTURED: u8 = 10;
static MAX_STONE_ALIGNED: u8 = 5;
static BS_TRUE: u8 = 1;
static BS_FALSE: u8 = 0;

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
                id: (i + 1) as u8,
                name: p.0.to_string(),
                is_ai: p.1,
                free_threes: 0,
            });
        }

        let mut r_vec: Vec<Box<dyn Rule>> = vec![];
        for rulename in rules.iter() {
            r_vec.push(match rulename {
                &"Base" => Box::new(BaseRule {}),
                &"Capture" => Box::new(Capture {}),
                &"FreeThrees" => Box::new(FreeThrees {}),
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
        val: Option<u8>,
    ) -> Result<&Game, MoveError> {
        let emu = match emulated {
            Some(b) => (b, true),
            None => (&mut self.board, false),
        };
        let board = emu.0;
        let v = if emu.1 {
            val.unwrap()
        } else {
            self.player_turn
        };
        let player = &self.players[(self.player_turn - 1) as usize];
        for rule in self.rules.iter() {
            if rule.r#type() == RuleType::FREE_THREES && !rule.valid(board, move_, &player) {
                return Err(MoveError::MoveForbidden);
            }
            if rule.r#type() == RuleType::CAPTURE && !rule.valid(board, move_, &player) {
                return Err(MoveError::MoveForbidden);
            }
            if board.stone_captured[player.id as usize - 1] >= MAX_STONE_CAPTURED {
                return Err(MoveError::GameEnded);
            }
            if rule.r#type() == RuleType::CONDITION && !rule.valid(board, move_, &player) {
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

        println!(
            "player turn: {:?} id: {:?}",
            self.players[(self.player_turn - 1) as usize].name,
            self.players[(self.player_turn - 1) as usize].id
        );
        println!("Player 1 score {}", self.board.get_score(1));
        println!("Player 2 score {}", self.board.get_score(2));
        Ok(self)
    }

    fn apply_move_consequences<'a>(
        move_: &Move,
        board: &mut Board,
    ) -> Result<Option<&'a Move>, MoveError> {
        //check for captures, winning conditions...

        if Game::game_has_ended(move_, board) {
            return Err(MoveError::GameEnded);
        }

        Ok(None)
    }

    pub fn check_restrictions(&self, move_: &Move, board: &mut Board) -> Result<(), MoveError> {
        let player = &self.players[(self.player_turn - 1) as usize];
        for rule in self.rules.iter() {
            if rule.r#type() == RuleType::CONDITION && !rule.valid(board, move_, &player) {
                return Err(MoveError::MoveForbidden);
            }
        }
        Ok(())
    }

    pub fn apply_to_choosen_opposite(
        vec: (i64, i64),
        res: u8,
        move_: &Move,
        board: &mut Board,
        p_id: u8,
        count: u8,
        f: fn(i64, i64, i64, i64, u8, &mut Board) -> u8,
    ) -> u8 {
        let mut tmp = res;
        for dir in [-1, 1].iter() {
            let n_vec = (vec.0 * dir, vec.1 * dir);
            tmp += f(
                move_.x as i64 + n_vec.0,
                move_.y as i64 + n_vec.1,
                n_vec.0,
                n_vec.1,
                p_id,
                board,
            );
        }
        tmp
    }
    pub fn void_after(_v: (i64, i64), res: u8, _i: &Move, _d: &mut Board, _a: u8, count: u8) -> u8 {
        count | res
    }
    pub fn apply_to_near_edges(
        move_: &Move,
        board: &mut Board,
        p_id: u8,
        count: u8,
        f: fn(i64, i64, i64, i64, u8, &mut Board) -> u8,
        f_after: fn((i64, i64), u8, &Move, &mut Board, u8, u8) -> u8,
    ) -> u8 {
        let mut result = 0;
        let mut res = 0;
        for vec in [(0, 1), (1, 1), (1, 0), (1, -1)].iter() {
            let res = Game::apply_to_choosen_opposite(*vec, res, move_, board, p_id, count, f);
            result = f_after(*vec, res, move_, board, p_id, result);
        }
        result
    }

    pub fn game_has_ended(move_: &Move, board: &mut Board) -> bool {
        fn check_capture_on_line(
            player_x: i64,
            player_y: i64,
            increment_x: i64,
            increment_y: i64,
            id: u8,
            board: &mut Board,
        ) -> u8 {
            let opposite_id = if id == 1 { 2 } else { 1 };
            let vals = [(-2, 0), (0, id), (1, opposite_id)];
            for (i, val) in vals.iter() {
                let x = (player_x + increment_x * i) as usize;
                let y = (player_y + increment_y * i) as usize;
                if !board.is_in_bounds(x, y) || board.get_fcoo(x, y) != *val {
                    return BS_FALSE;
                }
            }
            return BS_TRUE;
        }

        fn check_ennemy_possible_capture(
            px: i64,
            py: i64,
            vx: i64,
            vy: i64,
            p_id: u8,
            board: &mut Board,
        ) -> u8 {
            if 1 == Game::apply_to_near_edges(
                &Move {
                    x: px as u32,
                    y: py as u32,
                },
                board,
                p_id,
                0,
                check_capture_on_line,
                Game::void_after,
            ) {
                return BS_TRUE;
            }

            return if board.is_in_bounds(px as usize, py as usize)
                && board.get_fcoo(px as usize, py as usize) == p_id
            {
                check_ennemy_possible_capture(px + vx, py + vy, vx, vy, p_id, board)
            } else {
                BS_FALSE
            };
        }

        fn check_count(
            vec: (i64, i64),
            res: u8,
            move_: &Move,
            board: &mut Board,
            p_id: u8,
            count: u8,
        ) -> u8 {
            if count == BS_TRUE {
                return BS_TRUE;
            }
            if res >= MAX_STONE_ALIGNED {
                if 1 != Game::apply_to_choosen_opposite(
                    vec,
                    res,
                    move_,
                    board,
                    p_id,
                    count,
                    check_ennemy_possible_capture,
                ) {
                    return BS_TRUE;
                }
            }
            BS_FALSE
        }
        fn count_dir(px: i64, py: i64, vx: i64, vy: i64, val: u8, b: &mut Board) -> u8 {
            return if b.is_in_bounds(px as usize, py as usize)
                && b.get_fcoo(px as usize, py as usize) == val
            {
                1 + count_dir(px + vx, py + vy, vx, vy, val, b)
            } else {
                0
            };
        }

        let p_id = board.get(move_);

        return 1 == Game::apply_to_near_edges(move_, board, p_id, 0, count_dir, check_count);
    }
}
