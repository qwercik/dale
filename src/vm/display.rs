use sfml::graphics::*;

use std::vec::Vec;

const RESOLUTION: (usize, usize) = (64, 32);
const PIXEL_SIZE: f32 = 10.;

#[derive(Clone)]
pub struct Pixel<'a> {
    state: bool,
    graphics: RectangleShape<'a>,
}

impl<'a> Pixel<'a> {
    pub fn new() -> Pixel<'a> {
        let mut graphics = RectangleShape::with_size((PIXEL_SIZE, PIXEL_SIZE).into());
        graphics.set_fill_color(&Color::BLACK);
        
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
    pub fn new() -> Display<'a> {
        let pixel = Pixel::new();

        Display {
            buffer: vec![vec![pixel; RESOLUTION.1]; RESOLUTION.0],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let set: bool = self.buffer[y][x].state == value;
        self.buffer[y][x].state = value;
        
        set
    }
}
