#![allow(dead_code)]

use std::mem;

// XXX Think about this some more.
// Would it be cleaner to just use reader/writer functions?
// Pro: No unsafe block
// Con: Much more duplicated code (maybe a macro could avoid the duplication?)
pub enum IndReg {
    A = 1,
    F = 0,
    B = 3,
    C = 2,
    D = 5,
    E = 4,
    H = 7,
    L = 6,
}

pub enum CombReg {
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3,
}

pub struct Cpu {
    pub reg: [u16; 4],
    pub sp: u16,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: [0u16; 4],
            sp: 0,
            pc: 0,
        }
    }

    pub fn reg_ind(&mut self, reg: IndReg) -> &mut u8 {
        unsafe {
            let u8_vec: &mut [u8; 8] = mem::transmute(&mut self.reg);
            &mut u8_vec[reg as usize]
        }
    }

    pub fn reg_comb(&mut self, reg: CombReg) -> &mut u16 {
        &mut self.reg[reg as usize]
    }
}

#[test]
fn basic_reg_test() {
    let mut cpu = Cpu::new();
    *cpu.reg_ind(IndReg::A) = 0x1;
    assert!(*cpu.reg_ind(IndReg::A) == 0x1);
    assert!(*cpu.reg_comb(CombReg::AF) == 0x100);
}
