#![cfg(test)]
use super::*;

fn _test_ld_addr_r8<G, F>(r1: G, r2: F, value: u8, addr: u16, opcode: u8)
where
    F: Fn(&mut Cpu, u8),
    G: Fn(&mut Cpu, u16),
{
    let cpu = &mut Cpu::new();
    cpu.mem[0] = opcode;
    r2(cpu, value);
    r1(cpu, addr);

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[addr as usize], value);
}

#[test]
fn test_ld_bc_a() {
    _test_ld_addr_r8(
        Cpu::set_bc,
        |cpu, n| cpu.a = n,
        0x72,
        0x3401,
        opcodes::LD_BC_A,
    );
}

#[test]
fn test_ld_de_a() {
    _test_ld_addr_r8(
        Cpu::set_de,
        |cpu, n| cpu.a = n,
        0x72,
        0x3401,
        opcodes::LD_DE_A,
    );
}

#[test]
fn test_ld_hl_a() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.a = n,
        0x72,
        0x3401,
        opcodes::LD_HL_A,
    );
}

#[test]
fn test_ld_hl_b() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.b = n,
        0x72,
        0x3401,
        opcodes::LD_HL_B,
    );
}

#[test]
fn test_ld_hl_c() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.c = n,
        0x72,
        0x3401,
        opcodes::LD_HL_C,
    );
}

#[test]
fn test_ld_hl_d() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.d = n,
        0x72,
        0x3401,
        opcodes::LD_HL_D,
    );
}

#[test]
fn test_ld_hl_e() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.e = n,
        0x72,
        0x3401,
        opcodes::LD_HL_E,
    );
}

#[test]
fn test_ld_hl_h() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.h = n,
        0x34,
        0x3401,
        opcodes::LD_HL_H,
    );
}

#[test]
fn test_ld_hl_l() {
    _test_ld_addr_r8(
        Cpu::set_hl,
        |cpu, n| cpu.l = n,
        0x01,
        0x3401,
        opcodes::LD_HL_L,
    );
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

fn _test_ld_reg_reg<G, F>(r1: G, r2: F, value: u8, opcode: u8)
where
    G: Fn(&Cpu) -> u8,
    F: Fn(&mut Cpu, u8),
{
    let cpu = &mut Cpu::new();
    r2(cpu, value);
    cpu.mem[0] = opcode;
    cpu.tick().unwrap();
    assert_eq!(value, r1(cpu));
}

fn _test_ld_reg_addr<G, F>(getter: G, setter: F, value: u8, addr: u16, opcode: u8)
where
    G: Fn(&Cpu) -> u8,
    F: Fn(&mut Cpu, u16),
{
    let cpu = &mut Cpu::new();
    cpu.mem[0] = opcode;
    cpu.mem[addr as usize] = value;
    setter(cpu, addr);

    cpu.tick().unwrap();
    assert_eq!(getter(cpu), cpu.mem[addr as usize]);
}

#[test]
fn test_ld_a_a() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.a = n, 0x72, opcodes::LD_A_A);
}

#[test]
fn test_ld_a_b() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.b = n, 0x72, opcodes::LD_A_B);
}

#[test]
fn test_ld_a_c() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.c = n, 0x72, opcodes::LD_A_C);
}

#[test]
fn test_ld_a_d() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.d = n, 0x72, opcodes::LD_A_D);
}

#[test]
fn test_ld_a_e() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.e = n, 0x72, opcodes::LD_A_E);
}

#[test]
fn test_ld_a_h() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.h = n, 0x72, opcodes::LD_A_H);
}

#[test]
fn test_ld_a_l() {
    _test_ld_reg_reg(|cpu| cpu.a, |cpu, n| cpu.l = n, 0x72, opcodes::LD_A_L);
}

#[test]
fn test_ld_a_hl() {
    _test_ld_reg_addr(|cpu| cpu.a, Cpu::set_hl, 5, 0xb00b, opcodes::LD_A_HL);
}

