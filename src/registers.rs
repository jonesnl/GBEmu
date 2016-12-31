#![allow(dead_code)]

pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

enum WhichByte {
    Upper,
    Lower,
}

macro_rules! _reg_get {
    ($name:ident, $reg:ident) => {
        pub fn $name(&self) -> u16 {
            self.$reg
        }
    };

    ($name:ident, $reg:ident, $which_byte:path) => {
        pub fn $name(&self) -> u8 {
            match $which_byte {
                WhichByte::Upper => (self.$reg>>8) as u8,
                WhichByte::Lower => self.$reg as u8,
            }
        }
    };
}

macro_rules! _reg_put {
    ($name:ident, $reg:ident) => {
        pub fn $name(&mut self, val: u16) {
            self.$reg = val;
        }
    };

    ($name:ident, $reg:ident, $which_byte:path) => {
        pub fn $name(&mut self, val: u8) {
            match $which_byte {
                WhichByte::Upper => {
                    self.$reg = (self.$reg & 0xff) | ((val as u16)<<8);
                },
                WhichByte::Lower => {
                    self.$reg = (self.$reg & 0xff00) | (val as u16);
                },
            }
        }
    };
}

use self::WhichByte::*;
impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        }
    }

    // First byte functions
    _reg_get!(get_a, af, Upper);
    _reg_get!(get_f, af, Lower);
    _reg_get!(get_b, bc, Upper);
    _reg_get!(get_c, bc, Lower);
    _reg_get!(get_d, de, Upper);
    _reg_get!(get_e, de, Lower);
    _reg_get!(get_h, hl, Upper);
    _reg_get!(get_l, hl, Lower);

    _reg_put!(put_a, af, Upper);
    _reg_put!(put_f, af, Lower);
    _reg_put!(put_b, bc, Upper);
    _reg_put!(put_c, bc, Lower);
    _reg_put!(put_d, de, Upper);
    _reg_put!(put_e, de, Lower);
    _reg_put!(put_h, hl, Upper);
    _reg_put!(put_l, hl, Lower);

    // Now two byte functions
    _reg_get!(get_af, af);
    _reg_get!(get_bc, bc);
    _reg_get!(get_de, de);
    _reg_get!(get_hl, hl);
    _reg_get!(get_sp, sp);
    _reg_get!(get_pc, pc);

    _reg_put!(put_af, af);
    _reg_put!(put_bc, bc);
    _reg_put!(put_de, de);
    _reg_put!(put_hl, hl);
    _reg_put!(put_sp, sp);
    _reg_put!(put_pc, pc);
}

#[test]
fn basic_reg_test() {
    let mut regs = Registers::new();
    regs.put_a(0x1);
    assert_eq!(regs.get_a(), 0x1);
    assert_eq!(regs.get_f(), 0x00);
    regs.put_af(0x1112);
    assert_eq!(regs.get_a(), 0x11);
    assert_eq!(regs.get_f(), 0x12);

    regs.put_a(0x20);
    assert_eq!(regs.get_a(), 0x20);
    assert_eq!(regs.get_f(), 0x12);

    regs.put_f(0x30);
    assert_eq!(regs.get_a(), 0x20);
    assert_eq!(regs.get_f(), 0x30);
}
