mod config;

use std::env;
use std::vec::Vec;

use config::Config;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    match Config::parse(&arguments) {
        Ok(config) => {
            println!("Loading ROM {}", config.get_rom_filename());  
        },
        Err(_) => {
            eprintln!("Incorrect usage!");
            eprintln!("Use: dale file.rom");
        }
    };
}
