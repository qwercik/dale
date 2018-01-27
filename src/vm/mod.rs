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

        let mut window = RenderWindow::new(
            (640, 320),
            "Dale",
            Style::CLOSE,
            &Default::default(),
        );
        window.set_framerate_limit(60);

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
