
use piston_window::{G2d, GfxDevice};
use piston_window::context::Context;

pub trait Render {
    fn render(&mut self, gl: &mut G2d, ctx: Option<Context>, device: Option<&mut GfxDevice>);
}