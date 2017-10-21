#![allow(dead_code)]

mod opcodes;

use std::fmt;

use {u8_to_u16, u16_to_u8};
use errors::{Error, ErrorKind, Result};

const MEM_SIZE: usize = 64 * 1024;

//
// Memory offsets
//

const MEM_HW_IO_REG_OFFSET: usize = 0xff00;

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

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
                      "CPU [
    a: {:02x},
    b: {:02x},
    c: {:02x},
    d: {:02x},
    e: {:02x},
    f: {:02x},
    h: {:02x},
    l: {:02x},
    status: {:02x},
    sp: {:04x},
    pc: {:04x},
]",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.h,
            self.l,
            self.status,
            self.sp,
            self.pc,
        );
    }
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

        println!("{}", self);
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

    fn get_af(&mut self) -> u16 {
        u8_to_u16(self.a, self.f)
    }

    fn set_af(&mut self, n: u16) {
        let (a, f) = u16_to_u8(n);
        self.a = a;
        self.f = f;
    }

    fn get_bc(&mut self) -> u16 {
        u8_to_u16(self.b, self.c)
    }

    fn set_bc(&mut self, n: u16) {
        let (b, c) = u16_to_u8(n);
        self.b = b;
        self.c = c;
    }

    fn get_de(&mut self) -> u16 {
        u8_to_u16(self.d, self.e)
    }

    fn set_de(&mut self, n: u16) {
        let (d, e) = u16_to_u8(n);
        self.d = d;
        self.e = e;
    }

    fn get_hl(&mut self) -> u16 {
        u8_to_u16(self.h, self.l)
    }

    fn set_hl(&mut self, n: u16) {
        let (h, l) = u16_to_u8(n);
        self.h = h;
        self.l = l;
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
        let result = match opcode {
            opcodes::DI => self.di(),

            opcodes::JP_A16 => self.jp_a16(),
            opcodes::JP_HL => self.jp_hl(),
            opcodes::JP_C_A16 => self.jp_cc_a16(|cpu| cpu.status_is_set(StatusRegBit::Carry)),
            opcodes::JP_NC_A16 => self.jp_cc_a16(|cpu| !cpu.status_is_set(StatusRegBit::Carry)),
            opcodes::JP_Z_A16 => self.jp_cc_a16(|cpu| cpu.status_is_set(StatusRegBit::Zero)),
            opcodes::JP_NZ_A16 => self.jp_cc_a16(|cpu| !cpu.status_is_set(StatusRegBit::Zero)),

            opcodes::LD_A16_A => self.ld_a16_a(),
            opcodes::LD_A_D8 => self.ld_a_d8(),
            opcodes::LD_SP_HL => self.ld_sp_hl(),
            opcodes::LD_SP_NN => self.ld_sp_nn(),

            opcodes::LDH_A8_A => self.ldh_a8_a(),

            opcodes::NOP => self.nop(),
            s => Err(Error::new(
                ErrorKind::UnknownInstruction,
                format!(
                    "Unimplemented opcode {:02x}@{:04x}",
                    s,
                    self.pc - 1,
                ),
            )),
        };

        println!("{}", self);

        result
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
    /// Put value A into nn.
    ///
    /// **Use with**:
    ///
    /// nn = two byte immediate value. (LS byte first)
    fn ld_a16_a(&mut self) -> Result<()> {
        let snd_byte = self.get_next();
        let fst_byte = self.get_next();

        let addr = u8_to_u16(fst_byte, snd_byte) as usize;
        self.mem[addr] = self.a;

        println!("LD\t{:04x},A", addr);
        Ok(())
    }

    /// **Description**
    ///
    /// Put value d8 into A.
    ///
    /// **Use with**:
    ///
    /// d8 = one byte immediate value.
    fn ld_a_d8(&mut self) -> Result<()> {
        let n = self.get_next();

        self.a = n;

        println!("LD\tA,{:02x}", n);
        Ok(())
    }

    /// **Description**
    ///
    /// Put nn into Stack Pointer (SP).
    fn ld_sp_nn(&mut self) -> Result<()> {
        let fst_byte = self.get_next();
        let snd_byte = self.get_next();

        let addr = u8_to_u16(fst_byte, snd_byte);
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
    fn jp_a16(&mut self) -> Result<()> {
        let snd_byte = self.get_next();
        let fst_byte = self.get_next();

        let addr = u8_to_u16(fst_byte, snd_byte);
        self.pc = addr;

        println!("JP\t{:04x}", addr);
        Ok(())
    }

    /// **Description:**
    /// Jump to address contained in HL.
    ///
    fn jp_hl(&mut self) -> Result<()> {
        let addr = self.get_hl();
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
    fn jp_cc_a16<F>(&mut self, condition: F) -> Result<()>
    where
        F: Fn(&Cpu) -> bool,
    {
        let addr_snd_byte = self.get_next();
        let addr_fst_byte = self.get_next();

        let addr = u8_to_u16(addr_fst_byte, addr_snd_byte);

        if condition(&self) {
            self.pc = addr;
        }

        println!("JP cc\t{:04x}", addr);
        Ok(())
    }

    ///**Description:**
    /// Put A into memory address $FF00+n.
    ///
    ///**Use with:**
    /// n = one byte immediate value.
    fn ldh_a8_a(&mut self) -> Result<()> {
        let n = self.get_next() as usize;

        self.mem[MEM_HW_IO_REG_OFFSET + n] = self.a;

        println!("LDH\t{:02x},A", n);
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
    fn test_get_af() {
        let mut cpu = Cpu::new();
        cpu.a = 0x3f;
        cpu.f = 0x7c;

        assert_eq!(cpu.get_af(), 0x3f7c);
    }

    #[test]
    fn test_set_af() {
        let mut cpu = Cpu::new();
        cpu.set_af(0x3f7c);

        assert_eq!(cpu.a, 0x3f);
        assert_eq!(cpu.f, 0x7c);
    }

    #[test]
    fn test_get_bc() {
        let mut cpu = Cpu::new();
        cpu.b = 0x3f;
        cpu.c = 0x7c;

        assert_eq!(cpu.get_bc(), 0x3f7c);
    }

    #[test]
    fn test_set_bc() {
        let mut cpu = Cpu::new();
        cpu.set_bc(0x3f7c);

        assert_eq!(cpu.b, 0x3f);
        assert_eq!(cpu.c, 0x7c);
    }

    #[test]
    fn test_get_de() {
        let mut cpu = Cpu::new();
        cpu.d = 0x3f;
        cpu.e = 0x7c;

        assert_eq!(cpu.get_de(), 0x3f7c);
    }

    #[test]
    fn test_set_de() {
        let mut cpu = Cpu::new();
        cpu.set_de(0x3f7c);

        assert_eq!(cpu.d, 0x3f);
        assert_eq!(cpu.e, 0x7c);
    }

    #[test]
    fn test_get_hl() {
        let mut cpu = Cpu::new();
        cpu.h = 0x3f;
        cpu.l = 0x7c;

        assert_eq!(cpu.get_hl(), 0x3f7c);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x3f7c);

        assert_eq!(cpu.h, 0x3f);
        assert_eq!(cpu.l, 0x7c);
    }

    //
    // Instructions
    //

    #[test]
    fn test_ld_a16_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0x72;
        cpu.mem[0] = opcodes::LD_A16_A;
        cpu.mem[1] = 0x01;
        cpu.mem[2] = 0x34;

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x3401], 0x72);
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
    fn test_jp_hl() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = 0xe9;
        cpu.set_hl(0x0134);

        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 0x0134);
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

    #[test]
    fn test_jp_cc_a16() {
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

    #[test]
    fn test_ldh_a8_a() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = opcodes::LDH_A8_A;
        cpu.mem[1] = 0x04;
        cpu.a = 0x54;


        cpu.tick().unwrap();
        assert_eq!(0x54, cpu.mem[MEM_HW_IO_REG_OFFSET + 0x04]);
    }

}
