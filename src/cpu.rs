#![allow(dead_code)]

use registers::Registers;
use hw::memory::{Bus, BusWidth, Memory};

const INSTR_ARRAY_SIZE: usize = 256;

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

fn noop_instr(_: &mut Cpu) -> Result<(), ()> {
    Ok(())
}

fn stop_instr(_: &mut Cpu) -> Result<(), ()> {
    Err(())
}

fn add_instr(cpu: &mut Cpu) -> Result<(), ()> {
    let opcode = cpu.get_opcode();
    let a = cpu.regs.get_a();
    let arg_val = match opcode & 0xF {
        0 => cpu.regs.get_b(),
        1 => cpu.regs.get_c(),
        2 => cpu.regs.get_d(),
        3 => cpu.regs.get_e(),
        4 => cpu.regs.get_h(),
        6 => cpu.regs.get_l(),
        7 => cpu.regs.get_a(),
        _ => panic!("Unrecognized register!"),
    };
    let new_val = arg_val + a;
    // TODO set flags
    cpu.regs.put_a(new_val);
    Ok(())
}

fn cb_instr(cpu: &mut Cpu) -> Result<(), ()> {
    // Get next instruction
    cpu.incr_pc();
    let result = CB_INSTR[cpu.get_opcode() as usize](cpu);
    result
}

static INSTR: [fn(&mut Cpu) -> Result<(), ()> ; INSTR_ARRAY_SIZE] =
        [noop_instr, // 0x00
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        stop_instr, // 0x10
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x20
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x30
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x40
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x50
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x60
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x70
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        add_instr, // 0x80
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x90
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xa0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xb0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xc0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        cb_instr, // 0xcb
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xd0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xe0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xf0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        ];

static CB_INSTR: [fn(&mut Cpu) -> Result<(), ()> ; INSTR_ARRAY_SIZE] =
        [noop_instr, // 0x00
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x10
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x20
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x30
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x40
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x50
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x60
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x70
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x80
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x90
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xa0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xb0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xc0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xd0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xe0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xf0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        ];

#[test]
fn five_noops() {
    use hw::controller::MBC1;
    let mut rom = vec![0x00; 0xFFFF];
    rom[0] = 0x00u8;
    rom[1] = 0x00;
    rom[2] = 0x00;
    rom[3] = 0x00;
    rom[4] = 0x00;
    rom[5] = 0x10;

    let new_cartridge: Box<Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    let mut cpu = Cpu::new(new_memory);
    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_pc(), 6);
}

#[test]
fn add() {
    use hw::controller::MBC1;
    let mut rom = vec![0x00; 0xFFFF];
    rom[0] = 0x00u8;
    rom[1] = 0x87;
    rom[2] = 0x10;

    let new_cartridge: Box<Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    let mut cpu = Cpu::new(new_memory);
    cpu.regs.put_a(1);
    loop {
        if execute_instruction(&mut cpu).is_err() {
            break;
        }
    }
    assert_eq!(cpu.regs.get_a(), 2);
}
