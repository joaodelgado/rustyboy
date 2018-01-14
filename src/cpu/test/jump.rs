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

    cpu.set_flag(&Flag::Zero);
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

    cpu.set_flag(&Flag::Carry);
    cpu.tick().unwrap();
    assert_eq!(cpu.pc, 0x100);
}

#[test]
fn test_jr_cc_r8() {
    let mut cpu = Cpu::new();

    // check zero flag not set
    cpu.mem[0] = opcodes::JR_NZ_R8;
    cpu.mem[1] = 100;

    cpu.tick().unwrap();
    assert_eq!(cpu.pc, 102);

    // check zero flag set
    cpu.mem[102] = opcodes::JR_Z_R8;
    cpu.mem[103] = 0b11001110; // -50

    cpu.set_flag(&Flag::Zero);
    cpu.tick().unwrap();

    assert_eq!(cpu.pc, 54);

    // check carry flag not set
    cpu.mem[cpu.pc as usize] = opcodes::JR_NC_R8;
    cpu.mem[(cpu.pc + 1) as usize] = 20;

    cpu.tick().unwrap();
    assert_eq!(cpu.pc, 76);

    // check carry flag set
    cpu.mem[cpu.pc as usize] = opcodes::JR_C_R8;
    cpu.mem[(cpu.pc + 1) as usize] = 0b11101101; // -19

    cpu.set_flag(&Flag::Carry);
    cpu.tick().unwrap();
    assert_eq!(cpu.pc, 59);
}
