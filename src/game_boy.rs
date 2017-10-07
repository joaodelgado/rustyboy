#![allow(dead_code)]

use std::error::Error;

use cpu::Cpu;
use cartridge::Cartridge;
use Config;

const RAM_SIZE: usize = 32 * 1024;
const VRAM_SIZE: usize = 16 * 1024;

struct GameBoy {
    cpu: Cpu,
    cartridge: Cartridge,
    ram: [u8; RAM_SIZE],
    vram: [u8; VRAM_SIZE],
}

impl GameBoy {
    pub fn new(config: &Config) -> Result<GameBoy, Box<Error>> {
        Ok(GameBoy {
            cpu: Cpu::new(),
            cartridge: Cartridge::new(config)?,
            ram: [0; RAM_SIZE],
            vram: [0; VRAM_SIZE],
        })
    }
}
