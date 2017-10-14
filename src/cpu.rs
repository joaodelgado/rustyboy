#![allow(dead_code)]

use errors::{Error, ErrorKind, Result};

const MEM_SIZE: usize = 64 * 1024;

///
///  16bit Hi   Lo   Name/Function
///  AF    A    -    Accumulator & Flags
///  BC    B    C    BC
///  DE    D    E    DE
///  HL    H    L    HL
///  SP    -    -    Stack Pointer
///  PC    -    -    Program Counter/Pointer
///
pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    status: u8, // status flag: sign, zero, parity, carry, aux carry
    mem: [u8; MEM_SIZE],
}

pub enum StatusRegBit {
    Sign,
    Zero,
    Parity,
    Carry,
    AuxCarry,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            status: 0,
            mem: [0; MEM_SIZE],
        }
    }

    pub fn init(&mut self) {
        // TODO I'm assuming that we are running on a GB for now.
        // When we support multiple types, the value of the A register must change
        // accordingly:
        //  A=$01-GB/SGB, $FF-GBP, $11-GBC
        self.set_af(0x01b0);
        self.set_bc(0x0013);
        self.set_de(0x00d8);
        self.set_hl(0x014d);

        self.pc = 0x100;
    }

    //
    // Manage memory
    //

    pub fn set_mem(&mut self, i: usize, value: u8) {
        self.mem[i] = value
    }

    pub fn get_mem_range(&self, i: usize, j: usize) -> &[u8] {
        &self.mem[i..j]
    }

    pub fn set_mem_range(&mut self, i: usize, j: usize, data: &[u8]) {
        self.mem[i..j].copy_from_slice(data);
    }

    //
    // Manage registers
    //

    fn set_af(&mut self, n: u16) {
        self.a = (n >> 8) as u8;
        self.f = n as u8;
    }

    fn set_bc(&mut self, n: u16) {
        self.b = (n >> 8) as u8;
        self.c = n as u8;
    }

    fn set_de(&mut self, n: u16) {
        self.d = (n >> 8) as u8;
        self.e = n as u8;
    }

    fn set_hl(&mut self, n: u16) {
        self.h = (n >> 8) as u8;
        self.l = n as u8;
    }

    // Check if a certain flag is set
    fn status_is_set(&self, bit_enum: StatusRegBit) -> bool {
        match bit_enum {
            StatusRegBit::Sign => (self.status & 0b10000000) == 0b10000000,
            StatusRegBit::Zero => (self.status & 0b01000000) == 0b01000000,
            StatusRegBit::Parity => (self.status & 0b00100000) == 0b00100000,
            StatusRegBit::Carry => (self.status & 0b00010000) == 0b00010000,
            StatusRegBit::AuxCarry => (self.status & 0b000010000) == 0b00010000,
        }
    }

    // Set the defined status flag
    fn status_set(&mut self, bit_enum: StatusRegBit) {
        match bit_enum {
            StatusRegBit::Sign => self.status |= 0b10000000,
            StatusRegBit::Zero => self.status |= 0b01000000,
            StatusRegBit::Parity => self.status |= 0b00100000,
            StatusRegBit::Carry => self.status |= 0b00010000,
            StatusRegBit::AuxCarry => self.status |= 0b000010000,
        }
    }

    //
    // Tick
    //

    pub fn tick(&mut self) -> Result<()> {
        match self.get_next() {
            0x00 => self.nop(),
            0xc3 => self.jp_nn(),
            0xf3 => self.di(),
            s => Err(Error::new(
                ErrorKind::UnknownInstruction,
                format!(
                    "Unimplemented opcode {:02x}@{:04x}",
                    s,
                    self.pc - 1,
                ),
            )),
        }
    }

    fn get_next(&mut self) -> u8 {
        let result = self.mem[self.pc as usize];

        self.pc += 1;

        result
    }

    //
    // Opcodes
    //

    fn nop(&self) -> Result<()> {
        println!("NOP");
        Ok(())
    }

    ///**Description:**
    /// Jump to address nn.
    ///
    ///**Use with:**
    /// nn = two byte immediate value. (LS byte first.)
    ///
    fn jp_nn(&mut self) -> Result<()> {
        let fst_byte = self.get_next() as u16;
        let snd_byte = self.get_next() as u16;

        let addr = (snd_byte << 8) | fst_byte;
        self.pc = addr;
        println!("JP\t{:04x}", addr);
        Ok(())
    }

    /// **Description**
    ///
    /// This instruction disables interrupts but not
    /// immediately. Interrupts are disabled after
    /// instruction after DI is executed.
    ///
    /// **Flags affected**
    ///
    /// None
    fn di(&self) -> Result<()> {
        println!("DI");

        // TODO implement this

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_reg() {
        let mut cpu = Cpu::new();
        cpu.init();

        cpu.status_set(StatusRegBit::Sign);
        assert_eq!(cpu.status_is_set(StatusRegBit::Sign), true);

        cpu.status_set(StatusRegBit::Zero);
        assert_eq!(cpu.status_is_set(StatusRegBit::Zero), true);

        cpu.status_set(StatusRegBit::Parity);
        assert_eq!(cpu.status_is_set(StatusRegBit::Parity), true);

        cpu.status_set(StatusRegBit::Carry);
        assert_eq!(cpu.status_is_set(StatusRegBit::Carry), true);

        cpu.status_set(StatusRegBit::AuxCarry);
        assert_eq!(cpu.status_is_set(StatusRegBit::AuxCarry), true);

        cpu.status = 0;
        assert_eq!(cpu.status_is_set(StatusRegBit::AuxCarry), false);
    }

    #[test]
    fn test_jp_nn() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = 0xc3;
        cpu.mem[1] = 0x00;
        cpu.mem[2] = 0x01;

        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 0x100);
    }

    #[test]
    fn test_di() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = 0xf3;

        cpu.tick().unwrap();
    }
}
