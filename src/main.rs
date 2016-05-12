mod game_data;
mod hw;

use std::env;
use hw::controller::MBC1;
use game_data::Cartridge;

use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;


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

    let new_cartridge: Box<Cartridge> = MBC1::new(rom);
        
    // In order to use boxes, need to make a GameData trait instead
}
