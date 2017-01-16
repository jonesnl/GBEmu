#![allow(dead_code)]

mod instr_arrays;

use registers::Registers;
use hw::memory::{Bus, BusWidth, Memory};

use self::instr_arrays::*;

pub struct Cpu {
    regs: Registers,
    memory: Memory,
}

impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        Cpu {
            regs: Registers::new(),
            memory: memory,
        }
    }

    pub fn get_opcode(&self) -> u8 {
        let pc = self.regs.get_pc();
        self.read8(pc)
    }

    pub fn incr_pc(&mut self) {
        let pc = self.regs.get_pc();
        self.regs.put_pc(pc+1);
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

pub fn execute_instruction(cpu: &mut Cpu) -> Result<(), ()> {
    // Get instruction (XXX expand)
    // Look up instruction in instruction table
    // Execute instruction
    let result = INSTR[cpu.get_opcode() as usize](cpu);
    // wait
    cpu.incr_pc();
    result
}

pub fn noop_instr(_: &mut Cpu) -> Result<(), ()> {
    Ok(())
}

pub fn stop_instr(_: &mut Cpu) -> Result<(), ()> {
    Err(())
}

pub fn undef_instr(_: &mut Cpu) -> Result<(), ()> {
    Err(())
}

fn type_a_reg_val(cpu: &Cpu, opcode: u8) -> u8 {
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

fn type_a_reg_or_imm(cpu: &mut Cpu, opcode: u8, imm_opcode: u8) -> u8 {
    if imm_opcode == opcode {
        cpu.incr_pc();
        cpu.get_opcode()
    } else {
        type_a_reg_val(cpu, opcode)
    }
}

fn set_result_flags(cpu: &mut Cpu, new_val: u16) {
    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_h((1 & (new_val >> 4)) == 1); 
    cpu.regs.put_flag_c((1 & (new_val >> 8)) == 1);
}

pub fn add_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xc6) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val + arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(false);

    cpu.regs.put_a(new_val as u8);
    Ok(())
}

pub fn adc_instr(cpu: &mut Cpu) -> Result<(), ()> {
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
    Ok(())
}

pub fn sub_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xd6) as u16; 
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val - arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(true);

    cpu.regs.put_a(new_val as u8);
    Ok(())
}

pub fn sbc_instr(cpu: &mut Cpu) -> Result<(), ()> {
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
    Ok(())
}

pub fn and_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xe6);
    let a_val = cpu.regs.get_a();
    let new_val = a_val & arg_val;
    
    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(true);
    cpu.regs.put_flag_c(false);
    
    cpu.regs.put_a(new_val);
    Ok(())
}

pub fn xor_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xee);
    let a_val = cpu.regs.get_a();
    let new_val = a_val ^ arg_val;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(false);

    cpu.regs.put_a(new_val);
    Ok(())
}

pub fn or_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xf6);
    let a_val = cpu.regs.get_a();
    let new_val = a_val | arg_val;

    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_n(false);
    cpu.regs.put_flag_h(false);
    cpu.regs.put_flag_c(false);

    cpu.regs.put_a(new_val);
    Ok(())
}

pub fn cp_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_or_imm(cpu, opcode, 0xfe) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val - arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(true);

    Ok(())
}

pub fn ld_u8_imm_instr(cpu: &mut Cpu) -> Result<(), ()> {
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
        },
        0x3e => cpu.regs.put_a(imm_val),
        ____ => panic!("Unrecognized ld_u8_imm opcode {}", opcode),
    };
    Ok(())
}

pub fn ld_u16_imm_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();

    cpu.incr_pc();
    let lower_imm_val = cpu.get_opcode() as u16;
    cpu.incr_pc();
    let upper_imm_val = cpu.get_opcode() as u16;
    let imm_val = (upper_imm_val<<8) | lower_imm_val;

    match opcode {
        0x01 => cpu.regs.put_bc(imm_val),
        0x11 => cpu.regs.put_de(imm_val),
        0x21 => cpu.regs.put_hl(imm_val),
        0x31 => cpu.regs.put_sp(imm_val),
        ____ => panic!("Unrecognized ld_u16_imm opcode {}", opcode),
    };
    Ok(())
}

pub fn ld_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let from_val = type_a_reg_val(cpu, opcode);
    match (opcode>>3) & 0x7 {
        0 => cpu.regs.put_b(from_val),
        1 => cpu.regs.put_c(from_val),
        2 => cpu.regs.put_d(from_val),
        3 => cpu.regs.put_e(from_val),
        4 => cpu.regs.put_h(from_val),
        5 => cpu.regs.put_l(from_val),
        6 => {
            let reg_hl = cpu.regs.get_hl();
            cpu.write8(reg_hl, from_val);
        },
        7 => cpu.regs.put_a(from_val),
        _ => panic!("Unrecognized ld opcode {}", opcode),
    }
    Ok(())
}

pub fn ld_from_mem_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let from_addr = match opcode {
        0x0A => cpu.regs.get_bc(),
        0x1A => cpu.regs.get_de(),
        0x2A => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val + 1);
            val
        },
        0x3A => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val - 1);
            val
        },
        ____ => panic!("Unrecognized ld_from_mem opcode {}", opcode),
    };
    let from_val = cpu.read8(from_addr);
    cpu.regs.put_a(from_val);
    Ok(())
}

