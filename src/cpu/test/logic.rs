#![cfg(test)]
use super::*;

fn _test_or_a_reg<G>(r: G, opcode: u8)
where
    G: Fn(&mut Cpu, u8),
{
    let new_cpu = || {
        let mut cpu = Cpu::new();

        cpu.set_flag(&Flag::Zero);
        cpu.set_flag(&Flag::Sub);
        cpu.set_flag(&Flag::HalfCarry);
        cpu.set_flag(&Flag::Carry);

        cpu
    };

    //
    // Non non zero reg
    //

    let cpu = &mut new_cpu();
    cpu.a = 0b0000_0000;
    cpu.mem[0] = opcode;
    let value = 0b1001_0110;
    r(cpu, value);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, value);
    assert!(!cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Sub));
    assert!(!cpu.flag(&Flag::HalfCarry));
    assert!(!cpu.flag(&Flag::Carry));

    //
    // Test zero reg
    //
    let cpu = &mut new_cpu();
    cpu.a = 0b0000_0000;
    cpu.mem[0] = opcode;
    let value = 0b0000_0000;
    r(cpu, value);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, 0b0000_0000);
    assert!(cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Sub));
    assert!(!cpu.flag(&Flag::HalfCarry));
    assert!(!cpu.flag(&Flag::Carry));
}

fn _test_xor_a_reg<G>(r: G, opcode: u8)
where
    G: Fn(&mut Cpu, u8),
{
    let new_cpu = || {
        let mut cpu = Cpu::new();

        cpu.set_flag(&Flag::Zero);
        cpu.set_flag(&Flag::Sub);
        cpu.set_flag(&Flag::HalfCarry);
        cpu.set_flag(&Flag::Carry);

        cpu
    };

    //
    // Non non zero reg
    //

    let cpu = &mut new_cpu();
    cpu.a = 0b1001_0000;
    cpu.mem[0] = opcode;
    let value = 0b1001_0110;
    r(cpu, value);

    let expected = if cpu.a == value { 0 } else { 0b0000_0110 };

    cpu.tick().unwrap();

    assert_eq!(cpu.a, expected);
    assert!(cpu.flag(&Flag::Zero) == (expected == 0));
    assert!(!cpu.flag(&Flag::Sub));
    assert!(!cpu.flag(&Flag::HalfCarry));
    assert!(!cpu.flag(&Flag::Carry));

    //
    // Test zero reg
    //

    let cpu = &mut new_cpu();
    cpu.a = 0b0000_0000;
    cpu.mem[0] = opcode;
    let value = 0b0000_0000;
    r(cpu, value);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, 0b0000_0000);
    assert!(cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Sub));
    assert!(!cpu.flag(&Flag::HalfCarry));
    assert!(!cpu.flag(&Flag::Carry));
}

fn _test_and_a_reg<G>(r: G, opcode: u8)
where
    G: Fn(&mut Cpu, u8),
{
    let new_cpu = || {
        let mut cpu = Cpu::new();

        cpu.set_flag(&Flag::Zero);
        cpu.set_flag(&Flag::Sub);
        cpu.reset_flag(&Flag::HalfCarry);
        cpu.set_flag(&Flag::Carry);

        cpu
    };

    //
    // Non non zero reg
    //

    let cpu = &mut new_cpu();
    cpu.a = 0b1111_1111;
    cpu.mem[0] = opcode;
    let value = 0b1001_0110;
    r(cpu, value);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, value);
    assert!(!cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Sub));
    assert!(cpu.flag(&Flag::HalfCarry));
    assert!(!cpu.flag(&Flag::Carry));

    //
    // Test zero reg
    //
    let cpu = &mut new_cpu();
    cpu.a = 0b1111_1111;
    cpu.mem[0] = opcode;
    let value = 0b0000_0000;
    r(cpu, value);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, 0b0000_0000);
    assert!(cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Sub));
    assert!(cpu.flag(&Flag::HalfCarry));
    assert!(!cpu.flag(&Flag::Carry));
}

#[test]
fn test_or_a_a() {
    _test_or_a_reg(|cpu, value| cpu.a = value, opcodes::OR_A_A);
}

#[test]
fn test_or_a_b() {
    _test_or_a_reg(|cpu, value| cpu.b = value, opcodes::OR_A_B);
}

#[test]
fn test_or_a_c() {
    _test_or_a_reg(|cpu, value| cpu.c = value, opcodes::OR_A_C);
}

#[test]
fn test_or_a_d() {
    _test_or_a_reg(|cpu, value| cpu.d = value, opcodes::OR_A_D);
}

#[test]
fn test_or_a_e() {
    _test_or_a_reg(|cpu, value| cpu.e = value, opcodes::OR_A_E);
}

#[test]
fn test_or_a_h() {
    _test_or_a_reg(|cpu, value| cpu.h = value, opcodes::OR_A_H);
}

#[test]
fn test_or_a_l() {
    _test_or_a_reg(|cpu, value| cpu.l = value, opcodes::OR_A_L);
}

#[test]
fn test_or_a_hl() {
    _test_or_a_reg(
        |cpu, value| {
            cpu.set_hl(0xffe1);
            cpu.mem[0xffe1] = value
        },
        opcodes::OR_A_HL,
    );
}

#[test]
fn test_or_a_d8() {
    _test_or_a_reg(
        |cpu, value| {
            let i = (cpu.pc + 1) as usize;
            cpu.mem[i] = value;
        },
        opcodes::OR_A_D8,
    );
}

