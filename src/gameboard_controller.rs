use piston::input::GenericEvent;

use crate::Gameboard;

pub struct GameboardController {
    pub gameboard: Gameboard,
    pub selected_cell: Option<[usize; 2]>,
    cursor_pos: [f64; 2],
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};

        e.mouse_cursor(|pos| self.cursor_pos = pos);

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            println!(
                "Mouse moved '{} {}'",
                self.cursor_pos[0], self.cursor_pos[1]
            );
        }
        // if let Some(Button::Keyboard(key)) = e.press_args() {
        //     if let Some(ind) = self.selected_cell {
        //         match key {
        //             Key::D1 => self.gameboard.set(ind, 1),
        //             _ => {}
        //         }
        //     }
        // }
    }
}