#[test]
fn test_ld_a_bc() {
    _test_ld_reg_addr(|cpu| cpu.a, Cpu::set_bc, 5, 0xb00b, opcodes::LD_A_BC);
}

#[test]
fn test_ld_a_de() {
    _test_ld_reg_addr(|cpu| cpu.a, Cpu::set_de, 5, 0xb00b, opcodes::LD_A_DE);
}

#[test]
fn test_ld_b_a() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.a = n, 0x72, opcodes::LD_B_A);
}

#[test]
fn test_ld_b_b() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.b = n, 0x72, opcodes::LD_B_B);
}


#[test]
fn test_ld_b_c() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.c = n, 0x72, opcodes::LD_B_C);
}

#[test]
fn test_ld_b_d() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.d = n, 0x72, opcodes::LD_B_D);
}

#[test]
fn test_ld_b_e() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.e = n, 0x72, opcodes::LD_B_E);
}

#[test]
fn test_ld_b_h() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.h = n, 0x72, opcodes::LD_B_H);
}

#[test]
fn test_ld_b_l() {
    _test_ld_reg_reg(|cpu| cpu.b, |cpu, n| cpu.l = n, 0x72, opcodes::LD_B_L);
}

#[test]
fn test_ld_b_hl() {
    _test_ld_reg_addr(|cpu| cpu.b, Cpu::set_hl, 5, 0xb00b, opcodes::LD_B_HL);
}

#[test]
fn test_ld_c_b() {
    _test_ld_reg_reg(|cpu| cpu.c, |cpu, n| cpu.b = n, 0x72, opcodes::LD_C_B);
}


#[test]
fn test_ld_c_c() {
    _test_ld_reg_reg(|cpu| cpu.c, |cpu, n| cpu.c = n, 0x72, opcodes::LD_C_C);
}

#[test]
fn test_ld_c_d() {
    _test_ld_reg_reg(|cpu| cpu.c, |cpu, n| cpu.d = n, 0x72, opcodes::LD_C_D);
}

#[test]
fn test_ld_c_e() {
    _test_ld_reg_reg(|cpu| cpu.c, |cpu, n| cpu.e = n, 0x72, opcodes::LD_C_E);
}

#[test]
fn test_ld_c_h() {
    _test_ld_reg_reg(|cpu| cpu.c, |cpu, n| cpu.h = n, 0x72, opcodes::LD_C_H);
}

#[test]
fn test_ld_c_l() {
    _test_ld_reg_reg(|cpu| cpu.c, |cpu, n| cpu.l = n, 0x72, opcodes::LD_C_L);
}

#[test]
fn test_ld_c_hl() {
    _test_ld_reg_addr(|cpu| cpu.c, Cpu::set_hl, 5, 0xb00b, opcodes::LD_C_HL);
}

#[test]
fn test_ld_d_b() {
    _test_ld_reg_reg(|cpu| cpu.d, |cpu, n| cpu.b = n, 0x72, opcodes::LD_D_B);
}


#[test]
fn test_ld_d_c() {
    _test_ld_reg_reg(|cpu| cpu.d, |cpu, n| cpu.c = n, 0x72, opcodes::LD_D_C);
}

#[test]
fn test_ld_d_d() {
    _test_ld_reg_reg(|cpu| cpu.d, |cpu, n| cpu.d = n, 0x72, opcodes::LD_D_D);
}

#[test]
fn test_ld_d_e() {
    _test_ld_reg_reg(|cpu| cpu.d, |cpu, n| cpu.e = n, 0x72, opcodes::LD_D_E);
}

#[test]
fn test_ld_d_h() {
    _test_ld_reg_reg(|cpu| cpu.d, |cpu, n| cpu.h = n, 0x72, opcodes::LD_D_H);
}

#[test]
fn test_ld_d_l() {
    _test_ld_reg_reg(|cpu| cpu.d, |cpu, n| cpu.l = n, 0x72, opcodes::LD_D_L);
}

