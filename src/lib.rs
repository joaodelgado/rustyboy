use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub mod utils;
mod cpu;

pub struct Config {
    pub rom_name: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skip program name
        args.next();

        let rom_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide the rom name the first argument"),
        };

        Ok(Config { rom_name })
    }
}

pub fn read_rom(config: &Config) -> Result<Vec<u8>, Box<Error>> {
    let mut file = File::open(&config.rom_name)?;
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)?;

    Ok(data)
}
