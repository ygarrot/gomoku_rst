use super::game::{MoveError, Game};
use super::r#move::Move;
use super::board::Board;

fn gen_moves(board: &mut Board, game: &mut Game) -> Vec<Move> {
    let mut list = vec![];
    let size = board.size;
    for y in 0..size {
        for x in 0..size { 
            let mut _move = Move {
                x: x as u32,
                y: y as u32,
            };
            match game.check_restrictions(&_move, board) {
                Ok(_) => (),
                Err(e) => match e {
                    MoveError::MoveForbidden => continue,
                    _ => ()
                }
            }
            list.push(_move);
        }
    }
    list
}

pub fn minimax(
    mut board: &mut Board,
    target_player: u8,
    current_player: u8,
    depth: usize,
    mut alpha: i64,
    mut beta: i64,
    game: &mut Game,
    mve: Option<Move>
) -> (i64, Move) {
           use std::io::Write;

    writeln!(game.file, "{}", board.serialize());
    let move_save = match mve {
        Some(m) => {
            match game.r#move(&m, Some(&mut board), Some(current_player)) {
                Ok(_) => (),
                Err(e) => match e {
                    MoveError::GameEnded => return (board.get_score(current_player), m),
                    _ => ()
                }
            }
            if depth == 0 {
                return (board.get_score(current_player), m)
            }
            Some(m)
        } 
        None => None
    };

    let mut best_move = (std::i64::MAX, Move {x: 255, y: 255});
    if current_player == target_player {
        best_move = (std::i64::MIN, Move {x: 255, y: 255})
    }
    
    for m in gen_moves(&mut board, game) {
        let res = minimax(&mut board.clone(), target_player, if current_player == 1 {2} else {1}, depth - 1, alpha, beta, game, Some(m));
        if current_player == target_player {
            if best_move.0 < res.0 {
                best_move = res;
            }
            alpha = if alpha < res.0 {res.0} else {alpha};
        } else {
            if best_move.0 > res.0 {
                best_move = res;
            }
            beta = if beta > res.0 {res.0} else {beta};
        }
        if beta <= alpha {
            break;
        }
    }

    if let Some(m) = move_save {
        board.set(&m, 0); // this will be wrong for captures
    }
    best_move
}
