extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;


mod game {
    pub mod game;
    pub mod player;
    pub mod board;
    pub mod r#move;
}
mod gameboard;
mod gameboard_controller;
mod gameboard_view;

use game::game::Game;
use game::r#move::Move;

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("gomoku", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gameboard_view = GameboardView {
        gl: GlGraphics::new(opengl),
        settings: GameboardViewSettings::new(),
    };

    let mut game = Game::new(vec![("Robert", false), ("Michel", true)], 9, 0);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            match gameboard_controller.click_on {
                Some(x) => {
                    match gameboard_view.get_cursor_indexes(game.board.size, &args, x) {
                        Some(coo) => match game.r#move(&Move{x: coo[0], y: coo[1]}, None) {
                            Ok(_) => (),
                            Err(e) => panic!(e)
                        },
                        None => ()
                    }
                }
                None => (),
            };
            gameboard_view.render(&game.board, &args);
            gameboard_controller.click_on = None;
        }

        // if let Some(args) = e.update_args() {
        //     gameboard_view.update(&args);
        // }
    }
}
