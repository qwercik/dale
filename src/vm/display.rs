use sfml::graphics::*;

const RESOLUTION: (usize, usize) = (64, 32);

pub struct Display {
    buffer: [[bool; RESOLUTION.1]; RESOLUTION.0],
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; RESOLUTION.1]; RESOLUTION.0],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let set: bool = self.buffer[y][x] == value;
        self.buffer[y][x] = value;
        
        set
    }
}
