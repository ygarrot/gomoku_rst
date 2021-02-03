extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use std::fs::File;
use std::io::prelude::*;

#[path = "../game/"]
mod game {
    pub mod board;
    pub mod game;
    pub mod minimax;
    pub mod r#move;
    pub mod node;
    pub mod player;
    pub mod rules;
}

#[path = "../display/"]
mod display {
    pub mod game_information_view;
    pub mod gameboard_controller;
    pub mod gameboard_view;
}

use game::game::{Game};
use std::io::{self, BufReader};

pub use display::game_information_view::{GameInformationsView, GameInformationsViewSettings};
pub use display::gameboard_controller::GameboardController;
pub use display::gameboard_view::{GameboardView, GameboardViewSettings};

fn main() -> io::Result<()> {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("gomoku", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gameboard_view = GameboardView {
        gl: GlGraphics::new(opengl),
        settings: GameboardViewSettings::new(),
    };
    let mut game = Game::new(
        vec![("Player", false), ("AI", true)],
        9,
        1,
        vec!["Base", "Capture", "FreeThrees"],
    );
    let mut gameboard_controller = GameboardController::new();
    let mut events = Events::new(EventSettings::new());

    let file = File::open("foo.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            match gameboard_controller.click_on {
                Some(x) => match gameboard_view.get_cursor_indexes(game.board.size, &args, x) {
                    Some(_) => {
                        reader.read_line(&mut line)?;
                        println!("|{}|", line);
                        game.board._board = serde_json::from_str(&line).unwrap();
                        println!("deserialized = {:?}", game.board._board);
                        gameboard_controller.event(&e);
                        gameboard_view.render(&game.board, &args, gameboard_controller.cursor_pos);
                        gameboard_controller.click_on = None;
                        line.clear();
                    }
                    None => (),
                },
                None => (),
            };
        }
    }
    Ok(())
}