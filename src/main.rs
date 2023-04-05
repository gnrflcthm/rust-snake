mod entities;
mod enums;
mod game;
mod helpers;
mod shared;

extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use std::path::Path;

use entities::{Food, Snake};
use enums::Direction;
use find_folder::Search;
use game::Game;
use opengl_graphics::OpenGL;
use piston::{ButtonEvent, EventLoop};
use piston::{EventSettings, UpdateEvent};
use piston_window::{PistonWindow, WindowSettings};
use shared::Render;

fn main() {
    let opengl = OpenGL::V3_2;
    let window_title = String::from("Snek");
    let window_size = (500, 550);

    let mut window: PistonWindow = WindowSettings::new(window_title, window_size)
        .resizable(false)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let font_path = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(font_path.join("Minecraft.ttf")).unwrap();

    let mut game = Game::default();
    game.glyphs = Some(&mut glyphs);
    let mut snake = Snake::default();
    let mut food = Food::from_screen(&window_size);

    let mut event_settings = EventSettings::new();
    event_settings.ups = 8;
    window.set_event_settings(event_settings);

    while let Some(ev) = window.next() {
        window.draw_2d(&ev, |ctx, gl, device| {
            game.render(gl, Some(ctx), Some(device));
            snake.render(gl, Some(ctx), None);
            food.render(gl, Some(ctx), None);
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
