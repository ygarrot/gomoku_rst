use super::board::Board;
use super::game::{Game, MoveError};
use super::r#move::Move;

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
                    _ => (),
                },
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
    mve: Option<Move>,
) -> (i64, Move) {
    println!("{:?}", depth);
    // use std::io::Write;
    // writeln!(
    //     game.file,
    //     "{}",
    //     board.serialize(depth, current_player, game.global_turn)
    // );

    if depth == 0 {
        return (board.get_score(current_player), mve.unwrap());
    }
    let move_save = match mve {
        Some(m) => {
            match game.r#move(&m, Some(&mut board), Some(current_player)) {
                Ok(_) => (),
                Err(e) => match e {
                    MoveError::GameEnded => return (board.get_score(current_player), m),
                    _ => (),
                },
            }
            if depth == 0 {
                return (board.get_score(current_player), m);
            }
            Some(m)
        }
        None => None,
    };
    let mut best_move = (std::i64::MAX, Move { x: 255, y: 255 });
    let maximizing_player = if current_player == target_player {
        true
    } else {
        false
    };
    if maximizing_player {
        best_move.0 = std::i64::MIN;
        for node in gen_moves(&mut board, game) {
            let res = minimax(
                &mut board.clone(),
                target_player,
                if current_player == 1 { 2 } else { 1 },
                depth - 1,
                alpha,
                beta,
                game,
                Some(node),
            );
            //max
            if best_move.0 < res.0 {
                best_move = res;
            }
        }
    } else {
        best_move.0 = std::i64::MAX;
        for node in gen_moves(&mut board, game) {
            let res = minimax(
                &mut board.clone(),
                target_player,
                if current_player == 1 { 2 } else { 1 },
                depth - 1,
                alpha,
                beta,
                game,
                Some(node),
            );
            //min
            if best_move.0 > res.0 {
                best_move = res;
            }
        }
    }

    if let Some(m) = move_save {
        board.set(&m, 0); // this will be wrong for captures
    }
    best_move
}
