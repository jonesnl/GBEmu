use crate::hw::io::IO;

pub type BusWidth = u16;

pub trait Bus {
    fn write8(&mut self, addr: BusWidth, data: u8);

    fn read8(&self, addr: BusWidth) -> u8;

    fn write16(&mut self, addr: BusWidth, data: u16) {
        self._write16_using_write8(addr, data);
    }

    fn read16(&self, addr: BusWidth) -> u16 {
        self._read16_using_read8(addr)
    }

    fn _write16_using_write8(&mut self, addr: BusWidth, data: u16) {
        self.write8(addr + 1, (data >> 8) as u8);
        self.write8(addr, (data & 0xFF) as u8);
    }

    fn _read16_using_read8(&self, addr: BusWidth) -> u16 {
        let lower_byte = self.read8(addr) as u16;
        let upper_byte = self.read8(addr + 1) as u16;
        (upper_byte << 8) | lower_byte
    }
}

pub struct Memory {
    wram: Vec<u8>,
    hram: Vec<u8>,
    pub cartridge: Box<dyn Bus>,
    pub io: IO,
}

impl Memory {
    pub fn new(cartridge: Box<dyn Bus>) -> Memory {
        Memory {
            wram: vec![0u8; 8 * 1024],
            hram: vec![0u8; 0x7F],
            cartridge: cartridge,
            io: IO::new(),
        }
    }

    fn dma_func(&mut self, data: u8) {
        let addr = (data as u16) << 8;
        for i in 0..0xa0 {
            self.write8(0xfe00 + i, self.read8(addr + i));
        }
    }
}

impl Bus for Memory {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        match addr {
            0x0000..=0x7FFF => {
                (*self.cartridge).write8(addr, data);
            }
            0x8000..=0x9FFF => {
                self.io.write8(addr, data);
            }
            0xA000..=0xBFFF => {
                (*self.cartridge).write8(addr, data);
            }
            0xC000..=0xDFFF => {
                self.wram[(addr - 0xC000) as usize] = data;
            }
            0xE000..=0xFDFF => {
                self.wram[(addr - 0xE000) as usize] = data;
            }
            0xFF46 => self.dma_func(data),
            0xFE00..=0xFE9F => {
                self.io.write8(addr, data);
            }
            0xFEA0..=0xFEFF => {
                //println!("Unusable memory address {:04x}", addr);
            }
            0xFF00..=0xFF7F => {
                self.io.write8(addr, data);
            }
            0xFF80..=0xFFFE => {
                self.hram[(addr - 0xFF80) as usize] = data;
            }
            0xFFFF => {
                // TODO Interrupt enable register
            }
        };
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        match addr {
            0x000..=0x7FFF => (*self.cartridge).read8(addr),
            0x8000..=0x9FFF => self.io.read8(addr),
            0xA000..=0xBFFF => (*self.cartridge).read8(addr),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize],
            0xFF46 => 0,
            0xFE00..=0xFE9F => self.io.read8(addr),
            0xFEA0..=0xFEFF => {
                // Unusable memory address, actually a mirror of other memory
                0
            }
            0xFF00..=0xFF7F => self.io.read8(addr),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => {
                // TODO Interrupt enable register
                0
            }
        }
    }

    fn write16(&mut self, addr: BusWidth, data: u16) {
        self._write16_using_write8(addr, data);
    }

    fn read16(&self, addr: BusWidth) -> u16 {
        self._read16_using_read8(addr)
    }
}
