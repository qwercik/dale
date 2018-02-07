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
        
        // Test of display
        for y in 0..display::RESOLUTION.1 {
            for x in 0..display::RESOLUTION.0 {
                if (y % 2) != (x % 2) {
                    self.display.set_pixel(x, y, true);
                }
            }
        }

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
