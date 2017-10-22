#![cfg(test)]
use super::*;

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