pub fn ld_to_mem_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let to_addr = match opcode {
        0x02 => cpu.regs.get_bc(),
        0x12 => cpu.regs.get_de(),
        0x22 => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val + 1);
            val
        },
        0x32 => {
            let val = cpu.regs.get_hl();
            cpu.regs.put_hl(val - 1);
            val
        },
        ____ => panic!("Unrecognized ld_to_mem opcode {}", opcode),
    };
    let a_val = cpu.regs.get_a();
    cpu.write8(to_addr, a_val);
    Ok(())
}

pub fn ld_sp_to_imm_mem_instr(cpu: &mut Cpu) -> Result<(), ()> {
    cpu.incr_pc();
    let to_addr_lower = cpu.get_opcode() as u16;
    cpu.incr_pc();
    let to_addr_upper = cpu.get_opcode() as u16;

    let to_addr = (to_addr_upper<<8) | to_addr_lower;
    let value = cpu.regs.get_sp();
    cpu.write16(to_addr, value);
    Ok(())
}

fn ld_specialized_mem_addr(cpu: &mut Cpu, opcode: u8) -> u16 {
    match opcode & 0x0f {
        0x0 => {
            cpu.incr_pc();
            let imm_val = cpu.get_opcode() as u16;
            0xff00 | imm_val
        },
        0x2 => 0xff00 | (cpu.regs.get_c() as u16),
        0xa => {
            cpu.incr_pc();
            let addr_lower = cpu.get_opcode() as u16;
            cpu.incr_pc();
            let addr_upper = cpu.get_opcode() as u16;
            (addr_upper<<8) | addr_lower
        },
        ____ => panic!("Unrecognized specialized_mem_addr opcode {}", opcode),
    }
}

pub fn ld_from_mem_to_a_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();

    let mem_addr = ld_specialized_mem_addr(cpu, opcode);
    let mem_val = cpu.read8(mem_addr);
    cpu.regs.put_a(mem_val);
    Ok(())
}

pub fn ld_from_a_to_mem_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();

    let mem_addr = ld_specialized_mem_addr(cpu, opcode);
    let a_val = cpu.regs.get_a();
    cpu.write8(mem_addr, a_val);
    Ok(())
}

pub fn ld_hl_to_sp_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let hl_val = cpu.regs.get_hl();
    cpu.regs.put_sp(hl_val);
    Ok(())
}

pub fn ld_sp_plus_signed_imm_to_hl_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let sp_val = cpu.regs.get_sp();
    cpu.incr_pc();
    let signed_imm = cpu.get_opcode() as i8;
    let sized_signed_imm = signed_imm as i16;
    // XXX: There must be a better way to add signed and unsigned numbers while
    // allowing wrapping. TODO also be sure to test this!!!
    let new_sp_val = {
        if signed_imm < 0 {sp_val.wrapping_sub((-sized_signed_imm) as u16)}
        else {sp_val.wrapping_add(signed_imm as u16)}
    };
    cpu.regs.put_hl(new_sp_val);
    Ok(())
}

/*********** Control Flow *************/

pub fn jp_instr(cpu: &mut Cpu) -> Result<(), ()> {
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
        return Ok(());
    }

    let imm_val = (upper_imm_val<<8) | lower_imm_val;
    cpu.regs.put_pc(imm_val);
    Ok(())
}

pub fn cb_instr(cpu: &mut Cpu) -> Result<(), ()> {
    // Get next instruction
    cpu.incr_pc();
    let result = CB_INSTR[cpu.get_opcode() as usize](cpu);
    result
}

/*********** Tests ************/

macro_rules! setup_test {
    ( $( $x:expr ),* ) => {
        {
            use hw::controller::MBC1;
            let mut rom = Vec::<u8>::new();
            $(
                rom.push($x);
            )*
            rom.push(0xFD);
            rom.resize(0xFFFF, 0x00);
            let new_cartridge: Box<Bus> = MBC1::new(rom);
            let new_memory = Memory::new(new_cartridge);
            Cpu::new(new_memory)
        }
    };
}

#[test]
fn five_noops() {
    let mut cpu = setup_test![
        0x00, 0x00, 0x00, 0x00, 0x00
    ];

    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_pc(), 6);
}

#[test]
fn add() {
    let mut cpu = setup_test![
        0x87
    ];
    cpu.regs.put_a(1);

    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 2);
}

#[test]
fn sub() {
    let mut cpu = setup_test![
        0x97
    ];
    cpu.regs.put_a(1);
    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0);
}

#[test]
fn ld_imm() {
    let mut cpu = setup_test![
        0x3e, 0x10, 0x06, 0x20
    ];
    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0x10);
    assert_eq!(cpu.regs.get_b(), 0x20);
}

#[test]
fn ld() {
    let mut cpu = setup_test![
        0x3e, 0xff, 0x47, 0x48
    ];
    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 0xff);
    assert_eq!(cpu.regs.get_b(), 0xff);
    assert_eq!(cpu.regs.get_c(), 0xff);
    assert_eq!(cpu.regs.get_d(), 0x00);
}
