mod cartridge;
mod cpu;
mod errors;
mod debugger;
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

pub fn read_file(file_name: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)?;

    Ok(data)
}

#[inline]
pub fn u8_to_u16(b1: u8, b2: u8) -> u16 {
    (u16::from(b1) << 8) | u16::from(b2)
}

#[inline]
pub fn u16_to_u8(n: u16) -> (u8, u8) {
    (((n >> 8) as u8), (n as u8))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u8_to_u16() {
        assert_eq!(u8_to_u16(0xff, 0xff), 0xffff);
        assert_eq!(u8_to_u16(0xf0, 0x77), 0xf077);
    }

    #[test]
    fn test_u16_to_u8() {
        assert_eq!(u16_to_u8(0xffff), (0xff, 0xff));
        assert_eq!(u16_to_u8(0xf077), (0xf0, 0x77));
    }
}
