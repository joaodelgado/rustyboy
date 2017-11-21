#![cfg(test)]
use super::*;

mod alu;
mod jump;
mod logic;
mod stack;
mod load;

#[test]
fn test_get_flag() {
    let mut cpu = Cpu::new();
    cpu.init();

    cpu.status = 0xff;
    assert!(cpu.flag(Flag::Zero));
    assert!(cpu.flag(Flag::Sub));
    assert!(cpu.flag(Flag::HalfCarry));
    assert!(cpu.flag(Flag::Carry));

    // This assumes that `Flag::mask()` is implemented correctly
    cpu.status = Flag::Zero.mask();
    assert!(cpu.flag(Flag::Zero));
    cpu.status = !cpu.status;
    assert!(!cpu.flag(Flag::Zero));

    cpu.status = Flag::Sub.mask();
    assert!(cpu.flag(Flag::Sub));
    cpu.status = !cpu.status;
    assert!(!cpu.flag(Flag::Sub));

    cpu.status = Flag::HalfCarry.mask();
    assert!(cpu.flag(Flag::HalfCarry));
    cpu.status = !cpu.status;
    assert!(!cpu.flag(Flag::HalfCarry));

    cpu.status = Flag::Carry.mask();
    assert!(cpu.flag(Flag::Carry));
    cpu.status = !cpu.status;
    assert!(!cpu.flag(Flag::Carry));
}

#[test]
fn test_set_flag() {
    let mut cpu = Cpu::new();
    cpu.init();

    // This assumes that `Flag::mask()` is implemented correctly
    cpu.status = 0x00;
    cpu.set_flag(Flag::Zero);
    assert_eq!(cpu.status, Flag::Zero.mask());
    cpu.status = !Flag::Zero.mask();
    cpu.set_flag_to(Flag::Zero, true);
    assert_eq!(cpu.status, 0xff);

    cpu.status = 0x00;
    cpu.set_flag(Flag::Sub);
    assert_eq!(cpu.status, Flag::Sub.mask());
    cpu.status = !Flag::Sub.mask();
    cpu.set_flag_to(Flag::Sub, true);
    assert_eq!(cpu.status, 0xff);

    cpu.status = 0x00;
    cpu.set_flag(Flag::HalfCarry);
    assert_eq!(cpu.status, Flag::HalfCarry.mask());
    cpu.status = !Flag::HalfCarry.mask();
    cpu.set_flag_to(Flag::HalfCarry, true);
    assert_eq!(cpu.status, 0xff);

    cpu.status = 0x00;
    cpu.set_flag(Flag::Carry);
    assert_eq!(cpu.status, Flag::Carry.mask());
    cpu.status = !Flag::Carry.mask();
    cpu.set_flag_to(Flag::Carry, true);
    assert_eq!(cpu.status, 0xff);
}

#[test]
fn test_reset_flag() {
    let mut cpu = Cpu::new();
    cpu.init();

    // This assumes that `Flag::mask()` is implemented correctly
    cpu.status = 0xff;
    cpu.reset_flag(Flag::Zero);
    assert_eq!(cpu.status, !Flag::Zero.mask());
    cpu.status = Flag::Zero.mask();
    cpu.set_flag_to(Flag::Zero, false);
    assert_eq!(cpu.status, 0x00);

    cpu.status = 0xff;
    cpu.reset_flag(Flag::Sub);
    assert_eq!(cpu.status, !Flag::Sub.mask());
    cpu.status = Flag::Sub.mask();
    cpu.set_flag_to(Flag::Sub, false);
    assert_eq!(cpu.status, 0x00);

    cpu.status = 0xff;
    cpu.reset_flag(Flag::HalfCarry);
    assert_eq!(cpu.status, !Flag::HalfCarry.mask());
    cpu.status = Flag::HalfCarry.mask();
    cpu.set_flag_to(Flag::HalfCarry, false);
    assert_eq!(cpu.status, 0x00);

    cpu.status = 0xff;
    cpu.reset_flag(Flag::Carry);
    assert_eq!(cpu.status, !Flag::Carry.mask());
    cpu.status = Flag::Carry.mask();
    cpu.set_flag_to(Flag::Carry, false);
    assert_eq!(cpu.status, 0x00);
}

#[test]
fn test_get_af() {
    let mut cpu = Cpu::new();
    cpu.a = 0x3f;
    cpu.status = 0x7c;

    assert_eq!(cpu.get_af(), 0x3f7c);
}

#[test]
fn test_set_af() {
    let mut cpu = Cpu::new();
    cpu.set_af(0x3f7c);

    assert_eq!(cpu.a, 0x3f);
    assert_eq!(cpu.status, 0x7c);
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

#[test]
fn test_clone() {
    let mut cpu = Cpu::new();

    cpu.pc = 0x40;
    cpu.sp = 0xff12;
    cpu.mem[0] = 0xff;

    let clone = cpu.clone();

    assert_eq!(clone.pc, 0x40);
    assert_eq!(clone.sp, 0xff12);
    assert_eq!(clone.mem[0], 0xff);

    cpu.pc = 0x41;
    cpu.sp = 0;
    cpu.mem[0] = 0xf0;

    assert_eq!(clone.pc, 0x40);
    assert_eq!(clone.sp, 0xff12);
    assert_eq!(clone.mem[0], 0xff);
}


#[test]
fn test_load_from() {
    let mut cpu = Cpu::new();

    cpu.pc = 0x40;
    cpu.sp = 0xff12;
    cpu.mem[0] = 0xff;

    let mut cpu2 = Cpu::new();
    cpu2.load_from(&cpu);

    assert_eq!(cpu2.pc, 0x40);
    assert_eq!(cpu2.sp, 0xff12);
    assert_eq!(cpu2.mem[0], 0xff);
}
