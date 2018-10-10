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
    enum KeyPress {
        Empty,
        Pressed,
        Released,
    }
    
    let mut keypress = KeyPress::Empty;
    while !closed {
        let _v = vec![0x00, 0x00, 0xFF, 0x00,
                     0x00, 0xFF, 0x00, 0x00,
                     0xFF, 0x00, 0x00, 0x00,
                     0xFF, 0xFF, 0xFF, 0x00u8,
                     ];
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput {input, ..} => {
                        match (input.scancode, input.state) {
                            (0x39, glutin::ElementState::Pressed) => keypress = KeyPress::Pressed,
                            (0x39, glutin::ElementState::Released) => keypress = KeyPress::Released,
                            _ => keypress = KeyPress::Empty,
                        }
                    },
                    _ => return,
                },
                _ => return,
            }
        });
        if keypress == KeyPress::Released {
            println!("0x{:04x?}", cpu.regs);
            cpu.execute_instr().unwrap();
            cpu.memory.io.lcd.tick_update();
            keypress = KeyPress::Empty;
        }
        let lcd_vec = cpu.memory.io.lcd.lcd_display.as_bytes().to_vec();
        display::draw(&mut display, &program, lcd_vec, (160, 144));
    }

    /*
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

    let new_cartridge: Box<Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    let cpu = Cpu::new(new_memory);

    for x in 0x100..0x130 {
        if (x % 0x10) == 0 {println!("");}
        print!("{:0>2x} ", cpu.read8(x));
    }
    */
}
