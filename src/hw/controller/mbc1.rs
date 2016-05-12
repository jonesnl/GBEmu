// TODO remove after implimented
#![allow(dead_code)]

use std::error::Error;

use game_data::{BusType, Cartridge};

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank_num: u8,
    ram: Vec<u8>,
    ram_enable: bool,
    ram_bank_num: u8, // also the upper bits of rom bank num 
                      // if mode select is 0
    mode_select: u8, // TODO make enum?
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> Box<Cartridge> {
        Box::new(MBC1 {
            rom: rom,
            rom_bank_num: 0,
            ram: Vec::new(),
            ram_enable: false,
            ram_bank_num: 0,
            mode_select: 0,
        })
    }
}

impl Cartridge for MBC1 {
    fn write8(&mut self, addr: BusType, data: u8) {
        // mapping stuff
    }

    fn read8(&self, addr: BusType) -> u8 {
        0
    }
}
