use super::board::Board;
use super::r#move::Move;

#[derive(Clone, Debug)]
pub struct Node<'a> {
    pub r#move: Move,
    score: u8,
    pub turn: u8,
    parent: Option<&'a Node<'a>>,
    pub board: Board,
}

impl Node<'_> {
    pub fn max<'a>(a: &'a Node<'a>, b: &'a Node<'a>) -> &'a Node<'a> {
        if a.score > b.score {
            a
        } else {
            b
        }
    }

    pub fn min<'a>(a: &'a Node<'a>, b: &'a Node<'a>) -> &'a Node<'a> {
        if a.score < b.score {
            a
        } else {
            b
        }
    }

    pub fn new<'a>(nde: Option<&'a Node<'a>>, _turn: u8, mve: Move, b: Option<Board>) -> Node<'a> {
        Node {
            r#move: mve,
            score: 0,
            turn: _turn,
            parent: nde,
            board: match b {
                Some(b) => b,
                None => nde.unwrap().board.clone()
            },
        }
    }
    fn get_heuristic(&self) -> u64 {
        1
    }
    fn get_score(&self) -> u64 {
        1
    }
}
