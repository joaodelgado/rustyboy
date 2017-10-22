#![cfg(test)]
use super::*;

#[test]
fn test_inc_a() {
    let mut cpu = Cpu::new();

    cpu.a = 0x01;
    cpu.mem[0] = opcodes::INC_A;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_A;
    cpu.a = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 0x00);
}

#[test]
fn test_inc_b() {
    let mut cpu = Cpu::new();

    cpu.b = 0x01;
    cpu.mem[0] = opcodes::INC_B;

    cpu.tick().unwrap();
    assert_eq!(cpu.b, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_B;
    cpu.b = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.b, 0x00);
}

#[test]
fn test_inc_c() {
    let mut cpu = Cpu::new();

    cpu.c = 0x01;
    cpu.mem[0] = opcodes::INC_C;

    cpu.tick().unwrap();
    assert_eq!(cpu.c, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_C;
    cpu.c = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.c, 0x00);
}

#[test]
fn test_inc_d() {
    let mut cpu = Cpu::new();

    cpu.d = 0x01;
    cpu.mem[0] = opcodes::INC_D;

    cpu.tick().unwrap();
    assert_eq!(cpu.d, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_D;
    cpu.d = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.d, 0x00);
}

#[test]
fn test_inc_e() {
    let mut cpu = Cpu::new();

    cpu.e = 0x01;
    cpu.mem[0] = opcodes::INC_E;

    cpu.tick().unwrap();
    assert_eq!(cpu.e, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_E;
    cpu.e = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.e, 0x00);
}

#[test]
fn test_inc_h() {
    let mut cpu = Cpu::new();

    cpu.h = 0x01;
    cpu.mem[0] = opcodes::INC_H;

    cpu.tick().unwrap();
    assert_eq!(cpu.h, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_H;
    cpu.h = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.h, 0x00);
}

#[test]
fn test_inc_l() {
    let mut cpu = Cpu::new();

    cpu.h = 0x01;
    cpu.mem[0] = opcodes::INC_H;

    cpu.tick().unwrap();
    assert_eq!(cpu.h, 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_H;
    cpu.h = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.h, 0x00);
}

#[test]
fn test_inc_ahl() {
    let mut cpu = Cpu::new();

    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0x01;

    cpu.mem[0] = opcodes::INC_AHL;

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0xfee2], 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_AHL;
    cpu.mem[0xfee2] = 0xff;

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0xfee2], 0x00);
}

#[test]
fn test_inc_bc() {
    let mut cpu = Cpu::new();
    cpu.set_bc(0xfff9);

    cpu.mem[0] = opcodes::INC_BC;

    cpu.tick().unwrap();
    assert_eq!(cpu.get_bc(), 0xfffa);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_BC;
    cpu.set_bc(0xffff);

    cpu.tick().unwrap();
    assert_eq!(cpu.get_bc(), 0x0000);
}

#[test]
fn test_inc_de() {
    let mut cpu = Cpu::new();
    cpu.set_de(0xfff9);

    cpu.mem[0] = opcodes::INC_DE;

    cpu.tick().unwrap();
    assert_eq!(cpu.get_de(), 0xfffa);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_DE;
    cpu.set_de(0xffff);

    cpu.tick().unwrap();
    assert_eq!(cpu.get_de(), 0x0000);
}

#[test]
fn test_inc_hl() {
    let mut cpu = Cpu::new();
    cpu.set_hl(0xfff9);

    cpu.mem[0] = opcodes::INC_HL;

    cpu.tick().unwrap();
    assert_eq!(cpu.get_hl(), 0xfffa);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::INC_HL;
    cpu.set_hl(0xffff);

    cpu.tick().unwrap();
    assert_eq!(cpu.get_hl(), 0x0000);
}
