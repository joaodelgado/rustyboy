#![cfg(test)]
use super::*;

//
// INC
//

fn _test_inc_reg<G, S>(opcode: u8, reg_getter: G, reg_setter: S)
where
    G: Fn(&Cpu) -> u8,
    S: Fn(&mut Cpu, u8),
{
    let cpu = &mut Cpu::new();

    reg_setter(cpu, 0x01);
    cpu.mem[0] = opcode;

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0x02);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcode;
    reg_setter(cpu, 0xff);

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0x00);

    // Test Zero flag
    cpu.mem[cpu.pc as usize] = opcode;
    cpu.reset_flag(Flag::Zero);
    reg_setter(cpu, 0x01);

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero)); // Should be reset if the result is non 0

    cpu.mem[cpu.pc as usize] = opcode;
    cpu.set_flag(Flag::Zero);
    reg_setter(cpu, 0xff);

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero)); // Should be set if the result is 0

    // Test Sub flag
    cpu.mem[cpu.pc as usize] = opcode;
    cpu.set_flag(Flag::Sub);

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Sub)); // Should always be reset

    // Test HalfCarry flag
    cpu.mem[cpu.pc as usize] = opcode;
    cpu.set_flag(Flag::HalfCarry);
    reg_setter(cpu, 0b0000_0001);

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::HalfCarry)); // Should be reset if there's no carry on bit 3

    cpu.mem[cpu.pc as usize] = opcode;
    cpu.reset_flag(Flag::HalfCarry);
    reg_setter(cpu, 0b0000_1111);

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::HalfCarry)); // Should be set if there's carry on bit 3
}

#[test]
fn test_inc_a() {
    _test_inc_reg(opcodes::INC_A, |cpu| cpu.a, |cpu, n| cpu.a = n);
}

#[test]
fn test_inc_b() {
    _test_inc_reg(opcodes::INC_B, |cpu| cpu.b, |cpu, n| cpu.b = n);
}

#[test]
fn test_inc_c() {
    _test_inc_reg(opcodes::INC_C, |cpu| cpu.c, |cpu, n| cpu.c = n);
}

#[test]
fn test_inc_d() {
    _test_inc_reg(opcodes::INC_D, |cpu| cpu.d, |cpu, n| cpu.d = n);
}

#[test]
fn test_inc_e() {
    _test_inc_reg(opcodes::INC_E, |cpu| cpu.e, |cpu, n| cpu.e = n);
}

#[test]
fn test_inc_h() {
    _test_inc_reg(opcodes::INC_H, |cpu| cpu.h, |cpu, n| cpu.h = n);
}

#[test]
fn test_inc_l() {
    _test_inc_reg(opcodes::INC_L, |cpu| cpu.l, |cpu, n| cpu.l = n);
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

    // Test Zero flag
    cpu.mem[cpu.pc as usize] = opcodes::INC_AHL;
    cpu.reset_flag(Flag::Zero);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0x01;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero)); // Should be reset if the result is non 0

    cpu.mem[cpu.pc as usize] = opcodes::INC_AHL;
    cpu.set_flag(Flag::Zero);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0xff;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero)); // Should be set if the result is 0

    // Test Sub flag
    cpu.mem[cpu.pc as usize] = opcodes::INC_AHL;
    cpu.set_flag(Flag::Sub);

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Sub)); // Should always be reset

    // Test HalfCarry flag
    cpu.mem[cpu.pc as usize] = opcodes::INC_AHL;
    cpu.set_flag(Flag::HalfCarry);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0b0000_0001;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::HalfCarry)); // Should be reset if there's no carry on bit 3

    cpu.mem[cpu.pc as usize] = opcodes::INC_AHL;
    cpu.reset_flag(Flag::HalfCarry);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0b0000_1111;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::HalfCarry)); // Should be set if there's carry on bit 3
}

