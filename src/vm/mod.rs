mod display;

use sfml::graphics::*;
use sfml::window::*;

use config::Config;
use self::display::Display;

pub struct VirtualMachine<'a> {
    config: Config,
    display: Display<'a>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(config: Config) -> VirtualMachine<'a> {
        let display = Display::new();

        VirtualMachine { config, display }
    }

    pub fn run(&mut self) {
        println!("Running ROM {}", self.config.get_rom_filename());
        
        let window_resolution = (
            (display::RESOLUTION.0 * display::pixel::SIZE as usize) as u32,
            (display::RESOLUTION.1 * display::pixel::SIZE as usize) as u32,
        );
        let mut window = RenderWindow::new(
            window_resolution,
            "Dale",
            Style::CLOSE,
            &Default::default(),
        );
        window.set_framerate_limit(60);
        
        // Sprite display test
        let sprite = [
            0b11111111,
            0b10000001,
            0b10000001,
            0b10000001,
            0b10000001,
            0b10000001,
            0b11111111,
        ];
        self.display.draw_sprite(5, 5, &sprite);
        
        let sprite = [
            0b00100100,
            0b00000000,
            0b10000001,
            0b10000001,
            0b01000010,
            0b00100100,
            0b00011000,
        ];
        self.display.draw_sprite(20, 5, &sprite);


        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => return,
                    _ => continue,
                }
            }

            window.clear(&Color::BLACK);
            self.display.render(&mut window); 
            window.display();
        }
    }
}
