#![allow(dead_code)]

mod instr_arrays;

use crate::emu_log;
use crate::hw::memory::{Bus, BusWidth, Memory};
use crate::registers::Registers;

use self::instr_arrays::*;

pub type InstructionRetType = Result<BranchResult, ()>;

pub enum BranchResult {
    NoBranch,
    BranchTaken,
    BranchNotTaken,
}

use self::BranchResult::*;

pub struct Cpu {
    pub regs: Registers,
    pub memory: Memory,
    pub global_interrupt_flag: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        Cpu {
            regs: Registers::new(),
            memory: memory,
            global_interrupt_flag: false,
        }
    }

    pub fn get_opcode(&self) -> u8 {
        let pc = self.regs.get_pc();
        self.read8(pc)
    }

    pub fn incr_pc(&mut self) {
        let pc = self.regs.get_pc();
        self.regs.put_pc(pc.wrapping_add(1));
    }

    pub fn push_u8(&mut self, val: u8) {
        let sp = self.regs.get_sp();
        self.write8(sp, val);
        self.regs.put_sp(sp.wrapping_sub(1));
    }

    pub fn push_u16(&mut self, val: u16) {
        let sp = self.regs.get_sp();
        self.write16(sp, val);
        self.regs.put_sp(sp.wrapping_sub(2));
    }

    pub fn pop_u8(&mut self) -> u8 {
        let new_sp = self.regs.get_sp().wrapping_add(1);
        self.regs.put_sp(new_sp);
        self.read8(new_sp)
    }

    pub fn pop_u16(&mut self) -> u16 {
        let new_sp = self.regs.get_sp().wrapping_add(2);
        self.regs.put_sp(new_sp);
        self.read16(new_sp)
    }

    pub fn jump(&mut self, addr: u16) {
        self.regs.put_pc(addr.wrapping_sub(1));
    }

    pub fn execute_instr(&mut self) -> InstructionRetType {
        // Get instruction (XXX expand)
        // Look up instruction in instruction table
        // Execute instruction
        let opcode = self.get_opcode();
        emu_log!("PC: {:02x}, Instr: {:02x}", self.regs.get_pc(), opcode);
        let result = (INSTR[opcode as usize].func)(self);
        // wait
        self.incr_pc();
        result
    }
}

impl Bus for Cpu {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        self.memory.write8(addr, data);
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        self.memory.read8(addr)
    }

    fn write16(&mut self, addr: BusWidth, data: u16) {
        self.memory.write16(addr, data);
    }

    fn read16(&self, addr: BusWidth) -> u16 {
        self.memory.read16(addr)
    }
}

pub fn noop_instr(_: &mut Cpu) -> InstructionRetType {
    Ok(NoBranch)
}

// TODO
pub fn stop_instr(_: &mut Cpu) -> InstructionRetType {
    Err(())
}

// TODO
pub fn halt_instr(_: &mut Cpu) -> InstructionRetType {
    Err(())
}

// TODO
pub fn ei_instr(cpu: &mut Cpu) -> InstructionRetType {
    cpu.global_interrupt_flag = true;
    Ok(NoBranch)
}

// TODO
pub fn di_instr(cpu: &mut Cpu) -> InstructionRetType {
    cpu.global_interrupt_flag = false;
    Ok(NoBranch)
}

// TODO
pub fn undef_instr(_: &mut Cpu) -> InstructionRetType {
    Err(())
}

fn get_type_a_reg(cpu: &Cpu, opcode: u8) -> u8 {
    match opcode & 0x7 {
        0 => cpu.regs.get_b(),
        1 => cpu.regs.get_c(),
        2 => cpu.regs.get_d(),
        3 => cpu.regs.get_e(),
        4 => cpu.regs.get_h(),
        5 => cpu.regs.get_l(),
        6 => cpu.read8(cpu.regs.get_hl()),
        7 => cpu.regs.get_a(),
        _ => panic!("Unrecognized register!"),
    }
}

