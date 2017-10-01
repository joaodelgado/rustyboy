pub mod utils;
mod cpu;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

macro_rules! KB {
    ( $x:expr ) => { $x * 1024 };
}

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

pub fn read_file(file_name: &String) -> Result<Vec<u8>, Box<Error>> {
    let mut file = File::open(file_name)?;
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)?;

    Ok(data)
}

pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    Huc3,
    Huc1RamBattery,
}

impl CartridgeType {
    fn new(n: u8) -> Option<CartridgeType> {
        use CartridgeType::*;

        match n {
            0x00 => Some(RomOnly),
            0x01 => Some(Mbc1),
            0x02 => Some(Mbc1Ram),
            0x03 => Some(Mbc1RamBattery),
            0x05 => Some(Mbc2),
            0x06 => Some(Mbc2Battery),
            0x08 => Some(RomRam),
            0x09 => Some(RomRamBattery),
            0x0B => Some(Mmm01),
            0x0C => Some(Mmm01Ram),
            0x0D => Some(Mmm01RamBattery),
            0x0F => Some(Mbc3TimerBattery),
            0x10 => Some(Mbc3TimerRamBattery),
            0x11 => Some(Mbc3),
            0x12 => Some(Mbc3Ram),
            0x13 => Some(Mbc3RamBattery),
            0x19 => Some(Mbc5),
            0x1A => Some(Mbc5Ram),
            0x1B => Some(Mbc5RamBattery),
            0x1C => Some(Mbc5Rumble),
            0x1D => Some(Mbc5RumbleRam),
            0x1E => Some(Mbc5RumbleRamBattery),
            0x20 => Some(Mbc6),
            0x22 => Some(Mbc7SensorRumbleRamBattery),
            0xFC => Some(PocketCamera),
            0xFD => Some(BandaiTama5),
            0xFE => Some(Huc3),
            0xFF => Some(Huc1RamBattery),
            _ => None,
        }

    }
}

pub struct Cartridge {
    raw_data: Vec<u8>,
}

impl Cartridge {
    pub fn new(config: &Config) -> Result<Cartridge, Box<Error>> {
        Ok(Cartridge { raw_data: read_file(&config.rom_name)? })
    }

    pub fn entry_point(&self) -> u32 {
        utils::to_u32(&self.raw_data[0x100..0x104])
    }

    pub fn nintendo_logo(&self) -> &[u8] {
        &self.raw_data[0x104..0x134]
    }

    pub fn title(&self) -> String {
        String::from_utf8_lossy(&self.raw_data[0x134..0x144]).into_owned()
    }

    pub fn manufactor_code(&self) -> String {
        String::from_utf8_lossy(&self.raw_data[0x13f..0x143]).into_owned()
    }

    pub fn sgb(&self) -> bool {
        self.raw_data[0x14b] == 0x33 && self.raw_data[0x146] == 0x03
    }

    pub fn cartridge_type(&self) -> Option<CartridgeType> {
        CartridgeType::new(self.raw_data[0x147])
    }

    pub fn rom_size(&self) -> u32 {
        KB!(32) << self.raw_data[0x148]
    }

    pub fn ram_size(&self) -> Option<u32> {
        match self.raw_data[0x149] {
            0x01 => Some(KB!(2)),
            0x02 => Some(KB!(8)),
            0x03 => Some(KB!(32)),
            0x04 => Some(KB!(128)),
            0x05 => Some(KB!(64)),
            _ => None,
        }
    }

    pub fn destination_code(&self) {
        // 11 014A - Destination Code
        unimplemented!();
    }

    pub fn mask_rom_version(&self) {
        // 13 014C - Mask ROM Version number
        unimplemented!();
    }

    pub fn header_checksum(&self) {
        // 14 014D - Header Checksum
        unimplemented!();
    }

    pub fn global_checksum(&self) {
        // 15 014E-014F - Global Checksum
        unimplemented!();
    }
}
