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
// DEC
//

fn _test_dec_reg<G, S>(opcode: u8, reg_getter: G, reg_setter: S)
where
    G: Fn(&Cpu) -> u8,
    S: Fn(&mut Cpu, u8),
{
    let cpu = &mut Cpu::new();

    reg_setter(cpu, 0x02);
    cpu.mem[0] = opcode;

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0x01);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcode;
    reg_setter(cpu, 0x00);

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0xff);

    // Test Zero flag
    cpu.mem[cpu.pc as usize] = opcode;
    cpu.reset_flag(Flag::Zero);
    reg_setter(cpu, 0x02);

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero)); // Should be reset if the result is non 0

    cpu.mem[cpu.pc as usize] = opcode;
    cpu.set_flag(Flag::Zero);
    reg_setter(cpu, 0x01);

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero)); // Should be set if the result is 0

    // Test Sub flag
    cpu.mem[cpu.pc as usize] = opcode;
    cpu.set_flag(Flag::Sub);

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Sub)); // Should always be set

    // Test HalfCarry flag
    cpu.mem[cpu.pc as usize] = opcode;
    cpu.set_flag(Flag::HalfCarry);
    reg_setter(cpu, 0b0001_1111);

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::HalfCarry)); // Should be reset if there's no carry on bit 3

    cpu.mem[cpu.pc as usize] = opcode;
    cpu.reset_flag(Flag::HalfCarry);
    reg_setter(cpu, 0b0001_0000);

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::HalfCarry)); // Should be set if there's borrow on bit 3
}

#[test]
fn test_dec_a() {
    _test_dec_reg(opcodes::DEC_A, |cpu| cpu.a, |cpu, n| cpu.a = n);
}

#[test]
fn test_dec_b() {
    _test_dec_reg(opcodes::DEC_B, |cpu| cpu.b, |cpu, n| cpu.b = n);
}

#[test]
fn test_dec_c() {
    _test_dec_reg(opcodes::DEC_C, |cpu| cpu.c, |cpu, n| cpu.c = n);
}

#[test]
fn test_dec_d() {
    _test_dec_reg(opcodes::DEC_D, |cpu| cpu.d, |cpu, n| cpu.d = n);
}

#[test]
fn test_dec_e() {
    _test_dec_reg(opcodes::DEC_E, |cpu| cpu.e, |cpu, n| cpu.e = n);
}

#[test]
fn test_dec_h() {
    _test_dec_reg(opcodes::DEC_H, |cpu| cpu.h, |cpu, n| cpu.h = n);
}

#[test]
fn test_dec_l() {
    _test_dec_reg(opcodes::DEC_L, |cpu| cpu.l, |cpu, n| cpu.l = n);
}

#[test]
fn test_dec_ahl() {
    let mut cpu = Cpu::new();

    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0x02;

    cpu.mem[0] = opcodes::DEC_AHL;

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0xfee2], 0x01);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcodes::DEC_AHL;
    cpu.mem[0xfee2] = 0x00;

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0xfee2], 0xff);

    // Test Zero flag
    cpu.mem[cpu.pc as usize] = opcodes::DEC_AHL;
    cpu.reset_flag(Flag::Zero);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0x02;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::Zero)); // Should be reset if the result is non 0

    cpu.mem[cpu.pc as usize] = opcodes::DEC_AHL;
    cpu.set_flag(Flag::Zero);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0x01;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Zero)); // Should be set if the result is 0

    // Test Sub flag
    cpu.mem[cpu.pc as usize] = opcodes::DEC_AHL;
    cpu.set_flag(Flag::Sub);

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::Sub)); // Should always be reset

    // Test HalfCarry flag
    cpu.mem[cpu.pc as usize] = opcodes::DEC_AHL;
    cpu.set_flag(Flag::HalfCarry);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0b0001_1111;

    cpu.tick().unwrap();
    assert!(!cpu.flag(Flag::HalfCarry)); // Should be reset if there's no carry on bit 3

    cpu.mem[cpu.pc as usize] = opcodes::DEC_AHL;
    cpu.reset_flag(Flag::HalfCarry);
    cpu.set_hl(0xfee2);
    cpu.mem[0xfee2] = 0b0001_0000;

    cpu.tick().unwrap();
    assert!(cpu.flag(Flag::HalfCarry)); // Should be set if there's carry on bit 3
}

