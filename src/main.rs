#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate glium;
extern crate image;

mod hw;
mod cpu;
mod registers;
mod display;

use std::env;
use hw::controller::MBC1;
use hw::memory::Bus;
use hw::memory::Memory;
use cpu::Cpu;

use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

use std::io::Cursor;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let mut display = display::init_display(&mut events_loop);
    let program = display::create_program(&mut display);

    let mut closed = false;
    while !closed {
        let v = vec![0x00, 0x00, 0xFF, 0x00,
                     0x00, 0xFF, 0x00, 0x00,
                     0xFF, 0x00, 0x00, 0x00,
                     0xFF, 0xFF, 0xFF, 0x00u8,
                     ];
        display::draw(&mut display, &program, v, (2, 2));
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
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
