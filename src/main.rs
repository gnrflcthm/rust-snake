extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle::square;
use graphics::Graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use piston::{ButtonEvent, RenderArgs, UpdateArgs};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent};

enum GameState {
    Standby,
    InGame,
    GameOver
}
struct Game {
    background_color: [f32; 4],
    gl: GlGraphics,
    score: u32,
    state: GameState
}

impl Game {
    fn render(&mut self, _render_args: &RenderArgs) {
        self.gl.clear_color(self.background_color.clone());
    }

    fn add_score(&mut self) {
        self.score += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            background_color: [0.0, 0.0, 0.0, 1.0],
            gl: GlGraphics::new(OpenGL::V3_2),
            score: 0,
            state: GameState::Standby
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Block {
    position: (f64, f64),
}

struct Snake {
    head: Block,
    body: Vec<Block>,
    gl: GlGraphics,
    color: [f32; 4],
    direction: Direction,
    size: f64,
}

impl Snake {
    fn render(&mut self, render_args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(render_args.viewport(), |ctx, gl| {
            for b in &self.body {
                let block = square(b.position.0, b.position.1, self.size);
                rectangle(self.color, block, ctx.transform, gl);
            }

            let block = square(self.head.position.0, self.head.position.1, self.size);
            rectangle(self.color, block, ctx.transform, gl);
        });
    }

    fn update(&mut self) {
        let head_pos = self.head.position.clone();
        match self.direction {
            Direction::Down => self.head.position.1 += self.size,
            Direction::Up => self.head.position.1 -= self.size,
            Direction::Left => self.head.position.0 -= self.size,
            Direction::Right => self.head.position.0 += self.size,
        }

        if self.body.len() > 0 {
            let mut last_pos = self.body[0].position;
            let mut first = true;

            self.body = self
                .body
                .iter()
                .map(|block| {
                    if first {
                        first = false;
                        Block { position: head_pos }
                    } else {
                        let new_block = Block { position: last_pos };
                        last_pos = block.position;
                        new_block
                    }
                })
                .collect();

            // for body in self.body.iter() {
            //     if first {
            //         body.position = head_pos;
            //         first = false;
            //     } else {
            //         let temp_pos = body.position.clone();
            //         body.position = last_pos;
            //         let last_pos = temp_pos;
            //     }
            // }
        }
    }

    fn update_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            color: [0.0, 1.0, 0.0, 1.0],
            gl: GlGraphics::new(OpenGL::V3_2),
            head: Block {
                position: (0.0, 0.0),
            },
            body: vec![
                Block {
                    position: (0.0, 0.0),
                },
                Block {
                    position: (0.0, 0.0),
                },
            ],
            direction: Direction::Right,
            size: 25.0,
        }
    }
}

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

    let mut event_settings = EventSettings::new();
    event_settings.ups = 8;
    let mut events = Events::new(event_settings);

    while let Some(ev) = events.next(&mut window) {
        if let Some(args) = ev.render_args() {
            game.render(&args);
            snake.render(&args);
        }

        if let Some(args) = ev.update_args() {
            snake.update();
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

// Left = 57419
// Top = 57416
// Right = 57421
// Down = 57424
