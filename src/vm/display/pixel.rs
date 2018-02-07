use sfml::graphics::*;
use sfml::system::*;

pub const SIZE: f32 = 20.;
const FOREGROUND_COLOR: &Color = &Color::WHITE;
const BACKGROUND_COLOR: &Color = &Color::BLACK;

#[derive(Clone)]
pub struct Pixel<'a> {
    state: bool,
    graphics: RectangleShape<'a>,
}

impl<'a> Pixel<'a> {
    pub fn new(position_x: usize, position_y: usize) -> Self {
        let mut graphics = RectangleShape::with_size((SIZE, SIZE).into());
        graphics.set_fill_color(BACKGROUND_COLOR);
        
        graphics.set_position::<Vector2f>((
            position_x as f32 * SIZE,
            position_y as f32 * SIZE,
        ).into());
        
        Pixel {
            state: false,
            graphics: graphics,
        }
    }

    pub fn set(&mut self, new_state: bool) -> bool {
        let same_state = self.state == new_state;
        self.state = new_state;

        let pixel_color = match new_state {
            false => BACKGROUND_COLOR,
            true => FOREGROUND_COLOR,
        };
        self.graphics.set_fill_color(pixel_color);

        same_state
    }

    pub fn render(&self, window: &mut RenderWindow) {
        window.draw(&self.graphics);
    }
}
