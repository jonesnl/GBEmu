#![allow(dead_code)]

mod cpu;
mod debugger;
mod display;
mod hw;
mod registers;

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::cpu::Cpu;
use crate::hw::controller::MBC1;
use crate::hw::lcd::LcdControllerMode;
use crate::hw::memory::Bus;
use crate::hw::memory::Memory;

use rgb::ComponentBytes;
use structopt::StructOpt;

use glium::glutin;

// log_stderr for per instruction register prints?

thread_local! {
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

fn dump_memory(cpu: &Cpu) {
    let mut line_output = Vec::<String>::new();
    for i in (0..=0xfff0).step_by(0x10) {
        print!("{:04x}: ", i);
        (0..=0xf)
            .map(|x| format!("{:02x}", cpu.read8(i + x)))
            .for_each(|s| line_output.push(s));
        println!("{}", line_output.join(" "));
        line_output.clear();
    }
}

fn init_cpu(rom_path: &Path) -> Result<Cpu, String> {
    let mut file = File::open(rom_path).unwrap();

    let mut rom = Vec::new();

    match file.read_to_end(&mut rom) {
        Ok(_) => (),
        Err(m) => {
            println!("Error loading game: {}", m);
            return Err(format!("Error loading game: {}", m));
        }
    }
    // From this point on the rom should never be modified
    let rom = rom;

    let new_cartridge: Box<dyn Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    Ok(Cpu::new(new_memory))
}

fn main() {
    let opts = EmuOpts::from_args();
    let path = &opts.rom_path;

    if opts.verbose {
        set_verbose();
    }

    let mut cpu = match init_cpu(path) {
        Ok(cpu) => cpu,
        Err(string) => {
            println!("{}", string);
            return;
        }
    };

    let mut closed = false;

    let mut events_loop = glutin::EventsLoop::new();
    let mut display = display::init_display(&mut events_loop);
    let program = display::create_program(&mut display);
    let mut debugger = debugger::Debugger::new();

    while !closed {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => return,
            },
            _ => return,
        });

        debugger.tick(&mut cpu);

        cpu.execute_instr().unwrap();
        cpu.memory.io.lcd.tick_update();
        // thread::sleep(time::Duration::from_micros(10));

        if cpu.memory.io.lcd.drawing_state == LcdControllerMode::VerticalBlank(4560) {
            let lcd_vec = cpu.memory.io.lcd.lcd_display.as_bytes().to_vec();
            display::draw(&mut display, &program, lcd_vec, (160, 144));
        }
    }
}
