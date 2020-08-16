use super::board::Board;
use super::r#move::Move;

pub struct Node<'a> {
    pub r#move: Move,
    score: u8,
    pub turn: u8,
    parent: &'a Node<'a>,
    pub board: Board,
}

impl Node<'_> {
    pub fn max<'a>(a: &'a Node, b: &'a Node) -> &'a Node<'a> {
        if a.score > b.score {
            b
        } else {
            a
        }
    }

    pub fn min<'a>(a: &'a Node, b: &'a Node) -> &'a Node<'a> {
        if a.score < b.score {
            a
        } else {
            b
        }
    }

    pub fn new<'a>(nde: &'a Node<'a>, board: Board, _turn: u8, mve: Move) -> Node<'a> {
        Node {
            r#move: mve,
            score: 0,
            turn: _turn,
            parent: nde,
            board: board,
        }
    }
    fn get_heuristic(&self) -> u64 {
        1
    }
    fn get_score(&self) -> u64 {
        1
    }
}
