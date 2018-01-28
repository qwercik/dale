use sfml::graphics::*;
use sfml::system::*;
use std::vec::Vec;

pub const RESOLUTION: (usize, usize) = (64, 32);
pub const PIXEL_SIZE: f32 = 20.;

#[derive(Clone)]
pub struct Pixel<'a> {
    state: bool,
    graphics: RectangleShape<'a>,
}

impl<'a> Pixel<'a> {
    pub fn new(x: usize, y: usize) -> Self {
        let mut graphics = RectangleShape::with_size((PIXEL_SIZE, PIXEL_SIZE).into());
        graphics.set_fill_color(&Color::BLACK);
        let position: Vector2<f32> = (
            x as f32 * PIXEL_SIZE,
            y as f32 * PIXEL_SIZE,
        ).into();
        graphics.set_position(position);

        Pixel {
            state: false,
            graphics: graphics,
        }
    }
}

pub struct Display<'a> {
    buffer: Vec<Vec<Pixel<'a>>>,
}

impl<'a> Display<'a> {
    pub fn new() -> Self {
        let mut buffer = Vec::new();
        
        for y in 0..RESOLUTION.1 {
            buffer.push(Vec::new());
            for x in 0..RESOLUTION.0 {
                buffer[y].push(Pixel::new(x, y));
            }
        }

        Display {
            buffer,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let set: bool = self.buffer[y][x].state == value;
        self.buffer[y][x].state = value;
        if value {
            self.buffer[y][x].graphics.set_fill_color(&Color::WHITE); 
        } else {
            self.buffer[y][x].graphics.set_fill_color(&Color::BLACK); 
        }

        set
    }

    pub fn render(&self, window: &mut RenderWindow) {
        for y in &self.buffer {
            for x in y {
                window.draw(&x.graphics);
            }
        }
    }
}
