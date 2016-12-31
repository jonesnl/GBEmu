pub type BusWidth = u16;

pub trait Bus {
    fn write8(&mut self, addr: BusWidth, data: u8);

    fn read8(&self, addr: BusWidth) -> u8;
    
    fn write16(&mut self, addr: BusWidth, data: u16);

    fn read16(&self, addr: BusWidth) -> u16;

    fn _write16_using_write8(&mut self, addr: BusWidth, data: u16) {
        self.write8(addr+1, (data >> 8) as u8);
        self.write8(addr, (data & 0xFF) as u8);
    }

    fn _read16_using_read8(&self, addr: BusWidth) -> u16 {
        let lower_byte = self.read8(addr) as u16;
        let upper_byte = self.read8(addr + 1) as u16;
        (upper_byte << 8) & lower_byte
    }
}

pub struct Memory {
    vram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    hram: Vec<u8>,
    cartridge: Box<Bus>,
}

impl Memory {
    pub fn new(cartridge: Box<Bus>) -> Memory {
        Memory {
            vram: vec![0u8; 8 * 1024],
            wram: vec![0u8; 8 * 1024],
            oam: vec![0u8; 100],
            hram: vec![0u8; 0x7E],
            cartridge: cartridge,
        }
    }
}

impl Bus for Memory {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        match addr {
            0x0000...0x7FFF => {
                (*self.cartridge).write8(addr, data);
            },
            0x8000...0x9FFF => {
                self.vram[(addr-0x8000) as usize] = data;
            },
            0xA000...0xBFFF => {
                (*self.cartridge).write8(addr, data);
            },
            0xC000...0xDFFF => {
                self.wram[(addr-0xC000) as usize] = data;
            },
            0xE000...0xFDFF => {
                self.wram[(addr-0xE000) as usize] = data;
            },
            0xFE00...0xFE9F => {
                self.oam[(addr-0xFE00) as usize] = data;
            },
            0xFEA0...0xFEFF => {
                panic!("Unusable memory address {}", addr);
            },
            0xFF00...0xFF7F => {
                // TODO I/O
            },
            0xFF80...0xFFFE => {
                self.hram[(addr-0xFF80) as usize] = data;
            },
            0xFFFF => {
                // TODO Interrupt enable register
            },
            _ => {
                panic!("Illegal write to {}", addr);
            }
        };
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        match addr {
            0x000...0x7FFF => {
                (*self.cartridge).read8(addr)
            },
            0x8000...0x9FFF => {
                self.vram[(addr-0x8000) as usize]
            },
            0xA000...0xBFFF => {
                (*self.cartridge).read8(addr)
            },
            0xC000...0xDFFF => {
                self.wram[(addr-0xC000) as usize]
            },
            0xE000...0xFDFF => {
                self.wram[(addr-0xE000) as usize]
            },
            0xFE00...0xFE9F => {
                self.oam[(addr-0xFE00) as usize]
            },
            0xFEA0...0xFEFF => {
                panic!("Unusable memory address {}", addr);
            },
            0xFF00...0xFF7F => {
                // TODO I/O
                0
            },
            0xFF80...0xFFFE => {
                self.hram[(addr-0xFF80) as usize]
            },
            0xFFFF => {
                // TODO Interrupt enable register
                0
            },
            _ => {
                panic!("Illegal write to {}", addr);
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