#[test]
fn test_ld_d_hl() {
    _test_ld_reg_addr(|cpu| cpu.d, Cpu::set_hl, 5, 0xb00b, opcodes::LD_D_HL);
}

#[test]
fn test_ld_e_b() {
    _test_ld_reg_reg(|cpu| cpu.e, |cpu, n| cpu.b = n, 0x72, opcodes::LD_E_B);
}


#[test]
fn test_ld_e_c() {
    _test_ld_reg_reg(|cpu| cpu.e, |cpu, n| cpu.c = n, 0x72, opcodes::LD_E_C);
}

#[test]
fn test_ld_e_d() {
    _test_ld_reg_reg(|cpu| cpu.e, |cpu, n| cpu.d = n, 0x72, opcodes::LD_E_D);
}

#[test]
fn test_ld_e_e() {
    _test_ld_reg_reg(|cpu| cpu.e, |cpu, n| cpu.e = n, 0x72, opcodes::LD_E_E);
}

#[test]
fn test_ld_e_h() {
    _test_ld_reg_reg(|cpu| cpu.e, |cpu, n| cpu.h = n, 0x72, opcodes::LD_E_H);
}

#[test]
fn test_ld_e_l() {
    _test_ld_reg_reg(|cpu| cpu.e, |cpu, n| cpu.l = n, 0x72, opcodes::LD_E_L);
}

#[test]
fn test_ld_e_hl() {
    _test_ld_reg_addr(|cpu| cpu.e, Cpu::set_hl, 5, 0xb00b, opcodes::LD_E_HL);
}

#[test]
fn test_ld_h_b() {
    _test_ld_reg_reg(|cpu| cpu.h, |cpu, n| cpu.b = n, 0x72, opcodes::LD_H_B);
}


#[test]
fn test_ld_h_c() {
    _test_ld_reg_reg(|cpu| cpu.h, |cpu, n| cpu.c = n, 0x72, opcodes::LD_H_C);
}

#[test]
fn test_ld_h_d() {
    _test_ld_reg_reg(|cpu| cpu.h, |cpu, n| cpu.d = n, 0x72, opcodes::LD_H_D);
}

#[test]
fn test_ld_h_e() {
    _test_ld_reg_reg(|cpu| cpu.h, |cpu, n| cpu.e = n, 0x72, opcodes::LD_H_E);
}

#[test]
fn test_ld_h_h() {
    _test_ld_reg_reg(|cpu| cpu.h, |cpu, n| cpu.h = n, 0x72, opcodes::LD_H_H);
}

#[test]
fn test_ld_h_l() {
    _test_ld_reg_reg(|cpu| cpu.h, |cpu, n| cpu.l = n, 0x72, opcodes::LD_H_L);
}

#[test]
fn test_ld_h_hl() {
    _test_ld_reg_addr(|cpu| cpu.h, Cpu::set_hl, 5, 0xb00b, opcodes::LD_H_HL);
}

#[test]
fn test_ld_l_b() {
    _test_ld_reg_reg(|cpu| cpu.l, |cpu, n| cpu.b = n, 0x72, opcodes::LD_L_B);
}


#[test]
fn test_ld_l_c() {
    _test_ld_reg_reg(|cpu| cpu.l, |cpu, n| cpu.c = n, 0x72, opcodes::LD_L_C);
}

#[test]
fn test_ld_l_d() {
    _test_ld_reg_reg(|cpu| cpu.l, |cpu, n| cpu.d = n, 0x72, opcodes::LD_L_D);
}

#[test]
fn test_ld_l_e() {
    _test_ld_reg_reg(|cpu| cpu.l, |cpu, n| cpu.e = n, 0x72, opcodes::LD_L_E);
}

#[test]
fn test_ld_l_h() {
    _test_ld_reg_reg(|cpu| cpu.l, |cpu, n| cpu.h = n, 0x72, opcodes::LD_L_H);
}

