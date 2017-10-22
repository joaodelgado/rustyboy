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

fn test_push_16() {
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

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x1232], 0xff);
    assert_eq!(cpu.mem[0x1231], 0xff);

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x1230], 0x34);
    assert_eq!(cpu.mem[0x122f], 0x12);

    cpu.tick().unwrap();
    assert_eq!(cpu.mem[0x122e], 0xe2);
    assert_eq!(cpu.mem[0x122d], 0xfe);
}


fn test_pop_a16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    cpu.push_stack_u16(0xff15);
    cpu.push_stack_u16(0xffff);
    cpu.push_stack_u16(0x1234);
    cpu.push_stack_u16(0xfee2);

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
}
