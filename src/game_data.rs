pub type BusWidth = u16;

pub trait Bus {
    fn write8(&mut self, addr: BusWidth, data: u8);

    fn read8(&self, addr: BusWidth) -> u8;
    
    fn write16(&mut self, addr: BusWidth, data: u16) {
        self.write8(addr+1, (data >> 8) as u8);
        self.write8(addr, (data & 0xFF) as u8);
    }

    fn read16(&self, addr: BusWidth) -> u16 {
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
    fn new(cartridge: Box<Bus>) -> Memory {
        Memory {
            vram: vec![0u8; 8 * 1024],
            wram: vec![0u8; 8 * 1024],
            oam: vec![0u8; 100],
            hram: vec![0u8; 0x7E],
            cartridge: cartridge,
        }
    }
}
