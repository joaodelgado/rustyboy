#![allow(dead_code)]

macro_rules! KB {
    ( $x:expr ) => { $x * 1024 };
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
        use self::CartridgeType::*;

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

const INTERRUPTS_BEGIN: usize = 0x0000;
const INTERRUPTS_END: usize = 0x00ff;
const HEADER_BEGIN: usize = 0x0100;
const HEADER_END: usize = 0x014f;
const BANK0_BEGIN: usize = 0x0150;
const BANK0_END: usize = 0x3fff;

pub struct Cartridge {
    raw_data: Vec<u8>,
}

impl Cartridge {
    pub fn new(raw_data: Vec<u8>) -> Cartridge {
        Cartridge { raw_data: raw_data }
    }

    pub fn interrupts(&self) -> &[u8] {
        &self.raw_data[INTERRUPTS_BEGIN..INTERRUPTS_END + 1]
    }

    pub fn header(&self) -> &[u8] {
        &self.raw_data[HEADER_BEGIN..HEADER_END + 1]
    }

    pub fn bank0(&self) -> &[u8] {
        &self.raw_data[BANK0_BEGIN..BANK0_END + 1]
    }

    pub fn nintendo_logo(&self) -> &[u8] {
        &self.raw_data[0x104..0x133]
    }

    pub fn title(&self) -> String {
        String::from_utf8_lossy(&self.raw_data[0x134..0x143]).into_owned()
    }

    pub fn manufactor_code(&self) -> String {
        String::from_utf8_lossy(&self.raw_data[0x13f..0x142]).into_owned()
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
