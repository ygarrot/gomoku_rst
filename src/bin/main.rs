extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

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
    pub mod gameboard_controller;
    pub mod gameboard_view;
}

use game::game::{Game, MoveError};
use game::r#move::Move;
use game::minimax::minimax;

use display::gameboard_controller::GameboardController;
use display::gameboard_view::{GameboardView, GameboardViewSettings};

static AI_POWER_LVL: usize = 5;

fn main() {
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
    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            if game.players[(game.player_turn - 1) as usize].is_ai {
                let ret = minimax(
                    &mut game.board.clone(),
                    game.player_turn - 1,
                    game.player_turn - 1,
                    AI_POWER_LVL,
                    std::i64::MAX,
                    std::i64::MIN,
                    &mut game,
                    None,
                );
                match game.r#move(&ret.1, None, None) {
                    Ok(_) => (),
                    Err(e) => match e {
                        MoveError::GameEnded => return println!("Game has ended !"),
                        _ => (),
                    },
                }
            } else {
                match gameboard_controller.click_on {
                    Some(x) => match gameboard_view.get_cursor_indexes(game.board.size, &args, x) {
                        Some(coo) => match game.r#move(
                            &Move {
                                x: coo[0],
                                y: coo[1],
                            },
                            None,
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
            }
            gameboard_view.render(&game.board, &args, gameboard_controller.cursor_pos, &game);
            gameboard_controller.click_on = None;
        }
    }
}
