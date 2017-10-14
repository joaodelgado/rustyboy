mod cartridge;
mod cpu;
mod utils;
mod errors;
pub mod game_boy;

use std::fs::File;
use std::io::prelude::*;

use errors::{Error, ErrorKind, Result};

pub struct Config {
    pub rom_name: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config> {
        // Skip program name
        args.next();

        let rom_name = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Please provide the rom name the first argument",
                ))
            }
        };

        Ok(Config { rom_name })
    }
}

pub fn read_file(file_name: &String) -> Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)?;

    Ok(data)
}
