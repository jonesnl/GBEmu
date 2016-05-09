use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

pub struct GameData {
}

impl GameData {    
    pub fn new() -> GameData {
        GameData {}
    }

    pub fn load_from_file(&mut self, name: &str) -> Result<(), Box<Error>> {
        let path = Path::new(name);

        let mut file = try!(File::open(&path));

        let mut rom_dump = Vec::new();
        try!(file.read_to_end(&mut rom_dump));

        // basic check to make sure its a GB rom. More checking
        // should be done with the checksum.
        assert_eq!(&rom_dump[0x104..0x134], &NINTENDO_BITS[..]);
        Ok(())
    }
}


const NINTENDO_BITS: [u8; 48] = [0xCE, 0xED, 0x66, 0x66, 
        0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00,
        0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89,
        0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9,
        0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
        0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];
