extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::f64;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WOOD: [f32; 4] = [252.0, 186.0, 0.0, 3.0];
        const CIRCLE_COL: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
        const BOARD_SIZE: i64 = 19; // size: 19*19 dunno how to define
        const X: usize = 0;
        const Y: usize = 1;
        let square_size = [
            (args.window_size[X] as i64 / BOARD_SIZE + 2) as f64,
            (args.window_size[Y] as i64 / BOARD_SIZE + 2) as f64,
        ];

        let circle = rectangle::square(0.0, 0.0, 20.0);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            let cell_edge = Line::new(WOOD, 1.0);
            for i in 0..BOARD_SIZE {
                let x = (i + 1) as f64 * square_size[X];
                let y = (i + 1) as f64 * square_size[Y];
                let vline = [x, 0.0, x, args.window_size[Y]];
                cell_edge.draw(vline, &c.draw_state, c.transform, gl);
                let vline = [0.0, y, args.window_size[X], y];
                cell_edge.draw(vline, &c.draw_state, c.transform, gl);

                for j in 0..BOARD_SIZE {
                    let circle_transform = c.transform.trans(
                        x - 10.0,
                        square_size[Y] + (j as f64 * square_size[Y]) - 10.0,
                    );
                    ellipse(CIRCLE_COL, circle, circle_transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("gomoku", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
