mod opcodes;
mod printer;
mod test;

use std::fmt;

use self::printer::Printer;
use errors::{Error, ErrorKind, Result};
use {u16_to_u8, u8_to_u16};

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
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    status: u8, // status flag: sign, zero, parity, carry, aux carry
    mem: [u8; MEM_SIZE],
}

impl Clone for Cpu {
    fn clone(&self) -> Cpu {
        let mut clone = Cpu::new();
        clone.a = self.a;
        clone.b = self.b;
        clone.c = self.c;
        clone.d = self.d;
        clone.e = self.e;
        clone.h = self.h;
        clone.l = self.l;
        clone.sp = self.sp;
        clone.pc = self.pc;
        clone.status = self.status;
        clone.mem.clone_from_slice(&self.mem);

        clone
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "[a: {:02x}, b: {:02x}, c: {:02x}, d: {:02x}, e: {:02x}, h: {:02x}, l: {:02x}, status: {:08b}, sp: {:04x}, pc: {:04x}]",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.status, self.sp, self.pc,
        );
    }
}

pub enum Flag {
    Zero,
    Sub,
    HalfCarry,
    Carry,
}

impl Flag {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn mask(&self) -> u8 {
        match *self {
            Flag::Zero =>      0b1000_0000,
            Flag::Sub =>       0b0100_0000,
            Flag::HalfCarry => 0b0010_0000,
            Flag::Carry =>     0b0001_0000,
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
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

    /// Set this CPU's state to the given cpu
    pub fn load_from(&mut self, cpu: &Cpu) {
        self.a = cpu.a;
        self.b = cpu.b;
        self.c = cpu.c;
        self.d = cpu.d;
        self.e = cpu.e;
        self.h = cpu.h;
        self.l = cpu.l;
        self.sp = cpu.sp;
        self.pc = cpu.pc;
        self.status = cpu.status;
        self.mem.clone_from_slice(&cpu.mem);
    }

    //
    // Manage memory
    //

    pub fn set_mem(&mut self, i: usize, value: u8) {
        self.mem[i] = value
    }

    pub fn get_mem_range(&self, i: usize, j: usize) -> &[u8] {
        &self.mem[i..=j]
    }

    pub fn set_mem_range(&mut self, i: usize, j: usize, data: &[u8]) {
        self.mem[i..=j].copy_from_slice(data);
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
        u8_to_u16(self.a, self.status)
    }

    fn set_af(&mut self, n: u16) {
        let (a, f) = u16_to_u8(n);
        self.a = a;
        self.status = f;
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

    //
    // Flags
    //

    /// Check if a certain flag is set
    fn flag(&self, flag: &Flag) -> bool {
        (self.status & flag.mask()) > 0
    }

    /// Either set or reset a flag based on `value`
    fn set_flag_to(&mut self, flag: &Flag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.reset_flag(flag);
        }
    }

    /// Set the defined status flag
    fn set_flag(&mut self, flag: &Flag) {
        self.status |= flag.mask()
    }

    /// Reset the defined status flag
    fn reset_flag(&mut self, flag: &Flag) {
        self.status &= !flag.mask()
    }

    fn print_curr(&self) {
        self.print_instr(self.pc)
    }

    fn print_instr(&self, addr: u16) {
        Printer::new(self, addr).print_instr(addr)
    }

    //
    // Tick
    //

    pub fn tick(&mut self) -> Result<()> {
        self.print_curr();
        if self.peek_byte() == opcodes::PREFIX_CB {
            self.handle_cbprefixed()
        } else {
            self.handle_unprefixed()
        }
    }

    pub fn handle_unprefixed(&mut self) -> Result<()> {
        let opcode = self.consume_byte();

        print!("{:04x} - {:02x} ", self.pc, opcode);
        println!("{}", self);

        match opcode {
            opcodes::CALL_A16 => self.call_a16(),
            opcodes::CALL_NZ_A16 => self.call_cc_a16(|cpu| !cpu.flag(&Flag::Zero)),
            opcodes::CALL_Z_A16 => self.call_cc_a16(|cpu| cpu.flag(&Flag::Zero)),
            opcodes::CALL_NC_A16 => self.call_cc_a16(|cpu| !cpu.flag(&Flag::Carry)),
            opcodes::CALL_C_A16 => self.call_cc_a16(|cpu| cpu.flag(&Flag::Carry)),

            opcodes::DI => self.di(),

            opcodes::JP_A16 => self.jp_a16(),
            opcodes::JP_HL => self.jp_hl(),
            opcodes::JR_R8 => self.jr_r8(),

            opcodes::JP_C_A16 => self.jp_cc_a16(|cpu| cpu.flag(&Flag::Carry)),
            opcodes::JP_NC_A16 => self.jp_cc_a16(|cpu| !cpu.flag(&Flag::Carry)),
            opcodes::JP_Z_A16 => self.jp_cc_a16(|cpu| cpu.flag(&Flag::Zero)),
            opcodes::JP_NZ_A16 => self.jp_cc_a16(|cpu| !cpu.flag(&Flag::Zero)),

            opcodes::JR_NZ_R8 => self.jr_cc_r8(|cpu| !cpu.flag(&Flag::Zero)),
            opcodes::JR_Z_R8 => self.jr_cc_r8(|cpu| cpu.flag(&Flag::Zero)),
            opcodes::JR_NC_R8 => self.jr_cc_r8(|cpu| !cpu.flag(&Flag::Carry)),
            opcodes::JR_C_R8 => self.jr_cc_r8(|cpu| cpu.flag(&Flag::Carry)),

            opcodes::LD_BC_A => self.ld_addr_r8(Cpu::get_bc, |cpu| cpu.a),
            opcodes::LD_HL_A => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.a),
            opcodes::LD_DE_A => self.ld_addr_r8(Cpu::get_de, |cpu| cpu.a),
            opcodes::LD_A16_A => {
                let addr = self.consume_16_addr();
                self.ld_addr_a(|_| addr);
            }

            opcodes::LD_A_D8 => self.ld_a(|cpu| cpu.consume_byte()),
            opcodes::LD_A_A => self.ld_r8_r8(|cpu| cpu.a, |cpu, n| cpu.a = n),
            opcodes::LD_A_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.a = n),
            opcodes::LD_A_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.a = n),
            opcodes::LD_A_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.a = n),
            opcodes::LD_A_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.a = n),
            opcodes::LD_A_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.a = n),
            opcodes::LD_A_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.a = n),
            opcodes::LD_A_BC => self.ld_r8_r16(Cpu::get_bc, |cpu, n| cpu.a = n),
            opcodes::LD_A_DE => self.ld_r8_r16(Cpu::get_de, |cpu, n| cpu.a = n),
            opcodes::LD_A_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.a = n),
            opcodes::LD_A_A16 => self.ld_r8_a16(|cpu, n| cpu.a = n),

            opcodes::LD_B_A => self.ld_r8_r8(|cpu| cpu.a, |cpu, n| cpu.b = n),
            opcodes::LD_B_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.b = n),
            opcodes::LD_B_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.b = n),
            opcodes::LD_B_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.b = n),
            opcodes::LD_B_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.b = n),
            opcodes::LD_B_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.b = n),
            opcodes::LD_B_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.b = n),
            opcodes::LD_B_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.b = n),

            opcodes::LD_C_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.c = n),
            opcodes::LD_C_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.c = n),
            opcodes::LD_C_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.c = n),
            opcodes::LD_C_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.c = n),
            opcodes::LD_C_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.c = n),
            opcodes::LD_C_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.c = n),
            opcodes::LD_C_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.c = n),

            opcodes::LD_D_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.d = n),
            opcodes::LD_D_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.d = n),
            opcodes::LD_D_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.d = n),
            opcodes::LD_D_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.d = n),
            opcodes::LD_D_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.d = n),
            opcodes::LD_D_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.d = n),
            opcodes::LD_D_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.d = n),

            opcodes::LD_E_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.e = n),
            opcodes::LD_E_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.e = n),
            opcodes::LD_E_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.e = n),
            opcodes::LD_E_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.e = n),
            opcodes::LD_E_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.e = n),
            opcodes::LD_E_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.e = n),
            opcodes::LD_E_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.e = n),

            opcodes::LD_H_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.h = n),
            opcodes::LD_H_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.h = n),
            opcodes::LD_H_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.h = n),
            opcodes::LD_H_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.h = n),
            opcodes::LD_H_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.h = n),
            opcodes::LD_H_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.h = n),
            opcodes::LD_H_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.h = n),
            opcodes::LD_H_A => self.ld_r8_r8(|cpu| cpu.a, |cpu, n| cpu.h = n),

            opcodes::LD_L_B => self.ld_r8_r8(|cpu| cpu.b, |cpu, n| cpu.l = n),
            opcodes::LD_L_C => self.ld_r8_r8(|cpu| cpu.c, |cpu, n| cpu.l = n),
            opcodes::LD_L_D => self.ld_r8_r8(|cpu| cpu.d, |cpu, n| cpu.l = n),
            opcodes::LD_L_E => self.ld_r8_r8(|cpu| cpu.e, |cpu, n| cpu.l = n),
            opcodes::LD_L_H => self.ld_r8_r8(|cpu| cpu.h, |cpu, n| cpu.l = n),
            opcodes::LD_L_L => self.ld_r8_r8(|cpu| cpu.l, |cpu, n| cpu.l = n),
            opcodes::LD_L_HL => self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.l = n),
            opcodes::LD_L_A => self.ld_r8_r8(|cpu| cpu.a, |cpu, n| cpu.l = n),

            opcodes::LD_HL_B => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.b),
            opcodes::LD_HL_C => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.c),
            opcodes::LD_HL_D => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.d),
            opcodes::LD_HL_E => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.e),
            opcodes::LD_HL_H => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.h),
            opcodes::LD_HL_L => self.ld_addr_r8(Cpu::get_hl, |cpu| cpu.l),

            opcodes::LD_A_FF00C => self.ld_a(|cpu| cpu.mem[(0xff00 + u16::from(cpu.c)) as usize]),
            opcodes::LD_FF00C_A => self.ld_addr_a(|cpu| 0xff00 + u16::from(cpu.c)),

            opcodes::LD_HL_D8 => {
                let value = self.consume_byte();
                self.ld_addr_r8(Cpu::get_hl, |_| value);
            }
            opcodes::LD_HL_D16 => self.ld_r16_d16(Cpu::set_hl),
            opcodes::LD_SP_HL => self.ld_sp_hl(),
            opcodes::LD_DE_D16 => self.ld_r16_d16(Cpu::set_de),
            opcodes::LD_BC_D16 => self.ld_r16_d16(Cpu::set_bc),
            opcodes::LD_SP_D16 => self.ld_r16_d16(|cpu, n| cpu.sp = n),

            opcodes::LDH_A8_A => self.ldh_a8_a(),
            opcodes::LDH_A_A8 => self.ldh_a_a8(),
            opcodes::LDI_A_HL => self.ldi_a_hl(),
            opcodes::LDI_HL_A => self.ldi_hl_a(),
            opcodes::LDD_HL_A => self.ldd_hl_a(),
            opcodes::LDD_A_HL => self.ldd_a_hl(),

            opcodes::RET => self.ret(),
            opcodes::RET_NZ => self.ret_cc(|cpu| !cpu.flag(&Flag::Zero)),
            opcodes::RET_Z => self.ret_cc(|cpu| cpu.flag(&Flag::Zero)),
            opcodes::RET_NC => self.ret_cc(|cpu| !cpu.flag(&Flag::Carry)),
            opcodes::RET_C => self.ret_cc(|cpu| cpu.flag(&Flag::Carry)),

            opcodes::PUSH_A16_AF => self.push_a16(Cpu::get_af),
            opcodes::PUSH_A16_BC => self.push_a16(Cpu::get_bc),
            opcodes::PUSH_A16_DE => self.push_a16(Cpu::get_de),
            opcodes::PUSH_A16_HL => self.push_a16(Cpu::get_hl),

            opcodes::POP_A16_AF => self.pop_r16(Cpu::set_af),
            opcodes::POP_A16_BC => self.pop_r16(Cpu::set_bc),
            opcodes::POP_A16_DE => self.pop_r16(Cpu::set_de),
            opcodes::POP_A16_HL => self.pop_r16(Cpu::set_hl),

            opcodes::INC_A => self.inc_r8(|cpu| cpu.a, |cpu, n| cpu.a = n),
            opcodes::INC_B => self.inc_r8(|cpu| cpu.b, |cpu, n| cpu.b = n),
            opcodes::INC_C => self.inc_r8(|cpu| cpu.c, |cpu, n| cpu.c = n),
            opcodes::INC_D => self.inc_r8(|cpu| cpu.d, |cpu, n| cpu.d = n),
            opcodes::INC_E => self.inc_r8(|cpu| cpu.e, |cpu, n| cpu.e = n),
            opcodes::INC_H => self.inc_r8(|cpu| cpu.h, |cpu, n| cpu.h = n),
            opcodes::INC_L => self.inc_r8(|cpu| cpu.l, |cpu, n| cpu.l = n),
            opcodes::INC_AHL => self.inc_addr(Cpu::get_hl),
            opcodes::INC_BC => self.inc_r16(Cpu::get_bc, Cpu::set_bc),
            opcodes::INC_DE => self.inc_r16(Cpu::get_de, Cpu::set_de),
            opcodes::INC_HL => self.inc_r16(Cpu::get_hl, Cpu::set_hl),
            opcodes::INC_SP => self.inc_r16(|cpu| cpu.sp, |cpu, n| cpu.sp = n),

            opcodes::DEC_A => self.dec_r8(|cpu| cpu.a, |cpu, n| cpu.a = n),
            opcodes::DEC_B => self.dec_r8(|cpu| cpu.b, |cpu, n| cpu.b = n),
            opcodes::DEC_C => self.dec_r8(|cpu| cpu.c, |cpu, n| cpu.c = n),
            opcodes::DEC_D => self.dec_r8(|cpu| cpu.d, |cpu, n| cpu.d = n),
            opcodes::DEC_E => self.dec_r8(|cpu| cpu.e, |cpu, n| cpu.e = n),
            opcodes::DEC_H => self.dec_r8(|cpu| cpu.h, |cpu, n| cpu.h = n),
            opcodes::DEC_L => self.dec_r8(|cpu| cpu.l, |cpu, n| cpu.l = n),
            opcodes::DEC_AHL => self.dec_addr(Cpu::get_hl),
            opcodes::DEC_BC => self.dec_r16(Cpu::get_bc, Cpu::set_bc),
            opcodes::DEC_DE => self.dec_r16(Cpu::get_de, Cpu::set_de),
            opcodes::DEC_HL => self.dec_r16(Cpu::get_hl, Cpu::set_hl),
            opcodes::DEC_SP => self.dec_r16(|cpu| cpu.sp, |cpu, n| cpu.sp = n),

            opcodes::ADD_A_A => self.add_a(|cpu| cpu.a),
            opcodes::ADD_A_B => self.add_a(|cpu| cpu.b),
            opcodes::ADD_A_C => self.add_a(|cpu| cpu.c),
            opcodes::ADD_A_D => self.add_a(|cpu| cpu.d),
            opcodes::ADD_A_E => self.add_a(|cpu| cpu.e),
            opcodes::ADD_A_H => self.add_a(|cpu| cpu.h),
            opcodes::ADD_A_L => self.add_a(|cpu| cpu.l),
            opcodes::ADD_A_HL => {
                let addr = self.get_hl() as usize;
                self.add_a(|cpu| cpu.mem[addr])
            }
            opcodes::ADD_A_D8 => self.add_a(|cpu| cpu.consume_byte()),
            opcodes::ADD_HL_BC => self.add_hl(Cpu::get_bc),
            opcodes::ADD_HL_DE => self.add_hl(Cpu::get_de),
            opcodes::ADD_HL_HL => self.add_hl(Cpu::get_hl),
            opcodes::ADD_HL_SP => self.add_hl(|cpu| cpu.sp),
            opcodes::ADD_SP_R8 => self.add_sp_imm(),

            opcodes::SUB_A_A => self.sub_a(|cpu| cpu.a),
            opcodes::SUB_A_B => self.sub_a(|cpu| cpu.b),
            opcodes::SUB_A_C => self.sub_a(|cpu| cpu.c),
            opcodes::SUB_A_D => self.sub_a(|cpu| cpu.d),
            opcodes::SUB_A_E => self.sub_a(|cpu| cpu.e),
            opcodes::SUB_A_H => self.sub_a(|cpu| cpu.h),
            opcodes::SUB_A_L => self.sub_a(|cpu| cpu.l),
            opcodes::SUB_A_HL => {
                let subr = self.get_hl() as usize;
                self.sub_a(|cpu| cpu.mem[subr])
            }
            opcodes::SUB_A_D8 => self.sub_a(|cpu| cpu.consume_byte()),

            opcodes::AND_A_A => self.and_a(|cpu| cpu.a),
            opcodes::AND_A_B => self.and_a(|cpu| cpu.b),
            opcodes::AND_A_C => self.and_a(|cpu| cpu.c),
            opcodes::AND_A_D => self.and_a(|cpu| cpu.d),
            opcodes::AND_A_E => self.and_a(|cpu| cpu.e),
            opcodes::AND_A_H => self.and_a(|cpu| cpu.h),
            opcodes::AND_A_L => self.and_a(|cpu| cpu.l),
            opcodes::AND_A_HL => {
                let addr = self.get_hl() as usize;
                self.and_a(|cpu| cpu.mem[addr])
            }
            opcodes::AND_A_D8 => self.and_a(|cpu| cpu.consume_byte()),

            opcodes::XOR_A_A => self.xor_a(|cpu| cpu.a),
            opcodes::XOR_A_B => self.xor_a(|cpu| cpu.b),
            opcodes::XOR_A_C => self.xor_a(|cpu| cpu.c),
            opcodes::XOR_A_D => self.xor_a(|cpu| cpu.d),
            opcodes::XOR_A_E => self.xor_a(|cpu| cpu.e),
            opcodes::XOR_A_H => self.xor_a(|cpu| cpu.h),
            opcodes::XOR_A_L => self.xor_a(|cpu| cpu.l),
            opcodes::XOR_A_HL => {
                let addr = self.get_hl() as usize;
                self.xor_a(|cpu| cpu.mem[addr])
            }
            opcodes::XOR_A_D8 => self.xor_a(|cpu| cpu.consume_byte()),

            opcodes::OR_A_A => self.or_a(|cpu| cpu.a),
            opcodes::OR_A_B => self.or_a(|cpu| cpu.b),
            opcodes::OR_A_C => self.or_a(|cpu| cpu.c),
            opcodes::OR_A_D => self.or_a(|cpu| cpu.d),
            opcodes::OR_A_E => self.or_a(|cpu| cpu.e),
            opcodes::OR_A_H => self.or_a(|cpu| cpu.h),
            opcodes::OR_A_L => self.or_a(|cpu| cpu.l),
            opcodes::OR_A_HL => {
                let addr = self.get_hl() as usize;
                self.or_a(|cpu| cpu.mem[addr])
            }
            opcodes::OR_A_D8 => self.or_a(|cpu| cpu.consume_byte()),

            opcodes::LD_B_D8 => self.ld_r8_d8(|cpu, n| cpu.b = n),
            opcodes::LD_C_D8 => self.ld_r8_d8(|cpu, n| cpu.c = n),
            opcodes::LD_D_D8 => self.ld_r8_d8(|cpu, n| cpu.d = n),
            opcodes::LD_E_D8 => self.ld_r8_d8(|cpu, n| cpu.e = n),
            opcodes::LD_H_D8 => self.ld_r8_d8(|cpu, n| cpu.h = n),
            opcodes::LD_L_D8 => self.ld_r8_d8(|cpu, n| cpu.l = n),

            opcodes::CP_A => self.cp_a(|cpu| cpu.a),
            opcodes::CP_B => self.cp_a(|cpu| cpu.b),
            opcodes::CP_C => self.cp_a(|cpu| cpu.c),
            opcodes::CP_D => self.cp_a(|cpu| cpu.d),
            opcodes::CP_E => self.cp_a(|cpu| cpu.e),
            opcodes::CP_H => self.cp_a(|cpu| cpu.h),
            opcodes::CP_L => self.cp_a(|cpu| cpu.l),
            opcodes::CP_HL => self.cp_hl(),
            opcodes::CP_D8 => self.cp_a(Cpu::consume_byte),

            opcodes::ADC_A_A => self.adc_a(|cpu| cpu.a),
            opcodes::ADC_A_B => self.adc_a(|cpu| cpu.b),
            opcodes::ADC_A_C => self.adc_a(|cpu| cpu.c),
            opcodes::ADC_A_D => self.adc_a(|cpu| cpu.d),
            opcodes::ADC_A_E => self.adc_a(|cpu| cpu.e),
            opcodes::ADC_A_H => self.adc_a(|cpu| cpu.h),
            opcodes::ADC_A_L => self.adc_a(|cpu| cpu.l),
            opcodes::ADC_A_HL => {
                let addr = self.get_hl() as usize;
                self.adc_a(|cpu| cpu.mem[addr])
            }
            opcodes::ADC_A_D8 => self.adc_a(|cpu| cpu.consume_byte()),
            opcodes::RLCA => self.rlc_a(),
            opcodes::RRCA => self.rrca(),

            opcodes::LDHL_SP_R8 => self.ldhl_sp_r8(),
            opcodes::LD_A16_SP => self.ld_a16_sp(),

            opcodes::RST_00 => self.rst_a8(0),
            opcodes::RST_08 => self.rst_a8(0x08),
            opcodes::RST_10 => self.rst_a8(0x10),
            opcodes::RST_18 => self.rst_a8(0x18),
            opcodes::RST_20 => self.rst_a8(0x20),
            opcodes::RST_28 => self.rst_a8(0x28),
            opcodes::RST_30 => self.rst_a8(0x30),
            opcodes::RST_38 => self.rst_a8(0x38),

            opcodes::EI => self.ei(),

            opcodes::NOP => self.nop(),

            // unimplemented instructions, do nothing
            // FIXME after testing this should panic instead
            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {}

            s => {
                return Err(Error::new(
                    ErrorKind::UnknownInstruction,
                    format!("Unimplemented opcode {:02x}@{:04x}", s, self.pc - 1,),
                ))
            }
        };

        Ok(())
    }

    pub fn handle_cbprefixed(&mut self) -> Result<()> {
        self.consume_byte(); // Consume the cb prefix
        let opcode = self.consume_byte();

        print!("{:04x} - {:02x} ", self.pc, opcode);
        println!("{}", self);

        match opcode {
            opcodes::SRL_A => self.srl(|cpu| cpu.a, |cpu, n| cpu.a = n),
            opcodes::SRL_B => self.srl(|cpu| cpu.b, |cpu, n| cpu.b = n),
            opcodes::SRL_C => self.srl(|cpu| cpu.c, |cpu, n| cpu.c = n),
            opcodes::SRL_D => self.srl(|cpu| cpu.d, |cpu, n| cpu.d = n),
            opcodes::SRL_E => self.srl(|cpu| cpu.e, |cpu, n| cpu.e = n),
            opcodes::SRL_H => self.srl(|cpu| cpu.h, |cpu, n| cpu.h = n),
            opcodes::SRL_L => self.srl(|cpu| cpu.l, |cpu, n| cpu.l = n),
            opcodes::SRL_HL => self.srl(
                |cpu| cpu.mem[cpu.get_hl() as usize],
                |cpu, n| {
                    let addr = cpu.get_hl() as usize;
                    cpu.set_mem(addr, n);
                },
            ),

            s => {
                return Err(Error::new(
                    ErrorKind::UnknownInstruction,
                    format!("Unimplemented opcode {:02x}@{:04x}", s, self.pc - 1,),
                ))
            }
        }

        Ok(())
    }

    fn peek_byte(&self) -> u8 {
        self.mem[self.pc as usize]
    }

    fn consume_byte(&mut self) -> u8 {
        let result = self.peek_byte();

        self.pc += 1;

        result
    }

    fn consume_16_imm(&mut self) -> u16 {
        let snd_byte = self.consume_byte();
        let fst_byte = self.consume_byte();

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

    ///**Description:**
    ///  Call address n if following condition is true:
    ///
    ///  cc = NZ, Call if Z flag is reset.
    ///  cc = Z, Call if Z flag is set.
    ///  cc = NC, Call if C flag is reset.
    ///  cc = C, Call if C flag is set.
    ///
    ///**Use with:**
    ///  nn = two byte immediate value. (LS byte first.)
    fn call_cc_a16<F>(&mut self, condition: F)
    where
        F: Fn(&Cpu) -> bool,
    {
        if condition(self) {
            self.call_a16();
        }
    }

    /// **Description**
    ///
    /// Put value A into memory address nn.
    fn ld_addr_a<F>(&mut self, f: F)
    where
        F: Fn(&Cpu) -> u16,
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
    /// Put value r2 into r1.
    ///
    ///**Use with:**
    /// r1,r2 = A,B,C,D,E,H,L,
    fn ld_r8_r8<G, F>(&mut self, getter: G, setter: F)
    where
        G: Fn(&mut Cpu) -> u8,
        F: Fn(&mut Cpu, u8),
    {
        let r1 = getter(self);
        setter(self, r1);
    }

    /// **Description**
    ///
    /// Put value at (HL) into r1.
    ///
    ///**Use with:**
    /// r1 = A,B,C,D,E,H,L
    /// r2 = (HL)
    fn ld_r8_r16<G, F>(&mut self, getter: G, setter: F)
    where
        G: Fn(&Cpu) -> u16,
        F: Fn(&mut Cpu, u8),
    {
        let r2 = self.mem[getter(self) as usize] as u8;
        setter(self, r2);
    }

    /// **Description**
    ///
    /// Put value at (nn) address into r1.
    ///
    ///**Use with:**
    /// r1 = A,B,C,D,E,H,L
    /// r2 = (nn)
    fn ld_r8_a16<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu, u8),
    {
        let addr = self.consume_16_addr();
        let r2 = self.mem[addr as usize] as u8;
        f(self, r2);
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
        let n = self.consume_byte() as i8;
        self.pc = self.pc.wrapping_add(n as u16);
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

    fn ei(&self) {
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
        if condition(self) {
            self.jp_a16();
        } else {
            // Ensure that the address is consumed even if we don't jump
            self.consume_16_addr();
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
    /// Put memory address $FF00+n into A
    ///
    ///**Use with:**
    /// n = one byte immediate value.
    fn ldh_a_a8(&mut self) {
        let n = self.consume_byte() as usize;
        self.a = self.mem[MEM_HW_IO_REG_OFFSET + n];
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
        self.ld_r8_r16(Cpu::get_hl, |cpu, n| cpu.a = n);
        self.inc_r16(Cpu::get_hl, Cpu::set_hl);
    }

    ///**Description:**
    ///
    /// Put value at address HL into A. Increment HL.
    /// Same as: LD (HL),A - INC HL
    fn ldi_hl_a(&mut self) {
        self.ld_addr_a(Cpu::get_hl);
        self.inc_r16(Cpu::get_hl, Cpu::set_hl);
    }

    ///**Description:**
    ///
    /// Put value at address HL into A. Increment HL.
    /// Same as: LD (HL),A - DEC HL
    fn ldd_hl_a(&mut self) {
        self.ld_addr_a(Cpu::get_hl);
        self.dec_r16(Cpu::get_hl, Cpu::set_hl);
    }

    ///**Description:**
    ///
    /// Put value at address HL into A. Decrement HL.
    /// Same as: LD A,(HL) - DEC HL
    fn ldd_a_hl(&mut self) {
        self.ld_a(|cpu| cpu.mem[cpu.get_hl() as usize]);
        self.dec_r16(Cpu::get_hl, Cpu::set_hl);
    }

    ///**Description:**
    /// Pop two bytes from stack & jump to that address.
    fn ret(&mut self) {
        self.pc = self.pop_stack_u16();
    }

    ///**Description:**
    ///  Return if following condition is true:
    ///
    ///**Use with:**
    ///  cc = NZ, Return if Z flag is reset.
    ///  cc = Z, Return if Z flag is set.
    ///  cc = NC, Return if C flag is reset
    fn ret_cc<F>(&mut self, condition: F)
    where
        F: Fn(&Cpu) -> bool,
    {
        if condition(self) {
            self.ret();
        }
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
    /// Increment register n.
    ///
    ///**Use with:**
    /// n = A,B,C,D,E,H,L
    fn inc_r8<G, S>(&mut self, getter: G, setter: S)
    where
        G: Fn(&Cpu) -> u8,
        S: Fn(&mut Cpu, u8),
    {
        let old_value = getter(self);
        let new_value = old_value.wrapping_add(1);

        self.set_flag_to(&Flag::Zero, new_value == 0);
        self.set_flag_to(&Flag::HalfCarry, old_value & 0xf == 0xf);
        self.reset_flag(&Flag::Sub);

        setter(self, new_value);
    }

    ///**Description:**
    /// Increment byte at address (HL).
    ///
    ///**Use with:**
    /// n = A,B,C,D,E,H,L
    fn inc_addr<G>(&mut self, getter: G)
    where
        G: Fn(&Cpu) -> u16,
    {
        let addr = getter(self) as usize;
        let old_value = self.mem[addr];
        let new_value = old_value.wrapping_add(1);

        self.set_flag_to(&Flag::Zero, new_value == 0);
        self.set_flag_to(&Flag::HalfCarry, old_value & 0xf == 0xf);
        self.reset_flag(&Flag::Sub);

        self.mem[addr] = new_value;
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
        setter(self, curr_value.wrapping_add(1));
    }

    ///**Description:**
    /// Decrement register n.
    ///
    ///**Use with:**
    /// n = A,B,C,D,E,H,L
    fn dec_r8<G, S>(&mut self, getter: G, setter: S)
    where
        G: Fn(&Cpu) -> u8,
        S: Fn(&mut Cpu, u8),
    {
        let old_value = getter(self);
        let new_value = old_value.wrapping_sub(1);

        self.set_flag_to(&Flag::Zero, new_value == 0);
        self.set_flag_to(&Flag::HalfCarry, old_value & 0xf == 0);
        self.set_flag(&Flag::Sub);

        setter(self, new_value);
    }

    ///**Description:**
    /// Decrement byte at address (HL).
    ///
    ///**Use with:**
    /// n = A,B,C,D,E,H,L
    fn dec_addr<G>(&mut self, getter: G)
    where
        G: Fn(&Cpu) -> u16,
    {
        let addr = getter(self) as usize;
        let old_value = self.mem[addr];
        let new_value = old_value.wrapping_sub(1);

        self.set_flag_to(&Flag::Zero, new_value == 0);
        self.set_flag_to(&Flag::HalfCarry, old_value & 0xf == 0);
        self.set_flag(&Flag::Sub);

        self.mem[addr] = new_value;
    }

    ///**Description:**
    /// Decrement register nn.
    ///
    ///**Use with:**
    /// nn = BC,DE,HL,SP
    fn dec_r16<G, S>(&mut self, getter: G, setter: S)
    where
        G: Fn(&Cpu) -> u16,
        S: Fn(&mut Cpu, u16),
    {
        let curr_value = getter(self);
        setter(self, curr_value.wrapping_sub(1));
    }

    ///**Description:**
    ///  Add n to register A.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL),#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Reset.
    ///  H - Set if carry from bit 3.
    ///  C - Set if carry from bit 7.
    fn add_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        let old_value = self.a;
        let n = f(self);
        let result = old_value.wrapping_add(n);

        self.set_flag_to(&Flag::Zero, result == 0);
        self.reset_flag(&Flag::Sub);
        self.set_flag_to(&Flag::HalfCarry, (old_value & 0x0f) + (n & 0x0f) > 0x0f);
        self.set_flag_to(&Flag::Carry, u16::from(old_value) + u16::from(n) > 0xff);

        self.a = result;
    }

    ///**Description:**
    ///  Add n to HL.
    ///
    ///**Use with:**
    ///  n = BC,DE,HL,SP
    ///
    ///**Flags affected:**
    ///  Z - Not affected.
    ///  N - Reset.
    ///  H - Set if carry from bit 11.
    ///  C - Set if carry from bit 15.
    fn add_hl<F>(&mut self, f: F)
    where
        F: Fn(&Cpu) -> u16,
    {
        let old_value = self.get_hl();
        let n = f(self);
        let result = old_value.wrapping_add(n);

        self.reset_flag(&Flag::Sub);
        self.set_flag_to(
            &Flag::HalfCarry,
            (old_value & 0x0fff) + (n & 0x0fff) > 0x0fff,
        );
        self.set_flag_to(&Flag::Carry, u32::from(old_value) + u32::from(n) > 0xffff);

        self.set_hl(result);
    }

    ///**Description:**
    ///  Add n to SP.
    ///
    ///**Use with:**
    ///  n = one byte signed immediate value
    ///
    ///**Flags affected:**
    ///  Z - Reset.
    ///  N - Reset.
    ///  H - Set if carry from bit 11 (always set on subtraction).
    ///  C - Set if carry from bit 15 (always set on subtraction).
    #[cfg_attr(feature = "clippy", allow(cast_lossless))]
    fn add_sp_imm(&mut self) {
        let old_value = self.sp;
        let n = i16::from(self.consume_byte() as i8) as u16;
        let result = old_value.wrapping_add(n);

        self.reset_flag(&Flag::Zero);
        self.reset_flag(&Flag::Sub);
        self.set_flag_to(
            &Flag::HalfCarry,
            (old_value & 0x0fff) + (n & 0x0fff) > 0x0fff,
        );
        self.set_flag_to(
            &Flag::Carry,
            (u32::from(old_value)) + (u32::from(n)) > 0xffff,
        );

        self.sp = result;
    }

    ///**Description:**
    ///  Sub n to register A.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL),#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Set.
    ///  H - Set if carry from bit 3.
    ///  C - Set if carry from bit 7.
    fn sub_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        let old_value = self.a;
        let n = f(self);
        let result = old_value.wrapping_sub(n);

        self.set_flag_to(&Flag::Zero, result == 0);
        self.set_flag(&Flag::Sub);
        self.set_flag_to(&Flag::HalfCarry, (old_value & 0x0f) < (n & 0x0f));
        self.set_flag_to(&Flag::Carry, u16::from(old_value) < u16::from(n));

        self.a = result;
    }

    ///**Description:**
    ///  Logical AND n with register A, result in A.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL),#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Reset.
    ///  H - Set.
    ///  C - Reset.
    fn and_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        let result = self.a & f(self);
        self.a = result;

        self.set_flag_to(&Flag::Zero, result == 0);
        self.reset_flag(&Flag::Carry);
        self.set_flag(&Flag::HalfCarry);
        self.reset_flag(&Flag::Sub);
    }

    ///**Description:**
    ///  Logical XOR n with register A, result in A.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL),#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Reset.
    ///  H - Reset.
    ///  C - Reset.
    fn xor_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        let result = self.a ^ f(self);
        self.a = result;

        self.set_flag_to(&Flag::Zero, result == 0);
        self.reset_flag(&Flag::Carry);
        self.reset_flag(&Flag::HalfCarry);
        self.reset_flag(&Flag::Sub);
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
        let result = self.a | f(self);
        self.a = result;

        self.set_flag_to(&Flag::Zero, result == 0);
        self.reset_flag(&Flag::Carry);
        self.reset_flag(&Flag::HalfCarry);
        self.reset_flag(&Flag::Sub);
    }

    ///**Description:**
    /// If following condition is true then add n to current
    /// address and jump to it:
    ///
    ///**Use with:**
    /// n = one byte signed immediate value
    /// cc = NZ, Jump if Z flag is reset
    /// cc = Z, Jump if Z flag is set
    /// cc = NC, Jump if C flag is reset
    /// cc = C, Jump if C flag is set
    fn jr_cc_r8<F>(&mut self, condition: F)
    where
        F: Fn(&Cpu) -> bool,
    {
        if condition(self) {
            self.jr_r8();
        } else {
            // Ensure that the address is consumed even if we don't jump
            self.consume_byte();
        }
    }

    ///**Description:**
    ///  Put value n into nn.
    ///
    /// **Use with:**
    ///  nn = B,C,D,E,H,L,BC,DE,HL,SP
    ///  n = 8 bit immediate value
    fn ld_r8_d8<F>(&mut self, setter: F)
    where
        F: Fn(&mut Cpu, u8),
    {
        let value = self.consume_byte();
        setter(self, value);
    }

    ///**Description:**
    ///  Compare A with n. This is basically an A - n
    ///  subtraction instruction but the results are thrown
    ///  away.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero. (Set if A = n.)
    ///  N - Set.
    ///  H - Set if no borrow from bit 4.
    ///  C - Set for no borrow. (Set if A < n.)
    /// TODO: implement this with the SUB instruction instead
    fn cp_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        let n = f(self);
        let a = self.a;

        self.set_flag_to(&Flag::Zero, a == n);
        self.set_flag_to(&Flag::Carry, a < n);
        self.set_flag_to(&Flag::HalfCarry, (a & 0xF) < (n & 0xF));
        self.set_flag(&Flag::Sub);
    }

    ///**Description:**
    ///  Compare A with address at (HL), n. This is basically an A - n
    ///  subtraction instruction but the results are thrown
    ///  away.
    ///
    ///**Use with:**
    ///  n = (HL)
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero. (Set if A = n.)
    ///  N - Set.
    ///  H - Set if no borrow from bit 4.
    ///  C - Set for no borrow. (Set if A < n.)
    /// TODO: implement this with the SUB instruction instead
    fn cp_hl(&mut self) {
        let n = self.mem[self.get_hl() as usize];
        let a = self.a;

        self.set_flag_to(&Flag::Zero, a == n);
        self.set_flag_to(&Flag::Carry, a < n);
        self.set_flag_to(&Flag::HalfCarry, (a & 0xF) < (n & 0xF));
        self.set_flag(&Flag::Sub);
    }

    /// **Description**
    ///
    /// Put value at register r2 into memory address stored in r1.
    fn ld_addr_r8<G, F>(&mut self, r1: G, r2: F)
    where
        F: Fn(&Cpu) -> u8,
        G: Fn(&Cpu) -> u16,
    {
        let addr = r1(self) as usize;
        self.mem[addr] = r2(self);
    }

    ///**Description:**
    ///  Add n + Carry flag to A.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL),#
    ///
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Reset.
    ///  H - Set if carry from bit 3.
    ///  C - Set if carry from bit 7.
    fn adc_a<F>(&mut self, f: F)
    where
        F: Fn(&mut Cpu) -> u8,
    {
        let a = self.a;
        let carry = if self.flag(&Flag::Carry) { 1 } else { 0 };
        let n = f(self);
        let res = a.wrapping_add(n).wrapping_add(carry);

        self.set_flag_to(&Flag::Zero, res == 0);
        self.reset_flag(&Flag::Sub);
        self.set_flag_to(&Flag::HalfCarry, (a & 0x0f) + (n & 0x0f) + carry > 0x0f);
        self.set_flag_to(
            &Flag::Carry,
            u16::from(a) + u16::from(n) + u16::from(carry) > 0xff,
        );

        self.a = res;
    }

    ///**Description:**
    ///  Rotate A left. Old bit 7 to Carry flag.
    ///
    ///**Flags affected:**
    ///  Z - Reset.
    ///  N - Reset.
    ///  H - Reset.
    ///  C - Contains old bit 7 data.
    fn rlc_a(&mut self) {
        self.reset_flag(&Flag::Sub);
        self.reset_flag(&Flag::HalfCarry);

        let a = self.a;
        let carry = a >> 7;
        let res = ((a & 0x7f) << 1) | carry; // a & 0b01111111

        self.reset_flag(&Flag::Zero);
        self.reset_flag(&Flag::Sub);
        self.reset_flag(&Flag::HalfCarry);
        self.set_flag_to(&Flag::Carry, carry == 1);
        self.a = res;
    }

    ///**Description:**
    ///  Rotate A right. Old bit 0 to Carry flag.
    ///
    ///**Flags affected:**
    ///  Z - Reset.
    ///  N - Reset.
    ///  H - Reset.
    ///  C - Contains old bit 0 data.
    fn rrca(&mut self) {
        let lsb = self.a & 1;
        let a = (lsb << 7) | (self.a >> 1);

        self.reset_flag(&Flag::Zero);
        self.reset_flag(&Flag::Sub);
        self.reset_flag(&Flag::HalfCarry);
        self.set_flag_to(&Flag::Carry, lsb == 1);

        self.a = a;
    }

    ///**Description:**
    ///  Put SP + n effective address into HL.
    ///
    ///**Use with:**
    ///  n = one byte signed immediate value.
    ///
    ///**Flags affected:**
    ///  Z - Reset.
    ///  N - Reset.
    ///  H - Set or reset according to operation.
    ///  C - Set or reset according to operation.
    fn ldhl_sp_r8(&mut self) {
        let old_value = self.sp;

        self.add_sp_imm();
        let new_value = self.sp;

        self.set_hl(new_value);
        self.sp = old_value;
    }

    ///**Description:**
    ///  Put Stack Pointer (SP) at address n.
    ///
    ///**Use with:**
    ///  nn = two byte immediate address.
    fn ld_a16_sp(&mut self) {
        let addr = self.consume_16_addr();
        self.sp = addr;
    }

    ///**Description:**
    ///  Push present address onto stack.
    ///  Jump to address $0000 + n.
    ///
    ///**Use with:**
    ///  n = $00,$08,$10,$18,$20,$28,$30,$38
    fn rst_a8(&mut self, n: u8) {
        let curr_pc = self.pc;
        self.push_stack_u16(curr_pc);

        self.pc = u16::from(n);
    }

    ///**Description:**
    /// Shift n right into Carry.
    ///
    ///**Use with:**
    ///  n = A,B,C,D,E,H,L,(HL)
    ///**Flags affected:**
    ///  Z - Set if result is zero.
    ///  N - Reset.
    ///  H - Reset.
    ///  C - Set if the least significant bit of n is set.
    fn srl<G, S>(&mut self, g: G, s: S)
    where
        G: Fn(&Cpu) -> u8,
        S: Fn(&mut Cpu, u8),
    {
        let value = g(self);
        let result = value >> 1;
        let carry = value & 0x01 == 0x01;

        s(self, value >> 1);
        self.set_flag_to(&Flag::Carry, carry);
        self.set_flag_to(&Flag::Zero, result == 0);
    }
}
