use crate::enums::GameState;
use crate::shared::Render;
use graphics::clear;
use graphics::rectangle;
use piston_window::context::Context;
use piston_window::text::Text;
use piston_window::G2d;
use piston_window::GfxDevice;
use piston_window::Glyphs;

pub struct Game<'a> {
    background_color: [f32; 4],
    text_color: [f32; 4],
    font_size: u32,
    pub score: u32,
    state: GameState,
    pub glyphs: Option<&'a mut Glyphs>,
}

impl Game<'_> {
    fn draw_score_dashboard(&mut self, gl: &mut G2d, ctx: Context, device: &mut GfxDevice) {
        let text = Text::new_color(self.text_color, self.font_size);
        let position = [20.0, 37.5];

        let font = self.glyphs.as_mut().unwrap();

        let dashboard_background = [0.0, 0.0, 1.0, 1.0];
        let screen = &ctx.viewport.unwrap().rect;

        let rect = [0.0, 0.0, screen[2] as f64, 50.0];
        
        rectangle(dashboard_background, rect, ctx.transform, gl);


        text.draw_pos(
            &format!("Score: {}", self.score),
            position,
            *font,
            &ctx.draw_state,
            ctx.transform,
            gl,
        )
        .unwrap();

        font.factory.encoder.flush(device)
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn update_state(&mut self, state: GameState) {
        self.state = state;
    }
}

impl Default for Game<'_> {
    fn default() -> Self {
        Game {
            background_color: [0.0, 0.0, 0.0, 1.0],
            text_color: [1.0, 1.0, 1.0, 1.0],
            font_size: 25,
            score: 0,
            state: GameState::InGame,
            glyphs: None,
        }
    }
}

impl Render for Game<'_> {
    fn render(
        &mut self,
        gl: &mut piston_window::G2d,
        ctx: Option<Context>,
        device: Option<&mut GfxDevice>,
    ) {
        clear(self.background_color, gl);

        if let Some(ctx) = ctx {
            if let Some(device) = device {
                self.draw_score_dashboard(gl, ctx, device);
            }
        }
    }
}
