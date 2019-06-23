use crate::hw::lcd::LCD;
use crate::hw::memory::{Bus, BusWidth};

pub struct IO {
    ioram: Vec<u8>,
    pub lcd: LCD,
}

impl IO {
    pub fn new() -> IO {
        let mut io = IO {
            ioram: vec![0u8; 0x80], // FF00-FF7F
            lcd: LCD::new(),
        };
        io.ioram[0] = 0b0000_1111;
        io
    }

    // TODO joypad

    // TODO sound
}

impl Bus for IO {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        match addr {
            0x8000..=0x9FFF => {
                self.lcd.write8(addr, data);
            }
            0xFE00..=0xFE9F => {
                self.lcd.write8(addr, data);
            }
            0xFF00 => {
                // TODO joypad support
            }
            0xFF40..=0xFF4B => {
                self.lcd.write8(addr, data);
            }
            0xFF00..=0xFF7F => {
                self.ioram[(addr - 0xFF00) as usize] = data;
            }
            _ => {
                panic!("Unknown address: {}", addr);
            }
        }
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.lcd.read8(addr),
            0xFE00..=0xFE9F => self.lcd.read8(addr),
            0xFF40..=0xFF4B => self.lcd.read8(addr),
            0xFF00..=0xFF7F => self.ioram[(addr - 0xFF00) as usize],
            _ => {
                panic!("Unknown address: {}", addr);
            }
        }
    }
}
