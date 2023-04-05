mod entities;
mod enums;
mod game;
mod helpers;
mod shared;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate piston_window;

use std::path::Path;

use enums::Direction;
use opengl_graphics::{OpenGL};
use piston::{ButtonEvent, EventLoop};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent};
use entities::{Food, Snake};
use game::Game;
use piston_window::{PistonWindow, WindowSettings};
use shared::Render;

fn main() {
    let opengl = OpenGL::V3_2;
    let window_title = String::from("Snek");
    let window_size = (500, 500);

    let mut window: PistonWindow = WindowSettings::new(window_title, window_size)
        .resizable(false)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
     
     let font_path = Path::new("../assets/Minecraft.ttf");
    let mut glyphs = window.load_font(font_path);

    let mut game = Game::default();

    let mut snake = Snake::default();
    let mut food = Food::from_screen(&window_size);

    let mut event_settings = EventSettings::new();
    event_settings.ups = 8;
    window.set_event_settings(event_settings);

    while let Some(ev) = window.next() {
        window.draw_2d(&ev, |ctx, gl, _device| {
            game.render(gl, None);
            snake.render(gl, Some(ctx));
            food.render(gl, Some(ctx));
        });

        if ev.update_args().is_some() {
            snake.update();
            food.is_eaten(&snake);
            if food.is_eaten {
                game.add_score();
                snake.grow();
            }
            food.update(&window_size);
        }

        if let Some(args) = ev.button_args() {
            if let Some(code) = args.scancode {
                let direction = match code {
                    57416 => Direction::Up,
                    57419 => Direction::Left,
                    57421 => Direction::Right,
                    57424 => Direction::Down,
                    _ => snake.direction,
                };
                snake.update_direction(direction);
            }
        }
    }
}