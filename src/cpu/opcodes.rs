pub const CALL_A16: u8 = 0xcd;
pub const DI: u8 = 0xf3;
pub const JP_A16: u8 = 0xc3;
pub const JP_C_A16: u8 = 0xda;
pub const JP_HL: u8 = 0xe9;
pub const JP_NC_A16: u8 = 0xd2;
pub const JP_NZ_A16: u8 = 0xc2;
pub const JP_Z_A16: u8 = 0xca;
pub const JR_R8: u8 = 0x18;
pub const LDH_A8_A: u8 = 0xe0;
pub const LD_BC_A: u8 = 0x02;
pub const LD_DE_A: u8 = 0x12;
pub const LD_HL_A: u8 = 0x77;
pub const LD_A16_A: u8 = 0xea;
pub const LD_A_D8: u8 = 0x3e;
pub const LD_A_A: u8 = 0x7f;
pub const LD_A_B: u8 = 0x78;
pub const LD_A_C: u8 = 0x79;
pub const LD_A_D: u8 = 0x7a;
pub const LD_A_E: u8 = 0x7b;
pub const LD_A_H: u8 = 0x7c;
pub const LD_A_L: u8 = 0x7d;
pub const LD_A_BC: u8 = 0x0a;
pub const LD_A_DE: u8 = 0x1a;
pub const LD_A_HL: u8 = 0x7e;
pub const LD_A_A16: u8 = 0xfa;
pub const LD_B_B: u8 = 0x40;
pub const LD_B_C: u8 = 0x41;
pub const LD_B_D: u8 = 0x42;
pub const LD_B_E: u8 = 0x43;
pub const LD_B_H: u8 = 0x44;
pub const LD_B_L: u8 = 0x45;
pub const LD_B_HL: u8 = 0x46;
pub const LD_SP_HL: u8 = 0xf9;
pub const LD_BC_D16: u8 = 0x01;
pub const LD_DE_D16: u8 = 0x11;
pub const LD_HL_D16: u8 = 0x21;
pub const LD_SP_D16: u8 = 0x31;
pub const LDI_A_HL: u8 = 0x2a;
pub const PUSH_A16_AF: u8 = 0xf5;
pub const PUSH_A16_BC: u8 = 0xc5;
pub const PUSH_A16_DE: u8 = 0xd5;
pub const PUSH_A16_HL: u8 = 0xe5;
pub const POP_A16_AF: u8 = 0xf1;
pub const POP_A16_BC: u8 = 0xc1;
pub const POP_A16_DE: u8 = 0xd1;
pub const POP_A16_HL: u8 = 0xe1;
pub const NOP: u8 = 0x00;
pub const RET: u8 = 0xc9;
pub const INC_A: u8 = 0x3c;
pub const INC_B: u8 = 0x4c;
pub const INC_C: u8 = 0x0c;
pub const INC_D: u8 = 0x14;
pub const INC_E: u8 = 0x1c;
pub const INC_H: u8 = 0x24;
pub const INC_L: u8 = 0x2c;
pub const INC_AHL: u8 = 0x34;
pub const INC_BC: u8 = 0x03;
pub const INC_DE: u8 = 0x13;
pub const INC_HL: u8 = 0x23;
pub const INC_SP: u8 = 0x33;
pub const OR_A_A: u8 = 0xb7;
pub const OR_A_B: u8 = 0xb0;
pub const OR_A_C: u8 = 0xb1;
pub const OR_A_D: u8 = 0xb2;
pub const OR_A_E: u8 = 0xb3;
pub const OR_A_H: u8 = 0xb4;
pub const OR_A_L: u8 = 0xb5;
pub const OR_A_HL: u8 = 0xb6;
pub const OR_A_D8: u8 = 0xf6;
pub const JR_NZ_R8: u8 = 0x20;
pub const JR_Z_R8: u8 = 0x28;
pub const JR_NC_R8: u8 = 0x30;
pub const JR_C_R8: u8 = 0x38;
pub const LD_B_D8: u8 = 0x06;
pub const LD_C_D8: u8 = 0x0e;
pub const LD_D_D8: u8 = 0x16;
pub const LD_E_D8: u8 = 0x1e;
pub const LD_H_D8: u8 = 0x26;
pub const LD_L_D8: u8 = 0x2e;
pub const CP_A: u8 = 0xbf;
pub const CP_B: u8 = 0xb8;
pub const CP_C: u8 = 0xb9;
pub const CP_D: u8 = 0xba;
pub const CP_E: u8 = 0xbb;
pub const CP_H: u8 = 0xbc;
pub const CP_L: u8 = 0xbd;
pub const CP_HL: u8 = 0xbe;
pub const CP_D8: u8 = 0xfe;