fn _test_dec_reg16<G, S>(opcode: u8, reg_getter: G, reg_setter: S)
where
    G: Fn(&Cpu) -> u16,
    S: Fn(&mut Cpu, u16),
{
    let cpu = &mut Cpu::new();

    reg_setter(cpu, 0xfff9);
    cpu.mem[0] = opcode;

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0xfff8);

    // Test wrapping
    cpu.mem[cpu.pc as usize] = opcode;
    reg_setter(cpu, 0x0000);

    cpu.tick().unwrap();
    assert_eq!(reg_getter(cpu), 0xffff);

}

#[test]
fn test_dec_bc() {
    _test_dec_reg16(opcodes::DEC_BC, Cpu::get_bc, Cpu::set_bc);
}

#[test]
fn test_dec_de() {
    _test_dec_reg16(opcodes::DEC_DE, Cpu::get_de, Cpu::set_de);
}

#[test]
fn test_dec_hl() {
    _test_dec_reg16(opcodes::DEC_HL, Cpu::get_hl, Cpu::set_hl);
}

//
// ADD
//

fn _test_add_a<S>(opcode: u8, r: S)
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
    _test_add_a(opcodes::ADD_A_A, |cpu, n| cpu.a = n);
}

#[test]
fn test_add_b() {
    _test_add_a(opcodes::ADD_A_B, |cpu, n| cpu.b = n);
}

#[test]
fn test_add_c() {
    _test_add_a(opcodes::ADD_A_C, |cpu, n| cpu.c = n);
}

#[test]
fn test_add_d() {
    _test_add_a(opcodes::ADD_A_D, |cpu, n| cpu.d = n);
}

#[test]
fn test_add_e() {
    _test_add_a(opcodes::ADD_A_E, |cpu, n| cpu.e = n);
}

#[test]
fn test_add_h() {
    _test_add_a(opcodes::ADD_A_H, |cpu, n| cpu.h = n);
}

#[test]
fn test_add_l() {
    _test_add_a(opcodes::ADD_A_L, |cpu, n| cpu.l = n);
}

#[test]
fn test_add_a_hl() {
    _test_add_a(opcodes::ADD_A_HL, |cpu, value| {
        cpu.set_hl(0xffe1);
        cpu.mem[0xffe1] = value
    });
}

#[test]
fn test_add_a_d8() {
    _test_add_a(opcodes::ADD_A_D8, |cpu, value| {
        let i = (cpu.pc + 1) as usize;
        cpu.mem[i] = value;
    });
}

fn _test_add_hl<S>(opcode: u8, r: S)
where
    S: Fn(&mut Cpu, u16),
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
    cpu.set_hl(0x0000);
    r(cpu, 0x0000);
    cpu.mem[0] = opcode;
    let result = cpu.get_hl() + 0x0000;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), result);
    assert!(cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test non carry add
    //

    let cpu = &mut new_cpu();
    cpu.set_hl(0x0000);
    r(cpu, 0x0001);
    cpu.mem[0] = opcode;
    let result = cpu.get_hl() + 0x0001;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), result);
    assert!(cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test half carry add
    //

    let cpu = &mut new_cpu();
    cpu.set_hl(0x0fff);
    r(cpu, 0x0fff);
    cpu.mem[0] = opcode;
    let result = cpu.get_hl() + 0x0fff;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), result);
    assert!(cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test carry add
    //

    let cpu = &mut new_cpu();
    cpu.set_hl(0xffff);
    r(cpu, 0xffff);
    cpu.mem[0] = opcode;
    let result = cpu.get_hl().wrapping_add(0xffff);
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), result);
    assert!(cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));
}

#[test]
fn test_add_hl_bc() {
    _test_add_hl(opcodes::ADD_HL_BC, |cpu, n| cpu.set_bc(n));
}

#[test]
fn test_add_hl_de() {
    _test_add_hl(opcodes::ADD_HL_DE, |cpu, n| cpu.set_de(n));
}

#[test]
fn test_add_hl_hl() {
    _test_add_hl(opcodes::ADD_HL_HL, |cpu, n| cpu.set_hl(n));
}

#[test]
fn test_add_hl_sp() {
    _test_add_hl(opcodes::ADD_HL_SP, |cpu, n| cpu.sp = n);
}

