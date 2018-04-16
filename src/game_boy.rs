use Config;
use cartridge::Cartridge;
use cpu::Cpu;
use debugger::Debugger;
use errors::{Error, ErrorKind, Result};
use read_file;

const MEM_CARTRIDGE_INTERRUPTS_BEGIN: usize = 0x0000;
const MEM_CARTRIDGE_INTERRUPTS_END: usize = 0x00ff;
const MEM_CARTRIDGE_HEADER_BEGIN: usize = 0x0100;
const MEM_CARTRIDGE_HEADER_END: usize = 0x014f;
const MEM_CARTRIDGE_BANK_0_BEGIN: usize = 0x0150;
const MEM_CARTRIDGE_BANK_0_END: usize = 0x3fff;
const MEM_CARTRIDGE_BANK_1_BEGIN: usize = 0x4000;
const MEM_CARTRIDGE_BANK_1_END: usize = 0x7fff;

const MEM_CHECKSUM_BEGIN: usize = 0x104;
const MEM_CHECKSUM_END: usize = 0x133;

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
    debugger: Debugger,
}

impl GameBoy {
    pub fn new(config: &Config) -> Result<GameBoy> {
        // Initialize cartridge
        let cartridge_data = read_file(&config.rom_name)?;
        let cartridge = Cartridge::new(cartridge_data);

        Ok(GameBoy {
            cpu: Cpu::new(),
            cartridge,
            debugger: Debugger::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.init_memory();
        self.cpu.init();

        self.check_rom()?;

        // TODO Boot sequence (logo screen and musical notes)

        // TODO game loop
        println!("Running rom with title: {}", self.cartridge.title());
        if let Some(rom_type) = self.cartridge.cartridge_type() {
            println!("Running rom with type: {:?}", rom_type);
        }

        loop {
            if cfg!(feature = "debug") {
                self.debugger.tick(&mut self.cpu)?;
            } else {
                self.cpu.tick()?;
            }
        }
    }

    fn check_rom(&self) -> Result<()> {
        // Validate ROM checksum
        let sum = self.cpu
            .get_mem_range(MEM_CHECKSUM_BEGIN, MEM_CHECKSUM_END)
            .iter()
            .fold(25u8, |sum, v| sum.wrapping_add(*v));

        match sum {
            0 => Err(Error::new(
                ErrorKind::Validation,
                "ROM failed checksum validation",
            )),
            _ => Ok(()),
        }
    }

    fn init_memory(&mut self) {
        // Copy cartridge
        self.cpu.set_mem_range(
            MEM_CARTRIDGE_INTERRUPTS_BEGIN,
            MEM_CARTRIDGE_INTERRUPTS_END,
            self.cartridge.interrupts(),
        );
        self.cpu.set_mem_range(
            MEM_CARTRIDGE_HEADER_BEGIN,
            MEM_CARTRIDGE_HEADER_END,
            self.cartridge.header(),
        );
        self.cpu.set_mem_range(
            MEM_CARTRIDGE_BANK_0_BEGIN,
            MEM_CARTRIDGE_BANK_0_END,
            self.cartridge.bank0(),
        );
        // TODO implement this correctly using cartridge type
        self.cpu.set_mem_range(
            MEM_CARTRIDGE_BANK_1_BEGIN,
            MEM_CARTRIDGE_BANK_1_END,
            self.cartridge.bank1(),
        );

        // Initialize IO registers
        self.cpu.set_mem(TIMA, 0x00);
        self.cpu.set_mem(TMA, 0x00);
        self.cpu.set_mem(TAC, 0x00);
        self.cpu.set_mem(NR10, 0x80);
        self.cpu.set_mem(NR11, 0xbf);
        self.cpu.set_mem(NR12, 0xf3);
        self.cpu.set_mem(NR14, 0xbf);
        self.cpu.set_mem(NR21, 0x3f);
        self.cpu.set_mem(NR22, 0x00);
        self.cpu.set_mem(NR24, 0xbf);
        self.cpu.set_mem(NR30, 0x7f);
        self.cpu.set_mem(NR31, 0xff);
        self.cpu.set_mem(NR32, 0x9f);
        self.cpu.set_mem(NR33, 0xbf);
        self.cpu.set_mem(NR41, 0xff);
        self.cpu.set_mem(NR42, 0x00);
        self.cpu.set_mem(NR43, 0x00);
        self.cpu.set_mem(NR44, 0xbf);
        self.cpu.set_mem(NR50, 0x77);
        self.cpu.set_mem(NR51, 0xf3);
        self.cpu.set_mem(NR52, 0xf1); // TODO This assumes we are running on a GB
        self.cpu.set_mem(LCDC, 0x91);
        self.cpu.set_mem(SCY, 0x00);
        self.cpu.set_mem(SCX, 0x00);
        self.cpu.set_mem(LYC, 0x00);
        self.cpu.set_mem(BGP, 0xfc);
        self.cpu.set_mem(OBP0, 0xff);
        self.cpu.set_mem(OBP1, 0xff);
        self.cpu.set_mem(WY, 0x00);
        self.cpu.set_mem(WX, 0x00);

        self.cpu.set_mem(IE, 0x00);
    }
}
