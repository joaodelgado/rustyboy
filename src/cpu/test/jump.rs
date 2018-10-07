#![cfg(test)]
use super::*;

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
fn test_jr_r8() {
    let mut cpu = Cpu::new();
    cpu.mem[0] = opcodes::JR_R8;
    cpu.mem[1] = 15;

    cpu.tick().unwrap();
    assert_eq!(cpu.pc, 17);

    cpu.pc = 0xffee;
    cpu.mem[cpu.pc as usize] = opcodes::JR_R8;
    cpu.mem[(cpu.pc + 1) as usize] = 0xf1;
    cpu.tick().unwrap();

    assert_eq!(cpu.pc, 0xffe1);
}

fn _test_jp_flag_set(opcode: u8, flag: Flag, should_jump: bool) {
    let mut cpu = Cpu::new();

    let inital_pc = cpu.pc;

    cpu.mem[0] = opcode;
    cpu.mem[1] = 0x0;
    cpu.mem[2] = 0x01;

    cpu.set_flag(&flag);
    cpu.tick().unwrap();
    if should_jump {
        assert_eq!(cpu.pc, 0x100)
    } else {
        // +3 because we should skip the jump address
        // regardless if we jump or not
        assert_eq!(cpu.pc, inital_pc + 3);
    }
}


fn _test_jp_flag_reset(opcode: u8, flag: Flag, should_jump: bool) {
    let mut cpu = Cpu::new();

    let inital_pc = cpu.pc;

    cpu.mem[0] = opcode;
    cpu.mem[1] = 0x0;
    cpu.mem[2] = 0x01;

    cpu.reset_flag(&flag);
    cpu.tick().unwrap();
    if should_jump {
        assert_eq!(cpu.pc, 0x100)
    } else {
        // +3 because we should skip the jump address
        // regardless if we jump or not
        assert_eq!(cpu.pc, inital_pc + 3);
    }
}


#[test]
fn test_jp_nz_r8() {
    _test_jp_flag_set(opcodes::JP_NZ_A16, Flag::Zero, false);
    _test_jp_flag_reset(opcodes::JP_NZ_A16, Flag::Zero, true);
}

#[test]
fn test_jp_z_r8() {
    _test_jp_flag_set(opcodes::JP_Z_A16, Flag::Zero, true);
    _test_jp_flag_reset(opcodes::JP_Z_A16, Flag::Zero, false);
}

#[test]
fn test_jp_nc_r8() {
    _test_jp_flag_set(opcodes::JP_NC_A16, Flag::Carry, false);
    _test_jp_flag_reset(opcodes::JP_NC_A16, Flag::Carry, true);
}

#[test]
fn test_jp_c_r8() {
    _test_jp_flag_set(opcodes::JP_C_A16, Flag::Carry, true);
    _test_jp_flag_reset(opcodes::JP_C_A16, Flag::Carry, false);
}


fn _test_jr_flag_set(opcode: u8, flag: Flag, should_jump: bool) {
    let mut cpu = Cpu::new();

    let inital_pc = cpu.pc;
    let jump = 100;

    cpu.mem[0] = opcode;
    cpu.mem[1] = jump;

    cpu.set_flag(&flag);
    cpu.tick().unwrap();
    if should_jump {
        assert_eq!(cpu.pc, inital_pc + (jump as u16) + 2);
    } else {
        // +2 because we should skip the by containing the relative address
        // regardless if we jump or not
        assert_eq!(cpu.pc, inital_pc + 2);
    }
}


fn _test_jr_flag_reset(opcode: u8, flag: Flag, should_jump: bool) {
    let mut cpu = Cpu::new();

    let inital_pc = cpu.pc;
    let jump = 100;

    cpu.mem[0] = opcode;
    cpu.mem[1] = jump;

    cpu.reset_flag(&flag);
    cpu.tick().unwrap();
    if should_jump {
        assert_eq!(cpu.pc, inital_pc + (jump as u16) + 2);
    } else {
        // +2 because we should skip the relative address
        // regardless if we jump or not
        assert_eq!(cpu.pc, inital_pc + 2);
    }
}


#[test]
fn test_jr_nz_r8() {
    _test_jr_flag_set(opcodes::JR_NZ_R8, Flag::Zero, false);
    _test_jr_flag_reset(opcodes::JR_NZ_R8, Flag::Zero, true);
}

#[test]
fn test_jr_z_r8() {
    _test_jr_flag_set(opcodes::JR_Z_R8, Flag::Zero, true);
    _test_jr_flag_reset(opcodes::JR_Z_R8, Flag::Zero, false);
}

#[test]
fn test_jr_nc_r8() {
    _test_jr_flag_set(opcodes::JR_NC_R8, Flag::Carry, false);
    _test_jr_flag_reset(opcodes::JR_NC_R8, Flag::Carry, true);
}

#[test]
fn test_jr_c_r8() {
    _test_jr_flag_set(opcodes::JR_C_R8, Flag::Carry, true);
    _test_jr_flag_reset(opcodes::JR_C_R8, Flag::Carry, false);
}