fn _test_inc_reg16<G, S>(opcode: u8, reg_getter: G, reg_setter: S)
where
    G: Fn(&Cpu) -> u16,
    S: Fn(&mut Cpu, u16),
{
    let cpu = &mut Cpu::new();

    reg_setter(cpu, 0xfff9);
    cpu.mem[0] = opcode;

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0xfffa);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcode;
    reg_setter(cpu, 0xffff);

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0x0000);

}

#[test]
fn test_inc_bc() {
    _test_inc_reg16(opcodes::INC_BC, Cpu::get_bc, Cpu::set_bc);
}

#[test]
fn test_inc_de() {
    _test_inc_reg16(opcodes::INC_DE, Cpu::get_de, Cpu::set_de);
}

#[test]
fn test_inc_hl() {
    _test_inc_reg16(opcodes::INC_HL, Cpu::get_hl, Cpu::set_hl);
}


//
// ADD
//

fn _test_add_a_reg<S>(opcode: u8, r: S)
where
    S: Fn(&mut Cpu, u8),
{
    let new_cpu = || {
        let mut cpu = Cpu::new();

        cpu.set_flag(Flag::Zero);
        cpu.set_flag(Flag::Sub);
        cpu.set_flag(Flag::HalfCarry);
        cpu.set_flag(Flag::Carry);

        cpu
    };

    //
    // Test zero add
    //

    let cpu = &mut new_cpu();
    cpu.a = 0x00;
    r(cpu, 0x00);
    cpu.mem[0] = opcode;
    let result = cpu.a + 0x00;
    cpu.tick().unwrap();

    assert_eq!(cpu.a, result);
    assert!(cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test non carry add
    //

    let cpu = &mut new_cpu();
    cpu.a = 0x00;
    r(cpu, 0x01);
    cpu.mem[0] = opcode;
    let result = cpu.a + 0x01;
    cpu.tick().unwrap();

    assert_eq!(cpu.a, result);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test half carry add
    //

    let cpu = &mut new_cpu();
    cpu.a = 0x0f;
    r(cpu, 0x0f);
    cpu.mem[0] = opcode;
    let result = cpu.a + 0x0f;
    cpu.tick().unwrap();

    assert_eq!(cpu.a, result);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test carry add
    //

    let cpu = &mut new_cpu();
    cpu.a = 0xff;
    r(cpu, 0xff);
    cpu.mem[0] = opcode;
    let result = cpu.a.wrapping_add(0xff);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, result);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));
}

#[test]
fn test_add_a() {
    _test_add_a_reg(opcodes::ADD_A_A, |cpu, n| cpu.a = n);
}

#[test]
fn test_add_b() {
    _test_add_a_reg(opcodes::ADD_A_B, |cpu, n| cpu.b = n);
}

#[test]
fn test_add_c() {
    _test_add_a_reg(opcodes::ADD_A_C, |cpu, n| cpu.c = n);
}

#[test]
fn test_add_d() {
    _test_add_a_reg(opcodes::ADD_A_D, |cpu, n| cpu.d = n);
}

#[test]
fn test_add_e() {
    _test_add_a_reg(opcodes::ADD_A_E, |cpu, n| cpu.e = n);
}

#[test]
fn test_add_h() {
    _test_add_a_reg(opcodes::ADD_A_H, |cpu, n| cpu.h = n);
}

#[test]
fn test_add_l() {
    _test_add_a_reg(opcodes::ADD_A_L, |cpu, n| cpu.l = n);
}

#[test]
fn test_add_a_hl() {
    _test_add_a_reg(opcodes::ADD_A_HL, |cpu, value| {
        cpu.set_hl(0xffe1);
        cpu.mem[0xffe1] = value
    });
}

#[test]
fn test_add_a_d8() {
    _test_add_a_reg(opcodes::ADD_A_D8, |cpu, value| {
        let i = (cpu.pc + 1) as usize;
        cpu.mem[i] = value;
    });
}