fn put_type_a_reg(cpu: &mut Cpu, opcode: u8, val: u8) {
    match opcode & 0x7 {
        0 => cpu.regs.put_b(val),
        1 => cpu.regs.put_c(val),
        2 => cpu.regs.put_d(val),
        3 => cpu.regs.put_e(val),
        4 => cpu.regs.put_h(val),
        5 => cpu.regs.put_l(val),
        6 => {
            let hl = cpu.regs.get_hl();
            cpu.write8(hl, val);
        }
        7 => cpu.regs.put_a(val),
        _ => panic!("Unrecognized register!"),
    }
}

fn type_a_reg_or_imm(cpu: &mut Cpu, opcode: u8, imm_opcode: u8) -> u8 {
    if imm_opcode == opcode {
        cpu.incr_pc();
        cpu.get_opcode()
    } else {
        get_type_a_reg(cpu, opcode)
    }
}

// XXX half carry logic is incorrect
fn set_result_flags(cpu: &mut Cpu, new_val: u16) {
    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_h((1 & (new_val >> 4)) == 1);
    cpu.regs.put_flag_c((1 & (new_val >> 8)) == 1);
}

pub fn add_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xc6) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val + arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(false);

    cpu.regs.put_a(new_val as u8);
    Ok(NoBranch)
}

pub fn add_hl_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let hl_val = cpu.regs.get_hl() as u32;
    let arg_val = match opcode {
        0x09 => cpu.regs.get_bc(),
        0x19 => cpu.regs.get_de(),
        0x29 => cpu.regs.get_hl(),
        0x39 => cpu.regs.get_sp(),
        ____ => panic!("unrecognized opcode {}", opcode),
    } as u32;

    let new_val = hl_val + arg_val;

    let h_flag = {
        let mask = (1 << 12) - 1;
        if (hl_val & mask) + (arg_val & mask) > mask {
            true
        } else {
            false
        }
    };

    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(h_flag);
    cpu.regs.put_flag_c(new_val >> 16 != 0);

    cpu.regs.put_hl(new_val as u16);
    Ok(NoBranch)
}

pub fn add_sp_instr(cpu: &mut Cpu) -> InstructionRetType {
    cpu.incr_pc();
    let imm_val = cpu.get_opcode() as i8;
    let old_sp = cpu.regs.get_sp();
    let new_sp = u16_plus_i8(old_sp, imm_val);

    let h_flag = (old_sp >> 4) & 1 != (new_sp >> 4) & 1;
    let c_flag = (old_sp >> 7) != (new_sp >> 7);

    cpu.regs.put_sp(new_sp);
    cpu.regs.put_flag_z(false);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(h_flag);
    cpu.regs.put_flag_c(c_flag);

    Ok(NoBranch)
}

pub fn adc_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xce) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let carry_val = match cpu.regs.get_flag_c() {
        true => 1,
        false => 0,
    };
    let new_val = a_val + arg_val + carry_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(false);

    cpu.regs.put_a(new_val as u8);
    Ok(NoBranch)
}

pub fn sub_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xd6) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val - arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(true);

    cpu.regs.put_a(new_val as u8);
    Ok(NoBranch)
}

pub fn sbc_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xde) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let carry_val = match cpu.regs.get_flag_c() {
        true => 1,
        false => 0,
    };
    let new_val = a_val - (arg_val + carry_val);

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(true);

    cpu.regs.put_a(new_val as u8);
    Ok(NoBranch)
}

pub fn and_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xe6);
    let a_val = cpu.regs.get_a();
    let new_val = a_val & arg_val;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(true);
    cpu.regs.put_flag_c(false);

    cpu.regs.put_a(new_val);
    Ok(NoBranch)
}

pub fn xor_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xee);
    let a_val = cpu.regs.get_a();
    let new_val = a_val ^ arg_val;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(false);

    cpu.regs.put_a(new_val);
    Ok(NoBranch)
}

