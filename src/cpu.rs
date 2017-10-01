pub struct Cpu {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    G: u8,
    H: u8,
    sp: u16,
    pc: u16,
    flag: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        let cpu = Cpu {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            G: 0,
            H: 0,
            sp: 0x100,
            pc: 0xFFFE,
            flag: 0
        };
        cpu
    }
}
