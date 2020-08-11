extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

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
        let square_size = [
            (args.window_size[0] as i64 / BOARD_SIZE) as f64,
            (args.window_size[1] as i64 / BOARD_SIZE) as f64,
        ];

        let square = rectangle::square(0.0, 0.0, square_size[0] - 5.0);
        let circle = rectangle::square(0.0, 0.0, 20.0);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for i in 0..(BOARD_SIZE * BOARD_SIZE) {
                let x = (i % 19) as f64 * square_size[0];
                let y = (i / 19) as f64 * square_size[1] - 5.0;
                let transform = c.transform.trans(x, y);
                rectangle(WOOD, square, transform, gl);

                let x = x + (square_size[0] + 20.0) / 2.0;
                let y = y + (square_size[1] + 20.0) / 2.0;
                let circle_transform = c.transform.trans(x, y);
                ellipse(CIRCLE_COL, circle, circle_transform, gl);
                // let y = square_size * ((i as f64 * square_size) / args.window_size[0]).floor();
                // println!("x:{} y:{}", x, y);
                // println!(
                //     "x:{} round:{}",
                //     ((i as f64 * square_size) / args.window_size[0]),
                //     ((i as f64 * square_size) / args.window_size[0]).round()
                // );
                // println!(
                //     "win 0:{} win 1:{}",
                //     args.window_size[0], args.window_size[1]
                // );
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