pub fn or_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xf6);
    let a_val = cpu.regs.get_a();
    let new_val = a_val | arg_val;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(false);

    cpu.regs.put_a(new_val);
    Ok(NoBranch)
}

pub fn cp_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xfe) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val.wrapping_sub(arg_val);

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(true);

    Ok(NoBranch)
}

/************* Misc. Arithmatic Instructions ************/

fn u8_inc_dec_get_val(cpu: &Cpu, opcode: u8) -> u8 {
    // inc opcodes are even, whereas dec opcodes are odd and +1 from their
    // inc counterparts. We can use a bitmask to make a single test for
    // both opcodes
    match opcode & 0xfe {
        0x04 => cpu.regs.get_b(),
        0x0c => cpu.regs.get_c(),
        0x14 => cpu.regs.get_d(),
        0x1c => cpu.regs.get_e(),
        0x24 => cpu.regs.get_h(),
        0x2c => cpu.regs.get_l(),
        0x34 => {
            let addr = cpu.regs.get_hl();
            cpu.read8(addr)
        }
        0x3c => cpu.regs.get_a(),
        ____ => panic!("Unrecognized inc/dec opcode {}", opcode),
    }
}

fn u8_inc_dec_put_val(cpu: &mut Cpu, opcode: u8, new_val: u8) {
    // inc opcodes are even, whereas dec opcodes are odd and +1 from their
    // inc counterparts. We can use a bitmask to make a single test for
    // both opcodes
    match opcode & 0xfe {
        0x04 => cpu.regs.put_b(new_val),
        0x0c => cpu.regs.put_c(new_val),
        0x14 => cpu.regs.put_d(new_val),
        0x1c => cpu.regs.put_e(new_val),
        0x24 => cpu.regs.put_h(new_val),
        0x2c => cpu.regs.put_l(new_val),
        0x34 => {
            let addr = cpu.regs.get_hl();
            cpu.write8(addr, new_val);
        }
        0x3c => cpu.regs.put_a(new_val),
        ____ => panic!("Unrecognized inc/dec opcode {}", opcode),
    };
}

fn set_inc_dec_result_flags(cpu: &mut Cpu, _old_val: u8, new_val: u8) {
    cpu.regs.put_flag_z(new_val == 0);
    // XXX Fix half carry computation
    cpu.regs.put_flag_h((new_val >> 4) & 1 == 1);
}

pub fn inc_u8_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = u8_inc_dec_get_val(cpu, opcode);
    let new_val = old_val + 1;

    set_inc_dec_result_flags(cpu, old_val, new_val);
    cpu.regs.put_flag_n(false);

    u8_inc_dec_put_val(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn dec_u8_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = u8_inc_dec_get_val(cpu, opcode);
    let new_val = old_val.wrapping_sub(1);

    set_inc_dec_result_flags(cpu, old_val, new_val);
    cpu.regs.put_flag_n(true);

    u8_inc_dec_put_val(cpu, opcode, new_val);

    Ok(NoBranch)
}

fn u16_inc_dec_get_val(cpu: &Cpu, opcode: u8) -> u16 {
    match opcode & 0xf0 {
        0x00 => cpu.regs.get_bc(),
        0x10 => cpu.regs.get_de(),
        0x20 => cpu.regs.get_hl(),
        0x30 => cpu.regs.get_sp(),
        ____ => panic!("Unrecognized u16 inc/dec opcode: {}"),
    }
}

fn u16_inc_dec_put_val(cpu: &mut Cpu, opcode: u8, new_val: u16) {
    match opcode & 0xf0 {
        0x00 => cpu.regs.put_bc(new_val),
        0x10 => cpu.regs.put_de(new_val),
        0x20 => cpu.regs.put_hl(new_val),
        0x30 => cpu.regs.put_sp(new_val),
        ____ => panic!("Unrecognized u16 inc/dec opcode: {}"),
    }
}

