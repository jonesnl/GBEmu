

pub struct Cpu {
    regA: u8,
    regF: u8,
    regB: u8,
    regC: u8,
    regD: u8,
    regE: u8,
    regH: u8,
    regL: u8,
    sp: u16,
    pc: u16,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            regA: 0,
            regF: 0,
            regB: 0,
            regC: 0,
            regD: 0,
            regE: 0,
            regH: 0,
            regL: 0,
            sp: 0,
            pc: 0,
        }
    }
}
