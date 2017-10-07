#![allow(dead_code)]

use std::error::Error;

use cpu::Cpu;
use cartridge::Cartridge;
use Config;
use read_file;

const RAM_SIZE: usize = 32 * 1024;
const VRAM_SIZE: usize = 16 * 1024;

const MEM_CARTRIDGE_HEADER_BEGIN: usize = 0x100;
const MEM_CARTRIDGE_HEADER_END: usize = 0x14f;

pub struct GameBoy {
    pub cpu: Cpu,
    pub cartridge: Cartridge,
    pub ram: [u8; RAM_SIZE],
    pub vram: [u8; VRAM_SIZE],
}

impl GameBoy {
    pub fn new(config: &Config) -> Result<GameBoy, Box<Error>> {
        // Initialize cartridge
        let cartridge_data = read_file(&config.rom_name)?;
        let cartridge = Cartridge::new(cartridge_data);

        // Initialize ram
        let mut ram = [0; RAM_SIZE];
        ram[MEM_CARTRIDGE_HEADER_BEGIN..MEM_CARTRIDGE_HEADER_END]
            .copy_from_slice(cartridge.header());

        Ok(GameBoy {
            cpu: Cpu::new(),
            cartridge: cartridge,
            ram: ram,
            vram: [0; VRAM_SIZE],
        })
    }
}
