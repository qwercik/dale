mod display;

use sfml::graphics::*;
use sfml::window::*;

use config::Config;
use self::display::Display;

pub struct VirtualMachine {
    config: Config,
    display: Display,
}

impl VirtualMachine {
    pub fn new(config: Config) -> VirtualMachine {
        let display = Display::new();

        VirtualMachine { config, display }
    }

    pub fn run(&mut self) {
        println!("Running ROM {}", self.config.get_rom_filename());

        let mut window = RenderWindow::new(
            (640, 320),
            "Dale",
            Style::CLOSE,
            &Default::default(),
        );
        window.set_framerate_limit(60);


        self.display.set_pixel(10, 10, true);

        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => return,
                    _ => continue,
                }
            }

            window.clear(&Color::BLACK);
            window.display();
        }
    }
}
