
use piston_window::G2d;
use piston_window::context::Context;

pub trait Render {
    fn render(&self, gl: &mut G2d, ctx: Option<Context>);
}