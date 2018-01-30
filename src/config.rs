pub struct Config {
    rom_filename: String,
}


pub type ConfigResult = Result<Config, String>;

impl Config {
    pub fn parse(arguments: &[String]) -> ConfigResult {
        if arguments.len() != 2 {
            return Err(String::from("Invalid arguments amount"));
        }
        
        let rom_filename = arguments[1].clone();
        Ok(Config { rom_filename })
    }

    pub fn get_rom_filename(&self) -> &String {
        &self.rom_filename
    }
}
