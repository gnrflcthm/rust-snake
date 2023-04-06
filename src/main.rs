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

use entities::{Food, Snake};
use enums::{Direction, GameState};
use find_folder::Search;
use game::Game;
use opengl_graphics::OpenGL;
use piston::{ButtonEvent, ButtonState, EventLoop, Key, Window};
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
    snake.set_start_pos((250.0, 275.0));
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
            let size = window.size().clone();
            let screen_size = (size.width, size.height);
            
            snake.update();
            snake.check_state(&screen_size);

            if snake.is_dead {
                // TODO: Update Game State
                game.update_state(GameState::GameOver);
                println!("Score {}", game.score);
                std::process::exit(0);
            }

            food.is_eaten(&snake);
            if food.is_eaten {
                game.add_score();
                snake.grow();
            }
            food.update(&window_size, Some(&snake));
        }

        if let Some(args) = ev.button_args() {
            let direction = match args.state {
                ButtonState::Release => match args.button {
                    piston::Button::Keyboard(key) => match key {
                        Key::Up | Key::I => Direction::Up,
                        Key::Left | Key::J => Direction::Left,
                        Key::Right | Key::L => Direction::Right,
                        Key::Down | Key::K => Direction::Down,
                        _ => snake.direction,
                    },
                    _ => snake.direction,
                },
                ButtonState::Press => snake.direction,
            };
            snake.update_direction(direction);
        }
    }
}
