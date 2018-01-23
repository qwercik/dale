use config::Config;

pub struct VirtualMachine {
    config: Config,
}

impl VirtualMachine {
    pub fn new(config: Config) -> VirtualMachine {
        VirtualMachine { config }
    }

    pub fn run(&self) {
        println!("Running ROM {}", self.config.get_rom_filename());
    }
}
