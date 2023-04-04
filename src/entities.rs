

use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;
use graphics::rectangle::square;
use graphics::rectangle;
use rand::{thread_rng, Rng};
use crate::enums::Direction;

pub struct Food {
    color: [f32; 4],
    position: (f64, f64),
    pub is_eaten: bool,
    gl: GlGraphics,
    size: f64,
}

impl Food {
    pub fn is_eaten(&mut self, snake: &Snake) {
        self.is_eaten = self.position == snake.head.position;
    }

    pub fn render(&mut self, render_args: &RenderArgs) {
        self.gl.draw(render_args.viewport(), |ctx, gl| {
            let food = square(self.position.0, self.position.1, self.size);

            rectangle(self.color, food, ctx.transform, gl);
        });
    }

    pub fn update(&mut self, screen_size: &(u32, u32)) {
        if self.is_eaten {
            self.is_eaten = false;
            let (x, y) = Self::generate_new_position(self.size as u32, screen_size);
            self.position = (x as f64, y as f64);
        }
    }

    pub fn from_screen(screen_size: &(u32, u32)) -> Self {
        Food {
            color: [1.0, 0.0, 0.0, 1.0],
            gl: GlGraphics::new(OpenGL::V3_2),
            is_eaten: false,
            position: Food::generate_new_position(25, screen_size),
            size: 25.0,
        }
    }

    pub fn generate_new_position(food_size: u32, screen_size: &(u32, u32)) -> (f64, f64) {
        let (w, h) = screen_size;
        let x = thread_rng().gen_range(0..(w / food_size)) * food_size;
        let y = thread_rng().gen_range(0..(h / food_size)) * food_size;
        (x as f64, y as f64)
    }
}

pub struct Snake {
    head: Block,
    body: Vec<Block>,
    gl: GlGraphics,
    color: [f32; 4],
    pub direction: Direction,
    size: f64,
}

impl Snake {
    pub fn render(&mut self, render_args: &RenderArgs) {
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

    pub fn update(&mut self) {
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

    pub fn update_direction(&mut self, direction: Direction) {
        if !self.direction.is_opposite(direction) {
            self.direction = direction;
        }
    }

    pub fn grow(&mut self) {
        if let Some(block) = self.body.last() {
            self.body.push(Block {
                position: block.position,
            });
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

#[derive(Clone, Copy)]
struct Block {
    position: (f64, f64),
}


