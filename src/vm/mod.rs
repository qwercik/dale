use sfml::graphics::*;
use sfml::window::*;

use config::Config;

pub struct VirtualMachine {
    config: Config,
    window: RenderWindow,
}

impl VirtualMachine {
    pub fn new(config: Config) -> VirtualMachine {
        let mut window = RenderWindow::new(
            (640, 320),
            "Dale",
            Style::CLOSE,
            &Default::default(),
        );
        window.set_framerate_limit(60);


        VirtualMachine { config, window }
    }

    pub fn run(&mut self) {
        println!("Running ROM {}", self.config.get_rom_filename());

        loop {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => return,
                    _ => continue,
                }
            }

            self.window.clear(&Color::BLACK);
            self.window.display();
        }
    }
}