pub fn inc_u16_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = u16_inc_dec_get_val(cpu, opcode);
    let new_val = old_val + 1;
    u16_inc_dec_put_val(cpu, opcode, new_val);
    Ok(NoBranch)
}

pub fn dec_u16_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = u16_inc_dec_get_val(cpu, opcode);
    let new_val = old_val - 1;
    u16_inc_dec_put_val(cpu, opcode, new_val);
    Ok(NoBranch)
}

/************* Shift/Rotate instructions ******************/

pub fn rlca_instr(cpu: &mut Cpu) -> InstructionRetType {
    let a_val = cpu.regs.get_a();
    let new_a_val = a_val.rotate_left(1);

    cpu.regs.put_a(new_a_val);
    cpu.regs.put_flag_z(new_a_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c((a_val >> 7) == 1);

    Ok(NoBranch)
}

pub fn rla_instr(cpu: &mut Cpu) -> InstructionRetType {
    let a_val = cpu.regs.get_a();
    let old_c_flag = cpu.regs.get_flag_c();

    let new_a_val = (a_val << 1) | (old_c_flag as u8);

    cpu.regs.put_a(new_a_val);
    cpu.regs.put_flag_z(new_a_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c((a_val >> 7) == 1);

    Ok(NoBranch)
}

pub fn rrca_instr(cpu: &mut Cpu) -> InstructionRetType {
    let a_val = cpu.regs.get_a();
    let new_a_val = a_val.rotate_right(1);

    cpu.regs.put_a(new_a_val);
    cpu.regs.put_flag_z(new_a_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c((a_val & 1) == 1);

    Ok(NoBranch)
}

pub fn rrc_instr(cpu: &mut Cpu) -> InstructionRetType {
    let a_val = cpu.regs.get_a();
    let old_c_flag = cpu.regs.get_flag_c();

    let new_a_val = (a_val >> 1) | ((old_c_flag as u8) << 7);

    cpu.regs.put_a(new_a_val);
    cpu.regs.put_flag_z(new_a_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c((a_val & 1) == 1);

    Ok(NoBranch)
}

pub fn rlc_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_val = old_val.rotate_left(1);

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(old_val >> 7 == 1);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn rcc_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_val = old_val.rotate_right(1);

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(old_val & 1 == 1);

    Ok(NoBranch)
}

pub fn rl_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_carry = (old_val >> 7) == 1;
    let mut new_val = old_val << 1;
    new_val |= cpu.regs.get_flag_c() as u8;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(new_carry);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn rr_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_carry = old_val & 1 == 1;
    let mut new_val = old_val >> 1;
    new_val |= (cpu.regs.get_flag_c() as u8) << 7;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(new_carry);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn sla_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_carry = (old_val >> 7) != 0;
    let new_val = old_val << 1;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(new_carry);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn sra_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_carry = (old_val & 1) != 0;
    let mut new_val = old_val >> 1;
    new_val |= old_val & 0b1000_0000;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(new_carry);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn swap_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let old_upper_nibble = old_val >> 4;
    let old_lower_nibble = old_val & 0b1111;
    let new_val = (old_upper_nibble) | (old_lower_nibble << 4);

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(false);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

pub fn srl_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let old_val = get_type_a_reg(cpu, opcode);

    let new_carry = (old_val & 1) != 0;
    let new_val = old_val >> 1;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(new_carry);
    put_type_a_reg(cpu, opcode, new_val);

    Ok(NoBranch)
}

macro_rules! _bit {
    ($name:ident, $bit:expr) => {
        pub fn $name(cpu: &mut Cpu) -> InstructionRetType {
            let opcode = cpu.get_opcode();
            let test_val = get_type_a_reg(cpu, opcode);

            let test_bit_mask = 1 << $bit;
            cpu.regs.put_flag_z((test_val & test_bit_mask) != 0);
            cpu.regs.put_flag_n(false);
            cpu.regs.put_flag_h(true);

            Ok(NoBranch)
        }
    };
}

_bit!(bit0_instr, 0);
_bit!(bit1_instr, 1);
_bit!(bit2_instr, 2);
_bit!(bit3_instr, 3);
_bit!(bit4_instr, 4);
_bit!(bit5_instr, 5);
_bit!(bit6_instr, 6);
_bit!(bit7_instr, 7);

macro_rules! _res {
    ($name:ident, $bit:expr) => {
        pub fn $name(cpu: &mut Cpu) -> InstructionRetType {
            let opcode = cpu.get_opcode();
            let old_val = get_type_a_reg(cpu, opcode);

            put_type_a_reg(cpu, opcode, old_val & !(1 << $bit));

            Ok(NoBranch)
        }
    };
}

_res!(res0_instr, 0);
_res!(res1_instr, 1);
_res!(res2_instr, 2);
_res!(res3_instr, 3);
_res!(res4_instr, 4);
_res!(res5_instr, 5);
_res!(res6_instr, 6);
_res!(res7_instr, 7);

macro_rules! _set {
    ($name:ident, $bit:expr) => {
        pub fn $name(cpu: &mut Cpu) -> InstructionRetType {
            let opcode = cpu.get_opcode();
            let old_val = get_type_a_reg(cpu, opcode);

            put_type_a_reg(cpu, opcode, old_val | (1 << $bit));

            Ok(NoBranch)
        }
    };
}

_set!(set0_instr, 0);
_set!(set1_instr, 1);
_set!(set2_instr, 2);
_set!(set3_instr, 3);
_set!(set4_instr, 4);
_set!(set5_instr, 5);
_set!(set6_instr, 6);
_set!(set7_instr, 7);

/************* Load instructions *****************/

pub fn ld_u8_imm_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    cpu.incr_pc();
    let imm_val = cpu.get_opcode();
    match opcode {
        0x06 => cpu.regs.put_b(imm_val),
        0x0e => cpu.regs.put_c(imm_val),
        0x16 => cpu.regs.put_d(imm_val),
        0x1e => cpu.regs.put_e(imm_val),
        0x26 => cpu.regs.put_h(imm_val),
        0x2e => cpu.regs.put_l(imm_val),
        0x36 => {
            let reg_hl = cpu.regs.get_hl();
            cpu.write8(reg_hl, imm_val);
        }
        0x3e => cpu.regs.put_a(imm_val),
        ____ => panic!("Unrecognized ld_u8_imm opcode {}", opcode),
    };
    Ok(NoBranch)
}

pub fn ld_u16_imm_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    cpu.incr_pc();
    let lower_imm_val = cpu.get_opcode() as u16;
    cpu.incr_pc();
    let upper_imm_val = cpu.get_opcode() as u16;
    let imm_val = (upper_imm_val << 8) | lower_imm_val;

    match opcode {
        0x01 => cpu.regs.put_bc(imm_val),
        0x11 => cpu.regs.put_de(imm_val),
        0x21 => cpu.regs.put_hl(imm_val),
        0x31 => cpu.regs.put_sp(imm_val),
        ____ => panic!("Unrecognized ld_u16_imm opcode {}", opcode),
    };
    Ok(NoBranch)
}

