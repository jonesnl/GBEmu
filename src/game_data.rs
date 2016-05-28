use hw::memory::Memory;
use cpu::Cpu;

pub struct GameData {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl GameData {
    pub fn new(memory: Memory) -> GameData {
        GameData {
            cpu: Cpu::new(),
            memory: memory,
        }
    }
}