#[test]
fn test_add_sp_r8() {
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
    cpu.sp = 0x0017;
    cpu.mem[0] = opcodes::ADD_SP_R8;
    cpu.mem[1] = 0;
    cpu.tick().unwrap();

    assert_eq!(cpu.sp, 0x0017);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test non carry positive add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0x0017;
    cpu.mem[0] = opcodes::ADD_SP_R8;
    cpu.mem[1] = 1;
    cpu.tick().unwrap();

    assert_eq!(cpu.sp, 0x0018);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test negative add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0x0017;
    cpu.mem[0] = opcodes::ADD_SP_R8;
    cpu.mem[1] = -1 as i8 as u8;
    cpu.tick().unwrap();

    assert_eq!(cpu.sp, 0x0016);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));

    //
    // Test half carry add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0x0fff;
    cpu.mem[0] = opcodes::ADD_SP_R8;
    cpu.mem[1] = 1;
    cpu.tick().unwrap();

    assert_eq!(cpu.sp, 0x1000);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test carry add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0xffff;
    cpu.mem[0] = opcodes::ADD_SP_R8;
    cpu.mem[1] = 1;
    cpu.tick().unwrap();

    assert_eq!(cpu.sp, 0);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));
}

fn _test_adc_a<S>(opcode: u8, r: S, is_a: bool)
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
    cpu.reset_flag(Flag::Carry);
    let carry = if cpu.flag(Flag::Carry) { 1 } else { 0 };
    let result = cpu.a + 0x00 + carry;
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
    let carry = if cpu.flag(Flag::Carry) { 1 } else { 0 };
    let result = cpu.a + 0x01 + carry;
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
    let carry = if cpu.flag(Flag::Carry) { 1 } else { 0 };
    let result = cpu.a + 0x0f + carry;
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
    let carry = if cpu.flag(Flag::Carry) { 1 } else { 0 };
    let result = cpu.a.wrapping_add(0xff).wrapping_add(carry);
    cpu.tick().unwrap();

    assert_eq!(cpu.a, result);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));

    if !is_a {
        let cpu = &mut new_cpu();
        cpu.a = 0x80;
        r(cpu, 0x7f);
        cpu.mem[0] = opcode;
        let carry = if cpu.flag(Flag::Carry) { 1 } else { 0 };
        let result = cpu.a.wrapping_add(0x7f).wrapping_add(carry);
        cpu.tick().unwrap();

        assert_eq!(cpu.a, result);
        assert!(cpu.flag(Flag::Zero));
        assert!(!cpu.flag(Flag::Sub));
        assert!(cpu.flag(Flag::HalfCarry));
        assert!(cpu.flag(Flag::Carry));
    }
}


#[test]
fn test_adc_a() {
    _test_adc_a(opcodes::ADC_A_A, |cpu, n| cpu.a = n, true);
}


#[test]
fn test_adc_a_b() {
    _test_adc_a(opcodes::ADC_A_B, |cpu, n| cpu.b = n, false);
}


#[test]
fn test_adc_a_c() {
    _test_adc_a(opcodes::ADC_A_C, |cpu, n| cpu.c = n, false);
}

#[test]
fn test_adc_a_d() {
    _test_adc_a(opcodes::ADC_A_D, |cpu, n| cpu.d = n, false);
}

#[test]
fn test_adc_a_e() {
    _test_adc_a(opcodes::ADC_A_E, |cpu, n| cpu.e = n, false);
}

#[test]
fn test_adc_a_h() {
    _test_adc_a(opcodes::ADC_A_H, |cpu, n| cpu.h = n, false);
}

#[test]
fn test_adc_a_l() {
    _test_adc_a(opcodes::ADC_A_L, |cpu, n| cpu.l = n, false);
}

fn test_adc_a_hl() {
    _test_adc_a(
        opcodes::ADC_A_HL,
        |cpu, value| {
            cpu.set_hl(0xffe1);
            cpu.mem[0xffe1] = value
        },
        false,
    );
}

#[test]
fn test_adc_a_d8() {
    _test_adc_a(
        opcodes::ADC_A_D8,
        |cpu, value| {
            let i = (cpu.pc + 1) as usize;
            cpu.mem[i] = value;
        },
        false,
    );
}