pub fn ld_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let from_val = get_type_a_reg(cpu, opcode);
    match (opcode >> 3) & 0x7 {
        0 => cpu.regs.put_b(from_val),
        1 => cpu.regs.put_c(from_val),
        2 => cpu.regs.put_d(from_val),
        3 => cpu.regs.put_e(from_val),
        4 => cpu.regs.put_h(from_val),
        5 => cpu.regs.put_l(from_val),
        6 => {
            let reg_hl = cpu.regs.get_hl();
            cpu.write8(reg_hl, from_val);
        }
        7 => cpu.regs.put_a(from_val),
        _ => panic!("Unrecognized ld opcode {}", opcode),
    }
    Ok(NoBranch)
}

pub fn ld_from_mem_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let from_addr = match opcode {
        0x0A => cpu.regs.get_bc(),
        0x1A => cpu.regs.get_de(),
        0x2A => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val + 1);
            val
        }
        0x3A => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val - 1);
            val
        }
        ____ => panic!("Unrecognized ld_from_mem opcode {}", opcode),
    };
    let from_val = cpu.read8(from_addr);
    cpu.regs.put_a(from_val);
    Ok(NoBranch)
}

pub fn ld_to_mem_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let to_addr = match opcode {
        0x02 => cpu.regs.get_bc(),
        0x12 => cpu.regs.get_de(),
        0x22 => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val + 1);
            val
        }
        0x32 => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val - 1);
            val
        }
        ____ => panic!("Unrecognized ld_to_mem opcode {}", opcode),
    };
    let a_val = cpu.regs.get_a();
    cpu.write8(to_addr, a_val);
    Ok(NoBranch)
}

