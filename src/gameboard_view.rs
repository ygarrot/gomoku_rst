extern crate graphics;
extern crate num_derive;
extern crate piston;

use graphics::types::Color;
use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, Texture, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
use std::f64;
use std::path::Path;

use crate::game::board::Board;

static X: usize = 0;
static Y: usize = 1;

#[derive(Copy, Clone, num_derive::FromPrimitive)]
pub enum PLAYER {
    NoPlayer = 0,
    WhitePlayer = 1,
    BlackPlayer = 2,
}

static WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WOOD: [f32; 4] = [0.33, 0.13, 0.55, 1.0];
static CIRCLE_COL: [f32; 4] = [0.0, 1.0, 1.0, 0.2];

pub struct GameboardView<'a> {
    pub gl: GlGraphics,
    pub settings: GameboardViewSettings<'a>,
}

pub struct GameboardViewSettings<'a> {
    pub square_size: [f64; 2],
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub line_color: Color,

    pub circle_size: f64,
    pub circle_color: Color,
    pub circle_radius: f64,
    pub detection_radius: f64,

    pub board_edge_radius: f64,
    pub section_edge_radius: f64,
    pub cell_edge_radius: f64,
    pub selected_cell_background_color: Color,
    pub text_color: Color,
    bg_texture: Texture,
    font_glyph: GlyphCache<'a>,
}

impl GameboardViewSettings<'_> {
    pub fn new<'a>() -> GameboardViewSettings<'a> {
        let bg_texture =
            Texture::from_path(Path::new("./resources/wood.jpg"), &TextureSettings::new()).unwrap();
        let font_path = Path::new("./resources/FiraSans-Regular.ttf");
        let font_glyph = GlyphCache::new(font_path, (), TextureSettings::new()).unwrap();

        GameboardViewSettings {
            circle_size: 20.0,
            circle_radius: 20.0 / 2.0,
            detection_radius: 20.0 / 2.0 * 3.0,
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
            bg_texture: bg_texture,
            font_glyph: font_glyph,
        }
    }
}

impl GameboardView<'_> {
    pub fn new(gl: GlGraphics, settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            gl: gl,
            settings: settings,
        }
    }

    pub fn update_settings(&mut self, size: usize, args: &RenderArgs) {
        self.settings.square_size = [
            (args.window_size[X] as usize / size + 2) as f64,
            (args.window_size[Y] as usize / size + 2) as f64,
        ];

        self.settings.detection_radius = args.window_size[X].min(args.window_size[Y]) / 2.0
        // TODO : coder le code
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
        board_size: usize,
        args: &RenderArgs,
        user_input: [f64; 2],
    ) -> Option<[u32; 2]> {
        self.update_settings(board_size, args);
        let ref settings = self.settings;
        let square_size = settings.square_size;

        let x_cursor = (user_input[X] / square_size[X] as f64).round();
        let y_cursor = (user_input[Y] / square_size[Y] as f64).round();

        if x_cursor <= 0.0 || y_cursor <= 0.0 {
            return None;
        }

        let x = x_cursor * square_size[X];
        let y = y_cursor * square_size[Y];

        if x + settings.detection_radius > user_input[X]
            && x - settings.detection_radius < user_input[X]
            && y + settings.detection_radius > user_input[Y]
            && y - settings.detection_radius < user_input[Y]
        {
            Some([x_cursor as u32 - 1, y_cursor as u32 - 1])
        } else {
            None
        }
    }

    pub fn render(&mut self, board: &Board, args: &RenderArgs, mouse_cursor: [f64; 2]) {
        self.update_settings(board.size, args);
        let coo = self.get_cursor_indexes(board.size, &args, mouse_cursor);
        let ref mut settings = self.settings;
        let square_size = settings.square_size;

        let circle = rectangle::square(0.0, 0.0, settings.circle_size);
        let background = Image::new().rect([0.0, 0.0, args.window_size[X], args.window_size[Y]]);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(settings.background_color, gl);
            background.draw(&settings.bg_texture, &c.draw_state, c.transform, gl);
            
            let cell_edge = Line::new(settings.line_color, 1.0);
            for i in 0..board.size {
                let x = (i + 1) as f64 * square_size[X];
                let y = (i + 1) as f64 * square_size[Y];
                let vline = [x, 0.0, x, args.window_size[Y]];
                cell_edge.draw(vline, &c.draw_state, c.transform, gl);
                let vline = [0.0, y, args.window_size[X], y];
                cell_edge.draw(vline, &c.draw_state, c.transform, gl);
            }
            for i in 0..board.size {
                for j in 0..board.size {
                    let circle_transform = c.transform.trans(
                        (i + 1) as f64 * square_size[X] - settings.circle_radius,
                        square_size[Y] + (j as f64 * square_size[Y]) - settings.circle_radius,
                    );
                    let color = match num_traits::FromPrimitive::from_u8(board.get_fcoo(i, j)) {
                        Some(PLAYER::BlackPlayer) => BLACK,
                        Some(PLAYER::WhitePlayer) => WHITE,
                        _ => continue,
                    };
                    ellipse(color, circle, circle_transform, gl);
                }
            }
            if let Some(coo) = coo {
                let trans = c.transform.trans(
                    (coo[X] + 1) as f64 * settings.square_size[X] - settings.circle_radius,
                    square_size[Y] + (coo[Y] as f64 * square_size[Y]) - settings.circle_radius,
                );
                ellipse(CIRCLE_COL, circle, trans, gl);
            }
            text::Text::new_color(BLACK, 32)
                .draw("Hello world!", &mut settings.font_glyph, &c.draw_state, c.transform.trans(10.0, 100.0), gl,)
                .unwrap();
        });
    }
    
    pub fn update(&mut self, _args: &UpdateArgs) {}
}
