pub mod pixel;

use std::vec::Vec;
use sfml::graphics::*;
use self::pixel::Pixel;

pub const RESOLUTION: (usize, usize) = (64, 32);


pub struct Display<'a> {
    pixels: Vec<Vec<Pixel<'a>>>,
}

impl<'a> Display<'a> {
    pub fn new() -> Self {
        let mut pixels = Vec::new();
        for y in 0..RESOLUTION.1 {
            pixels.push(Vec::new());
            for x in 0..RESOLUTION.0 {
                pixels[y].push(Pixel::new(x, y));
            }
        }

        Display {
            pixels,
        }
    }

    pub fn set_pixel(&mut self, position_x: usize, position_y: usize, new_state: bool) -> bool {
        self.pixels[position_y][position_x].set(new_state)
    }
    
    pub fn clear_screen(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                pixel.set(false); 
            }
        }
    }
    
    pub fn render(&self, mut window: &mut RenderWindow) {
        for row in &self.pixels {
            for pixel in row {
                pixel.render(&mut window);
            }
        }
    }
}
