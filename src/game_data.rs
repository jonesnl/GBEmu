pub type BusType = u16;

pub trait Cartridge {
    fn write8(&mut self, addr: BusType, data: u8);

    fn read8(&self, addr: BusType) -> u8;
    
    fn write16(&mut self, addr: BusType, data: u16) {
        self.write8(addr+1, (data >> 8) as u8);
        self.write8(addr, (data & 0xFF) as u8);
    }

    fn read16(&self, addr: BusType) -> u16 {
        let lower_byte = self.read8(addr) as u16;
        let upper_byte = self.read8(addr + 1) as u16;
        (upper_byte << 8) & lower_byte
    }
}

