use crate::enums::Direction;
use crate::shared::Render;
use graphics::rectangle;
use graphics::rectangle::square;
use graphics::types::Matrix2d;
use piston_window::{context::Context, GfxDevice};
use piston_window::G2d;
use rand::{thread_rng, Rng};

pub struct Food {
    color: [f32; 4],
    position: (f64, f64),
    pub is_eaten: bool,
    size: f64,
}

impl Food {
    pub fn is_eaten(&mut self, snake: &Snake) {
        self.is_eaten = self.position == snake.head.position;
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

impl Render for Food {
    fn render(&mut self, gl: &mut G2d, ctx: Option<Context>, _: Option<&mut GfxDevice>) {
        let mut transform = Matrix2d::default();

        if let Some(ctx) = ctx {
            transform = ctx.transform;
        }

        let food = square(self.position.0, self.position.1, self.size);

        rectangle(self.color, food, transform, gl);
    }
}

pub struct Snake {
    head: Block,
    body: Vec<Block>,
    color: [f32; 4],
    pub direction: Direction,
    size: f64,
}

impl Snake {
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

impl Render for Snake {
    fn render(&mut self, gl: &mut G2d, ctx: Option<Context>, _: Option<&mut GfxDevice>) {
        let mut transform = Matrix2d::default();

        if let Some(ctx) = ctx {
            transform = ctx.transform;
        }

        for b in &self.body {
            let block = square(b.position.0, b.position.1, self.size);
            rectangle(self.color, block, transform, gl);
        }

        let block = square(self.head.position.0, self.head.position.1, self.size);
        rectangle(self.color, block, transform, gl);
    }
}

#[derive(Clone, Copy)]
struct Block {
    position: (f64, f64),
}
