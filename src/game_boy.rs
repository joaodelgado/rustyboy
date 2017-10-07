#![allow(dead_code)]

use std::error::Error;

use cpu::Cpu;
use cartridge::Cartridge;
use Config;
use read_file;

const RAM_SIZE: usize = 32 * 1024;
const VRAM_SIZE: usize = 16 * 1024;

pub struct GameBoy {
    pub cpu: Cpu,
    pub cartridge: Cartridge,
    pub ram: [u8; RAM_SIZE],
    pub vram: [u8; VRAM_SIZE],
}

impl GameBoy {
    pub fn new(config: &Config) -> Result<GameBoy, Box<Error>> {
        let cartridge_data = read_file(&config.rom_name)?;
        Ok(GameBoy {
            cpu: Cpu::new(),
            cartridge: Cartridge::new(cartridge_data)?,
            ram: [0; RAM_SIZE],
            vram: [0; VRAM_SIZE],
        })
    }
}
