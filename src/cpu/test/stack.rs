#![cfg(test)]
use super::*;

#[test]
fn test_call_a16() {
    let mut cpu = Cpu::new();
    cpu.pc = 0xff13;
    cpu.sp = 0xfffe;

    cpu.mem[0xff13] = opcodes::CALL_A16;
    cpu.mem[0xff14] = 0x24;
    cpu.mem[0xff15] = 0x35;

    cpu.tick().unwrap();

    assert_eq!(0xfffc, cpu.sp);
    assert_eq!(0x3524, cpu.pc);
    assert_eq!(0x16, cpu.mem[0xfffe]);
    assert_eq!(0xff, cpu.mem[0xfffd]);
}

fn _test_call_cc_a16<F>(flag_setter: F, opcode: u8)
where
    F: Fn(&mut Cpu),
{
    let cpu = &mut Cpu::new();
    cpu.pc = 0xff13;
    cpu.sp = 0xfffe;
    flag_setter(cpu);

    cpu.mem[0xff13] = opcode;
    cpu.mem[0xff14] = 0x24;
    cpu.mem[0xff15] = 0x35;

    cpu.tick().unwrap();

    assert_eq!(0xfffc, cpu.sp);
    assert_eq!(0x3524, cpu.pc);
    assert_eq!(0x16, cpu.mem[0xfffe]);
    assert_eq!(0xff, cpu.mem[0xfffd]);
}

#[test]
fn test_call_z_a16() {
    _test_call_cc_a16(|cpu| cpu.set_flag(&Flag::Zero), opcodes::CALL_Z_A16);
}

#[test]
fn test_call_c_a16() {
    _test_call_cc_a16(|cpu| cpu.set_flag(&Flag::Carry), opcodes::CALL_C_A16);
}

#[test]
fn test_call_nz_a16() {
    test_call_a16();
}

#[test]
fn test_call_nc_a16() {
    test_call_a16();
}

#[test]
fn test_ret() {
    let mut cpu = Cpu::new();

    cpu.mem[0x0] = opcodes::RET;
    cpu.mem[0xfffe] = 0x24;
    cpu.mem[0xfffd] = 0x35;
    cpu.sp = 0xfffc;

    cpu.tick().unwrap();

    assert_eq!(0xfffe, cpu.sp);
    assert_eq!(0x3524, cpu.pc);
}

fn _test_ret_cc<F>(f: F, opcode: u8)
where
    F: Fn(&mut Cpu),
{
    let cpu = &mut Cpu::new();

    cpu.mem[0x0] = opcode;
    cpu.mem[0xfffe] = 0x24;
    cpu.mem[0xfffd] = 0x35;
    cpu.sp = 0xfffc;
    f(cpu);

    cpu.tick().unwrap();

    assert_eq!(0xfffe, cpu.sp);
    assert_eq!(0x3524, cpu.pc);
}

#[test]
fn test_ret_nz() {
    _test_ret_cc(|cpu| cpu.reset_flag(&Flag::Zero), opcodes::RET_NZ);
}

#[test]
fn test_ret_z() {
    _test_ret_cc(|cpu| cpu.set_flag(&Flag::Zero), opcodes::RET_Z);
}

#[test]
fn test_ret_nc() {
    _test_ret_cc(|cpu| cpu.reset_flag(&Flag::Carry), opcodes::RET_NC);
}

#[test]
fn test_ret_c() {
    _test_ret_cc(|cpu| cpu.set_flag(&Flag::Carry), opcodes::RET_C);
}

#[test]
fn test_push_stack() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;

    cpu.push_stack(&[0xff]);
    assert_eq!(cpu.mem[0x1234], 0xff);
    assert_eq!(cpu.sp, 0x1233);

    cpu.push_stack(&[0x76, 0x91]);
    assert_eq!(cpu.mem[0x1233], 0x91);
    assert_eq!(cpu.mem[0x1232], 0x76);
    assert_eq!(cpu.sp, 0x1231);
}

#[test]
fn test_push_stack_u16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;

    cpu.push_stack_u16(0xffff);
    assert_eq!(cpu.mem[0x1234], 0xff);
    assert_eq!(cpu.mem[0x1233], 0xff);
    assert_eq!(cpu.sp, 0x1232);

    cpu.push_stack_u16(0x7291);
    assert_eq!(cpu.mem[0x1232], 0x91);
    assert_eq!(cpu.mem[0x1231], 0x72);
    assert_eq!(cpu.sp, 0x1230);
}

#[test]
fn test_pop_stack() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1233;
    cpu.mem[0x1236] = 0xff;
    cpu.mem[0x1235] = 0x91;
    cpu.mem[0x1234] = 0x72;

    assert_eq!(cpu.pop_stack(2), [0x72, 0x91]);
    assert_eq!(cpu.pop_stack(1), [0xff]);
    assert_eq!(cpu.sp, 0x1233 + 3);
}

#[test]
fn test_pop_stack_u16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1233;
    cpu.mem[0x1237] = 0x91;
    cpu.mem[0x1236] = 0x72;
    cpu.mem[0x1235] = 0xff;
    cpu.mem[0x1234] = 0xff;

    assert_eq!(cpu.pop_stack_u16(), 0xffff);
    assert_eq!(cpu.pop_stack_u16(), 0x7291);
    assert_eq!(cpu.sp, 0x1233 + 4);
}

#[test]
fn test_push_a16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    cpu.set_af(0xff15);
    cpu.set_bc(0xffff);
    cpu.set_de(0x1234);
    cpu.set_hl(0xfee2);

    cpu.mem[0] = opcodes::PUSH_A16_AF;
    cpu.mem[1] = opcodes::PUSH_A16_BC;
    cpu.mem[2] = opcodes::PUSH_A16_DE;
    cpu.mem[3] = opcodes::PUSH_A16_HL;

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x1234], 0x15);
    assert_eq!(cpu.mem[0x1233], 0xff);
    assert_eq!(cpu.sp, 0x1234 - 2);

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x1232], 0xff);
    assert_eq!(cpu.mem[0x1231], 0xff);
    assert_eq!(cpu.sp, 0x1234 - 4);

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x1230], 0x34);
    assert_eq!(cpu.mem[0x122f], 0x12);
    assert_eq!(cpu.sp, 0x1234 - 6);

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x122e], 0xe2);
    assert_eq!(cpu.mem[0x122d], 0xfe);
    assert_eq!(cpu.sp, 0x1234 - 8);
}

#[test]
fn test_pop_a16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    cpu.push_stack_u16(0xfee2);
    cpu.push_stack_u16(0x1234);
    cpu.push_stack_u16(0xffff);
    cpu.push_stack_u16(0xff15);

    cpu.mem[0] = opcodes::POP_A16_AF;
    cpu.mem[1] = opcodes::POP_A16_BC;
    cpu.mem[2] = opcodes::POP_A16_DE;
    cpu.mem[3] = opcodes::POP_A16_HL;

    cpu.tick().unwrap();
    assert_eq!(cpu.get_af(), 0xff15);

    cpu.tick().unwrap();
    assert_eq!(cpu.get_bc(), 0xffff);

    cpu.tick().unwrap();
    assert_eq!(cpu.get_de(), 0x1234);

    cpu.tick().unwrap();
    assert_eq!(cpu.get_hl(), 0xfee2);

    assert_eq!(cpu.sp, 0x1234);
}
