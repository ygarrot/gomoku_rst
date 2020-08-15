extern crate graphics;
extern crate piston;

use graphics::types::Color;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::f64;

static X: usize = 0;
static Y: usize = 1;

#[derive(Copy, Clone)]
pub enum PLAYER {
    WhitePlayer,
    BlackPlayer,
    NoPlayer,
}
static WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WOOD: [f32; 4] = [86.0, 35.0, 141.0, 1.0];
static CIRCLE_COL: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

pub struct GameboardView {
    pub gl: GlGraphics,
    pub settings: GameboardViewSettings,
}

pub struct GameboardViewSettings {
    pub goban: [[PLAYER; 19]; 19],
    pub square_size: [f64; 2],
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub line_color: Color,

    pub board_size: usize,
    pub circle_size: f64,
    pub circle_color: Color,
    pub circle_radius: f64,

    pub board_edge_radius: f64,
    pub section_edge_radius: f64,
    pub cell_edge_radius: f64,
    pub selected_cell_background_color: Color,
    pub text_color: Color,
}

impl GameboardViewSettings {
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            goban: [[PLAYER::NoPlayer; 19]; 19],
            board_size: 19,
            circle_size: 20.0,
            circle_radius: 20.0 / 2.0,
            position: [10.0; 2],
            square_size: [10.0; 2],
            size: 400.0,
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
            background_color: WOOD,
            line_color: BLACK,
            circle_color: CIRCLE_COL,
        }
    }
}

impl GameboardView {
    pub fn new(gl: GlGraphics, settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            gl: gl,
            settings: settings,
        }
    }

    pub fn update_settings(&mut self, args: &RenderArgs) {
        self.settings.square_size = [
            (args.window_size[X] as usize / self.settings.board_size + 2) as f64,
            (args.window_size[Y] as usize / self.settings.board_size + 2) as f64,
        ];
    }

    // pub fn draw_ellipse(&mut self, args: &RenderArgs, x: f64, y: f64) {
    //     let ref settings = self.settings;
    //     let circle = rectangle::square(0.0, 0.0, settings.circle_size);

    //     self.gl.draw(args.viewport(), |c, gl| {
    //         let circle_transform = c.transform.trans(x, y);
    //         ellipse(CIRCLE_COL_N, circle, circle_transform, gl);
    //     });
    // }

    pub fn get_cursor_indexes(
        &mut self,
        args: &RenderArgs,
        user_input: [f64; 2],
    ) -> Option<[i64; 2]> {
        self.update_settings(args);
        let ref settings = self.settings;
        let square_size = settings.square_size;

        let x_cursor = (user_input[X] / square_size[X] as f64).round();
        let y_cursor = (user_input[Y] / square_size[Y] as f64).round();
        let x = x_cursor * square_size[X];
        let y = y_cursor * square_size[Y];

        if x + settings.circle_radius > user_input[X]
            && x - settings.circle_radius < user_input[X]
            && y + settings.circle_radius > user_input[Y]
            && y - settings.circle_radius < user_input[Y]
        {
            println!("closest x: {}, y:{}", x_cursor, y_cursor);
            self.settings.goban[x_cursor as usize - 1][y_cursor as usize - 1] = PLAYER::BlackPlayer;
            Some([x_cursor as i64, y_cursor as i64])
        } else {
            None
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.update_settings(args);
        let ref settings = self.settings;
        let square_size = settings.square_size;

        let circle = rectangle::square(0.0, 0.0, settings.circle_size);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(settings.background_color, gl);
            let cell_edge = Line::new(settings.line_color, 1.0);
            for i in 0..settings.board_size {
                let x = (i + 1) as f64 * square_size[X];
                let y = (i + 1) as f64 * square_size[Y];
                let vline = [x, 0.0, x, args.window_size[Y]];
                cell_edge.draw(vline, &c.draw_state, c.transform, gl);
                let vline = [0.0, y, args.window_size[X], y];
                cell_edge.draw(vline, &c.draw_state, c.transform, gl);

                for j in 0..settings.board_size {
                    let circle_transform = c.transform.trans(
                        x - settings.circle_radius,
                        square_size[Y] + (j as f64 * square_size[Y]) - settings.circle_radius,
                    );
                    let color = match settings.goban[i][j] {
                        PLAYER::BlackPlayer => BLACK,
                        PLAYER::WhitePlayer => WHITE,
                        _ => CIRCLE_COL,
                    };
                    ellipse(color, circle, circle_transform, gl);
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
