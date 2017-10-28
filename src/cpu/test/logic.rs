#![cfg(test)]
use super::*;

#[test]
fn test_or_a_a() {
    let mut cpu = Cpu::new();
    cpu.a = 0b10000000;
    cpu.mem[0] = opcodes::OR_A_A;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_A;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_b() {
    let mut cpu = Cpu::new();
    cpu.a = 0b10000000;
    cpu.b = 0b10000000;
    cpu.mem[0] = opcodes::OR_A_B;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.c = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_B;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_c() {
    let mut cpu = Cpu::new();
    cpu.a = 0b10000011;
    cpu.b = 0b01000000;
    cpu.mem[0] = opcodes::OR_A_C;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.c = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_C;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_d() {
    let mut cpu = Cpu::new();
    cpu.a = 0xff;
    cpu.b = 0b10000000;
    cpu.mem[0] = opcodes::OR_A_D;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.d = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_D;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_e() {
    let mut cpu = Cpu::new();
    cpu.a = 0xff;
    cpu.e = 0b10000000;
    cpu.mem[0] = opcodes::OR_A_E;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.e = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_E;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_h() {
    let mut cpu = Cpu::new();
    cpu.a = 10;
    cpu.h = 0b10000000;
    cpu.mem[0] = opcodes::OR_A_H;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.h = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_H;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_l() {
    let mut cpu = Cpu::new();
    cpu.a = 0xf;
    cpu.b = 0b10000000;
    cpu.mem[0] = opcodes::OR_A_L;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.d = 0b00000000;
    cpu.mem[1] = opcodes::OR_A_L;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_hl() {
    let mut cpu = Cpu::new();
    cpu.a = 0xff;
    cpu.set_hl(0xffe1);
    cpu.mem[0] = opcodes::OR_A_HL;
    cpu.mem[0xffe1] = 0b11000000;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.set_hl(0xffe1);
    cpu.mem[1] = opcodes::OR_A_HL;
    cpu.mem[0xffe1] = 0;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_or_a_d8() {
    let mut cpu = Cpu::new();
    cpu.a = 0xff;
    cpu.mem[0] = opcodes::OR_A_D8;
    cpu.mem[1] = 0xf1;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero));

    cpu.a = 0b00000000;
    cpu.mem[2] = opcodes::OR_A_D8;
    cpu.mem[3] = 0;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}


#[test]
fn test_cp_d8() {
    let mut cpu = Cpu::new();
    cpu.a = 4;
    cpu.mem[0] = opcodes::CP_A;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));

    cpu.b = 10;
    cpu.mem[cpu.pc as usize] = opcodes::CP_B;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Carry));

    cpu.a = 0b00000011;
    cpu.c = 0b00010011;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::HalfCarry));

    cpu.a = 0x12;
    cpu.mem[cpu.pc as usize] = opcodes::CP_D8;
    cpu.mem[(cpu.pc + 1) as usize] = 0x12;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}

#[test]
fn test_cp_hl() {
    let mut cpu = Cpu::new();
    let addr = 0x1234;
    cpu.set_hl(addr);
    cpu.mem[0] = opcodes::CP_HL;
    cpu.a = 5;
    cpu.mem[addr as usize] = 5;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero));
}
