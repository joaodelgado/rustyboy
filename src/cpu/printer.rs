use cpu::opcodes;
use cpu::Cpu;
use u8_to_u16;

pub struct Printer<'a> {
    cpu: &'a Cpu,
}

impl<'a> Printer<'a> {
    pub fn new(cpu: &'a Cpu) -> Printer<'a> {
        Printer { cpu }
    }

    fn peek_byte(&self, addr: usize) -> u8 {
        self.cpu.mem[addr + 1]
    }

    fn print_byte(&self, addr: usize) -> String {
        format!("{:02x}", self.cpu.mem[addr + 1])
    }

    fn print_8_rel(&self, addr: usize) -> String {
        format!("r_{}", self.print_byte(addr))
    }

    fn print_8_imm(&self, addr: usize) -> String {
        format!("d_{}", self.print_byte(addr))
    }

    fn print_8_sig(&self, addr: usize) -> String {
        format!("s_{:02x}", self.cpu.mem[addr + 1] as i8)
    }

    fn print_16_imm(&self, addr: usize) -> String {
        let fst_byte = self.cpu.mem[addr + 2];
        let snd_byte = self.cpu.mem[addr + 1];

        format!("d_{:04x}", u8_to_u16(fst_byte, snd_byte))
    }

    fn read_16_addr(&self, addr: usize) -> String {
        let snd_byte = self.cpu.mem[addr + 1];
        let fst_byte = self.cpu.mem[addr + 2];

        format!("a_{:04x}", u8_to_u16(fst_byte, snd_byte))
    }

    pub fn print_instr(&self, addr: u16) {
        let addr = addr as usize;

        if self.peek_byte(addr) == opcodes::CB {
            self.print_cbprefixed(addr)
        } else {
            self.print_unprefixed(addr)
        }
    }

    fn print_unprefixed(&self, addr: usize) {
        let opcode = self.cpu.mem[addr];
        print!("({:02x}) ", opcode);
        match opcode {
            opcodes::CALL_A16 => println!("CALL\t{}", self.read_16_addr(addr)),
            opcodes::CALL_NZ_A16 => println!("CALL\tNZ,{}", self.read_16_addr(addr)),
            opcodes::CALL_Z_A16 => println!("CALL\tZ,{}", self.read_16_addr(addr)),
            opcodes::CALL_NC_A16 => println!("CALL\tNC,{}", self.read_16_addr(addr)),
            opcodes::CALL_C_A16 => println!("CALL\tC,{}", self.read_16_addr(addr)),

            opcodes::INC_A => println!("INC\tA"),
            opcodes::INC_B => println!("INC\tB"),
            opcodes::INC_C => println!("INC\tC"),
            opcodes::INC_D => println!("INC\tD"),
            opcodes::INC_E => println!("INC\tE"),
            opcodes::INC_H => println!("INC\tH"),
            opcodes::INC_L => println!("INC\tL"),
            opcodes::INC_AHL => println!("INC\t(HL)"),
            opcodes::INC_BC => println!("INC\tBC"),
            opcodes::INC_DE => println!("INC\tDE"),
            opcodes::INC_HL => println!("INC\tHL"),
            opcodes::INC_SP => println!("INC\tSP"),

            opcodes::DEC_A => println!("DEC\tA"),
            opcodes::DEC_B => println!("DEC\tB"),
            opcodes::DEC_C => println!("DEC\tC"),
            opcodes::DEC_D => println!("DEC\tD"),
            opcodes::DEC_E => println!("DEC\tE"),
            opcodes::DEC_H => println!("DEC\tH"),
            opcodes::DEC_L => println!("DEC\tL"),
            opcodes::DEC_AHL => println!("DEC\t(HL)"),
            opcodes::DEC_BC => println!("DEC\tBC"),
            opcodes::DEC_DE => println!("DEC\tDE"),
            opcodes::DEC_HL => println!("DEC\tHL"),
            opcodes::DEC_SP => println!("DEC\tSP"),

            opcodes::LD_BC_A => println!("LD\t(BC),A"),
            opcodes::LD_DE_A => println!("LD\t(DE),A"),
            opcodes::LD_A16_A => println!("LD\t{},A", self.read_16_addr(addr)),

            opcodes::LD_A_A => println!("LD\tA,A"),
            opcodes::LD_A_B => println!("LD\tA,B"),
            opcodes::LD_A_C => println!("LD\tA,C"),
            opcodes::LD_A_D => println!("LD\tA,D"),
            opcodes::LD_A_E => println!("LD\tA,E"),
            opcodes::LD_A_H => println!("LD\tA,H"),
            opcodes::LD_A_L => println!("LD\tA,L"),
            opcodes::LD_A_HL => println!("LD\tA,(HL)"),
            opcodes::LD_A_BC => println!("LD\tA,(BC)"),
            opcodes::LD_A_DE => println!("LD\tA,(DE)"),
            opcodes::LD_A_A16 => println!("LD\tA,{}", self.read_16_addr(addr)),
            opcodes::LD_B_A => println!("LD\tB,A"),
            opcodes::LD_B_B => println!("LD\tB,B"),
            opcodes::LD_B_C => println!("LD\tB,C"),
            opcodes::LD_B_D => println!("LD\tB,D"),
            opcodes::LD_B_E => println!("LD\tB,E"),
            opcodes::LD_B_H => println!("LD\tB,H"),
            opcodes::LD_B_L => println!("LD\tB,L"),
            opcodes::LD_B_HL => println!("LD\tB,(HL)"),
            opcodes::LD_C_B => println!("LD\tC,B,"),
            opcodes::LD_C_C => println!("LD\tC,C"),
            opcodes::LD_C_D => println!("LD\tC,D"),
            opcodes::LD_C_E => println!("LD\tC,E"),
            opcodes::LD_C_H => println!("LD\tC,H"),
            opcodes::LD_C_L => println!("LD\tC,L"),
            opcodes::LD_C_HL => println!("LD\tC,(HL)"),
            opcodes::LD_D_B => println!("LD\tD,B,"),
            opcodes::LD_D_C => println!("LD\tD,C"),
            opcodes::LD_D_D => println!("LD\tD,D"),
            opcodes::LD_D_E => println!("LD\tD,E"),
            opcodes::LD_D_H => println!("LD\tD,H"),
            opcodes::LD_D_L => println!("LD\tD,L"),
            opcodes::LD_D_HL => println!("LD\tD,(HL)"),
            opcodes::LD_E_B => println!("LD\tE,B,"),
            opcodes::LD_E_C => println!("LD\tE,C"),
            opcodes::LD_E_D => println!("LD\tE,D"),
            opcodes::LD_E_E => println!("LD\tE,E"),
            opcodes::LD_E_H => println!("LD\tE,H"),
            opcodes::LD_E_L => println!("LD\tE,L"),
            opcodes::LD_E_HL => println!("LD\tE,(HL)"),
            opcodes::LD_H_B => println!("LD\tH,B,"),
            opcodes::LD_H_C => println!("LD\tH,C"),
            opcodes::LD_H_D => println!("LD\tH,D"),
            opcodes::LD_H_E => println!("LD\tH,E"),
            opcodes::LD_H_H => println!("LD\tH,H"),
            opcodes::LD_H_L => println!("LD\tH,L"),
            opcodes::LD_H_HL => println!("LD\tH,(HL)"),
            opcodes::LD_H_A => println!("LD\tH,A"),
            opcodes::LD_L_B => println!("LD\tL,B,"),
            opcodes::LD_L_C => println!("LD\tL,C"),
            opcodes::LD_L_D => println!("LD\tL,D"),
            opcodes::LD_L_E => println!("LD\tL,E"),
            opcodes::LD_L_H => println!("LD\tL,H"),
            opcodes::LD_L_L => println!("LD\tL,L"),
            opcodes::LD_L_HL => println!("LD\tL,(HL)"),
            opcodes::LD_L_A => println!("LD\tL,A"),
            opcodes::LD_HL_A => println!("LD\t(HL),A"),
            opcodes::LD_HL_B => println!("LD\t(HL),B,"),
            opcodes::LD_HL_C => println!("LD\t(HL),C"),
            opcodes::LD_HL_D => println!("LD\t(HL),D"),
            opcodes::LD_HL_E => println!("LD\t(HL),E"),
            opcodes::LD_HL_H => println!("LD\t(HL),H"),
            opcodes::LD_HL_L => println!("LD\t(HL),L"),
            opcodes::LD_HL_D8 => println!("LD\t(HL),{}", self.print_8_imm(addr)),
            opcodes::LD_HL_D16 => println!("LD\tHL,{}", self.print_16_imm(addr)),
            opcodes::LD_A_D8 => println!("LD\tA,{}", self.print_8_imm(addr)),
            opcodes::LD_BC_D16 => println!("LD\tBC,{}", self.print_16_imm(addr)),
            opcodes::LD_DE_D16 => println!("LD\tDE,{}", self.print_16_imm(addr)),
            opcodes::LD_SP_D16 => println!("LD\tSP,{}", self.print_16_imm(addr)),
            opcodes::LD_SP_HL => println!("LD\tSP,HL"),
            opcodes::LD_A_FF00C => println!("LD\tA,(C)"),
            opcodes::LD_FF00C_A => println!("LD\t(C),A"),

            opcodes::LDH_A8_A => println!("LDH\ta_{},A", self.print_byte(addr)),
            opcodes::LDH_A_A8 => println!("LDH\tA,a_{}", self.print_byte(addr)),
            opcodes::LDI_A_HL => println!("LDI\tA,(HL)"),
            opcodes::LDI_HL_A => println!("LDI\t(HL),A"),
            opcodes::LDD_HL_A => println!("LDD\t(HL),A"),
            opcodes::LDD_A_HL => println!("LDD\tA,(HL)"),

            opcodes::JP_A16 => println!("JP\t{}", self.read_16_addr(addr)),
            opcodes::JP_HL => println!("JP\tHL"),
            opcodes::JP_NZ_A16 => println!("JP\tNZ,{}", self.read_16_addr(addr)),
            opcodes::JP_Z_A16 => println!("JP\tZ,{}", self.read_16_addr(addr)),
            opcodes::JP_NC_A16 => println!("JP\tNC,{}", self.read_16_addr(addr)),
            opcodes::JP_C_A16 => println!("JP\tC,{}", self.read_16_addr(addr)),

            opcodes::JR_R8 => println!("JR\t{}", self.print_8_rel(addr)),
            opcodes::JR_NZ_R8 => println!("JR\tNZ,{}", self.print_8_rel(addr)),
            opcodes::JR_Z_R8 => println!("JR\tZ,{}", self.print_8_rel(addr)),
            opcodes::JR_NC_R8 => println!("JR\tNC,{}", self.print_8_rel(addr)),
            opcodes::JR_C_R8 => println!("JR\tC,{}", self.print_8_rel(addr)),

            opcodes::PUSH_A16_AF => println!("PUSH\tAF"),
            opcodes::PUSH_A16_BC => println!("PUSH\tBC"),
            opcodes::PUSH_A16_DE => println!("PUSH\tDE"),
            opcodes::PUSH_A16_HL => println!("PUSH\tHL"),

            opcodes::POP_A16_AF => println!("POP\tAF"),
            opcodes::POP_A16_BC => println!("POP\tBC"),
            opcodes::POP_A16_DE => println!("POP\tDE"),
            opcodes::POP_A16_HL => println!("POP\tHL"),

            opcodes::LD_B_D8 => println!("LD\tB,{}", self.print_8_imm(addr)),
            opcodes::LD_C_D8 => println!("LD\tC,{}", self.print_8_imm(addr)),
            opcodes::LD_D_D8 => println!("LD\tD,{}", self.print_8_imm(addr)),
            opcodes::LD_E_D8 => println!("LD\tE,{}", self.print_8_imm(addr)),
            opcodes::LD_H_D8 => println!("LD\tH,{}", self.print_8_imm(addr)),
            opcodes::LD_L_D8 => println!("LD\tL,{}", self.print_8_imm(addr)),

            opcodes::RET => println!("RET"),
            opcodes::RET_NZ => println!("RET NZ"),
            opcodes::RET_Z => println!("RET Z"),
            opcodes::RET_NC => println!("RET NC"),
            opcodes::RET_C => println!("RET C"),

            opcodes::DI => println!("DI"),
            opcodes::NOP => println!("NOP"),

            opcodes::ADD_A_A => println!("ADD\tA,A"),
            opcodes::ADD_A_B => println!("ADD\tA,B"),
            opcodes::ADD_A_C => println!("ADD\tA,C"),
            opcodes::ADD_A_D => println!("ADD\tA,D"),
            opcodes::ADD_A_E => println!("ADD\tA,E"),
            opcodes::ADD_A_H => println!("ADD\tA,H"),
            opcodes::ADD_A_L => println!("ADD\tA,L"),
            opcodes::ADD_A_HL => println!("ADD\tA,HL"),
            opcodes::ADD_A_D8 => println!("ADD\tA,{}", self.print_8_imm(addr)),
            opcodes::ADD_HL_BC => println!("ADD\tHL,BC"),
            opcodes::ADD_HL_DE => println!("ADD\tHL,DE"),
            opcodes::ADD_HL_HL => println!("ADD\tHL,HL"),
            opcodes::ADD_HL_SP => println!("ADD\tHL,SP"),
            opcodes::ADD_SP_R8 => println!("ADD\tSP,{}", self.print_8_sig(addr)),

            opcodes::SUB_A_A => println!("SUB\tA,A"),
            opcodes::SUB_A_B => println!("SUB\tA,B"),
            opcodes::SUB_A_C => println!("SUB\tA,C"),
            opcodes::SUB_A_D => println!("SUB\tA,D"),
            opcodes::SUB_A_E => println!("SUB\tA,E"),
            opcodes::SUB_A_H => println!("SUB\tA,H"),
            opcodes::SUB_A_L => println!("SUB\tA,L"),
            opcodes::SUB_A_HL => println!("SUB\tA,HL"),
            opcodes::SUB_A_D8 => println!("SUB\tA,{}", self.print_8_imm(addr)),

            opcodes::AND_A_A => println!("AND\tA,A"),
            opcodes::AND_A_B => println!("AND\tA,B"),
            opcodes::AND_A_C => println!("AND\tA,C"),
            opcodes::AND_A_D => println!("AND\tA,D"),
            opcodes::AND_A_E => println!("AND\tA,E"),
            opcodes::AND_A_H => println!("AND\tA,H"),
            opcodes::AND_A_L => println!("AND\tA,L"),
            opcodes::AND_A_HL => println!("AND\tA,HL"),
            opcodes::AND_A_D8 => println!("AND\tA,{}", self.print_8_imm(addr)),

            opcodes::XOR_A_A => println!("XOR\tA,A"),
            opcodes::XOR_A_B => println!("XOR\tA,B"),
            opcodes::XOR_A_C => println!("XOR\tA,C"),
            opcodes::XOR_A_D => println!("XOR\tA,D"),
            opcodes::XOR_A_E => println!("XOR\tA,E"),
            opcodes::XOR_A_H => println!("XOR\tA,H"),
            opcodes::XOR_A_L => println!("XOR\tA,L"),
            opcodes::XOR_A_HL => println!("XOR\tA,HL"),
            opcodes::XOR_A_D8 => println!("XOR\tA,{}", self.print_8_imm(addr)),

            opcodes::OR_A_A => println!("OR\tA,A"),
            opcodes::OR_A_B => println!("OR\tA,B"),
            opcodes::OR_A_C => println!("OR\tA,C"),
            opcodes::OR_A_D => println!("OR\tA,D"),
            opcodes::OR_A_E => println!("OR\tA,E"),
            opcodes::OR_A_H => println!("OR\tA,H"),
            opcodes::OR_A_L => println!("OR\tA,L"),
            opcodes::OR_A_HL => println!("OR\tA,HL"),
            opcodes::OR_A_D8 => println!("OR\tA,{}", self.print_8_imm(addr)),

            opcodes::CP_A => println!("CP\tA,A"),
            opcodes::CP_B => println!("CP\tA,B"),
            opcodes::CP_C => println!("CP\tA,C"),
            opcodes::CP_D => println!("CP\tA,D"),
            opcodes::CP_E => println!("CP\tA,E"),
            opcodes::CP_H => println!("CP\tA,H"),
            opcodes::CP_L => println!("CP\tA,L"),
            opcodes::CP_HL => println!("CP\tA,{}", self.read_16_addr(addr)),
            opcodes::CP_D8 => println!("CP\tA,{}", self.print_8_imm(addr)),
            opcodes::ADC_A_A => println!("ADC\tA,A"),
            opcodes::ADC_A_B => println!("ADC\tA,B"),
            opcodes::ADC_A_C => println!("ADC\tA,C"),
            opcodes::ADC_A_D => println!("ADC\tA,D"),
            opcodes::ADC_A_E => println!("ADC\tA,E"),
            opcodes::ADC_A_H => println!("ADC\tA,H"),
            opcodes::ADC_A_L => println!("ADC\tA,L"),
            opcodes::ADC_A_HL => println!("ADC\tA,(HL)"),
            opcodes::ADC_A_D8 => println!("ADC\tA,{}", self.print_8_imm(addr)),
            opcodes::RLCA => println!("RLCA"),
            opcodes::RRCA => println!("RRCA"),

            opcodes::LDHL_SP_R8 => println!("LDHL\tSP,{}", self.print_8_sig(addr)),
            opcodes::LD_A16_SP => println!("LD\t{},SP", self.read_16_addr(addr)),

            opcodes::RST_00 => println!("RST\t0x00"),
            opcodes::RST_08 => println!("RST\t0x08"),
            opcodes::RST_10 => println!("RST\t0x10"),
            opcodes::RST_18 => println!("RST\t0x18"),
            opcodes::RST_20 => println!("RST\t0x20"),
            opcodes::RST_28 => println!("RST\t0x28"),
            opcodes::RST_30 => println!("RST\t0x30"),
            opcodes::RST_38 => println!("RST\t0x38"),

            opcodes::EI => println!("EI"),

            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
                println!("Undefined instruction {:02x}", opcode)
            }

            n => panic!("Unknown instruction {:02x}@{:04x}", n, addr),
        }
    }

    fn print_cbprefixed(&self, addr: usize) {
        let opcode = self.cpu.mem[addr];
        print!("({:02x}) ", opcode);
        match opcode {
            n => panic!("Unknown instruction {:02x}@{:04x}", n, addr),
        }
    }
}
