
use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;
use crate::enums::GameState;
use graphics::Graphics;
pub struct Game {
    background_color: [f32; 4],
    gl: GlGraphics,
    score: u32,
    state: GameState,
}

impl Game {
    pub fn render(&mut self, render_args: &RenderArgs) {
        self.gl.clear_color(self.background_color.clone());
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn update(&self) {
        
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            background_color: [0.0, 0.0, 0.0, 1.0],
            gl: GlGraphics::new(OpenGL::V3_2),
            score: 0,
            state: GameState::Standby,
        }
    }
}