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
        self.sp = 0xfffe;

        println!("{}", self);
    }

    //
    // Manage memory
    //

    pub fn set_mem(&mut self, i: usize, value: u8) {
        self.mem[i] = value
    }

    pub fn get_mem_range(&self, i: usize, j: usize) -> &[u8] {
        &self.mem[i..j + 1]
    }

    pub fn set_mem_range(&mut self, i: usize, j: usize, data: &[u8]) {
        self.mem[i..j + 1].copy_from_slice(data);
    }

    pub fn push_stack(&mut self, data: &[u8]) {
        let top = self.sp as usize;
        let bottom = top - data.len();

        self.set_mem_range(bottom + 1, top, data);

        self.sp = bottom as u16;
    }

    pub fn push_stack_u16(&mut self, n: u16) {
        let (b1, b2) = u16_to_u8(n);
        self.push_stack(&[b1, b2]);
    }

    pub fn pop_stack(&mut self, n: usize) -> &[u8] {
        let bottom = (self.sp as usize) + 1;
        let top = bottom + n - 1;

        self.sp = top as u16;

        self.get_mem_range(bottom, top)
    }

    pub fn pop_stack_u16(&mut self) -> u16 {
        let bytes = self.pop_stack(2);
        u8_to_u16(bytes[0], bytes[1])
    }

    //
    // Manage registers
    //

    fn get_af(&self) -> u16 {
        u8_to_u16(self.a, self.f)
    }

    fn set_af(&mut self, n: u16) {
        let (a, f) = u16_to_u8(n);
        self.a = a;
        self.f = f;
    }

    fn get_bc(&self) -> u16 {
        u8_to_u16(self.b, self.c)
    }

    fn set_bc(&mut self, n: u16) {
        let (b, c) = u16_to_u8(n);
        self.b = b;
        self.c = c;
    }

    fn get_de(&self) -> u16 {
        u8_to_u16(self.d, self.e)
    }

    fn set_de(&mut self, n: u16) {
        let (d, e) = u16_to_u8(n);
        self.d = d;
        self.e = e;
    }

    fn get_hl(&self) -> u16 {
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

    // Set the defined status flag
    fn status_reset(&mut self, bit_enum: StatusRegBit) {
        match bit_enum {
            StatusRegBit::Zero => self.status = 0,
            StatusRegBit::Sub => self.status = 0,
            StatusRegBit::HalfCarry => self.status = 0,
            StatusRegBit::Carry => self.status = 0,
        }
    }

    fn print_curr(&self) {
        self.print_instr(self.pc)
    }

    fn print_instr(&self, addr: u16) {
        let addr = addr as usize;
        let read_byte = || format!("{:02x}", self.mem[addr + 1]);
        let read_8_rel = || format!("r_{}", read_byte());
        let read_8_imm = || format!("d_{}", read_byte());
        let read_16_imm = || {
            let fst_byte = self.mem[addr + 1];
            let snd_byte = self.mem[addr + 2];

            format!("d_{:04x}", u8_to_u16(fst_byte, snd_byte))
        };
        let read_16_addr = || {
            let snd_byte = self.mem[addr + 1];
            let fst_byte = self.mem[addr + 2];

            format!("a_{:04x}", u8_to_u16(fst_byte, snd_byte))
        };

        let opcode = self.mem[addr];
        match opcode {
            opcodes::CALL_A16 => println!("CALL\t{}", read_16_addr()),

            opcodes::INC_BC => println!("INC\tBC"),
            opcodes::INC_DE => println!("INC\tDE"),
            opcodes::INC_HL => println!("INC\tHL"),
            opcodes::INC_SP => println!("INC\tSP"),

            opcodes::LD_BC_A => println!("LD\t(BC),A"),
            opcodes::LD_DE_A => println!("LD\t(DE),A"),
            opcodes::LD_HL_A => println!("LD\t(HL),A"),
            opcodes::LD_A16_A => println!("LD\t{},A", read_16_addr()),

            opcodes::LD_A_A => println!("LD\tA,A"),
            opcodes::LD_A_B => println!("LD\tA,B"),
            opcodes::LD_A_C => println!("LD\tA,C"),
            opcodes::LD_A_D => println!("LD\tA,D"),
            opcodes::LD_A_E => println!("LD\tA,E"),
            opcodes::LD_A_H => println!("LD\tA,H"),
            opcodes::LD_A_L => println!("LD\tA,L"),
            opcodes::LD_A_HL => println!("LD\tA,(HL)"),
            opcodes::LD_A_D8 => println!("LD\tA,{}", read_8_imm()),
            opcodes::LD_BC_D16 => println!("LD\tBC,{}", read_16_imm()),
            opcodes::LD_HL_D16 => println!("LD\tHL,{}", read_16_imm()),
            opcodes::LD_DE_D16 => println!("LD\tDE,{}", read_16_imm()),
            opcodes::LD_SP_D16 => println!("LD\tSP,{}", read_16_imm()),
            opcodes::LD_SP_HL => println!("LD\tSP,HL"),

            opcodes::LDH_A8_A => println!("LDH\ta_{},A", read_byte()),
            opcodes::LDI_A_HL => println!("LDI\tA,(HL)"),

            opcodes::JP_A16 => println!("JP\t{}", read_16_addr()),
            opcodes::JP_HL => println!("JP\tHL"),
            opcodes::JP_NZ_A16 => println!("JP\tNZ,{}", read_16_addr()),
            opcodes::JP_Z_A16 => println!("JP\tZ,{}", read_16_addr()),
            opcodes::JP_NC_A16 => println!("JP\tNC,{}", read_16_addr()),
            opcodes::JP_C_A16 => println!("JP\tC,{}", read_16_addr()),

            opcodes::JR_R8 => println!("JR\t{}", read_8_rel()),

            opcodes::PUSH_A16_AF => println!("PUSH\tAF"),
            opcodes::PUSH_A16_BC => println!("PUSH\tBC"),
            opcodes::PUSH_A16_DE => println!("PUSH\tDE"),
            opcodes::PUSH_A16_HL => println!("PUSH\tHL"),

            opcodes::POP_A16_AF => println!("POP\tAF"),
            opcodes::POP_A16_BC => println!("POP\tBC"),
            opcodes::POP_A16_DE => println!("POP\tDE"),
            opcodes::POP_A16_HL => println!("POP\tHL"),

            opcodes::RET => println!("RET"),

            opcodes::DI => println!("DI"),
            opcodes::NOP => println!("NOP"),
            opcodes::OR_A_A => println!("OR\tA,A"),
            opcodes::OR_A_B => println!("OR\tA,B"),
            opcodes::OR_A_C => println!("OR\tA,C"),
            opcodes::OR_A_D => println!("OR\tA,D"),
            opcodes::OR_A_E => println!("OR\tA,E"),
            opcodes::OR_A_H => println!("OR\tA,H"),
            opcodes::OR_A_L => println!("OR\tA,L"),
            opcodes::OR_A_HL => println!("OR\tA,HL"),
            opcodes::OR_A_D8 => println!("OR\tA,{}", read_8_imm()),

            n => println!("Unknown instruction {:02x}@{:04x}", n, addr),
        }
    }

    //
    // Tick
    //

    pub fn tick(&mut self) -> Result<()> {
        self.print_curr();

        let opcode = self.consume_byte();
        match opcode {
            opcodes::CALL_A16 => self.call_a16(),

            opcodes::DI => self.di(),

            opcodes::JP_A16 => self.jp_a16(),
            opcodes::JP_HL => self.jp_hl(),
            opcodes::JR_R8 => self.jr_r8(),
            opcodes::JP_C_A16 => self.jp_cc_a16(|cpu| cpu.status_is_set(StatusRegBit::Carry)),
            opcodes::JP_NC_A16 => self.jp_cc_a16(|cpu| !cpu.status_is_set(StatusRegBit::Carry)),
            opcodes::JP_Z_A16 => self.jp_cc_a16(|cpu| cpu.status_is_set(StatusRegBit::Zero)),
            opcodes::JP_NZ_A16 => self.jp_cc_a16(|cpu| !cpu.status_is_set(StatusRegBit::Zero)),

            opcodes::LD_BC_A => self.ld_addr_a(|cpu| cpu.get_bc()),
            opcodes::LD_DE_A => self.ld_addr_a(|cpu| cpu.get_de()),
            opcodes::LD_HL_A => self.ld_addr_a(|cpu| cpu.get_hl()),
            opcodes::LD_A16_A => self.ld_addr_a(|cpu| cpu.consume_16_addr()),

            opcodes::LD_A_D8 => self.ld_a(|cpu| cpu.consume_byte()),
            opcodes::LD_A_A => self.ld_a(|cpu| cpu.a),
            opcodes::LD_A_B => self.ld_a(|cpu| cpu.b),
            opcodes::LD_A_C => self.ld_a(|cpu| cpu.c),
            opcodes::LD_A_D => self.ld_a(|cpu| cpu.d),
            opcodes::LD_A_E => self.ld_a(|cpu| cpu.e),
            opcodes::LD_A_H => self.ld_a(|cpu| cpu.h),
            opcodes::LD_A_L => self.ld_a(|cpu| cpu.l),
            opcodes::LD_A_HL => self.ld_a_hl(),
            opcodes::LD_HL_D16 => self.ld_r16_d16(Cpu::set_hl),
            opcodes::LD_SP_HL => self.ld_sp_hl(),
            opcodes::LD_DE_D16 => self.ld_r16_d16(Cpu::set_de),
            opcodes::LD_BC_D16 => self.ld_r16_d16(Cpu::set_bc),
            opcodes::LD_SP_D16 => self.ld_r16_d16(|cpu, n| cpu.sp = n),

            opcodes::LDH_A8_A => self.ldh_a8_a(),
            opcodes::LDI_A_HL => self.ldi_a_hl(),
            opcodes::RET => self.ret(),

            opcodes::PUSH_A16_AF => self.push_a16(Cpu::get_af),
            opcodes::PUSH_A16_BC => self.push_a16(Cpu::get_bc),
            opcodes::PUSH_A16_DE => self.push_a16(Cpu::get_de),
            opcodes::PUSH_A16_HL => self.push_a16(Cpu::get_hl),

            opcodes::POP_A16_AF => self.pop_r16(Cpu::set_af),
            opcodes::POP_A16_BC => self.pop_r16(Cpu::set_bc),
            opcodes::POP_A16_DE => self.pop_r16(Cpu::set_de),
            opcodes::POP_A16_HL => self.pop_r16(Cpu::set_hl),

            opcodes::INC_BC => self.inc_r16(Cpu::get_bc, Cpu::set_bc),
            opcodes::INC_DE => self.inc_r16(Cpu::get_de, Cpu::set_de),
            opcodes::INC_HL => self.inc_r16(Cpu::get_hl, Cpu::set_hl),
            opcodes::INC_SP => self.inc_r16(|cpu| cpu.sp, |cpu, n| cpu.sp = n),

            opcodes::OR_A_A => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_B => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_C => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_D => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_E => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_H => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_L => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_HL => {
                let addr = self.get_hl() as usize;
                self.or_a(|cpu| cpu.mem[addr])
            }
            opcodes::OR_A_D8 => self.or_a_d8(),

            opcodes::NOP => self.nop(),
            s => {
                return Err(Error::new(
                    ErrorKind::UnknownInstruction,
                    format!(
                    "Unimplemented opcode {:02x}@{:04x}",
                    s,
                    self.pc - 1,
                ),
                ))
            }
        };

        println!("{}", self);

        Ok(())
    }

    fn consume_byte(&mut self) -> u8 {
        let result = self.mem[self.pc as usize];

        self.pc += 1;

        result
    }

    fn consume_16_imm(&mut self) -> u16 {
        let fst_byte = self.consume_byte();
        let snd_byte = self.consume_byte();

        u8_to_u16(fst_byte, snd_byte)
    }

    fn consume_16_addr(&mut self) -> u16 {
        let snd_byte = self.consume_byte();
        let fst_byte = self.consume_byte();

        u8_to_u16(fst_byte, snd_byte)
    }

    //
    // Opcodes
    //

    /// **Description**
    ///
    /// Push address of next instruction onto stack and then jump to address a16.
    ///
    /// **Use with**:
    ///
    /// a16 = two byte immediate value. (LS byte first)
    fn call_a16(&mut self) {
        let addr = self.consume_16_addr();

        // copy pc because self needs to be borrowed mutably
        // when pushing to the stack
        let pc = self.pc;
        self.push_stack_u16(pc);

        self.pc = addr;
    }

    /// **Description**
    ///
    /// Put value A into memory address nn.
    fn ld_addr_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u16,
    {
        let addr = f(self) as usize;
        self.mem[addr] = self.a;
    }

    /// **Description**
    ///
    /// Put a value into A.
    fn ld_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        self.a = f(self);
    }

    /// **Description**
    ///
    /// Put value at address stored in HL into A.
    fn ld_a_hl(&mut self) {
        let value = self.mem[self.get_hl() as usize];
        self.a = value;
    }

    /// **Description**
    ///
    /// Put d16 into register r16.
    fn ld_r16_d16<F>(&mut self, setter: F)
    where
        F: Fn(&mut Cpu, u16),
    {
        let value = self.consume_16_imm();
        setter(self, value);
    }

    /// **Description:**
    /// Jump to address nn.
    ///
    /// **Use with:**
    /// nn = two byte immediate value. (LS byte first.)
    ///
    fn jp_a16(&mut self) {
        let addr = self.consume_16_addr();
        self.pc = addr;
    }

    /// **Description:**
    /// Jump to address contained in HL.
    ///
    fn jp_hl(&mut self) {
        let addr = self.get_hl();
        self.pc = addr;
    }

    /// **Description:**
    ///
    ///  Add n to current address and jump to it.
    ///
    /// **Use with:**
    ///
    ///  n = one byte signed immediate value
    fn jr_r8(&mut self) {
        let n = self.consume_byte() as u16;
        self.pc += n;
    }

    /// **Description:**
    ///
    /// Put HL into Stack Pointer (SP).
    fn ld_sp_hl(&mut self) {
        self.sp = self.get_hl();
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
    fn di(&self) {
        // TODO implement this
    }

    fn nop(&self) {}

    ///**Description:**
    /// Jump to address n if following condition is true:
    /// cc = NZ, Jump if Z flag is reset
    /// cc = Z, Jump if Z flag is set
    /// cc = NC, Jump if C flag is reset
    /// cc = C, Jump if C flag is set
    ///
    ///**Use with:**
    /// nn = two byte immediate value. (LS byte first.)
    fn jp_cc_a16<F>(&mut self, condition: F)
    where
        F: Fn(&Cpu) -> bool,
    {
        let addr = self.consume_16_addr();

        if condition(&self) {
            self.pc = addr;
        }
    }

    ///**Description:**
    /// Put A into memory address $FF00+n.
    ///
    ///**Use with:**
    /// n = one byte immediate value.
    fn ldh_a8_a(&mut self) {
        let n = self.consume_byte() as usize;
        self.mem[MEM_HW_IO_REG_OFFSET + n] = self.a;
    }

    ///**Description:**
    ///
    /// Put value at address HL into A. Increment HL.
    /// Same as: LD A,(HL) - INC HL
    ///
    ///**Notes:**
    ///
    /// Implements LD A,(HLI) and LD,A(HLI+)
    fn ldi_a_hl(&mut self) {
        self.ld_a_hl();
        self.inc_r16(Cpu::get_hl, Cpu::set_hl);
    }

    ///**Description:**
    /// Pop two bytes from stack & jump to that address.
    fn ret(&mut self) {
        self.pc = self.pop_stack_u16();
    }

    ///
    /// Push register pair nn onto stack.
    /// Decrement Stack Pointer (SP) twice.
    ///
    ///**Use with:**
    /// nn = AF,BC,DE,HL
    fn push_a16<G>(&mut self, getter: G)
    where
        G: Fn(&Cpu) -> u16,
    {
        let reg_value = getter(self);
        self.push_stack_u16(reg_value);
    }

    ///**Description:
    /// Pop two bytes off stack into register pair nn.
    /// Increment Stack Pointer (SP) twice.
    ///
    ///**Use with:**
    /// nn = AF,BC,DE,HL
    fn pop_r16<F>(&mut self, setter: F)
    where
        F: Fn(&mut Cpu, u16),
    {
        let value = self.pop_stack_u16();
        setter(self, value);
    }

    ///**Description:**
    /// Increment register nn.
    ///
    ///**Use with:**
    /// nn = BC,DE,HL,SP
    fn inc_r16<G, S>(&mut self, getter: G, setter: S)
    where
        G: Fn(&Cpu) -> u16,
        S: Fn(&mut Cpu, u16),
    {
        let curr_value = getter(self);
        setter(self, curr_value + 1);
    }

    ///**Description:**
    ///  Logical OR n with register A, result in A.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL),#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Reset.
    ///  H - Reset.
    ///  C - Reset.
    fn or_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        self.status_reset(StatusRegBit::Zero);
        self.status_reset(StatusRegBit::Carry);
        self.status_reset(StatusRegBit::HalfCarry);
        self.status_reset(StatusRegBit::Sub);

        if (self.a | f(self)) == 0 {
            self.status_set(StatusRegBit::Zero);
        }
    }

    fn or_a_d8(&mut self) {
        self.status_reset(StatusRegBit::Zero);
        self.status_reset(StatusRegBit::Carry);
        self.status_reset(StatusRegBit::HalfCarry);
        self.status_reset(StatusRegBit::Sub);

        let value = self.consume_byte();
        if (self.a | value) == 0 {
            self.status_set(StatusRegBit::Zero);
        }
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

    #[test]
    fn test_push_stack() {
        let mut cpu = Cpu::new();
        cpu.sp = 0xfffe;

        cpu.push_stack(&[0xff, 0xee, 0xcc]);

        assert_eq!(0xcc, cpu.mem[0xfffe]);
        assert_eq!(0xee, cpu.mem[0xfffd]);
        assert_eq!(0xff, cpu.mem[0xfffc]);
        assert_eq!(0xfffb, cpu.sp);
    }

    #[test]
    fn test_push_stack_u16() {
        let mut cpu = Cpu::new();
        cpu.sp = 0xfffe;

        cpu.push_stack_u16(0xffee);

        assert_eq!(0xee, cpu.mem[0xfffe]);
        assert_eq!(0xff, cpu.mem[0xfffd]);
        assert_eq!(0xfffc, cpu.sp);
    }

    #[test]
    fn test_pop_stack() {
        let mut cpu = Cpu::new();
        cpu.sp = 0xfffb;

        cpu.mem[0xfffe] = 0xcc;
        cpu.mem[0xfffd] = 0xee;
        cpu.mem[0xfffc] = 0xff;

        {
            let result = cpu.pop_stack(3);
            assert_eq!([0xff, 0xee, 0xcc], result);
        }

        assert_eq!(0xfffe, cpu.sp);
    }

    #[test]
    fn test_pop_stack_u16() {
        let mut cpu = Cpu::new();
        cpu.sp = 0xfffc;

        cpu.mem[0xfffe] = 0xcc;
        cpu.mem[0xfffd] = 0xee;

        let result = cpu.pop_stack_u16();

        assert_eq!(0xeecc, result);
        assert_eq!(0xfffe, cpu.sp);
    }

    //
    // Instructions
    //

    #[test]
    fn test_ld_bc_a() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = opcodes::LD_BC_A;
        cpu.a = 0x72;
        cpu.set_bc(0x3401);

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x3401], 0x72);
    }

    #[test]
    fn test_ld_de_a() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = opcodes::LD_DE_A;
        cpu.a = 0x72;
        cpu.set_de(0x3401);

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x3401], 0x72);
    }

    #[test]
    fn test_ld_hl_a() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = opcodes::LD_HL_A;
        cpu.a = 0x72;
        cpu.set_hl(0x3401);

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x3401], 0x72);
    }

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
    fn test_ld_a_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0x72;
        cpu.mem[0] = opcodes::LD_A_A;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_b() {
        let mut cpu = Cpu::new();
        cpu.b = 0x72;
        cpu.mem[0] = opcodes::LD_A_B;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = Cpu::new();
        cpu.c = 0x72;
        cpu.mem[0] = opcodes::LD_A_C;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_d() {
        let mut cpu = Cpu::new();
        cpu.d = 0x72;
        cpu.mem[0] = opcodes::LD_A_D;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_e() {
        let mut cpu = Cpu::new();
        cpu.e = 0x72;
        cpu.mem[0] = opcodes::LD_A_E;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_h() {
        let mut cpu = Cpu::new();
        cpu.h = 0x72;
        cpu.mem[0] = opcodes::LD_A_H;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_l() {
        let mut cpu = Cpu::new();
        cpu.l = 0x72;
        cpu.mem[0] = opcodes::LD_A_L;

        cpu.tick().unwrap();
        assert_eq!(0x72, cpu.a);
    }

    #[test]
    fn test_ld_a_hl() {
        let mut cpu = Cpu::new();
        let addr = 0xb00b;
        cpu.mem[0] = opcodes::LD_A_HL;
        cpu.mem[addr as usize] = 5;
        cpu.set_hl(addr as u16);

        cpu.tick().unwrap();
        assert_eq!(cpu.a, cpu.mem[addr as usize]);
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
    fn test_jp_a16() {
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
    fn test_jp_r8() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = opcodes::JR_R8;
        cpu.mem[1] = 15;

        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 17);
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

    #[test]
    fn test_ldi_a_hl() {
        let mut cpu = Cpu::new();
        let addr = 0xb00b;
        cpu.mem[0] = opcodes::LDI_A_HL;
        cpu.mem[addr as usize] = 5;
        cpu.set_hl(addr as u16);

        cpu.tick().unwrap();
        assert_eq!(cpu.a, cpu.mem[addr as usize]);
        assert_eq!(cpu.get_hl(), 0xb00c);
    }


    #[test]
    fn test_ld_r16_d16() {
        let mut cpu = Cpu::new();
        cpu.mem[0] = opcodes::LD_HL_D16;
        cpu.mem[1] = 0x24;
        cpu.mem[2] = 0x35;

        cpu.tick().unwrap();
        assert_eq!(0x2435, cpu.get_hl());

        cpu.pc = 0;
        cpu.mem[0] = opcodes::LD_BC_D16;
        cpu.mem[1] = 0x24;
        cpu.mem[2] = 0x35;

        cpu.tick().unwrap();
        assert_eq!(0x2435, cpu.get_bc());

        cpu.pc = 0;
        cpu.mem[0] = opcodes::LD_DE_D16;
        cpu.mem[1] = 0x24;
        cpu.mem[2] = 0x35;

        cpu.tick().unwrap();
        assert_eq!(0x2435, cpu.get_de());

        cpu.pc = 0;
        cpu.mem[0] = opcodes::LD_SP_D16;
        cpu.mem[1] = 0x24;
        cpu.mem[2] = 0x35;

        cpu.tick().unwrap();
        assert_eq!(0x2435, cpu.sp);
    }

    #[test]
    fn test_call_a16() {
        let mut cpu = Cpu::new();
        cpu.pc = 0xff13;
        cpu.sp = 0xfffe;

        cpu.mem[0xff13] = opcodes::CALL_A16;
        cpu.mem[0xff14] = 0x24;
        cpu.mem[0xff15] = 0x35;

        cpu.tick().unwrap();

        assert_eq!(0xfffc, cpu.sp);
        assert_eq!(0x3524, cpu.pc);
        assert_eq!(0x16, cpu.mem[0xfffe]);
        assert_eq!(0xff, cpu.mem[0xfffd]);
    }

    #[test]
    fn test_ret() {
        let mut cpu = Cpu::new();

        cpu.mem[0x0] = opcodes::RET;
        cpu.mem[0xfffe] = 0x24;
        cpu.mem[0xfffd] = 0x35;
        cpu.sp = 0xfffc;

        cpu.tick().unwrap();

        assert_eq!(0xfffe, cpu.sp);
        assert_eq!(0x3524, cpu.pc);
    }
    fn test_push_16() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x1234;
        cpu.set_af(0xff15);
        cpu.set_bc(0xffff);
        cpu.set_de(0x1234);
        cpu.set_hl(0xfee2);

        cpu.mem[0] = opcodes::PUSH_A16_AF;
        cpu.mem[1] = opcodes::PUSH_A16_BC;
        cpu.mem[2] = opcodes::PUSH_A16_DE;
        cpu.mem[3] = opcodes::PUSH_A16_HL;

        cpu.tick().unwrap();

        assert_eq!(cpu.mem[0x1234], 0x15);
        assert_eq!(cpu.mem[0x1233], 0xff);

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x1232], 0xff);
        assert_eq!(cpu.mem[0x1231], 0xff);

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x1230], 0x34);
        assert_eq!(cpu.mem[0x122f], 0x12);

        cpu.tick().unwrap();
        assert_eq!(cpu.mem[0x122e], 0xe2);
        assert_eq!(cpu.mem[0x122d], 0xfe);
    }


    fn test_pop_a16() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x1234;
        cpu.push_stack_u16(0xff15);
        cpu.push_stack_u16(0xffff);
        cpu.push_stack_u16(0x1234);
        cpu.push_stack_u16(0xfee2);

        cpu.mem[0] = opcodes::POP_A16_AF;
        cpu.mem[1] = opcodes::POP_A16_BC;
        cpu.mem[2] = opcodes::POP_A16_DE;
        cpu.mem[3] = opcodes::POP_A16_HL;

        cpu.tick().unwrap();
        assert_eq!(cpu.get_af(), 0xff15);

        cpu.tick().unwrap();
        assert_eq!(cpu.get_bc(), 0xffff);

        cpu.tick().unwrap();
        assert_eq!(cpu.get_de(), 0x1234);

        cpu.tick().unwrap();
        assert_eq!(cpu.get_hl(), 0xfee2);
    }

    #[test]
    fn test_inc_r16() {
        let mut cpu = Cpu::new();
        cpu.set_bc(0xfff9);
        cpu.set_de(0x1234);
        cpu.set_hl(0xfee2);
        cpu.sp = 0;

        cpu.mem[0] = opcodes::INC_BC;
        cpu.mem[1] = opcodes::INC_DE;
        cpu.mem[2] = opcodes::INC_HL;
        cpu.mem[3] = opcodes::INC_SP;

        cpu.tick().unwrap();
        assert_eq!(cpu.get_bc(), 0xfffa);

        cpu.tick().unwrap();
        assert_eq!(cpu.get_de(), 0x1235);

        cpu.tick().unwrap();
        assert_eq!(cpu.get_hl(), 0xfee3);

        cpu.tick().unwrap();
        assert_eq!(cpu.sp, 1);
    }

    #[test]
    fn test_or_a_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000000;
        cpu.mem[0] = opcodes::OR_A_A;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_A;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_b() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000000;
        cpu.b = 0b10000000;
        cpu.mem[0] = opcodes::OR_A_B;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.c = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_B;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_c() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000011;
        cpu.b = 0b01000000;
        cpu.mem[0] = opcodes::OR_A_C;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.c = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_C;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_d() {
        let mut cpu = Cpu::new();
        cpu.a = 0xff;
        cpu.b = 0b10000000;
        cpu.mem[0] = opcodes::OR_A_D;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.d = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_D;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_e() {
        let mut cpu = Cpu::new();
        cpu.a = 0xff;
        cpu.e = 0b10000000;
        cpu.mem[0] = opcodes::OR_A_E;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.e = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_E;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_h() {
        let mut cpu = Cpu::new();
        cpu.a = 10;
        cpu.h = 0b10000000;
        cpu.mem[0] = opcodes::OR_A_H;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.h = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_H;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_l() {
        let mut cpu = Cpu::new();
        cpu.a = 0xf;
        cpu.b = 0b10000000;
        cpu.mem[0] = opcodes::OR_A_L;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.d = 0b00000000;
        cpu.mem[1] = opcodes::OR_A_L;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_hl() {
        let mut cpu = Cpu::new();
        cpu.a = 0xff;
        cpu.set_hl(0xffe1);
        cpu.mem[0] = opcodes::OR_A_HL;
        cpu.mem[0xffe1] = 0b11000000;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.set_hl(0xffe1);
        cpu.mem[1] = opcodes::OR_A_HL;
        cpu.mem[0xffe1] = 0;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }

    #[test]
    fn test_or_a_d8() {
        let mut cpu = Cpu::new();
        cpu.a = 0xff;
        cpu.mem[0] = opcodes::OR_A_D8;
        cpu.mem[1] = 0xf1;

        cpu.tick().unwrap();
        assert!(!cpu.status_is_set(StatusRegBit::Zero));

        cpu.a = 0b00000000;
        cpu.mem[2] = opcodes::OR_A_D8;
        cpu.mem[3] = 0;

        cpu.tick().unwrap();
        assert!(cpu.status_is_set(StatusRegBit::Zero));
    }
}
