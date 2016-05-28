mod game_data;
mod hw;
mod cpu;

use std::env;
use hw::controller::MBC1;
use hw::memory::Bus;
use hw::memory::Memory;
use game_data::GameData;

use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

fn main() {
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

    let new_cartridge: Box<Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    let game_data = GameData::new(new_memory);

    for x in 0x100..0x130 {
        if (x % 0x10) == 0 {println!("");}
        print!("{:0>2x} ", game_data.memory.read8(x));
    }
}
