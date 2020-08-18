use super::game::Game;
use super::node::Node;
use super::r#move::Move;

pub struct Minimax {
    game: Game,
}

impl Minimax {
    fn create_childrens<'a>(&mut self, node: &'a Node<'a>) -> Vec<Node<'a>> {
        let mut list = vec![];
        let size = node.board.size;
        for y in 0..size {
            for x in 0..size {
                let mut _move = Move {
                    x: x as u32,
                    y: y as u32,
                };
                // if not self.game.checkRestrictionMove(move, self.game):
                //     continue
                let mut newNode = Node::new(node, node.turn, _move);
                self.game.r#move(&mut _move, Some(&mut newNode.board));
                list.push(newNode);
            }
        }
        list
    }

    pub fn minimax<'a>(
        &mut self,
        first_node: &'a Node<'a>,
        depth: usize,
        maximize: bool,
        alpha: u8,
        beta: u8,
    ) -> Node<'a> {
        // type CmpFn = fn(&'a Node, &'a Node) -> &'a Node<'a>;
        // let mut cmp_fn = Node::min;
        // if maximize {
        //     cmp_fn = Node::max;
        // }
        let mut best_node = first_node;
        for node in self.create_childrens(first_node) {
            let new_node = self.minimax(&node, depth - 1, !maximize, alpha, beta);
            if maximize {
                best_node = Node::max(best_node, &new_node);
            }
            // } else {
            //     best_node = Node::min(best_node, new_node);
            // }
        }
        best_node
    }
}
