mod entities;
mod enums;
mod game;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use enums::Direction;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL};
use piston::window::WindowSettings;
use piston::{ButtonEvent};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent};
use entities::{Food, Snake};
use game::Game;

fn main() {
    let opengl = OpenGL::V3_2;
    let window_title = String::from("Snek");
    let window_size = (500, 500);

    let mut window: Window = WindowSettings::new(window_title, window_size)
        .resizable(false)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::default();
    let mut snake = Snake::default();
    let mut food = Food::from_screen(&window_size);

    let mut event_settings = EventSettings::new();
    event_settings.ups = 8;
    let mut events = Events::new(event_settings);

    while let Some(ev) = events.next(&mut window) {
        if let Some(args) = ev.render_args() {
            game.render(&args);
            snake.render(&args);
            food.render(&args);
        }

        if let Some(_) = ev.update_args() {
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