#[test]
fn test_ld_l_l() {
    _test_ld_reg_reg(|cpu| cpu.l, |cpu, n| cpu.l = n, 0x72, opcodes::LD_L_L);
}

#[test]
fn test_ld_l_hl() {
    _test_ld_reg_addr(|cpu| cpu.l, Cpu::set_hl, 5, 0xb00b, opcodes::LD_L_HL);
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
fn test_ld_r8_d8() {
    let mut cpu = Cpu::new();
    cpu.mem[0] = opcodes::LD_B_D8;
    cpu.mem[1] = 0x24;

    cpu.tick().unwrap();
    assert_eq!(0x24, cpu.b);

    cpu.pc = 0;
    cpu.mem[0] = opcodes::LD_C_D8;
    cpu.mem[1] = 0x35;

    cpu.tick().unwrap();
    assert_eq!(0x35, cpu.c);

    cpu.pc = 0;
    cpu.mem[0] = opcodes::LD_D_D8;
    cpu.mem[1] = 0x35;

    cpu.tick().unwrap();
    assert_eq!(0x35, cpu.d);

    cpu.pc = 0;
    cpu.mem[0] = opcodes::LD_E_D8;
    cpu.mem[1] = 0x35;

    cpu.tick().unwrap();
    assert_eq!(0x35, cpu.e);

    cpu.pc = 0;
    cpu.mem[0] = opcodes::LD_H_D8;
    cpu.mem[1] = 0x35;

    cpu.tick().unwrap();
    assert_eq!(0x35, cpu.h);

    cpu.pc = 0;
    cpu.mem[0] = opcodes::LD_L_D8;
    cpu.mem[1] = 0x35;

    cpu.tick().unwrap();
    assert_eq!(0x35, cpu.l);
}

#[test]
fn test_ld_r8_a16() {
    let mut cpu = Cpu::new();

    cpu.mem[0] = opcodes::LD_A_A16;
    cpu.mem[1] = 0x34;
    cpu.mem[2] = 0x12;
    cpu.mem[0x1234] = 15;

    cpu.tick().unwrap();
    assert_eq!(cpu.a, 15);
}

#[test]
fn test_ldhl_sp_r8() {
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
    cpu.mem[0] = opcodes::LDHL_SP_R8;
    cpu.mem[1] = 0;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), 0x0017);
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
    cpu.mem[0] = opcodes::LDHL_SP_R8;
    cpu.mem[1] = 1;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), 0x0018);
    assert_eq!(cpu.sp, 0x0017);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(!cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test negative add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0x0017;
    cpu.mem[0] = opcodes::LDHL_SP_R8;
    cpu.mem[1] = -1 as i8 as u8;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), 0x0016);
    assert_eq!(cpu.sp, 0x0017);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));

    //
    // Test half carry add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0x0fff;
    cpu.mem[0] = opcodes::LDHL_SP_R8;
    cpu.mem[1] = 1;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), 0x1000);
    assert_eq!(cpu.sp, 0x0fff);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(!cpu.flag(Flag::Carry));

    //
    // Test carry add
    //

    let cpu = &mut new_cpu();
    cpu.sp = 0xffff;
    cpu.mem[0] = opcodes::LDHL_SP_R8;
    cpu.mem[1] = 1;
    cpu.tick().unwrap();

    assert_eq!(cpu.get_hl(), 0);
    assert_eq!(cpu.sp, 0xffff);
    assert!(!cpu.flag(Flag::Zero));
    assert!(!cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));
}

#[test]
fn test_ld_a16_sp() {
    let mut cpu = Cpu::new();
    cpu.mem[0] = opcodes::LD_A16_SP;
    cpu.mem[1] = 0x12;
    cpu.mem[2] = 0x34;

    cpu.tick().unwrap();

    assert_eq!(cpu.sp, 0x3412);
}
