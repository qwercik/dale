extern crate sfml;

mod config;
mod vm;

use std::env;
use std::vec::Vec;

use config::Config;
use vm::VirtualMachine;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    match Config::parse(&arguments) {
        Ok(config) => {
            let vm = VirtualMachine::new(config);
            vm.run();
        },
        Err(_) => {
            eprintln!("Incorrect usage!");
            eprintln!("Use: dale file.rom");
        }
    };
}
