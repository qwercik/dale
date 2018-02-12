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
    
    pub fn draw_sprite(&mut self, position_x: usize, position_y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;

        for y in 0..sprite.len() {
            for x in 0..8 {
                let state = self.set_pixel(x + position_x, y + position_y, sprite[y] & (1 << x) != 0);

                if state {
                    collision = true;
                }
            }
        }

        collision
    }


    pub fn render(&self, mut window: &mut RenderWindow) {
        for row in &self.pixels {
            for pixel in row {
                pixel.render(&mut window);
            }
        }
    }
}
