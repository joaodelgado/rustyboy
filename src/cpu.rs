#![allow(dead_code)]

///
///  16bit Hi   Lo   Name/Function
///  AF    A    -    Accumulator & Flags
///  BC    B    C    BC
///  DE    D    E    DE
///  HL    H    L    HL
///  SP    -    -    Stack Pointer
///  PC    -    -    Program Counter/Pointer
///
#[derive(Default)]
pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    status: u8, // status flag: sign, zero, parity, carry, aux carry
}

pub enum StatusRegBit {
    Sign,
    Zero,
    Parity,
    Carry,
    AuxCarry,
}

impl Cpu {
    pub fn init(&mut self) {
        // TODO I'm assuming that we are running on a GB for now.
        // When we support multiple types, the value of the A register must change
        // accordingly:
        //  A=$01-GB/SGB, $FF-GBP, $11-GBC
        self.set_af(0x01b0);
        self.set_bc(0x0013);
        self.set_de(0x00d8);
        self.set_hl(0x014d);
    }

    fn set_af(&mut self, n: u16) {
        self.a = (n >> 8) as u8;
        self.f = n as u8;
    }

    fn set_bc(&mut self, n: u16) {
        self.b = (n >> 8) as u8;
        self.c = n as u8;
    }

    fn set_de(&mut self, n: u16) {
        self.d = (n >> 8) as u8;
        self.e = n as u8;
    }

    fn set_hl(&mut self, n: u16) {
        self.h = (n >> 8) as u8;
        self.l = n as u8;
    }

    // Check if a certain flag is set
    pub fn status_is_set(&self, bit_enum: StatusRegBit) -> bool {
        match bit_enum {
            StatusRegBit::Sign => (self.status & 0b10000000) == 0b10000000,
            StatusRegBit::Zero => (self.status & 0b01000000) == 0b01000000,
            StatusRegBit::Parity => (self.status & 0b00100000) == 0b00100000,
            StatusRegBit::Carry => (self.status & 0b00010000) == 0b00010000,
            StatusRegBit::AuxCarry => (self.status & 0b000010000) == 0b00010000,
        }
    }

    // Set the defined status flag
    pub fn status_set(&mut self, bit_enum: StatusRegBit) {
        match bit_enum {
            StatusRegBit::Sign => self.status |= 0b10000000,
            StatusRegBit::Zero => self.status |= 0b01000000,
            StatusRegBit::Parity => self.status |= 0b00100000,
            StatusRegBit::Carry => self.status |= 0b00010000,
            StatusRegBit::AuxCarry => self.status |= 0b000010000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_reg() {
        let mut cpu = Cpu::new();

        cpu.status_set(StatusRegBit::Sign);
        assert_eq!(cpu.status_is_set(StatusRegBit::Sign), true);

        cpu.status_set(StatusRegBit::Zero);
        assert_eq!(cpu.status_is_set(StatusRegBit::Zero), true);

        cpu.status_set(StatusRegBit::Parity);
        assert_eq!(cpu.status_is_set(StatusRegBit::Parity), true);

        cpu.status_set(StatusRegBit::Carry);
        assert_eq!(cpu.status_is_set(StatusRegBit::Carry), true);

        cpu.status_set(StatusRegBit::AuxCarry);
        assert_eq!(cpu.status_is_set(StatusRegBit::AuxCarry), true);

        cpu.status = 0;
        assert_eq!(cpu.status_is_set(StatusRegBit::AuxCarry), false);
    }
}