pub fn ld_sp_to_imm_mem_instr(cpu: &mut Cpu) -> InstructionRetType {
    cpu.incr_pc();
    let to_addr_lower = cpu.get_opcode() as u16;
    cpu.incr_pc();
    let to_addr_upper = cpu.get_opcode() as u16;

    let to_addr = (to_addr_upper << 8) | to_addr_lower;
    let value = cpu.regs.get_sp();
    cpu.write16(to_addr, value);
    Ok(NoBranch)
}

fn ld_specialized_mem_addr(cpu: &mut Cpu, opcode: u8) -> u16 {
    match opcode & 0x0f {
        0x0 => {
            cpu.incr_pc();
            let imm_val = cpu.get_opcode() as u16;
            0xff00 | imm_val
        }
        0x2 => 0xff00 | (cpu.regs.get_c() as u16),
        0xa => {
            cpu.incr_pc();
            let addr_lower = cpu.get_opcode() as u16;
            cpu.incr_pc();
            let addr_upper = cpu.get_opcode() as u16;
            (addr_upper << 8) | addr_lower
        }
        ____ => panic!("Unrecognized specialized_mem_addr opcode {}", opcode),
    }
}

pub fn ld_from_mem_to_a_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let mem_addr = ld_specialized_mem_addr(cpu, opcode);
    let mem_val = cpu.read8(mem_addr);
    cpu.regs.put_a(mem_val);
    Ok(NoBranch)
}

pub fn ld_from_a_to_mem_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let mem_addr = ld_specialized_mem_addr(cpu, opcode);
    let a_val = cpu.regs.get_a();
    cpu.write8(mem_addr, a_val);
    Ok(NoBranch)
}

pub fn ld_hl_to_sp_instr(cpu: &mut Cpu) -> InstructionRetType {
    let hl_val = cpu.regs.get_hl();
    cpu.regs.put_sp(hl_val);
    Ok(NoBranch)
}

// XXX: There must be a better way to add signed and unsigned numbers while
// allowing wrapping. TODO also be sure to test this!!!
fn u16_plus_i8(val1_u16: u16, val2_i8: i8) -> u16 {
    let val2_i16 = val2_i8 as i16;
    if val2_i8 < 0 {
        val1_u16.wrapping_sub(val2_i16.wrapping_neg() as u16)
    } else {
        val1_u16.wrapping_add(val2_i16 as u16)
    }
}

pub fn ld_sp_plus_signed_imm_to_hl_instr(cpu: &mut Cpu) -> InstructionRetType {
    let sp_val = cpu.regs.get_sp();
    cpu.incr_pc();
    let signed_imm = cpu.get_opcode() as i8;
    let new_sp_val = u16_plus_i8(sp_val, signed_imm);
    cpu.regs.put_hl(new_sp_val);
    Ok(NoBranch)
}

/*********** Control Flow *************/

