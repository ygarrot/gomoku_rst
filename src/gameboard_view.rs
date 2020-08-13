extern crate graphics;
extern crate piston;

use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::f64;

pub struct GameboardView {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub settings: GameboardViewSettings,
}

pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub line_color: Color,

    pub board_size: i64,
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
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WOOD: [f32; 4] = [252.0, 186.0, 0.0, 3.0];
        const CIRCLE_COL: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

        GameboardViewSettings {
            board_size: 19,
            circle_size: 20.0,
            circle_radius: 20.0 / 2.0,
            position: [10.0; 2],
            size: 400.0,
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
            background_color: BLACK,
            line_color: WOOD,
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

    pub fn render(&mut self, args: &RenderArgs) {
        let ref settings = self.settings;
        use graphics::*;
        const X: usize = 0;
        const Y: usize = 1;
        let circle = rectangle::square(0.0, 0.0, settings.circle_size);
        let square_size = [
            (args.window_size[X] as i64 / settings.board_size + 2) as f64,
            (args.window_size[Y] as i64 / settings.board_size + 2) as f64,
        ];
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
                    ellipse(settings.circle_color, circle, circle_transform, gl);
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
