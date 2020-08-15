// mod game {
//     pub mod game {
//         pub struct Game;
//     }
// }

mod game {
    pub mod game;
    pub mod player;
    pub mod board;
    pub mod r#move;
}

use game::game::Game;

fn main() {
    let v = vec![("Robert", false), ("Michel", true)];
    let g = Game::new(v, 19, 0);
    println!("{:?}", g);
}
