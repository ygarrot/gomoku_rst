extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

mod game {
    // pub mod minimax;
    // pub mod node;
    pub mod board;
    pub mod game;
    pub mod r#move;
    pub mod player;
    pub mod rules;
}
mod gameboard_controller;
mod gameboard_view;

use game::game::{Game, MoveError};
use game::r#move::Move;
use graphics::*;

pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
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
        vec![("Robert", false), ("Michel", true)],
        9,
        1,
        vec!["Base"],
    );

    let mut gameboard_controller = GameboardController::new();
    let mut events = Events::new(EventSettings::new());
    let font_path = "./resources/FiraSans-Regular.ttf";
    let mut font_glyph = GlyphCache::new(font_path, (), TextureSettings::new()).unwrap();
    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            gameboard_view.gl.draw(args.viewport(), |c, gl| {
                graphics::clear([255.0, 255.0, 255.0, 1.0], gl);
                graphics::text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                    .draw(
                        "Hello world!",
                        &mut font_glyph,
                        &c.draw_state,
                        c.transform.trans(10.0, 100.0),
                        gl,
                    )
                    .unwrap()
            });
            match gameboard_controller.click_on {
                Some(x) => match gameboard_view.get_cursor_indexes(game.board.size, &args, x) {
                    Some(coo) => match game.r#move(
                        &Move {
                            x: coo[0],
                            y: coo[1],
                        },
                        None,
                    ) {
                        Ok(_) => (),
                        Err(e) => match e {
                            MoveError::MoveForbidden => {
                                println!("Move [{}, {}] forbidden!", coo[0], coo[1])
                            }
                            MoveError::GameEnded => return println!("Game has ended !"),
                        },
                    },
                    None => (),
                },
                None => (),
            };

            gameboard_view.render(&game.board, &args, gameboard_controller.cursor_pos);
            gameboard_controller.click_on = None;
        }
        // if let Some(args) = e.update_args() {
        //     gameboard_view.update(&args);
        // }
    }
}
