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

static WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WOOD: [f32; 4] = [0.33, 0.13, 0.55, 1.0];
static CIRCLE_COL: [f32; 4] = [0.0, 1.0, 1.0, 0.2];

pub struct GameInformationsView<'a> {
    pub gl: GlGraphics,
    pub settings: GameInformationsViewSettings<'a>,
}

pub struct GameInformationsViewSettings<'a> {
    font_glyph: GlyphCache<'a>,
    pub background_color: Color,
    bg_texture: Texture,
}
impl GameInformationsViewSettings<'_> {
    pub fn new<'a>() -> GameInformationsViewSettings<'a> {
        let bg_texture =
            Texture::from_path(Path::new("./resources/wood.jpg"), &TextureSettings::new()).unwrap();
        let font_path = Path::new("./resources/FiraSans-Regular.ttf");
        let font_glyph = GlyphCache::new(font_path, (), TextureSettings::new()).unwrap();
        GameInformationsViewSettings {
            background_color: WOOD,
            bg_texture: bg_texture,
            font_glyph: font_glyph,
        }
    }
}

impl GameInformationsView<'_> {
pub fn new(gl: GlGraphics, settings: GameInformationsViewSettings) -> GameInformationsView {
        GameInformationsView {
            gl: gl,
            settings: settings,
        }
    }
    pub fn render(&mut self, board: &Board, args: &RenderArgs, mouse_cursor: [f64; 2]) {

        let ref mut settings = self.settings;
        //let background = Image::new().rect([0.0, 0.0, args.window_size[X], args.window_size[Y]]);
        self.gl.draw(args.viewport(), |c, gl| {
            //clear(settings.background_color, gl);
            //background.draw(&settings.bg_texture, &c.draw_state, c.transform, gl);
        })
    }
}

