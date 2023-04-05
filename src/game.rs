
use crate::enums::GameState;
use graphics::{clear};
use crate::shared::Render;
use piston_window::G2d;
use piston_window::context::Context;


pub struct Game {
    background_color: [f32; 4],
    score: u32,
    state: GameState,
}

impl Game {
    fn draw_ui(&self, gl: &mut G2d) {
        todo!();
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            background_color: [0.0, 0.0, 0.0, 1.0],
            score: 0,
            state: GameState::Standby,
        }
    }
}

impl Render for Game {
    fn render(&self, gl: &mut piston_window::G2d, _: Option<Context>) {
        clear(self.background_color, gl);
        // self.draw_ui(gl);
    }
}