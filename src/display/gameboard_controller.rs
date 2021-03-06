pub use crate::display::gameboard_view::{GameboardView, GameboardViewSettings};
use piston::input::GenericEvent;

pub struct GameboardController {
    pub selected_cell: Option<[usize; 2]>,
    pub click_on: Option<[f64; 2]>,
    pub cursor_pos: [f64; 2],
}

impl GameboardController {
    pub fn new() -> GameboardController {
        GameboardController {
            selected_cell: None,
            click_on: None,
            cursor_pos: [0.0, 0.0],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, MouseButton};

        e.mouse_cursor(|pos| self.cursor_pos = pos);
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.click_on = Some(self.cursor_pos);
        }
    }
}
