const RAM_SIZE: usize = 32 * 1024;
const VRAM_SIZE: usize = 16 * 1024;

struct GameBoy {
    cpu: Cpu,
    ram: [u8; RAM_SIZE],
    vram: [u8; VRAM_SIZE],
}

impl GameBoy {
    pub fn new() -> GameBoy {
        GameBoy {
            cpu: Cpu::new(),
            ram: [0; RAM_SIZE],
            vram: [0; VRAM_SIZE],
        }
    }
}
