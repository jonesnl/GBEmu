#![allow(dead_code)]
#![allow(unused_imports)]

mod hw;
mod cpu;
mod registers;
mod display;

use std::env;
use crate::hw::controller::MBC1;
use crate::hw::memory::Bus;
use crate::hw::memory::Memory;
use crate::cpu::Cpu;

use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

use std::{thread, time};

use std::io::Cursor;

use rgb::ComponentBytes;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let mut display = display::init_display(&mut events_loop);
    let program = display::create_program(&mut display);

    if std::env::args().len() != 2 {
        println!("Argument count is not 2!");
        std::process::exit(1);
    }

    let filename = env::args().nth(1).unwrap();

    let path = Path::new(&filename);

    let mut file = File::open(&path).unwrap();

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

        println!("0x{:04x?}", cpu.regs);
        cpu.execute_instr().unwrap();
        cpu.memory.io.lcd.tick_update();
        thread::sleep(time::Duration::from_millis(10));

        let lcd_vec = cpu.memory.io.lcd.lcd_display.as_bytes().to_vec();
        display::draw(&mut display, &program, lcd_vec, (160, 144));

        if emu_state == EmuState::Step {
            emu_state = EmuState::Paused;
        }
    }
}
