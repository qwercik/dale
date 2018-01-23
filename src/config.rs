pub struct Config {
    rom_filename: String,
}

pub enum ParseError {
    InvalidArgumentsAmount,
}

pub type ConfigResult = Result<Config, ParseError>;

impl Config {
    pub fn parse(arguments: &[String]) -> ConfigResult {
        if arguments.len() != 2 {
            return Err(ParseError::InvalidArgumentsAmount);
        }
        
        let rom_filename = arguments[1].clone();
        Ok(Config { rom_filename })
    }

    pub fn get_rom_filename(&self) -> &String {
        &self.rom_filename
    }
}
