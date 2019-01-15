#![allow(dead_code)]
#![allow(unused_imports)]

mod hw;
mod cpu;
mod registers;
mod display;

use std::env;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::cell::RefCell;

use crate::hw::controller::MBC1;
use crate::hw::memory::Bus;
use crate::hw::memory::Memory;
use crate::cpu::Cpu;
use crate::hw::lcd::LcdControllerMode;

use std::{thread, time};

use std::io::Cursor;

use rgb::ComponentBytes;
use structopt::StructOpt;

// log_stderr for per instruction register prints?

thread_local!{
    static VERBOSE: RefCell<bool> = RefCell::new(false);
}

#[macro_export]
macro_rules! emu_log {
    () => ({
        use crate::VERBOSE;
        VERBOSE.with(|f| {
            if *f.borrow() == true {
                println!();
            }
        });
    });
    ($($arg:tt)*) => ({
        use crate::VERBOSE;
        VERBOSE.with(|f| {
            if *f.borrow() == true {
                println!($($arg)*);
            }
        });
    })
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct EmuOpts {
    /// Activate verbose mode
    #[structopt(short = "v")]
    verbose: bool,

    #[structopt(parse(from_os_str))]
    rom_path: PathBuf,
}

fn set_verbose() {
    VERBOSE.with(|f| {
        *f.borrow_mut() = true;
    });
}

fn main() {
    use glium::{glutin, Surface};

    let opts = EmuOpts::from_args();
    let path = &opts.rom_path;

    if opts.verbose {
        set_verbose();
    }

    let mut events_loop = glutin::EventsLoop::new();
    let mut display = display::init_display(&mut events_loop);
    let program = display::create_program(&mut display);

    let mut file = File::open(path).unwrap();

    let mut rom = Vec::new();
    
    match file.read_to_end(&mut rom) {
        Ok(_) => (),
        Err(m) => {
            println!("Error loading game: {}", m);
            return;
        }
    }
    // From this point on the rom should never be modified
    let rom = rom;

    let new_cartridge: Box<dyn Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    let mut cpu = Cpu::new(new_memory);

    let mut closed = false;

    #[derive(PartialEq)]
    enum EmuState {
        Running,
        Step,
        Paused,
    }
    
    let mut emu_state = EmuState::Paused;
    let mut i = 0u64;
    while !closed {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput {input, ..} => {
                        match (input.scancode, input.state) {
                            (0x39, glutin::ElementState::Pressed) => emu_state = EmuState::Step,
                            (0x1C, glutin::ElementState::Pressed) => emu_state = EmuState::Running,
                            _ => (),
                        }
                    },
                    _ => return,
                },
                _ => return,
            }
        });

        if emu_state == EmuState::Paused {
            continue;
        }

        if emu_state == EmuState::Step {
            emu_log!("Instr number: {}", i);
            emu_log!("{:04x?}", cpu.regs);
            emu_log!("Flags: z: {}, c: {}, h: {}, n: {}", cpu.regs.get_flag_z(),
                       cpu.regs.get_flag_c(), cpu.regs.get_flag_h(), cpu.regs.get_flag_n());
        }
        cpu.execute_instr().unwrap();
        cpu.memory.io.lcd.tick_update();
        // thread::sleep(time::Duration::from_micros(10));

        if cpu.memory.io.lcd.drawing_state == LcdControllerMode::VerticalBlank(4560) {
            let lcd_vec = cpu.memory.io.lcd.lcd_display.as_bytes().to_vec();
            display::draw(&mut display, &program, lcd_vec, (160, 144));
        }

        if emu_state == EmuState::Step {
            emu_state = EmuState::Paused;
        }
        if emu_state == EmuState::Step {
            emu_log!("");
        }
        i += 1;
    }
}
