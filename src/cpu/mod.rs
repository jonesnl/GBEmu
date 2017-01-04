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

fn type_a_reg_val(cpu: &Cpu, opcode: &u8) -> u8 {
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

fn set_result_flags(cpu: &mut Cpu, new_val: u16) {
    cpu.regs.put_flag_z(new_val == 0);
    cpu.regs.put_flag_h((1 & (new_val >> 4)) == 1); 
    cpu.regs.put_flag_c((1 & (new_val >> 8)) == 1);
}

pub fn add_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_val(&cpu, &opcode) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val + arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(false);

    cpu.regs.put_a(new_val as u8);
    Ok(())
}

pub fn sub_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let arg_val = type_a_reg_val(&cpu, &opcode) as u16;
    let a_val = cpu.regs.get_a() as u16;
    let new_val = a_val - arg_val;

    set_result_flags(cpu, new_val);
    cpu.regs.put_flag_n(true);

    cpu.regs.put_a(new_val as u8);
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