pub fn jr_imm8_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();
    let orig_pc = cpu.regs.get_pc();

    let should_jump = match opcode {
        0x18 => true,
        0x20 => !cpu.regs.get_flag_z(),
        0x28 => cpu.regs.get_flag_z(),
        0x30 => !cpu.regs.get_flag_c(),
        0x38 => cpu.regs.get_flag_c(),
        ____ => panic!("Unrecognized jr opcode {}", opcode),
    };

    cpu.incr_pc();
    let imm_val = cpu.get_opcode() as i8;

    if !should_jump {
        return Ok(BranchNotTaken);
    }

    let new_pc = u16_plus_i8(orig_pc.wrapping_add(2), imm_val);
    cpu.jump(new_pc);
    Ok(BranchTaken)
}

pub fn jp_imm16_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let should_jump = match opcode {
        0xc3 => true,
        0xc2 => !cpu.regs.get_flag_z(),
        0xca => cpu.regs.get_flag_z(),
        0xd2 => !cpu.regs.get_flag_c(),
        0xda => cpu.regs.get_flag_c(),
        ____ => panic!("Unrecognized jp opcode {}", opcode),
    };

    cpu.incr_pc();
    let lower_imm_val = cpu.get_opcode() as u16;
    cpu.incr_pc();
    let upper_imm_val = cpu.get_opcode() as u16;

    if !should_jump {
        return Ok(BranchNotTaken);
    }

    let new_pc = (upper_imm_val << 8) | lower_imm_val;
    cpu.jump(new_pc);
    Ok(BranchTaken)
}

pub fn jp_hl_instr(cpu: &mut Cpu) -> InstructionRetType {
    let addr = cpu.regs.get_hl();
    let jump_addr = cpu.read16(addr);

    cpu.jump(jump_addr);
    Ok(BranchNotTaken)
}

pub fn restart_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let restart_addr = match opcode {
        0xc7 => 0x00,
        0xcf => 0x08,
        0xd7 => 0x10,
        0xdf => 0x18,
        0xe7 => 0x20,
        0xef => 0x28,
        0xf7 => 0x30,
        0xff => 0x38,
        ____ => panic!("Unrecognized rst opcode {}", opcode),
    } as u16;

    cpu.jump(restart_addr);
    Ok(NoBranch)
}

pub fn call_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let should_call = match opcode {
        0xc4 => !cpu.regs.get_flag_z(),
        0xcc => cpu.regs.get_flag_z(),
        0xcd => true,
        0xd4 => !cpu.regs.get_flag_c(),
        0xdc => cpu.regs.get_flag_c(),
        ____ => panic!("Unrecognized call opcode {}", opcode),
    };

    if !should_call {
        return Ok(BranchNotTaken);
    }

    let pc = cpu.regs.get_pc();
    let jump_addr = cpu.read16(pc.wrapping_add(1));

    cpu.push_u16(pc.wrapping_add(3));

    cpu.jump(jump_addr);

    Ok(BranchTaken)
}

pub fn ret_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let should_return = match opcode {
        0xc0 => !cpu.regs.get_flag_z(),
        0xc8 => cpu.regs.get_flag_z(),
        0xc9 => true,
        0xd0 => !cpu.regs.get_flag_c(),
        0xd8 => cpu.regs.get_flag_c(),
        0xd9 => {
            // TODO enable interrupts here
            true
        }
        ____ => panic!("Unrecognized ret opcode {}", opcode),
    };

    if !should_return {
        return Ok(BranchNotTaken);
    }

    let ret_addr = cpu.pop_u16();

    cpu.jump(ret_addr);

    Ok(BranchTaken)
}

/*********** Special code ********************/

pub fn push_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let val = match opcode {
        0xc5 => cpu.regs.get_bc(),
        0xd5 => cpu.regs.get_de(),
        0xe5 => cpu.regs.get_hl(),
        0xf5 => cpu.regs.get_af(),
        ____ => panic!("Unrecognized push instruction {}", opcode),
    };

    cpu.push_u16(val);

    Ok(NoBranch)
}

