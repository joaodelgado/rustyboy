#![cfg(test)]
use super::*;

mod inc;
mod jump;
mod logic;
mod stack;
mod load;

#[test]
fn test_status_reg() {
    let mut cpu = Cpu::new();
    cpu.init();

    cpu.set_flag(Flag::Zero);
    assert_eq!(cpu.flag(Flag::Zero), true);

    cpu.set_flag(Flag::Sub);
    assert_eq!(cpu.flag(Flag::Sub), true);

    cpu.set_flag(Flag::HalfCarry);
    assert_eq!(cpu.flag(Flag::HalfCarry), true);

    cpu.set_flag(Flag::Carry);
    assert_eq!(cpu.flag(Flag::Carry), true);
}

#[test]
fn test_get_af() {
    let mut cpu = Cpu::new();
    cpu.a = 0x3f;
    cpu.f = 0x7c;

    assert_eq!(cpu.get_af(), 0x3f7c);
}

#[test]
fn test_set_af() {
    let mut cpu = Cpu::new();
    cpu.set_af(0x3f7c);

    assert_eq!(cpu.a, 0x3f);
    assert_eq!(cpu.f, 0x7c);
}

#[test]
fn test_get_bc() {
    let mut cpu = Cpu::new();
    cpu.b = 0x3f;
    cpu.c = 0x7c;

    assert_eq!(cpu.get_bc(), 0x3f7c);
}

#[test]
fn test_set_bc() {
    let mut cpu = Cpu::new();
    cpu.set_bc(0x3f7c);

    assert_eq!(cpu.b, 0x3f);
    assert_eq!(cpu.c, 0x7c);
}

#[test]
fn test_get_de() {
    let mut cpu = Cpu::new();
    cpu.d = 0x3f;
    cpu.e = 0x7c;

    assert_eq!(cpu.get_de(), 0x3f7c);
}

#[test]
fn test_set_de() {
    let mut cpu = Cpu::new();
    cpu.set_de(0x3f7c);

    assert_eq!(cpu.d, 0x3f);
    assert_eq!(cpu.e, 0x7c);
}

#[test]
fn test_get_hl() {
    let mut cpu = Cpu::new();
    cpu.h = 0x3f;
    cpu.l = 0x7c;

    assert_eq!(cpu.get_hl(), 0x3f7c);
}

#[test]
fn test_set_hl() {
    let mut cpu = Cpu::new();
    cpu.set_hl(0x3f7c);

    assert_eq!(cpu.h, 0x3f);
    assert_eq!(cpu.l, 0x7c);
}

#[test]
fn test_push_stack() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xfffe;

    cpu.push_stack(&[0xff, 0xee, 0xcc]);

    assert_eq!(0xcc, cpu.mem[0xfffe]);
    assert_eq!(0xee, cpu.mem[0xfffd]);
    assert_eq!(0xff, cpu.mem[0xfffc]);
    assert_eq!(0xfffb, cpu.sp);
}

#[test]
fn test_push_stack_u16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xfffe;

    cpu.push_stack_u16(0xffee);

    assert_eq!(0xee, cpu.mem[0xfffe]);
    assert_eq!(0xff, cpu.mem[0xfffd]);
    assert_eq!(0xfffc, cpu.sp);
}

#[test]
fn test_pop_stack() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xfffb;

    cpu.mem[0xfffe] = 0xcc;
    cpu.mem[0xfffd] = 0xee;
    cpu.mem[0xfffc] = 0xff;

    {
        let result = cpu.pop_stack(3);
        assert_eq!([0xff, 0xee, 0xcc], result);
    }

    assert_eq!(0xfffe, cpu.sp);
}

#[test]
fn test_pop_stack_u16() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xfffc;

    cpu.mem[0xfffe] = 0xcc;
    cpu.mem[0xfffd] = 0xee;

    let result = cpu.pop_stack_u16();

    assert_eq!(0xeecc, result);
    assert_eq!(0xfffe, cpu.sp);
}

//
// Instructions
//

#[test]
fn test_di() {
    let mut cpu = Cpu::new();
    cpu.mem[0] = 0xf3;

    cpu.tick().unwrap();
}