#[test]
fn test_xor_a_a() {
    _test_xor_a_reg(|cpu, value| cpu.a = value, opcodes::XOR_A_A);
}

#[test]
fn test_xor_a_b() {
    _test_xor_a_reg(|cpu, value| cpu.b = value, opcodes::XOR_A_B);
}

#[test]
fn test_xor_a_c() {
    _test_xor_a_reg(|cpu, value| cpu.c = value, opcodes::XOR_A_C);
}

#[test]
fn test_xor_a_d() {
    _test_xor_a_reg(|cpu, value| cpu.d = value, opcodes::XOR_A_D);
}

#[test]
fn test_xor_a_e() {
    _test_xor_a_reg(|cpu, value| cpu.e = value, opcodes::XOR_A_E);
}

#[test]
fn test_xor_a_h() {
    _test_xor_a_reg(|cpu, value| cpu.h = value, opcodes::XOR_A_H);
}

#[test]
fn test_xor_a_l() {
    _test_xor_a_reg(|cpu, value| cpu.l = value, opcodes::XOR_A_L);
}

#[test]
fn test_xor_a_hl() {
    _test_xor_a_reg(
        |cpu, value| {
            cpu.set_hl(0xffe1);
            cpu.mem[0xffe1] = value
        },
        opcodes::XOR_A_HL,
    );
}

#[test]
fn test_xor_a_d8() {
    _test_xor_a_reg(
        |cpu, value| {
            let i = (cpu.pc + 1) as usize;
            cpu.mem[i] = value;
        },
        opcodes::XOR_A_D8,
    );
}

#[test]
fn test_and_a_a() {
    _test_and_a_reg(|cpu, value| cpu.a = value, opcodes::AND_A_A);
}

#[test]
fn test_and_a_b() {
    _test_and_a_reg(|cpu, value| cpu.b = value, opcodes::AND_A_B);
}

#[test]
fn test_and_a_c() {
    _test_and_a_reg(|cpu, value| cpu.c = value, opcodes::AND_A_C);
}

#[test]
fn test_and_a_d() {
    _test_and_a_reg(|cpu, value| cpu.d = value, opcodes::AND_A_D);
}

#[test]
fn test_and_a_e() {
    _test_and_a_reg(|cpu, value| cpu.e = value, opcodes::AND_A_E);
}

#[test]
fn test_and_a_h() {
    _test_and_a_reg(|cpu, value| cpu.h = value, opcodes::AND_A_H);
}

#[test]
fn test_and_a_l() {
    _test_and_a_reg(|cpu, value| cpu.l = value, opcodes::AND_A_L);
}

#[test]
fn test_and_a_hl() {
    _test_and_a_reg(
        |cpu, value| {
            cpu.set_hl(0xffe1);
            cpu.mem[0xffe1] = value
        },
        opcodes::AND_A_HL,
    );
}

#[test]
fn test_and_a_d8() {
    _test_and_a_reg(
        |cpu, value| {
            let i = (cpu.pc + 1) as usize;
            cpu.mem[i] = value;
        },
        opcodes::AND_A_D8,
    );
}

#[test]
fn test_cp_a() {
    let mut cpu = Cpu::new();
    cpu.a = 4;
    cpu.mem[0] = opcodes::CP_A;

    cpu.tick().unwrap();
    assert!(cpu.flag(&Flag::Zero));

    cpu.b = 10;
    cpu.mem[cpu.pc as usize] = opcodes::CP_B;

    cpu.tick().unwrap();
    assert!(cpu.flag(&Flag::Carry));

    cpu.a = 0b00000011;
    cpu.c = 0b00010011;

    cpu.tick().unwrap();
    assert!(cpu.flag(&Flag::HalfCarry));

    cpu.a = 0x12;
    cpu.mem[cpu.pc as usize] = opcodes::CP_D8;
    cpu.mem[(cpu.pc + 1) as usize] = 0x12;

    cpu.tick().unwrap();
    assert!(cpu.flag(&Flag::Zero));
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
    assert!(cpu.flag(&Flag::Zero));
}

#[test]
fn test_rlca() {
    let mut cpu = Cpu::new();
    cpu.a = 0x01;
    cpu.mem[0] = opcodes::RLCA;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 2);
    assert!(!cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Carry));

    cpu.reset_status();
    cpu.a = 0;
    cpu.mem[cpu.pc as usize] = opcodes::RLCA;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0);
    assert!(!cpu.flag(&Flag::Zero));
    assert!(!cpu.flag(&Flag::Carry));

    cpu.reset_status();
    cpu.a = 0b10000001;
    cpu.mem[cpu.pc as usize] = opcodes::RLCA;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0b00000011);
    assert!(!cpu.flag(&Flag::Zero));
    assert!(cpu.flag(&Flag::Carry));
}

#[test]
fn test_rrca() {
    let mut cpu = Cpu::new();
    cpu.mem[0] = opcodes::RRCA;
    cpu.a = 0x9b;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0xcd);
    assert!(cpu.flag(&Flag::Carry));
    assert!(!cpu.flag(&Flag::Zero));

    cpu.reset_status();
    cpu.mem[cpu.pc as usize] = opcodes::RRCA;
    cpu.a = 0;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0);
    assert!(!cpu.flag(&Flag::Carry));
    assert!(!cpu.flag(&Flag::Zero));

    cpu.reset_status();
    cpu.mem[cpu.pc as usize] = opcodes::RRCA;
    cpu.a = 0x80;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0x40);
    assert!(!cpu.flag(&Flag::Carry));
    assert!(!cpu.flag(&Flag::Zero));
}
