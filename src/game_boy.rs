use std::error::Error;

use cpu::Cpu;
use cartridge::Cartridge;
use Config;
use read_file;

const RAM_SIZE: usize = 64 * 1024;

const MEM_CARTRIDGE_HEADER_BEGIN: usize = 0x100;
const MEM_CARTRIDGE_HEADER_END: usize = 0x14f;

//
// Memory map
//

// IO Registers
const TIMA: usize = 0xff05;
const TMA: usize = 0xff06;
const TAC: usize = 0xff07;
const NR10: usize = 0xff10;
const NR11: usize = 0xff11;
const NR12: usize = 0xff12;
const NR14: usize = 0xff14;
const NR21: usize = 0xff16;
const NR22: usize = 0xff17;
const NR24: usize = 0xff19;
const NR30: usize = 0xff1a;
const NR31: usize = 0xff1b;
const NR32: usize = 0xff1c;
const NR33: usize = 0xff1e;
const NR41: usize = 0xff20;
const NR42: usize = 0xff21;
const NR43: usize = 0xff22;
const NR44: usize = 0xff23;
const NR50: usize = 0xff24;
const NR51: usize = 0xff25;
const NR52: usize = 0xff26;
const LCDC: usize = 0xff40;
const SCY: usize = 0xff42;
const SCX: usize = 0xff43;
const LYC: usize = 0xff45;
const BGP: usize = 0xff47;
const OBP0: usize = 0xff48;
const OBP1: usize = 0xff49;
const WY: usize = 0xff4a;
const WX: usize = 0xff4b;

// Interrupt enable flag
const IE: usize = 0xffff;


pub struct GameBoy {
    cpu: Cpu,
    cartridge: Cartridge,
    ram: [u8; RAM_SIZE],
}

impl GameBoy {
    pub fn new(config: &Config) -> Result<GameBoy, Box<Error>> {
        // Initialize cartridge
        let cartridge_data = read_file(&config.rom_name)?;
        let cartridge = Cartridge::new(cartridge_data);

        Ok(GameBoy {
            cpu: Cpu::default(),
            cartridge: cartridge,
            ram: [0; RAM_SIZE],
        })
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        self.init_memory();
        self.cpu.init();

        self.check_rom()?;

        // TODO Boot sequence (logo screen and musical notes)

        // TODO game loop
        println!("Running rom with title: {}", self.cartridge.title());

        Ok(())
    }

    fn check_rom(&self) -> Result<(), &'static str> {
        // Validate ROM checksum
        let sum = self.cartridge.power_up_memory().iter().fold(
            25u8,
            |sum, v| {
                sum.wrapping_add(*v)
            },
        );

        match sum {
            0 => Err("ROM failed checksum validation"),
            _ => Ok(()),
        }
    }

    fn init_memory(&mut self) {
        // Initialize ram
        // Copy cartridge header
        self.ram[MEM_CARTRIDGE_HEADER_BEGIN..MEM_CARTRIDGE_HEADER_END]
            .copy_from_slice(self.cartridge.header());

        self.ram[TIMA] = 0x00;
        self.ram[TMA] = 0x00;
        self.ram[TAC] = 0x00;
        self.ram[NR10] = 0x80;
        self.ram[NR11] = 0xbf;
        self.ram[NR12] = 0xf3;
        self.ram[NR14] = 0xbf;
        self.ram[NR21] = 0x3f;
        self.ram[NR22] = 0x00;
        self.ram[NR24] = 0xbf;
        self.ram[NR30] = 0x7f;
        self.ram[NR31] = 0xff;
        self.ram[NR32] = 0x9f;
        self.ram[NR33] = 0xbf;
        self.ram[NR41] = 0xff;
        self.ram[NR42] = 0x00;
        self.ram[NR43] = 0x00;
        self.ram[NR44] = 0xbf;
        self.ram[NR50] = 0x77;
        self.ram[NR51] = 0xf3;
        self.ram[NR52] = 0xf1; // TODO This assumes we are running on a GB
        self.ram[LCDC] = 0x91;
        self.ram[SCY] = 0x00;
        self.ram[SCX] = 0x00;
        self.ram[LYC] = 0x00;
        self.ram[BGP] = 0xfc;
        self.ram[OBP0] = 0xff;
        self.ram[OBP1] = 0xff;
        self.ram[WY] = 0x00;
        self.ram[WX] = 0x00;
        self.ram[IE] = 0x00;
    }
}
