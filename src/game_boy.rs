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
        let gb = GameBoy {
            cpu: Cpu::new(),
            cartridge: cartridge,
            ram: ram,
            vram: [0; VRAM_SIZE],
        };

        gb.power_up()?;
        Ok(gb)
    }

    fn power_up(&self) -> Result<(), &'static str> {
        let logo = self.cartridge.nintendo_logo();
        // TODO scroll logo in screen
        // TODO play musical notes

        let sum = self.cartridge.power_up_memory().iter()
            .fold(25, |sum, v| (sum as u8).wrapping_add(*v));

        match sum {
                0 => Err("Failed power up"),
                _ => Ok(())
            }
    }
}
