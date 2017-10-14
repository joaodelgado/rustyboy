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
    Zero,
    Sub,
    HalfCarry,
    Carry,
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

    fn get_hl(&mut self) -> u16 {
        ((self.h as u16)) << 8 | self.l as u16
    }

    fn set_hl(&mut self, n: u16) {
        self.h = (n >> 8) as u8;
        self.l = n as u8;
    }

    // Check if a certain flag is set
    fn status_is_set(&self, bit_enum: StatusRegBit) -> bool {
        match bit_enum {
            StatusRegBit::Zero => (self.status & 0b10000000) == 0b10000000,
            StatusRegBit::Sub => (self.status & 0b01000000) == 0b01000000,
            StatusRegBit::HalfCarry => (self.status & 0b00100000) == 0b00100000,
            StatusRegBit::Carry => (self.status & 0b00010000) == 0b00010000,
        }
    }

    // Set the defined status flag
    fn status_set(&mut self, bit_enum: StatusRegBit) {
        match bit_enum {
            StatusRegBit::Zero => self.status |= 0b10000000,
            StatusRegBit::Sub => self.status |= 0b01000000,
            StatusRegBit::HalfCarry => self.status |= 0b00100000,
            StatusRegBit::Carry => self.status |= 0b00010000,
        }
    }

    //
    // Tick
    //

    pub fn tick(&mut self) -> Result<()> {
        let opcode = self.get_next();
        match opcode {
            0x00 => self.nop(),
            0x31 => self.ld_sp_nn(),
            0xc3 => self.jp_nn(),
            0xf9 => self.ld_sp_hl(),
            0xf3 => self.di(),
            0xc2 | 0xca | 0xd2 | 0xda => self.jp_cc_nn(opcode),
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

    /// **Description**
    ///
    /// Put nn into Stack Pointer (SP).
    fn ld_sp_nn(&mut self) -> Result<()> {
        let snd_byte = self.get_next() as u16;
        let fst_byte = self.get_next() as u16;

        let addr = (snd_byte << 8) | fst_byte;
        self.sp = addr;

        println!("LD\tSP,{:04x}", addr);
        Ok(())
    }

    /// **Description:**
    /// Jump to address nn.
    ///
    /// **Use with:**
    /// nn = two byte immediate value. (LS byte first.)
    ///
    fn jp_nn(&mut self) -> Result<()> {
        let snd_byte = self.get_next() as u16;
        let fst_byte = self.get_next() as u16;

        let addr = (fst_byte << 8) | snd_byte;
        self.pc = addr;
        println!("JP\t{:04x}", addr);
        Ok(())
    }

    /// **Descriptio**
    ///
    /// Put HL into Stack Pointer (SP).
    fn ld_sp_hl(&mut self) -> Result<()> {
        self.sp = self.get_hl();

        println!("LD\tSP,HL");
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
        // TODO implement this

        println!("DI");
        Ok(())
    }

    fn nop(&self) -> Result<()> {
        println!("NOP");
        Ok(())
    }

    ///**Description:**
    /// Jump to address n if following condition is true:
    /// cc = NZ, Jump if Z flag is reset
    /// cc = Z, Jump if Z flag is set
    /// cc = NC, Jump if C flag is reset
    /// cc = C, Jump if C flag is set
    ///
    ///**Use with:**
    /// nn = two byte immediate value. (LS byte first.)
    fn jp_cc_nn(&mut self, opcode: u8) -> Result<()> {
        let addr_snd_byte = self.get_next() as u16;
        let addr_fst_byte = self.get_next() as u16;

        let addr = ((addr_fst_byte << 8) | addr_snd_byte) as u16;

        match opcode {
            0xc2 =>
                if !self.status_is_set(StatusRegBit::Zero) {
                    self.pc = addr;
                },
            0xca =>
                if self.status_is_set(StatusRegBit::Zero) {
                    self.pc = addr;
                },
            0xd2 =>
                if !self.status_is_set(StatusRegBit::Carry) {
                    self.pc = addr;
                },
            _ => // 0xda
                if self.status_is_set(StatusRegBit::Carry) {
                    self.pc = addr;
                },
        }
        println!("JP cc\t{:04x}", addr);
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

        cpu.status_set(StatusRegBit::Zero);
        assert_eq!(cpu.status_is_set(StatusRegBit::Zero), true);

        cpu.status_set(StatusRegBit::Sub);
        assert_eq!(cpu.status_is_set(StatusRegBit::Sub), true);

        cpu.status_set(StatusRegBit::HalfCarry);
        assert_eq!(cpu.status_is_set(StatusRegBit::HalfCarry), true);

        cpu.status_set(StatusRegBit::Carry);
        assert_eq!(cpu.status_is_set(StatusRegBit::Carry), true);
    }

    #[test]
    fn test_ld_sp_nn() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = 0x31;
        cpu.mem[1] = 0x01;
        cpu.mem[2] = 0x34;
        cpu.sp = 0;

        cpu.tick().unwrap();
        assert_eq!(cpu.sp, 0x0134);
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
    fn test_ld_sp_hl() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = 0xf9;
        cpu.sp = 0;
        cpu.h = 0x01;
        cpu.l = 0x34;

        cpu.tick().unwrap();
        assert_eq!(cpu.sp, 0x0134);
    }

    #[test]
    fn test_di() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = 0xf3;

        cpu.tick().unwrap();
    }

    fn test_jp_cc_nn() {
        let mut cpu = Cpu::new();

        // check zero flag not set
        cpu.mem[0] = 0xc2;
        cpu.mem[1] = 0;
        cpu.mem[2] = 0x01;

        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 0x100);

        // check zero flag set
        cpu.mem[0] = 0xca;
        cpu.mem[1] = 0;
        cpu.mem[2] = 0x01;
        cpu.pc = 0;

        cpu.status_set(StatusRegBit::Zero);
        cpu.tick().unwrap();

        assert_eq!(cpu.pc, 0x100);

        // check carry flag not set
        cpu.mem[0] = 0xd2;
        cpu.mem[1] = 0;
        cpu.mem[2] = 0x01;
        cpu.pc = 0;

        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 0x100);

        // check carry flag set
        cpu.mem[0] = 0xda;
        cpu.mem[1] = 0;
        cpu.mem[2] = 0x01;
        cpu.pc = 0;

        cpu.status_set(StatusRegBit::Carry);
        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 0x100);
    }
}
