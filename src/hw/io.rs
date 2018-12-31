use crate::hw::memory::{Bus, BusWidth};
use crate::hw::lcd::{LCD};

pub struct IO {
    ioram: Vec<u8>,
    pub lcd: LCD,
}

impl IO {
    pub fn new() -> IO {
        IO {
            ioram: vec![0u8; 0x80], // FF00-FF7F
            lcd: LCD::new()
        }
    }

    // TODO joypad

    // TODO sound
}

impl Bus for IO {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        match addr {
            0x8000..=0x9FFF => {
                self.lcd.write8(addr, data);
            },
            0xFF40..=0xFF4B => {
                self.lcd.write8(addr, data);
            },
            0xFF00..=0xFF7F => {
                self.ioram[(addr - 0xFF00) as usize] = data;
            },
            _ => {
                panic!("Unknown command");
            },
        }
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        match addr {
            0x8000..=0x9FFF => {
                self.lcd.read8(addr)
            },
            0xFF40..=0xFF4B => {
                self.lcd.read8(addr)
            },
            0xFF00..=0xFF7F => {
                self.ioram[(addr - 0xFF00) as usize]
            },
            _ => {
                self.ioram[addr as usize]
            },

        }
    }
}