pub fn pop_instr(cpu: &mut Cpu) -> InstructionRetType {
    let opcode = cpu.get_opcode();

    let val = cpu.pop_u16();

    match opcode {
        0xc1 => cpu.regs.put_bc(val),
        0xd1 => cpu.regs.put_de(val),
        0xe1 => cpu.regs.put_hl(val),
        0xf1 => cpu.regs.put_af(val),
        ____ => panic!("Unrecognized pop instruction {}", opcode),
    };

    Ok(NoBranch)
}

pub fn daa_instr(cpu: &mut Cpu) -> InstructionRetType {
    let a_val = cpu.regs.get_a();
    let mut new_a_val = a_val % 10;
    let upper_a_val = a_val / 10;

    new_a_val |= (upper_a_val % 10) << 4;
    println!("{} {} {}", a_val, new_a_val, upper_a_val);
    cpu.regs.put_a(new_a_val);
    Ok(NoBranch)
}

pub fn cpl_instr(cpu: &mut Cpu) -> InstructionRetType {
    let a_val = cpu.regs.get_a();
    let new_a_val = !a_val;

    cpu.regs.put_a(new_a_val);
    cpu.regs.put_flag_n(true);
    cpu.regs.put_flag_h(true);

    Ok(NoBranch)
}

pub fn scf_instr(cpu: &mut Cpu) -> InstructionRetType {
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(true);

    Ok(NoBranch)
}

pub fn ccf_instr(cpu: &mut Cpu) -> InstructionRetType {
    let old_c_flag = cpu.regs.get_flag_c();

    cpu.regs.put_flag_c(!old_c_flag);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);

    Ok(NoBranch)
}

pub fn cb_instr(cpu: &mut Cpu) -> InstructionRetType {
    // Get next instruction
    cpu.incr_pc();
    let result = (CB_INSTR[cpu.get_opcode() as usize].func)(cpu);
    result
}

/*********** Tests ************/

#[allow(unused_macros)]
macro_rules! setup_test {
    ( $( $x:expr ),* ) => {
        {
            use crate::hw::controller::MBC1;
            let mut rom = vec![0u8;0x100];
            $(
                rom.push($x);
            )*
            rom.push(0xFD);
            rom.resize(0xFFFF, 0x00);
            let new_cartridge: Box<dyn Bus> = MBC1::new(rom);
            let new_memory = Memory::new(new_cartridge);
            Cpu::new(new_memory)
        }
    };
}

#[test]
fn five_noops() {
    let mut cpu = setup_test![0x00, 0x00, 0x00, 0x00, 0x00];

    loop {
        if cpu.execute_instr().is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_pc(), 0x106);
}

#[test]
fn add() {
    let mut cpu = setup_test![0x87];
    cpu.regs.put_a(1);

    loop {
        if cpu.execute_instr().is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 2);
}

#[test]
fn sub() {
    let mut cpu = setup_test![0x97];
    cpu.regs.put_a(1);
    loop {
        if cpu.execute_instr().is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0);
}

#[test]
fn ld_imm() {
    let mut cpu = setup_test![0x3e, 0x10, 0x06, 0x20];
    loop {
        if cpu.execute_instr().is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0x10);
    assert_eq!(cpu.regs.get_b(), 0x20);
}

#[test]
fn ld() {
    let mut cpu = setup_test![0x3e, 0xff, 0x47, 0x48];
    loop {
        if cpu.execute_instr().is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0xff);
    assert_eq!(cpu.regs.get_b(), 0xff);
    assert_eq!(cpu.regs.get_c(), 0xff);
    assert_eq!(cpu.regs.get_d(), 0x00);
}

#[test]
fn daa() {
    let mut cpu = setup_test![0x3e, 0x10, 0x27];
    loop {
        if cpu.execute_instr().is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0x16);
}
