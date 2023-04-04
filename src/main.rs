extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle::square;
use graphics::Graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use piston::{ButtonEvent, RenderArgs, UpdateArgs};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent};
use rand::{thread_rng, Rng};

enum GameState {
    Standby,
    InGame,
    GameOver,
}
struct Game {
    background_color: [f32; 4],
    gl: GlGraphics,
    score: u32,
    state: GameState,
}

impl Game {
    fn render(&mut self, render_args: &RenderArgs) {
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
            state: GameState::Standby,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, direction: Self) -> bool {
        let dir = self.to_owned();
        match direction {
            Self::Down => dir == Self::Up,
            Self::Up => dir == Self::Down,
            Self::Right => dir == Self::Left,
            Self::Left => dir == Self::Right,
        }
    }
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
        }
    }

    fn update_direction(&mut self, direction: Direction) {
        if !self.direction.is_opposite(direction) {
            self.direction = direction;
        }
    }

    fn grow(&mut self) {
        if let Some(block) = self.body.last() {
            self.body.push(Block { position: block.position });
        }
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

struct Food {
    color: [f32; 4],
    position: (f64, f64),
    is_eaten: bool,
    gl: GlGraphics,
    size: f64,
}

impl Food {
    fn is_eaten(&mut self, snake: &Snake) {
        self.is_eaten = self.position == snake.head.position;
    }

    fn render(&mut self, render_args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(render_args.viewport(), |ctx, gl| {
            let food = square(self.position.0, self.position.1, self.size);

            rectangle(self.color, food, ctx.transform, gl);
        });
    }

    fn update(&mut self, screen_size: &(u32, u32)) {
        if self.is_eaten {
            self.is_eaten = false;
            let (x, y) = Self::generate_new_position(self.size as u32, screen_size);
            self.position = (x as f64, y as f64);
        }
    }

    fn from_screen(screen_size: &(u32, u32)) -> Self {
        Food {
            color: [1.0, 0.0, 0.0, 1.0],
            gl: GlGraphics::new(OpenGL::V3_2),
            is_eaten: false,
            position: Food::generate_new_position(25, screen_size),
            size: 25.0
        }
    }

    pub fn generate_new_position(food_size: u32, screen_size: &(u32, u32)) -> (f64, f64) {
        let (w, h) = screen_size;
        let x = thread_rng().gen_range(0..(w / food_size)) * food_size;
        let y = thread_rng().gen_range(0..(h / food_size)) * food_size;
        (x as f64, y as f64)
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

        if let Some(args) = ev.update_args() {
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

// Left = 57419
// Top = 57416
// Right = 57421
// Down = 57424
